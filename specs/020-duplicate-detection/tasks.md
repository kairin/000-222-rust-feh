# Tasks: Exact Duplicate Detection and Safe Quarantine (020)

**Input**: `specs/020-duplicate-detection/spec.md`, `specs/020-duplicate-detection/plan.md`, and `docs/UI-UX-REDESIGN.md` (`D10` through `D13`), refined by the `gemini-4-argon-high` system-architect audit.

**Prerequisites**: Read `.agents/CONSTITUTION.md`, `.agents/GUARDRAILS.md`, `spec.md`, and `plan.md` before any task.

**Orchestrator and Worker Rules**:
- Only the orchestrator edits `tasks.md` and commits green checkpoints on a `YYYYMMDD-HHMMSS-short-name` branch.
- Do not modify `Cargo.toml`, `Cargo.lock`, `.gitignore`, `.envrc`, or CI workflows. Zero new crates are approved.
- Keep business logic out of `src/main.rs`. Keep every new or edited function at 50 lines or fewer.
- Never implement file deletion anywhere (`FR-001`).
- Default worker budget: <= 3 files touched, timeout 300s, max 2 same-tier retries before Opus escalation.
- Never give two active workers write access to the same file (`FD-3`).

---

## Phase 1: Baseline and Setup

- [x] T001 [Role: Orchestrator | Write: none | Deps: none] Run `cargo test --locked` and `cargo clippy --locked --all-targets -- -D warnings` on the timestamped feature branch, and record baseline pass counts. Stop if baseline checks fail. *(Baseline: `cargo test --locked` passed across all targets — 128 lib, 2 ignored, plus all integration/unit/perf binaries; `cargo clippy` not installed locally per `.agents/ENV.md`.)*

---

## Phase 2: Foundational Domain Types (Blocking Prerequisite)

### Tests for Phase 2 (Write FIRST and confirm compile/test failure before T003)

- [x] T002 [Role: Worker (Haiku/Luna) | Write: `src/types.rs` | Deps: `T001`] Add unit tests in `src/types.rs` for `DuplicateGroup`, `DuplicateScanSummary`, `DuplicateScanMsg`, `QuarantineMode::default()`, and `ActionKind::QuarantineDuplicate` using the exact signatures in `plan.md`.

### Implementation for Phase 2

- [x] T003 [Role: Worker (Haiku/Luna) | Write: `src/types.rs`, `src/ui_logic.rs` | Deps: `T002`] Add `DuplicateGroup`, `DuplicateScanSummary`, `DuplicateScanMsg`, `QuarantineMode`, and `ActionKind::QuarantineDuplicate` in `src/types.rs`, plus the exhaustive match arm `ActionKind::QuarantineDuplicate => "Quarantine duplicate"` in `src/ui_logic.rs:1370-1380` (and matching `format_action_outcome` unit test). Verify: `cargo test --locked --lib types::tests` and `cargo test --locked --lib` pass.

**Checkpoint**: `cargo test --locked --lib` compiles and passes cleanly.

---

## Phase 3 & Phase 4: Parallel Core Streams (Disjoint Write Scopes)

After `T003` passes, the orchestrator may run **Stream 3A** (`T004 -> T005`), **Stream 3B** (`T006`), and **Stream 4** (`T007 -> T008`) in parallel git worktrees because their file sets do not overlap. Within Stream 3A and Stream 4, run test task before implementation task sequentially.

### Stream 3A: User Story 1 - Exact Duplicate Scanner Engine (Priority: P1, MVP)

- [x] T004 [Role: Worker (Haiku/Luna) | Write: `src/scanner.rs`, `tests/unit/duplicate_scan.rs` | Deps: `T003`] Wire `#[cfg(test)] #[path = "../tests/unit/duplicate_scan.rs"] mod duplicate_scan;` in `src/scanner.rs` and write failing unit tests covering: unique sizes skip byte reads, 4 KiB prefix mismatch, >64 KiB late-byte mismatch, 3-file exact group, 0-byte pair, standalone 1-inode 2-path Unix hardlink group, mixed hardlink + differing same-size file, unreadable file warning, `<current_dir>/duplicates` exclusion, and `AtomicBool` cancellation.
- [x] T005 [Role: Worker (Haiku/Luna) | Write: `src/scanner.rs` | Deps: `T004`] Implement `find_exact_duplicates_streaming` and <=50-line helpers in `src/scanner.rs`: Stage A `(InodeKey, Vec<PathBuf>)` size/inode buckets with immediate emit for 0-byte and 1-inode multi-path groups, Stage B 4 KiB representative reads, and Stage C 1-FD-at-a-time sequential 64 KiB `open`/`seek`/`read`/close checks with `Arc<AtomicBool>` cancellation. Verify: `cargo test --locked --lib duplicate_scan` passes. Stop/escalate to Opus if 2 attempts fail or any function exceeds 50 lines.

