// SPDX-License-Identifier: MIT
//! Directory scanner (walkdir-based, with streaming scan support).

use std::collections::{BTreeMap, HashSet};
use std::fs::{self, File, Metadata};
use std::io::{self, Read, Seek, SeekFrom};
use std::os::fd::OwnedFd;
use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};
use walkdir::WalkDir;

use crate::types::{
    DuplicateGroup, DuplicateScanMsg, DuplicateScanSummary, FileStatus, ImageEntry, ScanInventory,
};

pub const MAGICK_IDENTIFY_CAP: usize = 500;
pub const SCAN_WARNING_CAP: usize = 50;
/// Emit partial scan updates every N native images so the list/feh can be used early.
pub const SCAN_PROGRESS_EVERY: usize = 200;
const DUP_PREFIX_BYTES: usize = 4096;
const DUP_CHUNK_BYTES: usize = 65_536;
const DUP_PROGRESS_EVERY: usize = 64;

/// Format a walkdir error for the activity log (feature 004 / 011).
pub fn format_walk_warning(err: &walkdir::Error) -> String {
    let path = err
        .path()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|| "(unknown path)".to_string());
    if err
        .io_error()
        .is_some_and(|io| io.kind() == io::ErrorKind::PermissionDenied)
    {
        format!("Permission denied, skipped: {path}")
    } else {
        format!("Scan skip: {err} ({path})")
    }
}

/// Cap warning volume; append summary when truncated (feature 004 edge case).
pub fn summarize_scan_warnings(mut warnings: Vec<String>) -> Vec<String> {
    if warnings.len() <= SCAN_WARNING_CAP {
        return warnings;
    }
    let omitted = warnings.len() - SCAN_WARNING_CAP;
    warnings.truncate(SCAN_WARNING_CAP);
    warnings.push(format!(
        "… {omitted} more scan warning(s) omitted (cap {SCAN_WARNING_CAP})"
    ));
    warnings
}

/// Result of scanning a directory (feature 005).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScanResult {
    pub entries: Vec<ImageEntry>,
    pub warnings: Vec<String>,
    pub inventory: ScanInventory,
}

/// Scan `dir` for images. `magick_identify` runs per-file ImageMagick probes (slow; off by default).
pub fn scan_images(dir: &Path, recursive: bool, magick_identify: bool) -> ScanResult {
    let cancel = Arc::new(AtomicBool::new(false));
    scan_images_streaming(dir, recursive, magick_identify, &cancel, |_, _, _| {})
}

struct ScanWalkState<'a> {
    entries: &'a mut Vec<ImageEntry>,
    non_image_skipped: &'a mut usize,
    magick_identify_calls: &'a mut usize,
    magick_truncated: &'a mut bool,
    since_last_emit: &'a mut usize,
    magick_bin: Option<&'a std::path::Path>,
    magick_identify: bool,
    cancel: &'a Arc<AtomicBool>,
}

fn emit_scan_partial(
    state: &ScanWalkState<'_>,
    on_partial: &mut impl FnMut(&[ImageEntry], usize, bool),
) {
    on_partial(
        state.entries.as_slice(),
        *state.non_image_skipped,
        *state.magick_truncated,
    );
}

fn record_native_image(
    path: std::path::PathBuf,
    state: &mut ScanWalkState<'_>,
    on_partial: &mut impl FnMut(&[ImageEntry], usize, bool),
) {
    state.entries.push(ImageEntry::new(path));
    *state.since_last_emit += 1;
    if *state.since_last_emit >= SCAN_PROGRESS_EVERY {
        *state.since_last_emit = 0;
        emit_scan_partial(state, on_partial);
    }
}

