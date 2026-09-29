use super::*;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::sync::atomic::AtomicUsize;
use std::time::{Duration, Instant};

static NEXT_FIXTURE: AtomicUsize = AtomicUsize::new(0);

struct Fixture(PathBuf);

impl Fixture {
    fn new() -> Self {
        let id = NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!("rust-feh-probe-{}-{id}", std::process::id()));
        fs::create_dir(&root).unwrap();
        fs::create_dir(root.join("images")).unwrap();
        Self(root)
    }

    fn images(&self, count: usize) -> PathBuf {
        let images = self.0.join("images");
        for i in 0..count {
            fs::write(images.join(format!("{i:04}.tiff")), b"synthetic metadata").unwrap();
        }
        images
    }

    fn executable(&self, name: &str, body: &str) -> PathBuf {
        let bin = self.0.join(name);
        fs::write(&bin, body).unwrap();
        fs::set_permissions(&bin, fs::Permissions::from_mode(0o700)).unwrap();
        bin
    }

    fn scan(&self, count: usize, bin: Option<&Path>, enabled: bool) -> (usize, usize, bool) {
        let cancel = Arc::new(AtomicBool::new(false));
        let (entries, _, skipped, truncated) = walk_scan_files(
            &self.images(count), false, enabled, bin, &cancel, &mut |_, _, _| {},
        );
        (entries.len(), skipped, truncated)
    }

    fn calls(&self) -> usize {
        fs::read_to_string(self.0.join("calls")).unwrap_or_default().lines().count()
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).unwrap();
    }
}

fn counting_probe(f: &Fixture, outcome: &str) -> PathBuf {
    f.executable("magick", &format!(
        "#!/bin/sh\nprintf 'call\\n' >> '{}'\n{outcome}\n", f.0.join("calls").display(),
    ))
}

#[test]
fn failed_probes_respect_attempt_budget() {
    let f = Fixture::new();
    let bin = counting_probe(&f, "exit 1");
    assert_eq!(f.scan(507, Some(&bin), true), (0, 507, true));
    assert_eq!(f.calls(), MAGICK_IDENTIFY_CAP);
}

#[test]
fn successful_probes_keep_budget_and_classification() {
    let f = Fixture::new();
    let bin = counting_probe(&f, "printf TIFF");
    assert_eq!(f.scan(507, Some(&bin), true), (500, 7, true));
    assert_eq!(f.calls(), MAGICK_IDENTIFY_CAP);
}

#[test]
fn successful_exit_requires_format_output() {
    let f = Fixture::new();
    for outcome in ["exit 0", "printf TIFF >&2", "printf TIFF; exit 1"] {
        let bin = counting_probe(&f, outcome);
        assert_eq!(f.scan(1, Some(&bin), true), (0, 1, false), "{outcome}");
    }
    let bin = counting_probe(&f, "printf TIFF");
    assert_eq!(f.scan(1, Some(&bin), true), (1, 0, false));
}

#[test]
fn noisy_output_cannot_block_a_successful_probe() {
    let f = Fixture::new();
    let bin = f.executable("magick", "#!/usr/bin/python3\nimport os\nos.write(1,b'TIFF'*262144)\nos.write(2,b'e'*1048576)\n");
    let start = Instant::now();
    assert_eq!(f.scan(1, Some(&bin), true), (1, 0, false));
    assert!(start.elapsed() < Duration::from_secs(3));
}

#[test]
fn inherited_stdout_does_not_delay_probe_completion() {
    let f = Fixture::new();
    let bin = f.executable("magick", "#!/bin/sh\n/usr/bin/sleep 0.9 &\nprintf TIFF\n");
    let start = Instant::now();
    assert_eq!(f.scan(1, Some(&bin), true), (1, 0, false));
    assert!(start.elapsed() < Duration::from_millis(700));
}

#[test]
fn missing_and_disabled_probes_do_not_consume_budget() {
    let f = Fixture::new();
    let bin = counting_probe(&f, "printf TIFF");
    assert_eq!(f.scan(507, Some(&bin), false), (0, 507, false));
    assert_eq!(f.scan(507, None, true), (0, 507, false));
    assert_eq!(f.calls(), 0);
}

#[test]
fn cancelled_probe_is_reaped_without_admitting_result() {
    let f = Fixture::new();
    let bin = f.executable("magick", &format!(
        "#!/usr/bin/python3\nimport os,time\nopen({:?},'w').write(str(os.getpid()))\ntime.sleep(1.5)\nprint('TIFF')\n",
        f.0.join("pid").to_str().unwrap(),
    ));
    let images = f.images(2);
    let cancel = Arc::new(AtomicBool::new(false));
    let signal = Arc::clone(&cancel);
    let task = std::thread::spawn(move || {
        walk_scan_files(&images, false, true, Some(&bin), &signal, &mut |_, _, _| {})
    });
    let deadline = Instant::now() + Duration::from_secs(3);
    while !f.0.join("pid").exists() && Instant::now() < deadline {
        std::thread::sleep(Duration::from_millis(2));
    }
    assert!(f.0.join("pid").exists(), "probe child did not start");
    let start = Instant::now();
    cancel.store(true, Ordering::Relaxed);
    let (entries, _, _, _) = task.join().unwrap();
    assert!(start.elapsed() < Duration::from_millis(700), "cancel waited {:?}", start.elapsed());
    assert!(entries.is_empty(), "cancelled result was admitted");
    let pid = fs::read_to_string(f.0.join("pid")).unwrap();
    assert!(!Path::new("/proc").join(pid).exists(), "child was not reaped");
}

#[test]
fn magick_uses_identify_subcommand_and_literal_path() {
    let f = Fixture::new();
    let odd = f.0.join("images/space ; $(touch injected) ü.ppm");
    fs::write(&odd, b"P6\n1 1\n255\n\xff\x00\x00").unwrap();
    let bin = f.executable("magick", &format!(
        "#!/bin/sh\nprintf '%s\\n' \"$@\" > '{}'\nprintf PPM\n", f.0.join("args").display(),
    ));
    assert_eq!(f.scan(0, Some(&bin), true), (1, 0, false));
    let args = fs::read_to_string(f.0.join("args")).unwrap();
    let args: Vec<_> = args.lines().collect();
    assert_eq!(args, vec!["identify", "-ping", "-format", "%m", odd.to_str().unwrap()]);
    assert!(!f.0.join("images/injected").exists());
}

#[test]
fn identify_binary_omits_magick_subcommand() {
    let f = Fixture::new();
    let bin = f.executable("identify", &format!(
        "#!/bin/sh\nprintf '%s\\n' \"$@\" > '{}'\nprintf TIFF\n", f.0.join("args").display(),
    ));
    assert_eq!(f.scan(1, Some(&bin), true), (1, 0, false));
    let args = fs::read_to_string(f.0.join("args")).unwrap();
    assert_eq!(args.lines().next(), Some("-ping"));
}

#[test]
#[ignore = "requires installed ImageMagick; run explicitly in qualified environment"]
fn real_magick_classifies_valid_ppm() {
    let f = Fixture::new();
    fs::write(f.0.join("images/real.ppm"), b"P6\n1 1\n255\n\xff\x00\x00").unwrap();
    let bin = which::which("magick").expect("explicit test requires magick");
    assert_eq!(f.scan(0, Some(&bin), true), (1, 0, false));
}