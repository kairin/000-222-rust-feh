use super::*;
use std::fs;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::MutexGuard;

struct Fixture {
    dir: PathBuf,
    old_path: Option<std::ffi::OsString>,
    _path_lock: MutexGuard<'static, ()>,
}

impl Fixture {
    fn new() -> Self {
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let path_lock = crate::test_support::PATH_LOCK.lock().unwrap();
        let dir = std::env::temp_dir().join(format!(
            "resize-safety-{}-{}", std::process::id(), NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&dir).unwrap();
        fs::create_dir(dir.join("bin")).unwrap();
        let old_path = std::env::var_os("PATH");
        std::env::set_var("PATH", dir.join("bin"));
        Self { dir, old_path, _path_lock: path_lock }
    }

    fn source(&self) -> PathBuf {
        let path = self.dir.join("source.png");
        image::RgbaImage::from_pixel(12, 8, image::Rgba([13, 25, 73, 127]))
            .save(&path).unwrap();
        path
    }

    fn opts(&self, path: &Path) -> ProcessOptions {
        ProcessOptions { output_path: Some(path.to_path_buf()), ..Default::default() }
    }

    fn failing_magick(&self) -> PathBuf {
        use std::os::unix::fs::PermissionsExt;
        let marker = self.dir.join("called");
        let script = self.dir.join("bin/magick");
        fs::write(&script, format!(
            "#!/bin/sh\nprintf called >> '{}'\nfor arg do last=\"$arg\"; done\nprintf damaged > \"$last\"\nexit 1\n",
            marker.display()
        )).unwrap();
        fs::set_permissions(script, fs::Permissions::from_mode(0o700)).unwrap();
        marker
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        match &self.old_path {
            Some(path) => std::env::set_var("PATH", path),
            None => std::env::remove_var("PATH"),
        }
        fs::remove_dir_all(&self.dir).unwrap();
    }
}

#[test]
fn invalid_percent_preserves_source_and_existing_output() {
    let f = Fixture::new();
    let src = f.source();
    let original = fs::read(&src).unwrap();
    for percent in [0.0, -10.0, f32::NAN, f32::NEG_INFINITY] {
        for same_path in [false, true] {
            let out = if same_path { src.clone() } else { f.dir.join("out.png") };
            fs::write(&out, &original).unwrap();
            let result = process_image(&src, &ProcessOptions { percent: Some(percent), ..f.opts(&out) });
            assert!(result.is_err(), "accepted {percent:?}");
            assert_eq!(fs::read(&out).unwrap(), original);
            assert_eq!(fs::read(&src).unwrap(), original);
        }
    }
}

#[test]
fn zero_dimensions_preserve_in_place_original() {
    let f = Fixture::new();
    let src = f.source();
    let original = fs::read(&src).unwrap();
    for (width, height) in [(Some(0), Some(0)), (Some(0), None), (None, Some(0))] {
        let result = process_image(&src, &ProcessOptions { width, height, ..f.opts(&src) });
        assert!(result.is_err());
        assert_eq!(fs::read(&src).unwrap(), original, "invalid resize modified original");
    }
}

#[test]
fn invalid_input_never_invokes_external_tool_or_creates_parent() {
    let f = Fixture::new();
    let marker = f.failing_magick();
    let src = f.source();
    let out = f.dir.join("not-created/out.png");
    // Start with a bounded case so the unfixed implementation fails the
    // assertion before infinity can trigger an enormous allocation.
    for percent in [f32::NAN, -2.0, 0.0, f32::INFINITY] {
        let _ = process_image(&src, &ProcessOptions { percent: Some(percent), ..f.opts(&out) });
        assert!(!marker.exists(), "invalid resize reached tool");
        assert!(!out.parent().unwrap().exists(), "invalid resize created directory");
    }
}

#[test]
fn failed_encoder_preserves_existing_destination() {
    let f = Fixture::new();
    let out = f.source();
    let original = fs::read(&out).unwrap();
    let image = DynamicImage::new_rgba8(0, 0);
    assert!(write_image(&image, &out, None, None).is_err());
    assert_eq!(fs::read(&out).unwrap(), original);
}

#[test]
fn failed_external_and_native_paths_preserve_output() {
    let f = Fixture::new();
    let marker = f.failing_magick();
    let source = f.dir.join("corrupt.png");
    fs::write(&source, b"not an image").unwrap();
    let out = f.source();
    let original = fs::read(&out).unwrap();
    assert!(process_image(&source, &f.opts(&out)).is_err());
    assert!(marker.exists(), "external failure not exercised");
    assert_eq!(fs::read(out).unwrap(), original);
    assert_eq!(fs::read(source).unwrap(), b"not an image");
}

#[test]
fn failed_external_in_place_does_not_destroy_fallback_input() {
    let f = Fixture::new();
    let marker = f.failing_magick();
    let src = f.source();
    process_image(&src, &ProcessOptions { percent: Some(50.0), ..f.opts(&src) }).unwrap();
    assert!(marker.exists());
    assert_eq!(image::image_dimensions(src).unwrap(), (6, 4));
}

#[test]
fn valid_native_output_retains_permissions_and_has_no_temporary_leak() {
    use std::os::unix::fs::PermissionsExt;
    let f = Fixture::new();
    let src = f.source();
    fs::set_permissions(&src, fs::Permissions::from_mode(0o640)).unwrap();
    process_image(&src, &ProcessOptions { width: Some(6), ..f.opts(&src) }).unwrap();
    assert_eq!(image::image_dimensions(&src).unwrap(), (6, 4));
    assert_eq!(fs::metadata(&src).unwrap().permissions().mode() & 0o777, 0o640);
    assert_eq!(fs::read_dir(&f.dir).unwrap().count(), 2);
}

#[test]
fn invalid_service_resize_preserves_prior_recovery_backup() {
    let f = Fixture::new();
    let source = f.source();
    let original = fs::read(&source).unwrap();
    let backup = f.dir.join("source.bak.png");
    fs::write(&backup, b"previous recovery bytes").unwrap();
    let op = ImageOperation::Resize {
        width: Some(0), height: None, percent: None, fit: None,
        filter: None, quality: None,
    };
    let result = ImageToolsService::new(None).process_single(
        &source, op, OutputPolicy::InPlaceWithBackup { backup_suffix: ".bak".into() });
    assert!(result.is_err());
    assert_eq!(fs::read(source).unwrap(), original);
    assert_eq!(fs::read(backup).unwrap(), b"previous recovery bytes");
}