fn try_magick_probe(path: std::path::PathBuf, state: &mut ScanWalkState<'_>) -> bool {
    if !state.magick_identify || state.magick_bin.is_none() {
        return false;
    }
    if *state.magick_identify_calls >= MAGICK_IDENTIFY_CAP {
        *state.magick_truncated = true;
        return false;
    }
    if state.cancel.load(Ordering::Relaxed) {
        return false;
    }
    // Budget attempts, including failures, rather than only successful images.
    *state.magick_identify_calls += 1;
    if !is_magick_image(&path, state.magick_bin, state.cancel) {
        return false;
    }
    state
        .entries
        .push(ImageEntry::with_status(path, FileStatus::MagickDetected));
    *state.since_last_emit += 1;
    true
}

fn classify_walk_file(
    path: std::path::PathBuf,
    state: &mut ScanWalkState<'_>,
    on_partial: &mut impl FnMut(&[ImageEntry], usize, bool),
) {
    if is_native_image(&path) {
        record_native_image(path, state, on_partial);
        return;
    }
    if is_obvious_non_image(&path) {
        *state.non_image_skipped += 1;
        return;
    }
    if try_magick_probe(path, state) {
        return;
    }
    *state.non_image_skipped += 1;
}

fn resolve_magick_bin(magick_identify: bool) -> Option<std::path::PathBuf> {
    if !magick_identify {
        return None;
    }
    which::which("magick")
        .ok()
        .or_else(|| which::which("identify").ok())
        .or_else(|| which::which("convert").ok())
}

fn walk_scan_files(
    dir: &Path,
    recursive: bool,
    magick_identify: bool,
    magick_bin: Option<&Path>,
    cancel: &Arc<AtomicBool>,
    on_partial: &mut impl FnMut(&[ImageEntry], usize, bool),
) -> (Vec<ImageEntry>, Vec<String>, usize, bool) {
    let mut entries = Vec::new();
    let mut warnings = Vec::new();
    let mut non_image_skipped = 0usize;
    let mut magick_identify_calls = 0usize;
    let mut magick_truncated = false;
    let mut since_last_emit = 0usize;
    let walker = if recursive {
        WalkDir::new(dir).follow_links(false)
    } else {
        WalkDir::new(dir).follow_links(false).max_depth(1)
    };
    for entry in walker.into_iter() {
        if cancel.load(Ordering::Relaxed) {
            break;
        }
        match entry {
            Ok(e) if e.file_type().is_file() => {
                let mut state = ScanWalkState {
                    entries: &mut entries,
                    non_image_skipped: &mut non_image_skipped,
                    magick_identify_calls: &mut magick_identify_calls,
                    magick_truncated: &mut magick_truncated,
                    since_last_emit: &mut since_last_emit,
                    magick_bin,
                    magick_identify,
                    cancel,
                };
                classify_walk_file(e.path().to_path_buf(), &mut state, on_partial);
            }
            Ok(_) => {}
            Err(e) => warnings.push(format_walk_warning(&e)),
        }
    }
    (entries, warnings, non_image_skipped, magick_truncated)
}

/// Extension-first directory walk; optional incremental `on_partial` callbacks.
pub fn scan_images_streaming(
    dir: &Path,
    recursive: bool,
    magick_identify: bool,
    cancel: &Arc<AtomicBool>,
    mut on_partial: impl FnMut(&[ImageEntry], usize, bool),
) -> ScanResult {
    let magick_bin = resolve_magick_bin(magick_identify);
    let (mut entries, warnings, non_image_skipped, magick_truncated) =
        walk_scan_files(dir, recursive, magick_identify, magick_bin.as_deref(), cancel, &mut on_partial);
    entries.sort_by(|a, b| a.path.cmp(&b.path));
    let inventory = ScanInventory::from_entries(&entries, non_image_skipped, magick_truncated);
    ScanResult {
        entries,
        warnings: summarize_scan_warnings(warnings),
        inventory,
    }
}

fn is_native_image(path: &Path) -> bool {
    match path.extension().and_then(|e| e.to_str()) {
        Some(ext) => {
            let e = ext.to_lowercase();
            matches!(
                e.as_str(),
                "jpg" | "jpeg" | "png" | "webp" | "gif" | "bmp"
            )
        }
        None => false,
    }
}

