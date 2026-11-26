// SPDX-License-Identifier: MIT

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::mpsc;

use crate::types::{DuplicateScanMsg, DuplicateScanSummary};

static NEXT_DIR_ID: AtomicUsize = AtomicUsize::new(0);

struct TestDir(PathBuf);

impl TestDir {
    fn new(label: &str) -> Self {
        let id = NEXT_DIR_ID.fetch_add(1, Ordering::Relaxed);
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("target")
            .join(format!("dup-scan-test-{label}-{}-{id}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).unwrap();
        Self(root)
    }

    fn path(&self) -> &Path {
        &self.0
    }

    fn write(&self, rel: &str, bytes: &[u8]) -> PathBuf {
        let full = self.0.join(rel);
        if let Some(parent) = full.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(&full, bytes).unwrap();
        full
    }
}

impl Drop for TestDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn patterned_bytes(len: usize, seed: u8) -> Vec<u8> {
    (0..len).map(|i| seed.wrapping_add((i % 251) as u8)).collect()
}

fn run_scan(
    candidates: &[PathBuf],
    current_dir: Option<&Path>,
    generation: u64,
    cancelled: bool,
) -> (DuplicateScanSummary, Vec<DuplicateScanMsg>) {
    let cancel = AtomicBool::new(cancelled);
    let (tx, rx) = mpsc::channel();
    let summary = crate::scanner::find_exact_duplicates_streaming(
        candidates,
        current_dir,
        generation,
        &cancel,
        &tx,
    );
    let msgs: Vec<_> = rx.try_iter().collect();
    (summary, msgs)
}

#[test]
fn unique_file_sizes_produce_zero_groups() {
    let dir = TestDir::new("unique-sizes");
    let candidates = vec![
        dir.write("a.jpg", &[1u8; 10]),
        dir.write("b.jpg", &[1u8; 20]),
        dir.write("c.jpg", &[1u8; 30]),
    ];
    let (summary, _) = run_scan(&candidates, Some(dir.path()), 1, false);
    assert!(summary.groups.is_empty());
    assert_eq!(summary.total_candidates, 3);
    assert!(!summary.cancelled);
}

#[test]
fn same_size_differing_in_first_4kib_produce_zero_groups() {
    let dir = TestDir::new("diff-prefix");
    let bytes_a = patterned_bytes(8192, 1);
    let mut bytes_b = bytes_a.clone();
    bytes_b[128] ^= 0xff;
    let candidates = vec![dir.write("a.jpg", &bytes_a), dir.write("b.jpg", &bytes_b)];
    let (summary, _) = run_scan(&candidates, Some(dir.path()), 2, false);
    assert!(summary.groups.is_empty());
    assert_eq!(summary.total_candidates, 2);
}

#[test]
fn same_size_over_64kib_differing_at_70000_produce_zero_groups() {
    let dir = TestDir::new("diff-tail");
    let bytes_a = patterned_bytes(100 * 1024, 5);
    let mut bytes_b = bytes_a.clone();
    bytes_b[70_000] ^= 0xff;
    let candidates = vec![dir.write("a.jpg", &bytes_a), dir.write("b.jpg", &bytes_b)];
    let (summary, _) = run_scan(&candidates, Some(dir.path()), 3, false);
    assert!(summary.groups.is_empty());
    assert_eq!(summary.total_candidates, 2);
}

#[test]
fn identical_over_64kib_groups_picks_keep_and_streams_messages() {
    let dir = TestDir::new("identical-3");
    let payload = patterned_bytes(100 * 1024, 9);
    let sub_b = dir.write("sub/b.jpg", &payload);
    let root_z = dir.write("z.jpg", &payload);
    let root_a = dir.write("a.jpg", &payload);
    let candidates = vec![sub_b, root_z, root_a.clone()];
    let (summary, msgs) = run_scan(&candidates, Some(dir.path()), 42, false);
    assert_eq!(summary.groups.len(), 1);
    let group = &summary.groups[0];
    assert_eq!(group.byte_len, 100 * 1024);
    assert_eq!(group.members.len(), 3);
    assert_eq!(group.keep.len(), 1);
    assert!(group.keep.contains(&root_a));
    assert!(msgs
        .iter()
        .any(|m| matches!(m, DuplicateScanMsg::Progress { generation: 42, .. })));
    assert!(msgs.iter().any(|m| matches!(
        m,
        DuplicateScanMsg::Complete { generation: 42, summary: s } if s == &summary
    )));
}

#[test]
fn two_zero_byte_files_form_one_group() {
    let dir = TestDir::new("zero-bytes");
    let candidates = vec![dir.write("a.jpg", b""), dir.write("b.jpg", b"")];
    let (summary, _) = run_scan(&candidates, Some(dir.path()), 5, false);
    assert_eq!(summary.groups.len(), 1);
    assert_eq!(summary.groups[0].byte_len, 0);
    assert_eq!(summary.groups[0].members.len(), 2);
    assert_eq!(summary.groups[0].keep.len(), 1);
}

#[cfg(unix)]
#[test]
fn unix_hardlink_pair_forms_one_group() {
    let dir = TestDir::new("hardlink-pair");
    let orig = dir.write("orig.jpg", &patterned_bytes(1024, 3));
    let link = dir.path().join("link.jpg");
    fs::hard_link(&orig, &link).unwrap();
    let (summary, _) = run_scan(&[orig.clone(), link.clone()], Some(dir.path()), 6, false);
    assert_eq!(summary.groups.len(), 1);
    assert_eq!(summary.groups[0].byte_len, 1024);
    assert_eq!(summary.groups[0].members.len(), 2);
    assert!(summary.groups[0].members.contains(&orig));
    assert!(summary.groups[0].members.contains(&link));
}

#[cfg(unix)]
#[test]
fn unix_hardlink_pair_plus_differing_same_size_file_excludes_outlier() {
    let dir = TestDir::new("hardlink-mixed");
    let hl_a = dir.write("hl_a.jpg", &[1u8; 8192]);
    let hl_b = dir.path().join("hl_b.jpg");
    fs::hard_link(&hl_a, &hl_b).unwrap();
    let diff = dir.write("diff.jpg", &[2u8; 8192]);
    let (summary, _) = run_scan(&[hl_a.clone(), hl_b.clone(), diff], Some(dir.path()), 7, false);
    assert_eq!(summary.groups.len(), 1);
    assert_eq!(summary.groups[0].members.len(), 2);
    assert!(summary.groups[0].members.contains(&hl_a));
    assert!(summary.groups[0].members.contains(&hl_b));
}

#[test]
fn nonexistent_candidate_records_warning_and_keeps_valid_groups() {
    let dir = TestDir::new("missing-candidate");
    let missing = dir.path().join("missing.jpg");
    let dup_a = dir.write("dup_a.jpg", &[42u8; 256]);
    let dup_b = dir.write("dup_b.jpg", &[42u8; 256]);
    let (summary, _) = run_scan(&[missing, dup_a.clone(), dup_b.clone()], Some(dir.path()), 8, false);
    assert!(!summary.warnings.is_empty());
    assert_eq!(summary.groups.len(), 1);
    assert_eq!(summary.groups[0].members.len(), 2);
    assert!(summary.groups[0].members.contains(&dup_a));
    assert!(summary.groups[0].members.contains(&dup_b));
}

#[test]
fn quarantine_subfolder_excluded_unless_it_is_current_dir() {
    let dir = TestDir::new("quarantine-exclusion");
    let payload = patterned_bytes(512, 11);
    let root_a = dir.write("a.jpg", &payload);
    let q_a = dir.write("duplicates/a.jpg", &payload);
    let q_b = dir.write("duplicates/b.jpg", &payload);
    let (outer, _) = run_scan(&[root_a, q_a.clone()], Some(dir.path()), 9, false);
    assert!(outer.groups.is_empty());
    let q_dir = dir.path().join("duplicates");
    let (inner, _) = run_scan(&[q_a, q_b], Some(&q_dir), 10, false);
    assert_eq!(inner.groups.len(), 1);
    assert_eq!(inner.groups[0].members.len(), 2);
}

#[test]
fn pre_cancelled_scan_stops_cleanly_and_emits_complete() {
    let dir = TestDir::new("pre-cancelled");
    let payload = patterned_bytes(100 * 1024, 13);
    let candidates = vec![dir.write("a.jpg", &payload), dir.write("b.jpg", &payload)];
    let (summary, msgs) = run_scan(&candidates, Some(dir.path()), 99, true);
    assert!(summary.cancelled);
    assert!(summary.groups.is_empty());
    assert!(msgs
        .iter()
        .any(|m| matches!(m, DuplicateScanMsg::Complete { generation: 99, .. })));
}
