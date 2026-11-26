# Validation Results: Feature 020 - Exact Duplicate Detection and Safe Quarantine

**Date**: 2026-11-26

**Branch**: `20261126-152214-duplicate-detection`

**Architect Review**: `gemini-4-argon-high` (`SOUND_WITH_REFINEMENTS`, all eight refinements applied before task dispatch)

The follow-up Astra subagent reviewed the implementation at `6d95dad` and found no demonstrated feature defect that required expanding the scope. It identified gaps in the evidence for cross-filesystem moves, cancellation during active reads, and desktop interaction. A local cross-device probe subsequently covered the move through the production quarantine helper.

## Verification Commands and Results

| Command | Exit Code | Result |
|---------|-----------|--------|
| `RUSTFLAGS="-D warnings" cargo check --locked --all-targets` | `0` | Zero compiler errors or warnings across `lib`, `bin`, and all test targets. |
| `RUSTFLAGS="-D warnings" cargo test --locked --verbose` | `0` | Passed `258` tests (`0` failed, `3` ignored: `145` lib, `8` bin, `105` unit/integration/perf, including `33` in `unit_ui_logic`). |
| `cargo test --locked sc003_filter_10k_under_200ms -- --exact --nocapture` | `0` | Passed `1` test in `0.01s` (under the 200 ms budget). |
| `cargo build --locked --release` | `0` | Built the stripped LTO binary `target/release/rust-feh` (11 MiB). |
| `git diff --exit-code main -- Cargo.toml Cargo.lock .gitignore .envrc .github/` | `0` | Zero changes to dependencies, lockfile, ignore rules, env, or CI. |
| `cargo clippy --locked --all-targets -- -D warnings` | Not run | `clippy-driver` is absent on this host (`.agents/ENV.md`). `RUSTFLAGS="-D warnings"` ran locally, and CI runs `cargo clippy`. |

The follow-up run passed 258 tests with 3 ignored using `RUSTFLAGS="-D warnings" cargo test --offline --locked --verbose` and built the release binary with `cargo build --offline --locked --release`. Three tests in the first run could not write to the sandboxed home cache. They passed when rerun with access to that cache. The focused filter check passed in 0.01 s. Both Clippy and rustfmt were unavailable locally.

## Success Criteria Matrix

| ID | Criterion | Status | Evidence |
|----|-----------|--------|----------|
| SC-001 | Zero false positives or negatives on exact, differing, hardlink, and 0-byte fixtures | PASS (automated) | 10 unit tests in `tests/unit/duplicate_scan.rs`. |
| SC-002 | Preserve files across same-filesystem and cross-filesystem moves and name collisions | PASS (fixtures) | Seven quarantine integration tests cover guards, collisions and pruning. A separate local probe called `quarantine_group_extras` across different filesystem devices. Two same-named copies moved to distinct paths with exact bytes, and the Keep copy and existing destination file stayed intact. |
| SC-003 | Preserve initial folder listing and 10k filter speed | Filter passed, listing inspected | `sc003_filter_10k_under_200ms` finished in 0.01 s. The initial `scan_images_streaming` path was unchanged in the diff. No fresh listing benchmark was run. |
| SC-004 | Zero content reads for 10k size-unique files and at most one open file descriptor during reads | Code inspection | Stage A drops unique sizes before content reads. `read_chunk_at` opens and drops one file handle per read. The review did not instrument a 10k-file run. |
| SC-005 | `Cancel` and folder rescan stop reads within one 64 KiB chunk and drop stale generations | Pre-cancel test and code inspection | `pre_cancelled_scan_stops_cleanly_and_emits_complete` covers an already-cancelled scan. The code checks cancellation before each chunk and rejects stale generations. Active-read cancellation timing and interactive rescan were not measured. |
| SC-006 | Tests, Clippy and release build pass with zero manifest additions | Tests and build passed, local Clippy unavailable | 258 tests passed and the release build succeeded. `Cargo.toml` and `Cargo.lock` had no changes. |

## Security Audit (`T014`)

- **No permanent delete action (`FR-001`)**: The only new direct `remove_file` call removes the temporary `feh` filelist. Quarantine reuses the move helper, which removes the source on a cross-filesystem move after verifying the destination.
- **Subprocess safety**: `duplicate_group_spawn_command` (`src/ui_logic.rs:85-106`) and `open_duplicate_group_in_feh` (`src/main.rs:2179-2208`) pass discrete `Vec<String>` items to `Command::new("feh").args(&args)` with zero shell interpolation.
- **Symlink boundary**: `collect_stage_a_buckets` (`src/scanner.rs:479`) calls `std::fs::symlink_metadata` and skips non-regular files.
- **Quarantine safety**: `quarantine_group_extras` (`src/ui_logic.rs:1759-1796`) requires `group.keep.len() >= 1`, creates `dest_dir` with `create_dir_all`, rejects files already in `dest_dir`, and interleaves `plan_loss_proof_move` plus `execute_move_plan` per file.

## Closeout Sweep Dispositions

- **Fixed in Phase 8 (`T017`..`T021`)**:
  - `COMMENT-01`: Updated the `render_detached_inspector_windows` doc comment (`src/main.rs:3149-3158`) to describe 8 sections in STE prose.
  - `DUP-01`: Changed `default_keep_for_group` (`src/ui_logic.rs:1728-1730`) to call `pick_default_keep` (`src/scanner.rs:367-379`) without extra path clones.
  - `DRIFT-01`: Synced `specs/020-duplicate-detection/plan.md` signatures and added `format_byte_size_formats_zero_b_kib_and_mib` (`tests/unit/ui_logic.rs:559-565`) for `format_byte_size` (`src/ui_logic.rs:1459-1469`).
  - `DRIFT-02`: Documented the standalone `feh` round-trip note and duplicate row preview selection in `README.md`.
  - `DRIFT-04`: Synced `docs/UI-UX-REDESIGN.md`, `AGENTS.md`, `docs/PROJECT-HISTORY.md`, and `specs/020-duplicate-detection/validation-results.md`.
- **Retained by design (`DOCUMENT_ONLY`)**:
  - `DUP-03`: `quarantine_duplicate_group` (`src/main.rs:2210-2255`) calls `advance_stage_after_move` (`src/main.rs:2224`) after each moved file. This keeps inventory and selection consistent if a later move fails. Duplicate group sizes are small.
  - `DRIFT-03`: Stage A updates `checked_files / total_files` (`src/scanner.rs:494-503`) for each candidate batch. Stage B and Stage C keep the spinner active at `N / N` with per-64-KiB-chunk `AtomicBool` checks (`src/scanner.rs:449-452`).
- **Out of scope or untouched**:
  - `SCR-01` through `SCR-05`: Historical Feature 001/003 scripts, upstream Spec Kit templates, `.claude/` files under Issue `#162`, and pre-existing `#190` prose.
  - `DUP-02`: Two distinct 30-line `feh` launcher contracts remain separate.

## Limitations

- `cargo clippy` is not installed on this host (`.agents/ENV.md`). Local runs enforced `RUSTFLAGS="-D warnings"`.
- Automated tests cover scanner stages, UI state, `feh` argument vectors, and filesystem quarantine moves. The follow-up did not run an interactive desktop walkthrough of `Tools -> Duplicates…`.
- The shared `feh` filelist writes one displayed path per line. Filenames containing newlines or non-UTF-8 bytes may not round-trip through it.