/// Extensions that are never worth a per-file `magick identify` subprocess (videos, archives, etc.).
fn is_obvious_non_image(path: &Path) -> bool {
    match path.extension().and_then(|e| e.to_str()) {
        Some(ext) => {
            let e = ext.to_lowercase();
            matches!(
                e.as_str(),
                // video
                "mp4" | "mov" | "avi" | "mkv" | "webm" | "m4v" | "mpg" | "mpeg" | "wmv" | "flv"
                    | "vtx"
                    // audio
                    | "mp3" | "wav" | "flac" | "aac" | "ogg" | "m4a"
                    // archives / packages
                    | "zip" | "rar" | "7z" | "tar" | "gz" | "bz2" | "xz" | "deb" | "rpm"
                    // documents / data
                    | "pdf" | "doc" | "docx" | "xls" | "xlsx" | "ppt" | "pptx" | "txt" | "md"
                    | "db" | "sqlite" | "json" | "xml" | "html" | "css" | "js" | "rs" | "py"
            )
        }
        None => false,
    }
}

// Drain only a bounded chunk per poll: noisy output must not starve cancellation.
// A nonblocking socket also avoids waiting for descendants retaining stdout.
fn drain_probe_output(output: &mut UnixStream) -> io::Result<bool> {
    let mut seen = false;
    let mut buffer = [0; 4096];
    for _ in 0..16 {
        match output.read(&mut buffer) {
            Ok(0) => break,
            Ok(_) => seen = true,
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => break,
            Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        }
    }
    Ok(seen)
}

fn is_magick_image(path: &Path, magick_bin: Option<&Path>, cancel: &AtomicBool) -> bool {
    let Some(bin) = magick_bin else {
        return false;
    };
    let Ok(path) = std::path::absolute(path) else {
        return false;
    };
    let mut command = Command::new(bin);
    match bin.file_name().and_then(|name| name.to_str()) {
        Some("convert") => {
            command.arg("-ping").arg(path).args(["-format", "%m", "info:"]);
        }
        name => {
            if name == Some("magick") {
                command.arg("identify");
            }
            command.args(["-ping", "-format", "%m"]).arg(path);
        }
    }
    let Ok((mut output, writer)) = UnixStream::pair() else {
        return false;
    };
    if output.set_nonblocking(true).is_err() {
        return false;
    }
    let Ok(mut child) = command.stdin(Stdio::null()).stdout(Stdio::from(OwnedFd::from(writer)))
        .stderr(Stdio::null()).spawn() else {
        return false;
    };
    let mut has_format = false;
    loop {
        if cancel.load(Ordering::Relaxed) {
            let _ = child.kill();
            let _ = child.wait();
            return false;
        }
        match drain_probe_output(&mut output) {
            Ok(seen) => has_format |= seen,
            Err(_) => {
                let _ = child.kill();
                let _ = child.wait();
                return false;
            }
        }
        match child.try_wait() {
            Ok(Some(status)) => {
                // The child can emit its final bytes between the drain and wait.
                has_format |= drain_probe_output(&mut output).unwrap_or(false);
                return status.success() && has_format && !cancel.load(Ordering::Relaxed);
            }
            Ok(None) => std::thread::sleep(std::time::Duration::from_millis(2)),
            Err(_) => {
                let _ = child.kill();
                let _ = child.wait();
                return false;
            }
        }
    }
}

#[cfg(unix)]
type InodeKey = (u64, u64);
#[cfg(not(unix))]
type InodeKey = u64;

#[cfg(unix)]
fn inode_key(meta: &Metadata, _seq: &mut u64) -> InodeKey {
    use std::os::unix::fs::MetadataExt;
    (meta.dev(), meta.ino())
}

#[cfg(not(unix))]
fn inode_key(_meta: &Metadata, seq: &mut u64) -> InodeKey {
    let id = *seq;
    *seq += 1;
    id
}

