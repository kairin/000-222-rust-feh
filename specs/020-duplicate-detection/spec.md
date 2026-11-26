# Feature Specification: Exact Duplicate Detection and Safe Quarantine

**Feature Directory**: `specs/020-duplicate-detection`

**Created**: 2026-11-26

**Status**: Approved for implementation (Argon system-architect reviewed)

**Design Authority**: `docs/UI-UX-REDESIGN.md` (`D10` through `D13`)

**Input**: Owner scope decision of 2026-11-26. Add v1 exact byte-duplicate detection with zero new crates. Include full-resolution verification in `feh` and collision-safe quarantine moves with zero file deletion.

## Why rust-feh owns this workflow

`feh` views and cycles an image list, but `feh` cannot find byte-identical files or move extra copies safely. `rust-feh` owns folder scanning, list filtering, `feh` `--filelist` orchestration, and loss-proof moves. `feh` stays the full-size viewer (Constitution Principle I).

## User Scenarios & Testing *(mandatory)*

### User Story 1 - On-demand exact duplicate scan (Priority: P1)

A user opens an image folder and wants to find byte-identical copies. Initial folder browsing stays fast. Once the folder scan finishes, the user opens `Tools -> Duplicates` and clicks `Scan for duplicates`. The app scans the loaded list in the background, shows progress and `Cancel`, and lists exact duplicate groups.

**Why this priority**: Exact duplicate discovery is the foundation. Verification and quarantine depend on accurate, cancellable groups.

**Independent Test**: Build a synthetic folder with unique files, same-size differing files, Unix hardlinks, and byte-identical copies. Run the duplicate scan. Confirm that only true byte-identical groups appear, unique-size files skip byte reads, open file descriptors stay bounded to one file at a time, and `Cancel` stops active work.

**Acceptance Scenarios**:

1. **Given** three identical JPEGs and two same-size JPEGs that differ at byte 8192, **When** `Scan for duplicates` runs, **Then** the app reports one 3-file group and excludes the differing pair.
2. **Given** a newly opened folder, **When** the folder loads, **Then** zero duplicate byte reads run until the user starts `Scan for duplicates`.
3. **Given** an active duplicate scan, **When** the user clicks `Cancel` or rescans a folder, **Then** the worker stops within one 64 KiB read and drops stale results.

---

### User Story 2 - Verify duplicate groups in feh and filter the browser (Priority: P2)

Before moving any file, the user inspects a duplicate group to choose which copy to keep. In `Duplicates`, each candidate shows its path, byte size, and available dimensions. Clicking `Open group in feh` cycles only that group at full size. Toggling `Duplicates only` restricts both Flat list and Folder tree views to duplicate entries.

**Why this priority**: The owner requires manual visual verification before any move or external deletion.

**Independent Test**: Create two duplicate groups and emit a group filelist. Confirm that `feh` arguments include only that group in deterministic order. Confirm that `Duplicates only` filters both `list_indices_with_duplicates` (Flat list) and `tree_visible_rows_with_duplicates` (Folder tree).

**Acceptance Scenarios**:

1. **Given** a 3-file duplicate group and installed `feh`, **When** the user clicks `Open group in feh`, **Then** `rust-feh` writes a 3-path `--filelist` and spawns `feh`.
2. **Given** duplicate results for the current `images_revision`, **When** the user enables `Duplicates only`, **Then** both Flat list and Folder tree show only unresolved duplicate files.
3. **Given** `feh` is absent from `PATH`, **When** the user views a group, **Then** `Open group in feh` disables with the missing-`feh` hint while scan and quarantine stay usable.

---

### User Story 3 - Quarantine extra copies with zero deletion (Priority: P3)

After verifying a group, the user moves extra copies into a quarantine folder. The default folder is `<current_dir>/duplicates`, or the user picks a custom folder. The main folder becomes clean while every file stays intact on disk for manual external deletion.

**Why this priority**: Completes cleanup while enforcing the owner rule against data loss and auto-deletion.

**Independent Test**: Mark one file `Keep` and two same-basename files from different subfolders (`a/dup.jpg`, `b/dup.jpg`) for move into `<current_dir>/duplicates`. Run `Move to quarantine`. Confirm that `duplicates/` is created, `Keep` stays untouched, moved files get distinct collision-safe names and verified byte lengths, and zero files delete.

**Acceptance Scenarios**:

1. **Given** a 2-file duplicate group, **When** the group loads, **Then** one file defaults to `Keep` (shortest relative path, then lexicographic path) and the UI blocks unchecking the last `Keep` file.
2. **Given** one `Keep` file and two `Move` files (`sub1/a.jpg`, `sub2/a.jpg`), **When** `Move to quarantine` runs, **Then** `rust-feh` creates the destination folder, plans and executes each file sequentially so neither move overwrites the other, logs `ActionOutcome` with `ActionKind::QuarantineDuplicate`, and prunes resolved groups.
3. **Given** zero duplicate groups remain after moves, **When** pruning completes, **Then** `Duplicates only` resets to `false` so the browser does not stay blank.

---

### Edge Cases