### Stream 3B: User Story 1 - Tool Capability Timing

- [x] T006 [P] [Role: Worker (Haiku/Luna) | Write: `src/tool_caps.rs` | Deps: `T003`] Add an `OperationTiming` row for on-demand exact duplicate scanning in `src/tool_caps.rs` (`Handler::RustFeh`, `SpeedTier::Medium`) and update `src/tool_caps.rs` unit tests. Verify: `cargo test --locked --lib tool_caps::tests` passes.

### Stream 4: User Story 2 - Pure Filter, Keep Defaults, and feh Group Command (Priority: P2)

- [x] T007 [Role: Worker (Haiku/Luna) | Write: `tests/unit/ui_logic.rs` | Deps: `T003`] Add failing unit tests in `tests/unit/ui_logic.rs` for: `InspectorSection::Duplicates` in `InspectorSection::ALL`, group sorting and `default_keep_for_group`, `active_duplicate_path_set`, `list_indices_with_duplicates` (Flat list), `tree_visible_rows_with_duplicates` (Folder tree), `feh_duplicate_group_filelist_path`, and `duplicate_group_spawn_command` arg vectors.
- [x] T008 [Role: Worker (Haiku/Luna) | Write: `src/ui_logic.rs`, `src/main.rs` | Deps: `T007`] Implement `InspectorSection::Duplicates` (`InspectorSection::ALL` len 8), `default_keep_for_group`, `active_duplicate_path_set`, `list_indices_with_duplicates`, `tree_visible_rows_with_duplicates`, `feh_duplicate_group_filelist_path`, and `duplicate_group_spawn_command` in `src/ui_logic.rs`, plus the 2 mandatory stub arms in `src/main.rs:2678` (`InspectorSection::Duplicates => ("Duplicates", 480.0),`) and `src/main.rs:2759` (`InspectorSection::Duplicates => {},`). Verify: `cargo test --locked --test unit_ui_logic` passes.

**Checkpoint**: Merge Streams 3A, 3B, and 4. Confirm `cargo test --locked` and `cargo clippy --locked --all-targets -- -D warnings` pass.

---

## Phase 5: User Story 3 - Sequential Loss-Proof Quarantine Moves (Priority: P3)

### Tests for User Story 3 (Write FIRST and confirm failure before T010)

- [x] T009 [Role: Worker (Haiku/Luna) | Write: `tests/integration/feature_016_actions.rs` | Deps: `T005`, `T008`] Add a `feature_020_duplicates` test module inside `tests/integration/feature_016_actions.rs` (leaving `Cargo.toml` untouched) testing: refusal to uncheck the sole `Keep` path, `create_dir_all` creation of `<current_dir>/duplicates`, sequential move of two same-basename files from different subfolders (`sub1/dup.jpg`, `sub2/dup.jpg`) producing distinct suffixed files with identical bytes, rejection of files already inside `dest_dir`, group pruning, and zero deleted files.

### Implementation for User Story 3

- [x] T010 [Role: Worker (Haiku/Luna) | Write: `src/ui_logic.rs` | Deps: `T009`] Implement `toggle_group_keep` (`keep.len() >= 1`), `resolve_quarantine_dir`, `quarantine_group_extras` (`create_dir_all` once, reject `parent == Some(dest_dir)`, and interleave `plan_loss_proof_move` + `execute_move_plan` sequentially per file), and `prune_duplicate_groups_after_moves` in `src/ui_logic.rs`. Verify: `cargo test --locked --test feature_016_actions` passes.

**Checkpoint**: All library, unit, and integration suites pass before GUI wiring.

---

## Phase 6: Thin GUI Integration in `src/main.rs` (Single Sequential Owner)