fn filter_duplicate_candidates(candidates: &[PathBuf], current_dir: Option<&Path>) -> Vec<PathBuf> {
    let quarantine_dir = current_dir.map(|cur| cur.join("duplicates"));
    let mut seen = HashSet::with_capacity(candidates.len());
    let mut filtered = Vec::with_capacity(candidates.len());
    for path in candidates {
        if quarantine_dir.as_ref().is_some_and(|q| path.starts_with(q)) {
            continue;
        }
        if seen.insert(path.clone()) {
            filtered.push(path.clone());
        }
    }
    filtered
}

fn pick_default_keep(members: &[PathBuf], current_dir: Option<&Path>) -> HashSet<PathBuf> {
    let mut keep = HashSet::new();
    if let Some(best) = members.iter().min_by_key(|p| {
        let rel = current_dir.and_then(|r| p.strip_prefix(r).ok()).unwrap_or(p);
        (rel.components().count(), rel.as_os_str().len(), (*p).clone())
    }) {
        keep.insert(best.clone());
    }
    keep
}

fn build_duplicate_group(
    byte_len: u64,
    inode_classes: Vec<Vec<PathBuf>>,
    current_dir: Option<&Path>,
) -> DuplicateGroup {
    let mut members: Vec<PathBuf> = inode_classes.into_iter().flatten().collect();
    members.sort();
    let keep = pick_default_keep(&members, current_dir);
    DuplicateGroup {
        byte_len,
        members,
        keep,
    }
}

fn finish_duplicate_scan(
    mut summary: DuplicateScanSummary,
    generation: u64,
    tx: &mpsc::Sender<DuplicateScanMsg>,
    cancelled: bool,
) -> DuplicateScanSummary {
    if cancelled {
        summary.cancelled = true;
        summary.groups.clear();
    } else {
        summary
            .groups
            .sort_by(|a, b| (std::cmp::Reverse(a.byte_len), a.members.first()).cmp(&(
                std::cmp::Reverse(b.byte_len),
                b.members.first(),
            )));
    }
    summary.warnings = summarize_scan_warnings(summary.warnings);
    let _ = tx.send(DuplicateScanMsg::Complete {
        generation,
        summary: summary.clone(),
    });
    summary
}

fn read_chunk_at(path: &Path, offset: u64, max_len: usize) -> io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    if offset > 0 {
        file.seek(SeekFrom::Start(offset))?;
    }
    let mut buf = vec![0u8; max_len];
    let mut read_total = 0;
    while read_total < max_len {
        match file.read(&mut buf[read_total..]) {
            Ok(0) => break,
            Ok(n) => read_total += n,
            Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        }
    }
    buf.truncate(read_total);
    Ok(buf)
}

fn partition_by_chunk(
    inode_classes: Vec<Vec<PathBuf>>,
    offset: u64,
    chunk_size: usize,
    cancel: &AtomicBool,
    warnings: &mut Vec<String>,
) -> Result<Vec<Vec<Vec<PathBuf>>>, ()> {
    let mut sub_buckets: BTreeMap<Vec<u8>, Vec<Vec<PathBuf>>> = BTreeMap::new();
    for paths in inode_classes {
        if cancel.load(Ordering::Relaxed) {
            return Err(());
        }
        match read_chunk_at(&paths[0], offset, chunk_size) {
            Ok(chunk) if chunk.len() == chunk_size => {
                sub_buckets.entry(chunk).or_default().push(paths);
            }
            Ok(_) => warnings.push(format!(
                "Duplicate scan skip: {} (unexpected EOF)",
                paths[0].display()
            )),
            Err(e) => warnings.push(format!("Duplicate scan skip: {} ({e})", paths[0].display())),
        }
    }
    Ok(sub_buckets.into_values().collect())
}