- **Zero-byte files**: Group 0-byte regular image files together with zero byte reads, and show `(0 B)`.
- **Unreadable or vanished files**: Skip files that fail `symlink_metadata` or `read`. Push a bounded warning to the activity log and continue scanning.
- **Symlinks and Unix hardlinks**: Keep `follow_links(false)` and check `symlink_metadata`. On Unix, group paths by `(st_dev, st_ino)`. Emit single-inode multi-path hardlink groups immediately with zero byte reads, and flatten multi-path inodes after byte comparison. Never pair an identical `PathBuf` with itself.
- **File descriptor cap**: Open, seek, read 64 KiB, and close one file at a time in Stage C so large buckets never hit `EMFILE`.
- **Quarantine loop and same-folder guard**: Exclude `<current_dir>/duplicates` from scan candidates unless `<current_dir>` is itself that folder. Reject moving any file whose parent directory already equals `dest_dir`.
- **Stale results and active directory scans**: Disable `Scan for duplicates` while `self.scanning` is true. Cancel and clear duplicate state inside `scan_directory` when a folder changes or rescans.
- **Pinned target safety**: Respect `PanelPin::Image` and `advance_stage_after_move` so moves never hijack a pinned preview or unrelated selection.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: `rust-feh` MUST NOT auto-delete any file or expose an in-app permanent delete button in v1.
- **FR-002**: `scan_images_streaming` MUST stay metadata-free. Duplicate scans MUST run only on explicit user demand when `self.scanning` is false.
- **FR-003**: Exact matching MUST use pure Rust `std` in three stages: `symlink_metadata` size + Unix `(dev, ino)` grouping, 4 KiB header slice, and 1-FD-at-a-time 64 KiB chunk reads.
- **FR-004**: The worker MUST check `Arc<AtomicBool>` between files and 64 KiB chunks, and send `DuplicateScanMsg` progress/completion over `std::sync::mpsc`.
- **FR-005**: `Tools -> Duplicates` MUST use `InspectorSection::Duplicates` in the existing detached-window map without adding a third pane.
- **FR-006**: Groups MUST sort deterministically (largest byte size first, then first path). Default `Keep` MUST pick the shortest relative path, then lexicographic `PathBuf`.
- **FR-007**: Pure helpers and UI controls MUST keep at least one `Keep` member per `DuplicateGroup`.
- **FR-008**: `Open group in feh` MUST write `filelist-dup-<pid>.txt` in `runtime_cache_dir()` and spawn `feh` via `Command` argument vectors without shell interpolation of image paths.
- **FR-009**: `Move to quarantine` MUST create `dest_dir` with `std::fs::create_dir_all`, reject candidates already inside `dest_dir`, and interleave `plan_loss_proof_move` plus `execute_move_plan` sequentially per file so same-basename duplicates never overwrite one another.
- **FR-010**: Quarantine moves MUST log `ActionOutcome` (`ActionKind::QuarantineDuplicate`), update `self.images` and `images_revision`, update `duplicate_results_revision`, preserve `PanelPin::Image`, and prune groups with fewer than two remaining files.
- **FR-011**: The browser bar MUST offer a `Duplicates only` toggle when active groups exist, updating both `ListIndexKey` (Flat list) and `TreeRowsKey` (Folder tree), and resetting to `false` when zero groups remain.
- **FR-012**: Business logic MUST stay in `types`, `scanner`, `ui_logic`, and `tool_caps` with zero `egui` imports. `src/main.rs` MUST keep functions at 50 lines or fewer.

### Key Entities

- **`DuplicateGroup`**: `pub struct DuplicateGroup { pub byte_len: u64, pub members: Vec<PathBuf>, pub keep: HashSet<PathBuf> }` (invariant: `1 <= keep.len() <= members.len()`).
- **`DuplicateScanSummary`**: `pub struct DuplicateScanSummary { pub total_candidates: usize, pub checked_files: usize, pub groups: Vec<DuplicateGroup>, pub warnings: Vec<String>, pub cancelled: bool }`.
- **`DuplicateScanMsg`**: `Progress { generation: u64, checked_files: usize, total_files: usize }` or `Complete { generation: u64, summary: DuplicateScanSummary }`.
- **`QuarantineMode`**: `DefaultSubfolder` (`<current_dir>/duplicates`) or `CustomDir(PathBuf)`.
- **`ActionKind::QuarantineDuplicate`**: Activity-log variant on `ActionKind` (keeping the six `ContextAction` variants unchanged).

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Zero false positives and zero false negatives across unique, same-size differing, byte-identical, Unix hardlink, and 0-byte test fixtures.
- **SC-002**: Zero data loss or overwrite across same-filesystem moves, cross-filesystem moves, and same-basename subfolder collisions.
- **SC-003**: `sc003_filter_10k_under_200ms` and initial folder listing speed do not regress.
- **SC-004**: On 10,000 size-unique files, the duplicate scan reads zero content bytes beyond `symlink_metadata` and holds at most one open file descriptor at any step.
- **SC-005**: `Cancel` or folder rescan stops worker reads within one 64 KiB chunk and drops stale `generation` messages.
- **SC-006**: `cargo test --locked`, `cargo clippy --locked --all-targets -- -D warnings`, and `cargo build --release` pass with zero `Cargo.toml` edits.

## Assumptions

- Visual similarity hashing (dHash for resized or re-encoded files) is deferred to phase 2. Version 1 matches exact byte-identical files only.
- The user deletes quarantined files manually outside `rust-feh` after visual verification.