- [ ] T011 [Role: Worker (Haiku/Luna) | Write: `src/main.rs` | Deps: `T006`, `T010`] Add `ActiveDuplicateJob` and duplicate state fields, `Tools -> Duplicates…` menu item (`render_tools_menu`), `InspectorSection::Duplicates` window body (`Scan for duplicates` disabled while `self.scanning`, progress label, `Cancel`), `is_activity_busy()` and 100ms repaint hook, `scan_directory()` cancel/clear hook, and `Drop` cleanup (`cancel` + remove `filelist-dup-<pid>.txt`) in `src/main.rs` (all functions <= 50 lines). Verify: `cargo check --all-targets` and `cargo clippy --locked --all-targets -- -D warnings` pass.
- [ ] T012 [Role: Worker (Haiku/Luna) | Write: `src/main.rs` | Deps: `T011`] Wire the `Duplicates only` toggle in the browser bar into both `ListIndexKey`/`compute_list_indices` (`src/main.rs:370, 577`) and `TreeRowsKey`/`tree_visible_rows_with_duplicates` (`src/main.rs:386, 3226-3251`), and wire `Open group in feh` (disabled with hint when `feh` is missing) plus candidate byte-size and dimension labels in `src/main.rs`. Verify: `cargo test --locked` and `cargo clippy --locked --all-targets -- -D warnings` pass.
- [ ] T013 [Role: Worker (Haiku/Luna) | Write: `src/main.rs` | Deps: `T012`] Wire `Keep` checkboxes, quarantine target selector (`<folder>/duplicates` vs `Choose folder…` via `pick_action_destination`), and `Move to quarantine` (`quarantine_group_extras`, `advance_stage_after_move`, `record_action_outcome` with `ActionKind::QuarantineDuplicate`, `prune_duplicate_groups_after_moves`, updating `duplicate_results_revision` to `self.images_revision`, and resetting `duplicates_only = false` when zero groups remain) in `src/main.rs`. Verify: `cargo test --locked` and `cargo clippy --locked --all-targets -- -D warnings` pass.

**Checkpoint**: Full binary and test suite compile and pass with zero warnings.

---

## Phase 7: Security Audit, Docs, and Final Gate

- [ ] T014 [Role: Orchestrator | Write: none | Deps: `T013`] Run the `.agents/GUARDRAILS.md` security audit on `git diff`: confirm `Command` arg vectors have zero shell interpolation of image paths, `symlink_metadata` blocks symlink traversal, `quarantine_group_extras` runs sequentially with `create_dir_all`, and zero `remove_file` calls exist outside `execute_move_plan` and `filelist-dup-<pid>.txt` temp cleanup.
- [ ] T015 [P] [Role: Worker (Haiku/Luna) | Write: `README.md`, `CHANGELOG.md` | Deps: `T013`] Document `Tools -> Duplicates`, `Duplicates only`, `Open group in feh`, and the zero-deletion quarantine workflow in `README.md` and `CHANGELOG.md` (lint prose with `ste-lint.py`).
- [ ] T016 [Role: Orchestrator | Write: `specs/020-duplicate-detection/validation-results.md` | Deps: `T014`, `T015`] Run `cargo test --locked --verbose`, `cargo clippy --locked --all-targets -- -D warnings`, `cargo test sc003_filter_10k_under_200ms -- --exact --nocapture`, `cargo build --release`, and `git diff --exit-code Cargo.toml Cargo.lock .gitignore .envrc .github/`. Record command outputs in `specs/020-duplicate-detection/validation-results.md`.

---

## Dependencies and Execution Order

1. `T001` -> `T002` -> `T003` (Sequential foundation).
2. Parallel worktree fan-out after `T003` (max 3 concurrent workers):
   - **Stream 3A**: `T004` -> `T005` (`src/scanner.rs`, `tests/unit/duplicate_scan.rs`)
   - **Stream 3B**: `T006` (`src/tool_caps.rs`)
   - **Stream 4**: `T007` -> `T008` (`tests/unit/ui_logic.rs`, `src/ui_logic.rs`, 2 stub lines in `src/main.rs`)
3. `T009` -> `T010` (After `T005` and `T008` merge).
4. `T011` -> `T012` -> `T013` (Single sequential `src/main.rs` owner after `T006` and `T010`).
5. `T014` and `T015` (Parallel after `T013`), then `T016` (Final gate).
