use super::*;

struct Fixture(PathBuf);

impl Fixture {
    fn new() -> Self {
        static NEXT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        let path = std::env::temp_dir().join(format!(
            "rust-feh-inventory-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir(&path).unwrap();
        Self(path)
    }

    fn image(&self, name: &str) -> PathBuf {
        let path = self.0.join(name);
        image::RgbaImage::from_pixel(12, 8, image::Rgba([25, 50, 75, 255]))
            .save(&path)
            .unwrap();
        path
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.0).unwrap();
    }
}

fn app(dir: &Path, paths: &[PathBuf]) -> RustFehApp {
    // Construct the actual production App without startup disk I/O or a display.
    let mut app = RustFehApp::new(
        String::new(),
        false,
        ToolCapabilities {
            feh_available: false,
            magick_available: false,
            magick_binary: None,
            magick_cache_available: false,
            magick_cache_ready: false,
        },
        false,
        false,
        (
            WindowPreferences::default(),
            FehLaunchList::default(),
            ActionPrefs::default(),
        ),
    );
    app.current_dir = Some(dir.to_path_buf());
    app.images = paths.iter().cloned().map(ImageEntry::new).collect();
    app.scan_inventory = Some(ScanInventory::from_entries(&app.images, 7, true));
    app
}

fn processed(source: &Path, dest: &Path) -> ProcessedResult {
    ProcessedResult {
        source_path: source.to_path_buf(),
        dest_path: dest.to_path_buf(),
        operation: ImageOperation::Convert {
            target_format: "png".into(),
            quality: None,
        },
        cache_iri: None,
        was_cache_hit: false,
        materialized_for_fast: None,
    }
}

fn render_tree(app: &mut RustFehApp) -> Vec<PathBuf> {
    let context = egui::Context::default();
    let root = app.current_dir.clone();
    let _ = context.run(egui::RawInput::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            app.render_tree_image_list(ui, root.as_deref(), 300.0, 20.0);
        });
    });
    app.tree_rows_cache
        .iter()
        .filter_map(|row| row.entry_index)
        .map(|index| app.images[index].path.clone())
        .collect()
}

fn assert_inventory(app: &RustFehApp) {
    assert_eq!(
        app.scan_inventory.as_ref().unwrap(),
        &ScanInventory::from_entries(&app.images, 7, true)
    );
}

#[test]
fn processed_add_refreshes_warm_list() {
    let f = Fixture::new();
    let source = f.image("a.png");
    let output = f.image("b.png");
    let mut app = app(&f.0, &[source.clone()]);
    assert_eq!(app.compute_list_indices(), (1, vec![0]));
    app.tools_on_processed(processed(&source, &output));
    assert_eq!(app.compute_list_indices(), (2, vec![0, 1]));
    assert_inventory(&app);
}

#[test]
fn processed_add_refreshes_actual_tree_frame() {
    let f = Fixture::new();
    let source = f.image("a.png");
    let output = f.image("b.png");
    let mut app = app(&f.0, &[source.clone()]);
    assert_eq!(render_tree(&mut app), vec![source.clone()]);
    app.tools_on_processed(processed(&source, &output));
    assert_eq!(render_tree(&mut app), vec![source, output]);
}

#[test]
fn optimized_add_refreshes_both_caches() {
    let f = Fixture::new();
    let source = f.image("a.png");
    let output = f.image("b.png");
    let mut app = app(&f.0, &[source.clone()]);
    assert_eq!(app.compute_list_indices(), (1, vec![0]));
    assert_eq!(render_tree(&mut app), vec![source.clone()]);
    app.tools_finish_prepare_fast(vec![output.clone()], f.0.clone());
    assert_eq!(app.compute_list_indices(), (2, vec![0, 1]));
    assert_eq!(render_tree(&mut app), vec![source, output]);
    assert_inventory(&app);
    assert_eq!(app.images[1].asset_status, AssetStatus::Optimized);
}

#[test]
fn derived_action_refreshes_list_tree_and_inventory() {
    let f = Fixture::new();
    let source = f.image("a.png");
    let output = f.0.join("b.png");
    let mut app = app(&f.0, &[source.clone()]);
    assert_eq!(app.compute_list_indices(), (1, vec![0]));
    assert_eq!(render_tree(&mut app), vec![source.clone()]);
    app.run_derived_action(
        ContextAction::ResizeCopy,
        &source,
        &ProcessOptions {
            width: Some(6),
            height: Some(4),
            output_path: Some(output.clone()),
            ..ProcessOptions::default()
        },
    );
    assert_eq!(image::image_dimensions(&output).unwrap(), (6, 4));
    assert_eq!(app.compute_list_indices(), (2, vec![0, 1]));
    assert_eq!(render_tree(&mut app), vec![source, output]);
    assert_inventory(&app);
}

#[test]
fn moved_selection_advances_and_inventory_matches_tree() {
    let f = Fixture::new();
    let first = f.image("a.png");
    let second = f.image("b.png");
    let mut app = app(&f.0, &[first.clone(), second.clone()]);
    app.selected = Some(first.clone());
    assert_eq!(render_tree(&mut app), vec![first.clone(), second.clone()]);
    app.advance_stage_after_move(&first);
    assert_eq!(app.selected, Some(second.clone()));
    assert_eq!(app.compute_list_indices(), (1, vec![0]));
    assert_eq!(render_tree(&mut app), vec![second]);
    assert_inventory(&app);
}

#[test]
fn moving_unselected_entry_preserves_selection_and_recounts_converted() {
    let f = Fixture::new();
    let first = f.image("a.png");
    let second = f.image("b.png");
    let mut app = app(&f.0, &[first.clone(), second.clone()]);
    app.images[1].status = rust_feh::types::FileStatus::Converted;
    app.scan_inventory = Some(ScanInventory::from_entries(&app.images, 7, true));
    app.selected = Some(first.clone());
    app.advance_stage_after_move(&second);
    assert_eq!(app.selected, Some(first));
    assert_inventory(&app);
}

#[test]
fn updating_existing_entry_invalidates_revision_without_duplicates() {
    let f = Fixture::new();
    let source = f.image("a.png");
    let mut app = app(&f.0, &[source.clone()]);
    app.images_revision = u64::MAX;
    app.compute_list_indices();
    render_tree(&mut app);
    app.tools_on_processed(processed(&source, &source));
    assert_eq!(app.images_revision, 0);
    assert_eq!(app.images.len(), 1);
    assert_eq!(app.images[0].asset_status, AssetStatus::Processed);
    assert_eq!(app.compute_list_indices(), (1, vec![0]));
    assert_eq!(render_tree(&mut app), vec![source]);
    assert_inventory(&app);
}

#[test]
fn moving_final_entry_clears_selection_and_counts() {
    let f = Fixture::new();
    let source = f.image("a.png");
    let mut app = app(&f.0, &[source.clone()]);
    app.selected = Some(source.clone());
    app.advance_stage_after_move(&source);
    assert_eq!(app.selected, None);
    assert_eq!(app.compute_list_indices(), (0, vec![]));
    assert!(render_tree(&mut app).is_empty());
    assert_inventory(&app);
}