fn collect_stage_a_buckets(
    candidates: &[PathBuf],
    generation: u64,
    cancel: &AtomicBool,
    tx: &mpsc::Sender<DuplicateScanMsg>,
    summary: &mut DuplicateScanSummary,
) -> Result<BTreeMap<u64, BTreeMap<InodeKey, Vec<PathBuf>>>, ()> {
    let mut size_buckets: BTreeMap<u64, BTreeMap<InodeKey, Vec<PathBuf>>> = BTreeMap::new();
    let mut seq = 0u64;
    for path in candidates {
        if cancel.load(Ordering::Relaxed) {
            return Err(());
        }
        match fs::symlink_metadata(path) {
            Ok(meta) if meta.file_type().is_file() => {
                let key = inode_key(&meta, &mut seq);
                size_buckets
                    .entry(meta.len())
                    .or_default()
                    .entry(key)
                    .or_default()
                    .push(path.clone());
            }
            Ok(_) => {}
            Err(e) => summary
                .warnings
                .push(format!("Duplicate scan skip: {} ({e})", path.display())),
        }
        summary.checked_files += 1;
        if summary.checked_files % DUP_PROGRESS_EVERY == 0
            || summary.checked_files == summary.total_candidates
        {
            let _ = tx.send(DuplicateScanMsg::Progress {
                generation,
                checked_files: summary.checked_files,
                total_files: summary.total_candidates,
            });
        }
    }
    Ok(size_buckets)
}

fn refine_bucket_from_offset(
    byte_len: u64,
    initial: Vec<Vec<PathBuf>>,
    current_dir: Option<&Path>,
    cancel: &AtomicBool,
    summary: &mut DuplicateScanSummary,
) -> Result<(), ()> {
    let mut pending = vec![initial];
    let mut offset = 0u64;
    while offset < byte_len && !pending.is_empty() {
        let max_chunk = if offset == 0 {
            DUP_PREFIX_BYTES
        } else {
            DUP_CHUNK_BYTES
        };
        let chunk_size = max_chunk.min((byte_len - offset) as usize);
        let next_offset = offset + chunk_size as u64;
        let mut next_pending = Vec::new();
        for class in pending {
            for sub in partition_by_chunk(class, offset, chunk_size, cancel, &mut summary.warnings)?
            {
                let count: usize = sub.iter().map(Vec::len).sum();
                if count < 2 {
                    continue;
                }
                if sub.len() == 1 || next_offset >= byte_len {
                    summary
                        .groups
                        .push(build_duplicate_group(byte_len, sub, current_dir));
                } else {
                    next_pending.push(sub);
                }
            }
        }
        pending = next_pending;
        offset = next_offset;
    }
    Ok(())
}

/// Find byte-identical files using 3-stage metadata, prefix, and 1-FD chunk reads.
pub fn find_exact_duplicates_streaming(
    candidates: &[PathBuf],
    current_dir: Option<&Path>,
    generation: u64,
    cancel: &AtomicBool,
    tx: &mpsc::Sender<DuplicateScanMsg>,
) -> DuplicateScanSummary {
    let filtered = filter_duplicate_candidates(candidates, current_dir);
    let mut summary = DuplicateScanSummary {
        total_candidates: filtered.len(),
        ..Default::default()
    };
    if cancel.load(Ordering::Relaxed) {
        return finish_duplicate_scan(summary, generation, tx, true);
    }
    let Ok(size_buckets) =
        collect_stage_a_buckets(&filtered, generation, cancel, tx, &mut summary)
    else {
        return finish_duplicate_scan(summary, generation, tx, true);
    };
    for (byte_len, inode_map) in size_buckets {
        let inode_classes: Vec<Vec<PathBuf>> = inode_map.into_values().collect();
        let total_paths: usize = inode_classes.iter().map(Vec::len).sum();
        if total_paths < 2 {
            continue;
        }
        if byte_len == 0 || inode_classes.len() == 1 {
            summary
                .groups
                .push(build_duplicate_group(byte_len, inode_classes, current_dir));
            continue;
        }
        if refine_bucket_from_offset(byte_len, inode_classes, current_dir, cancel, &mut summary)
            .is_err()
        {
            return finish_duplicate_scan(summary, generation, tx, true);
        }
    }
    finish_duplicate_scan(summary, generation, tx, false)
}

#[cfg(test)]
#[path = "../tests/unit/scanner_probes.rs"]
mod probe_tests;

