// SPDX-License-Identifier: MIT
//! Integration tests for feature 016: context-menu file actions
//! (save-copy, move, resize-copy, convert) without GUI.

use std::fs;
use std::path::PathBuf;

use rust_feh::image_proc::{process_image, ProcessOptions};
use rust_feh::ui_logic::{execute_move_plan, plan_loss_proof_move, save_copy_to};

/// Create a small test PNG with the given color.
fn make_test_png(dir: &std::path::Path, color: [u8; 4]) -> PathBuf {
    let path = dir.join("test.png");
    let img = image::RgbaImage::from_pixel(32, 32, image::Rgba(color));
    img.save(&path).unwrap();
    path
}

/// Create a larger test PNG for resize testing.
fn make_large_test_png(dir: &std::path::Path, color: [u8; 4]) -> PathBuf {
    let path = dir.join("large.png");
    let img = image::RgbaImage::from_pixel(200, 200, image::Rgba(color));
    img.save(&path).unwrap();
    path
}

#[test]
fn save_copy_basic_and_collision() {
    let dir = std::env::temp_dir().join(format!("rust-feh-016-save-copy-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();

    let src_dir = dir.join("src");
    let dest_dir = dir.join("dest");
    fs::create_dir_all(&src_dir).unwrap();
    fs::create_dir_all(&dest_dir).unwrap();

    let src = make_test_png(&src_dir, [255, 0, 0, 255]);
    let src_content = fs::read(&src).unwrap();

    // First save: should succeed
    let copy1 = save_copy_to(&src, &dest_dir).expect("first save_copy_to");
    assert!(copy1.is_file(), "first copy should exist");
    assert_eq!(
        fs::read(&copy1).unwrap(),
        src_content,
        "first copy content should match source"
    );
    assert!(src.is_file(), "source should still exist");

    // Second save to same dest (collision): should get a -1 suffix
    let copy2 = save_copy_to(&src, &dest_dir).expect("second save_copy_to");
    assert!(copy2.is_file(), "second copy should exist");
    assert_ne!(copy1, copy2, "second copy should have different path");
    assert!(
        copy2.to_string_lossy().contains("-1"),
        "suffix should be -1"
    );

    // First copy should be untouched
    assert!(copy1.is_file(), "first copy should still exist");
    assert_eq!(
        fs::read(&copy1).unwrap(),
        src_content,
        "first copy content should be unchanged"
    );

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn move_to_advances_and_is_loss_proof() {
    let dir = std::env::temp_dir().join(format!("rust-feh-016-move-basic-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();

    let src_dir = dir.join("src");
    let dest_dir = dir.join("dest");
    fs::create_dir_all(&src_dir).unwrap();
    fs::create_dir_all(&dest_dir).unwrap();

    let src = make_test_png(&src_dir, [0, 255, 0, 255]);
    let src_content_before = fs::read(&src).unwrap();

    // Plan and execute move
    let plan = plan_loss_proof_move(&src, &dest_dir).expect("plan_loss_proof_move");
    let moved = execute_move_plan(&plan).expect("execute_move_plan");

    // Verify: source is gone, destination exists with correct content
    assert!(!src.is_file(), "source should no longer exist");
    assert!(moved.is_file(), "moved file should exist");
    assert_eq!(
        fs::read(&moved).unwrap(),
        src_content_before,
        "moved file content should match original"
    );

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn move_to_duplicate_name_destination_gets_suffixed() {
    let dir = std::env::temp_dir().join(format!(
        "rust-feh-016-move-collision-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();

    let src_dir = dir.join("src");
    let dest_dir = dir.join("dest");
    fs::create_dir_all(&src_dir).unwrap();
    fs::create_dir_all(&dest_dir).unwrap();

    // Create source: red pixel
    let src = make_test_png(&src_dir, [255, 0, 0, 255]);
    let src_content = fs::read(&src).unwrap();

    // Create pre-existing file in dest with same name but different content
    let existing = dest_dir.join("test.png");
    let existing_img = image::RgbaImage::from_pixel(32, 32, image::Rgba([0, 0, 255, 255]));
    existing_img.save(&existing).unwrap();
    let existing_content = fs::read(&existing).unwrap();

    // Plan and execute move
    let plan = plan_loss_proof_move(&src, &dest_dir).expect("plan_loss_proof_move");
    let moved = execute_move_plan(&plan).expect("execute_move_plan");

    // Verify: source gone, pre-existing file untouched, moved file at -1 suffix
    assert!(!src.is_file(), "source should be gone");
    assert!(
        moved.to_string_lossy().contains("-1"),
        "moved file should have -1 suffix"
    );
    assert!(moved.is_file(), "moved file should exist at suffixed path");
    assert_eq!(
        fs::read(&moved).unwrap(),
        src_content,
        "moved file content should match source"
    );

    // Pre-existing should be exactly as it was
    assert!(existing.is_file(), "pre-existing file should still exist");
    assert_eq!(
        fs::read(&existing).unwrap(),
        existing_content,
        "pre-existing file should be untouched (SC-003: no overwrites)"
    );

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn resize_copy_produces_smaller_output_original_untouched() {
    let dir = std::env::temp_dir().join(format!("rust-feh-016-resize-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();

    let src = make_large_test_png(&dir, [0, 128, 255, 255]);
    let src_content_before = fs::read(&src).unwrap();
    let src_img = image::open(&src).unwrap();
    let orig_width = src_img.width();
    let orig_height = src_img.height();
    assert_eq!(orig_width, 200, "source should be 200x200");
    assert_eq!(orig_height, 200, "source should be 200x200");

    let output_path = dir.join("photo_resized.jpg");
    let opts = ProcessOptions {
        width: None,
        height: None,
        percent: Some(50.0),
        fit: None,
        filter: None,
        target_format: Some("jpg".into()),
        quality: Some(80),
        output_path: Some(output_path.clone()),
    };

    let result = process_image(&src, &opts).expect("process_image");
    assert!(result.is_file(), "output file should exist");
    assert_eq!(
        result, output_path,
        "output path should match requested output_path"
    );

    // Check output dimensions (50% of 200 = 100, with some rounding tolerance)
    let resized_img = image::open(&result).unwrap();
    let resized_width = resized_img.width();
    let resized_height = resized_img.height();
    assert!(
        (95..=105).contains(&resized_width),
        "resized width should be ~100, got {resized_width}"
    );
    assert!(
        (95..=105).contains(&resized_height),
        "resized height should be ~100, got {resized_height}"
    );

    // Source should be untouched
    assert_eq!(
        fs::read(&src).unwrap(),
        src_content_before,
        "source should be unchanged"
    );

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn convert_format_png_to_jpg() {
    let dir = std::env::temp_dir().join(format!("rust-feh-016-convert-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();

    let src = make_test_png(&dir, [128, 64, 192, 255]);
    let src_content_before = fs::read(&src).unwrap();

    let output_path = dir.join("photo.jpg");
    let opts = ProcessOptions {
        width: None,
        height: None,
        percent: None,
        fit: None,
        filter: None,
        target_format: Some("jpg".into()),
        quality: None,
        output_path: Some(output_path.clone()),
    };

    let result = process_image(&src, &opts).expect("process_image");
    assert!(result.is_file(), "output file should exist");
    assert_eq!(
        result, output_path,
        "output path should match requested output_path"
    );

    // Verify it's a valid image (can be opened)
    let converted = image::open(&result).expect("output should be decodable as image");
    assert!(
        converted.width() > 0,
        "converted image should have valid width"
    );
    assert!(
        converted.height() > 0,
        "converted image should have valid height"
    );

    // Source should be untouched
    assert_eq!(
        fs::read(&src).unwrap(),
        src_content_before,
        "source PNG should be unchanged"
    );

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn actions_work_with_spaces_and_non_ascii_names() {
    let dir = std::env::temp_dir().join(format!("rust-feh-016-unicode-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();

    // Test save_copy with spaces and non-ASCII
    let src_copy_dir = dir.join("src_copy");
    let dest_copy_dir = dir.join("dest_copy");
    fs::create_dir_all(&src_copy_dir).unwrap();
    fs::create_dir_all(&dest_copy_dir).unwrap();

    let src_copy_path = src_copy_dir.join("vacation café day.png");
    let img = image::RgbaImage::from_pixel(32, 32, image::Rgba([100, 150, 200, 255]));
    img.save(&src_copy_path).unwrap();
    let src_copy_content = fs::read(&src_copy_path).unwrap();

    let copy_result = save_copy_to(&src_copy_path, &dest_copy_dir).expect("save_copy_to");
    assert!(copy_result.is_file(), "copied file should exist");
    assert_eq!(
        fs::read(&copy_result).unwrap(),
        src_copy_content,
        "copied content should match"
    );

    // The filename should preserve the original name (possibly with suffix if collision)
    let copy_filename = copy_result
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap();
    assert!(
        copy_filename.starts_with("vacation café day"),
        "copied filename should preserve non-ASCII characters, got: {copy_filename}"
    );

    // Test move with spaces and non-ASCII
    let src_move_dir = dir.join("src_move");
    let dest_move_dir = dir.join("dest_move");
    fs::create_dir_all(&src_move_dir).unwrap();
    fs::create_dir_all(&dest_move_dir).unwrap();

    let src_move_path = src_move_dir.join("vacation café day.png");
    let img2 = image::RgbaImage::from_pixel(32, 32, image::Rgba([200, 150, 100, 255]));
    img2.save(&src_move_path).unwrap();
    let src_move_content = fs::read(&src_move_path).unwrap();

    let plan = plan_loss_proof_move(&src_move_path, &dest_move_dir).expect("plan_loss_proof_move");
    let moved = execute_move_plan(&plan).expect("execute_move_plan");

    // Source gone, destination exists with matching content
    assert!(!src_move_path.is_file(), "source should be gone after move");
    assert!(moved.is_file(), "moved file should exist");
    assert_eq!(
        fs::read(&moved).unwrap(),
        src_move_content,
        "moved content should match original"
    );

    // Filename should preserve non-ASCII
    let move_filename = moved
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap();
    assert!(
        move_filename.starts_with("vacation café day"),
        "moved filename should preserve non-ASCII characters, got: {move_filename}"
    );

    let _ = fs::remove_dir_all(&dir);
}

mod feature_020_duplicates {
    use std::collections::HashSet;
    use std::fs;
    use std::path::{Path, PathBuf};

    use rust_feh::types::{DuplicateGroup, QuarantineMode};
    use rust_feh::ui_logic::{
        prune_duplicate_groups_after_moves, quarantine_group_extras, resolve_quarantine_dir,
        toggle_group_keep,
    };

    fn setup_temp_dir(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("rust-feh-020-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn toggle_group_keep_enforces_at_least_one_keep() {
        let a = PathBuf::from("/tmp/a.jpg");
        let b = PathBuf::from("/tmp/b.jpg");
        let mut group = DuplicateGroup {
            byte_len: 10,
            members: vec![a.clone(), b.clone()],
            keep: HashSet::from([a.clone()]),
        };

        assert!(!toggle_group_keep(&mut group, &a));
        assert_eq!(group.keep.len(), 1);
        assert!(group.keep.contains(&a));

        assert!(toggle_group_keep(&mut group, &b));
        assert_eq!(group.keep.len(), 2);
        assert!(group.keep.contains(&b));

        assert!(toggle_group_keep(&mut group, &a));
        assert_eq!(group.keep.len(), 1);
        assert!(!group.keep.contains(&a));
        assert!(group.keep.contains(&b));
    }

    #[test]
    fn resolve_quarantine_dir_handles_default_and_custom() {
        let current = Path::new("/tmp/photos");
        assert_eq!(
            resolve_quarantine_dir(current, &QuarantineMode::DefaultSubfolder),
            current.join("duplicates")
        );
        let custom = PathBuf::from("/tmp/custom-quarantine");
        assert_eq!(
            resolve_quarantine_dir(current, &QuarantineMode::CustomDir(custom.clone())),
            custom
        );
    }

    #[test]
    fn quarantine_group_extras_creates_dir_and_handles_same_basename_collisions() {
        let dir = setup_temp_dir("same-basename");
        let keep_path = dir.join("keep.jpg");
        let sub1_dup = dir.join("sub1").join("dup.jpg");
        let sub2_dup = dir.join("sub2").join("dup.jpg");
        fs::create_dir_all(sub1_dup.parent().unwrap()).unwrap();
        fs::create_dir_all(sub2_dup.parent().unwrap()).unwrap();
        let bytes = b"identical-image-payload-020";
        fs::write(&keep_path, bytes).unwrap();
        fs::write(&sub1_dup, bytes).unwrap();
        fs::write(&sub2_dup, bytes).unwrap();

        let dest_dir = dir.join("duplicates");
        assert!(!dest_dir.exists());
        let group = DuplicateGroup {
            byte_len: bytes.len() as u64,
            members: vec![keep_path.clone(), sub1_dup.clone(), sub2_dup.clone()],
            keep: HashSet::from([keep_path.clone()]),
        };

        let results = quarantine_group_extras(&group, &dest_dir);
        assert!(dest_dir.is_dir());
        assert_eq!(results.len(), 2);
        assert_eq!(fs::read(&keep_path).unwrap(), bytes);
        assert!(!sub1_dup.exists() && !sub2_dup.exists());

        let mut moved_paths = HashSet::new();
        for (_, res) in results {
            let dest = res.expect("quarantine move should succeed");
            assert_eq!(dest.parent(), Some(dest_dir.as_path()));
            assert_eq!(fs::read(&dest).unwrap(), bytes);
            assert!(moved_paths.insert(dest));
        }
        assert_eq!(moved_paths.len(), 2);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn quarantine_group_extras_preserves_preexisting_file_in_dest_dir() {
        let dir = setup_temp_dir("preexisting-dest");
        let dest_dir = dir.join("duplicates");
        fs::create_dir_all(&dest_dir).unwrap();
        let preexisting = dest_dir.join("dup.jpg");
        let preexisting_bytes = b"preexisting-unrelated-bytes";
        fs::write(&preexisting, preexisting_bytes).unwrap();

        let keep_path = dir.join("keep.jpg");
        let sub1_dup = dir.join("sub1").join("dup.jpg");
        let sub2_dup = dir.join("sub2").join("dup.jpg");
        fs::create_dir_all(sub1_dup.parent().unwrap()).unwrap();
        fs::create_dir_all(sub2_dup.parent().unwrap()).unwrap();
        let bytes = b"duplicate-bytes";
        fs::write(&keep_path, bytes).unwrap();
        fs::write(&sub1_dup, bytes).unwrap();
        fs::write(&sub2_dup, bytes).unwrap();

        let group = DuplicateGroup {
            byte_len: bytes.len() as u64,
            members: vec![keep_path.clone(), sub1_dup, sub2_dup],
            keep: HashSet::from([keep_path]),
        };

        let results = quarantine_group_extras(&group, &dest_dir);
        assert_eq!(results.len(), 2);
        assert_eq!(fs::read(&preexisting).unwrap(), preexisting_bytes);
        for (_, res) in results {
            let dest = res.expect("suffixed quarantine move should succeed");
            assert_ne!(dest, preexisting);
            assert_eq!(fs::read(&dest).unwrap(), bytes);
        }
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn quarantine_group_extras_rejects_member_already_in_dest_dir() {
        let dir = setup_temp_dir("already-in-dest");
        let dest_dir = dir.join("duplicates");
        fs::create_dir_all(&dest_dir).unwrap();
        let keep_path = dir.join("keep.jpg");
        let in_dest = dest_dir.join("dup.jpg");
        let bytes = b"dup-bytes";
        fs::write(&keep_path, bytes).unwrap();
        fs::write(&in_dest, bytes).unwrap();

        let group = DuplicateGroup {
            byte_len: bytes.len() as u64,
            members: vec![keep_path.clone(), in_dest.clone()],
            keep: HashSet::from([keep_path]),
        };

        let results = quarantine_group_extras(&group, &dest_dir);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].0, in_dest);
        assert!(results[0].1.is_err());
        assert_eq!(fs::read(&in_dest).unwrap(), bytes);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn quarantine_group_extras_rejects_empty_keep_guard() {
        let dir = setup_temp_dir("empty-keep");
        let dest_dir = dir.join("duplicates");
        let a = dir.join("a.jpg");
        let b = dir.join("b.jpg");
        fs::write(&a, b"same").unwrap();
        fs::write(&b, b"same").unwrap();

        let group = DuplicateGroup {
            byte_len: 4,
            members: vec![a.clone(), b.clone()],
            keep: HashSet::new(),
        };

        let results = quarantine_group_extras(&group, &dest_dir);
        assert!(!results.is_empty());
        assert!(results.iter().all(|(_, res)| res.is_err()));
        assert!(a.is_file() && b.is_file());
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn prune_duplicate_groups_drops_resolved_and_reseeds_keep() {
        let root = PathBuf::from("/tmp/photos");
        let a = root.join("a.jpg");
        let b = root.join("sub/b.jpg");
        let c = root.join("c.jpg");
        let d = root.join("d.jpg");
        let e = root.join("nested/e.jpg");

        let mut groups = vec![
            DuplicateGroup {
                byte_len: 20,
                members: vec![a.clone(), b.clone()],
                keep: HashSet::from([a.clone()]),
            },
            DuplicateGroup {
                byte_len: 10,
                members: vec![c.clone(), d.clone(), e.clone()],
                keep: HashSet::from([c.clone()]),
            },
        ];
        let moved = HashSet::from([b, c]);

        prune_duplicate_groups_after_moves(&mut groups, &moved, Some(&root));
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].members, vec![d.clone(), e]);
        assert!(!groups[0].keep.is_empty());
        assert!(groups[0].keep.contains(&d));
    }
}
