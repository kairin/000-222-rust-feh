# Validation Results: Feature 020 - Exact Duplicate Detection and Safe Quarantine

**Date**: 2026-11-26

**Branch**: `20261126-152214-duplicate-detection`

**Architect Review**: `gemini-4-argon-high` (`SOUND_WITH_REFINEMENTS`, all eight refinements applied before task dispatch)

## Verification Commands and Results

| Command | Exit Code | Result |
|---------|-----------|--------|
| `RUSTFLAGS="-D warnings" cargo check --locked --all-targets` | `0` | Zero compiler errors or warnings across `lib`, `bin`, and all test targets. |
| `RUSTFLAGS="-D warnings" cargo test --locked --verbose` | `0` | Passed `257` tests (`0` failed, `3` ignored: `145` lib, `8` bin, `104` unit/integration/perf). |
| `cargo test --locked sc003_filter_10k_under_200ms -- --exact --nocapture` | `0` | Passed `1` test in `0.01s` (under the 200 ms budget). |
| `cargo build --locked --release` | `0` | Built the stripped LTO binary `target/release/rust-feh` (11 MiB). |
| `git diff --exit-code main -- Cargo.toml Cargo.lock .gitignore .envrc .github/` | `0` | Zero changes to dependencies, lockfile, ignore rules, env, or CI. |
| `cargo clippy --locked --all-targets -- -D warnings` | Not run | `clippy-driver` is absent on this host (`.agents/ENV.md`). `RUSTFLAGS="-D warnings"` ran locally, and CI runs `cargo clippy`. |

## Success Criteria Matrix

| ID | Criterion | Status | Evidence |
|----|-----------|--------|----------|
| SC-001 | Zero false positives or negatives on exact, differing, hardlink, and 0-byte fixtures | PASS (automated) | 10 unit tests in `tests/unit/duplicate_scan.rs`. |
| SC-002 | Zero data loss or overwrite across moves and same-basename collisions | PASS (automated) | 7 integration tests in `tests/integration/feature_016_actions.rs`. |
| SC-003 | Zero regression in initial folder listing or 10k filter speed | PASS (automated) | `scan_images_streaming` stays unchanged, and `sc003_filter_10k_under_200ms` finished in `0.01s`. |
| SC-004 | Zero content reads for size-unique files and 1 open FD max in Stage B/C | PASS (automated + audit) | `read_chunk_at` (`src/scanner.rs:418-435`) opens and drops one `File` handle at a time. Unique sizes drop at Stage A. |
| SC-005 | `Cancel` and folder rescan stop reads within one 64 KiB chunk and drop stale generations | PASS (automated + audit) | `pre_cancelled_scan_stops_cleanly_and_emits_complete` (`tests/unit/duplicate_scan.rs:201-211`) and `poll_duplicate_job` (`src/main.rs:2218-2240`). |
| SC-006 | Full test suite and release build pass with zero `Cargo.toml` additions | PASS (automated) | Passed `257` tests and release build with empty `Cargo.toml` and `Cargo.lock` diffs. |

## Security Audit (`T014`)

- **Zero user-file deletion (`FR-001`)**: `git diff main -- src/` adds only `remove_file(feh_duplicate_temp_path())` inside `Drop` (`filelist-dup-<pid>.txt` cache cleanup). Zero user images are deleted anywhere.
- **Subprocess safety**: `duplicate_group_spawn_command` (`src/ui_logic.rs:85-106`) and `open_duplicate_group_in_feh` (`src/main.rs:2101-2130`) pass discrete `Vec<String>` items to `Command::new("feh").args(&args)` with zero shell interpolation.
- **Symlink boundary**: `collect_stage_a_buckets` (`src/scanner.rs:476`) calls `std::fs::symlink_metadata` and skips non-regular files.
- **Quarantine safety**: `quarantine_group_extras` (`src/ui_logic.rs:1753-1790`) requires `group.keep.len() >= 1`, creates `dest_dir` with `create_dir_all`, rejects files already in `dest_dir`, and interleaves `plan_loss_proof_move` plus `execute_move_plan` per file.

## Limitations

- `cargo clippy` is not installed on this host (`.agents/ENV.md`). Local runs enforced `RUSTFLAGS="-D warnings"`.
- Automated tests cover scanner stages, UI state, `feh` argument vectors, and filesystem quarantine moves. Interactive desktop clicks on `Tools -> Duplicates…` remain available for manual review via `./target/release/rust-feh`.
