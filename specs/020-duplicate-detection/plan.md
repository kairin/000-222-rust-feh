# Implementation Plan: Exact Duplicate Detection and Safe Quarantine

**Branch**: `20261126-152214-duplicate-detection` | **Date**: 2026-11-26 | **Spec**: [spec.md](spec.md)

**Input**: `specs/020-duplicate-detection/spec.md` and `docs/UI-UX-REDESIGN.md` (`D10` through `D13`), refined by the `gemini-4-argon-high` system-architect review.

## Summary

Add on-demand exact duplicate detection, full-size group inspection in `feh`, a `Duplicates only` browser filter across Flat and Tree views, and sequential collision-safe quarantine moves. Enforce zero file deletion. Build the 3-stage matcher in `scanner.rs` with pure Rust `std` and 1-FD-at-a-time chunk reads. Put state, filter, filelist, and move helpers in `types.rs` and `ui_logic.rs`. Keep `main.rs` to thin `egui` and channel glue.

## Technical Context

**Language/Version**: Rust stable, edition 2021

**Primary Dependencies**: Existing `Cargo.toml` crates only (`eframe` 0.30, `rfd` 0.15, `walkdir` 2, `image` 0.25, `which` 6, `arboard` 3, `serde` 1, `serde_json` 1). **Zero new crates.**

**Storage**: Local filesystem only. Group filelists use `~/.cache/rust-feh/filelist-dup-<pid>.txt`. Custom destination folders reuse `ActionPrefs` (`~/.config/rust-feh/action-prefs.json`).

**Testing**: `cargo test --locked`, `cargo clippy --locked --all-targets -- -D warnings`, `cargo build --release`

**Target Platform**: Linux desktop (X11 and Wayland with `feh` subprocess handoff)

**Project Type**: Single native Rust desktop application (`lib` and `bin`)

**Performance Goals**: Zero extra I/O during initial folder listing. Zero byte reads for size-unique files and single-inode hardlink pairs. Maximum 1 open file descriptor in Stage B/C. Keep `sc003_filter_10k_under_200ms` under 200 ms and RSS under 150 MB.

**Constraints**: Zero file deletion (`FR-001`). Zero network access (`K8`). Functions of 50 lines or fewer (`K10`). Single sequential owner for `src/main.rs`. Zero `egui` types in core modules.

**Scale/Scope**: Folders of 10,000+ images across the five existing core modules plus `main.rs` and tests.

## Exact Matching Algorithm (Pure `std`, 1-FD Bounded, Hardlink-Safe)

1. **Candidate snapshot**: Deduplicate `PathBuf` items from loaded `ImageEntry` rows. Exclude paths inside `<current_dir>/duplicates` unless `<current_dir>` equals that folder.
2. **Stage A (size and inode buckets)**: Call `std::fs::symlink_metadata` without following symlinks. Skip non-regular or unreadable files and push warnings. Group paths by `byte_len`, and within each size group merge paths sharing Unix `(st_dev, st_ino)` into `(InodeKey, Vec<PathBuf>)` entries (on non-Unix, give each unique path its own `InodeKey`). Drop size buckets with total flattened `PathBuf` count `< 2`. Emit `DuplicateGroup` immediately with zero reads when `byte_len == 0` or when a bucket has `1` `InodeKey` with `>= 2` paths.
3. **Stage B (4 KiB prefix buckets)**: For each size bucket with `byte_len > 0` and `>= 2` `InodeKey` items, open, read at most 4096 bytes, and close one representative path (`paths[0]`) per `InodeKey`. Partition by exact `Vec<u8>` prefix. Drop sub-buckets whose flattened path count is `< 2`. Emit immediately if `byte_len <= 4096` or if a sub-bucket has `1` `InodeKey` with `>= 2` paths.
4. **Stage C (1-FD sequential 64 KiB chunk verification)**: For remaining sub-buckets (`byte_len > 4096` and `>= 2` `InodeKey` items), step through offsets `4096, 4096 + 65536, ... < byte_len`. At each offset and for each `InodeKey` representative: check `cancel.load(Ordering::Relaxed)`, open the file, `seek(SeekFrom::Start(offset))`, read up to 64 KiB, and close the file before the next open (holding at most 1 FD). Sub-partition by read bytes, dropping classes with flattened path count `< 2` and emitting single-inode multi-path classes immediately. At EOF, flatten all `Vec<PathBuf>` in each class and emit sorted `DuplicateGroup` items.

## Locked Cross-Module Signatures

- **`src/types.rs`**:
  - `pub struct DuplicateGroup { pub byte_len: u64, pub members: Vec<PathBuf>, pub keep: HashSet<PathBuf> }`
  - `pub struct DuplicateScanSummary { pub total_candidates: usize, pub checked_files: usize, pub groups: Vec<DuplicateGroup>, pub warnings: Vec<String>, pub cancelled: bool }`
  - `pub enum DuplicateScanMsg { Progress { generation: u64, checked_files: usize, total_files: usize }, Complete { generation: u64, summary: DuplicateScanSummary } }`
  - `#[derive(Default)] pub enum QuarantineMode { #[default] DefaultSubfolder, CustomDir(PathBuf) }`
  - `ActionKind::QuarantineDuplicate` (keeps `ContextAction` at its six staged-image variants).