#[cfg(test)]
#[path = "../tests/unit/duplicate_scan.rs"]
mod duplicate_scan;

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    fn scanner_test_dir(name: &str) -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("target")
            .join(name)
    }

    #[test]
    fn scan_nonexistent_dir_returns_empty() {
        let r = scan_images(Path::new("/nonexistent-rust-feh-test-dir"), false, false);
        assert!(r.entries.is_empty());
        assert!(
            r.warnings.iter().any(|w| w.starts_with("Scan skip:")),
            "expected scan skip warning, got: {:?}",
            r.warnings
        );
        assert_eq!(r.inventory.non_image_skipped, 0);
    }

    #[test]
    fn scan_skips_obvious_non_images_without_magick_probe() {
        let dir = scanner_test_dir(&format!("scanner-skip-video-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("photo.jpg"), b"x").unwrap();
        fs::write(dir.join("clip.mp4"), b"fake").unwrap();
        fs::write(dir.join("bundle.zip"), b"fake").unwrap();

        let r = scan_images(&dir, false, true);
        assert_eq!(r.entries.len(), 1);
        assert!(r.entries[0].path.ends_with("photo.jpg"));
        assert_eq!(r.inventory.non_image_skipped, 2);

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn scan_finds_supported_extension() {
        let dir = scanner_test_dir("rust-feh-scanner-test");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("photo.jpg"), b"x").unwrap();
        fs::write(dir.join("notes.txt"), b"x").unwrap();

        let r = scan_images(&dir, false, false);
        assert_eq!(r.entries.len(), 1);
        assert!(r.entries[0].path.ends_with("photo.jpg"));
        assert_eq!(r.inventory.native_listed, 1);
        assert_eq!(r.inventory.non_image_skipped, 1);

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn summarize_scan_warnings_truncates_with_summary() {
        let warnings: Vec<_> = (0..60).map(|i| format!("warn {i}")).collect();
        let out = summarize_scan_warnings(warnings);
        assert_eq!(out.len(), SCAN_WARNING_CAP + 1);
        assert!(out.last().unwrap().contains("10 more scan warning"));
    }

    #[test]
    fn summarize_scan_warnings_unchanged_under_cap() {
        let warnings = vec!["a".to_string(), "b".to_string()];
        let out = summarize_scan_warnings(warnings.clone());
        assert_eq!(out, warnings);
    }

    #[test]
    fn format_walk_warning_scan_skip_for_nonexistent_root() {
        let warning = WalkDir::new("/nonexistent-rust-feh-format-walk-test")
            .into_iter()
            .find_map(|e| e.err())
            .map(|e| format_walk_warning(&e))
            .expect("walkdir should error on missing root");
        assert!(warning.starts_with("Scan skip:"));
    }

    #[test]
    #[ignore = "manual: times full venice folder when present"]
    fn bench_venice_scan_skips_mp4_quickly() {
        use std::time::Instant;
        let dir = Path::new("/home/kkk/Desktop/venice");
        if !dir.is_dir() {
            return;
        }
        let start = Instant::now();
        let r = scan_images(dir, true, true);
        eprintln!(
            "venice: {} entries, {} skipped, {:?}",
            r.entries.len(),
            r.inventory.non_image_skipped,
            start.elapsed()
        );
        assert!(start.elapsed().as_secs() < 30, "venice scan should finish in <30s after mp4 skip");
    }

    #[cfg(unix)]
    #[test]
    fn format_walk_warning_permission_denied_prefix() {
        use std::os::unix::fs::PermissionsExt;

        let dir = scanner_test_dir("rust-feh-format-walk-perms");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let secret = dir.join("secret");
        fs::create_dir_all(&secret).unwrap();
        fs::set_permissions(&secret, fs::Permissions::from_mode(0o000)).unwrap();

        let result = scan_images(&dir, true, false);
        assert!(
            result
                .warnings
                .iter()
                .any(|w| w.starts_with("Permission denied")),
            "got: {:?}",
            result.warnings
        );

        fs::set_permissions(&secret, fs::Permissions::from_mode(0o755)).unwrap();
        let _ = fs::remove_dir_all(&dir);
    }
}