- **`src/scanner.rs`**:
  - `pub fn find_exact_duplicates_streaming(candidates: &[PathBuf], current_dir: Option<&Path>, generation: u64, cancel: &AtomicBool, tx: &mpsc::Sender<DuplicateScanMsg>) -> DuplicateScanSummary`
- **`src/ui_logic.rs`**:
  - `InspectorSection::Duplicates` (`InspectorSection::ALL` grows from 7 to 8 entries)
  - `pub fn default_keep_for_group(members: &[PathBuf], root: Option<&Path>) -> HashSet<PathBuf>`
  - `pub fn toggle_group_keep(group: &mut DuplicateGroup, candidate: &Path) -> bool`
  - `pub fn active_duplicate_path_set(groups: &[DuplicateGroup]) -> HashSet<PathBuf>`
  - `pub fn list_indices_with_duplicates(images: &[ImageEntry], root: Option<&Path>, search: &str, sort: SortMode, duplicate_paths: Option<&HashSet<PathBuf>>) -> Vec<usize>`
  - `pub fn tree_visible_rows_with_duplicates(images: &[ImageEntry], root: Option<&Path>, search: &str, sort: SortMode, expanded: &HashSet<String>, root_skipped: usize, duplicate_paths: Option<&HashSet<PathBuf>>) -> Vec<TreeRow>`
  - `pub fn feh_duplicate_group_filelist_path(cache_dir: &Path, pid: u32) -> PathBuf`
  - `pub fn duplicate_group_spawn_command(filelist_path: &Path, members: &[PathBuf]) -> Result<(String, Vec<String>), String>`
  - `pub fn resolve_quarantine_dir(current_dir: &Path, mode: &QuarantineMode) -> PathBuf`
  - `pub fn quarantine_group_extras(group: &DuplicateGroup, dest_dir: &Path) -> Vec<(PathBuf, Result<PathBuf, String>)>`
  - `pub fn prune_duplicate_groups_after_moves(groups: &mut Vec<DuplicateGroup>, moved_sources: &HashSet<PathBuf>, root: Option<&Path>)`

## Constitution Check

*GATE: Verified by `gemini-4-argon-high` system-architect. Re-check before PR review.*

| Principle / Rule | Status | Alignment Notes |
|------------------|--------|-----------------|
| I. Thin-Wrapper Architecture | PASS | `feh` cannot detect or quarantine duplicates. `rust-feh` groups files and delegates full-size group viewing to `feh --filelist`. |
| II. Pure Rust, Minimal Dependencies | PASS | Uses `std::fs` and `std::io` only. Modifies neither `Cargo.toml` nor `Cargo.lock`. |
| III. Clean Module Separation | PASS | Adds no 6th module. Uses `types.rs`, `scanner.rs`, `ui_logic.rs`, and `tool_caps.rs`, with UI glue in `main.rs`. |
| IV. Linux-First, feh-Centric | PASS | Disables `Open group in feh` cleanly when `feh` is missing while keeping scan and quarantine usable. |
| V. Performance Awareness | PASS | Leaves `scan_images_streaming` untouched. Runs off-thread via `mpsc` and `AtomicBool` with 1 open FD max. |
| Security Checkpoint (`GUARDRAILS.md`) | PASS | Passes `Command` arg vectors without shell concatenation, uses `symlink_metadata`, creates `dest_dir` safely, and interleaves `plan_loss_proof_move` + `execute_move_plan` sequentially. Run `/security-review` before merge. |

## Source Seams Reference

- **Action log formatting**: `ActionKind` in `src/types.rs:298-301` and `action_kind_label` in `src/ui_logic.rs:1370-1380`.
- **Detached tool windows**: `InspectorSection` and `InspectorSection::ALL` in `src/ui_logic.rs:1633-1654`. Title/width match, window loop, and menu in `src/main.rs:2670-2765` and `src/main.rs:2845-2870`.
- **Flat and Tree caches**: `ListIndexKey` and `compute_list_indices` in `src/main.rs:370, 577-607`. `TreeRowsKey` and `tree_rows_cache` in `src/main.rs:386, 475-477, 3226-3251`. `list_indices` and `tree_visible_rows` in `src/ui_logic.rs:642, 955`.
- **Activity, rescan, and shutdown hooks**: `is_activity_busy` in `src/main.rs:879-881`, `scan_directory` in `src/main.rs:3879`, and `Drop for RustFehApp` in `src/main.rs:556-565`.
- **Loss-proof move and selection**: `plan_loss_proof_move` and `execute_move_plan` in `src/ui_logic.rs:1532-1610`. `advance_stage_after_move` in `src/main.rs:3663-3681`.
- **Integration test target**: Attach new integration tests as a submodule inside `tests/integration/feature_016_actions.rs` so `Cargo.toml` (`[[test]]` list at lines 40-74) stays untouched.

## Complexity Tracking

Zero constitution violations exist.
