# 019 Validation Results

## U0 pre-U1 baseline

- Command: `cargo test --verbose` in the configured environment, run outside the restricted sandbox so existing filesystem tests could write their handoff/cache files.
- Result: passed across all targets; library 139 passed, 0 failed, 2 ignored; binary 8 passed; three ignored tests across the suite. No product or test changes were needed.
- Command: `cargo test sc003_filter_10k_under_200ms -- --exact --nocapture`.
- Result: one 10k filter test passed. This is an automated filter check, not a numeric user-visible scroll baseline.
- Baseline GUI: `evidence/baseline-large-window.png`, captured at the Large preset (1280×960 content, 1330×1047 including decorations). The Open action was hidden inside Browse/Details; center was mostly empty with a false Loading status. GUI process exited cleanly.
- Release build: baseline GUI was captured from the pre-U1 release binary. A same-fixture pre-U1 run sampled resident memory three times at 108,664 KiB (~106.1 MiB); workload was identical invalid JPEGs, so it does not represent image preview decoding. Root orchestrator holds raw run context.

## U1 implementation checkpoint

- `cargo check --bin rust-feh`: passed after removing retired fixed-width Inspector sizing code.
- `cargo test --test unit_ui_logic --test feature_001_validation --test feature_005_list`: passed (45 passed, 1 ignored), including the new Idle preview message regression and existing list/scan/filter coverage.
- Root native GUI check at 1280px loaded-folder and 640px compact: central browser and selected preview render; selection reaches preview; resizing works; compact Preview/Back preserves selection; filter no-match and Clear filter work. The native folder picker opened and a user manually selected a folder; its scan completed. This was manual UI interaction, not automated picker completion, and no fixture-specific selection is claimed. Initial header/column and preview-width issues were corrected before later checks. Screenshots remain in `/tmp` and are not added to the repository.
- Implementation: central workspace now owns Open folder, Filter, existing Sort and List/Tree modes, Up/current path, subfolder drill-down, inventory/count, empty/filter/no-match/scanning text, and the existing virtualized flat/tree rows. Existing right preview is resizable when a folder is loaded; narrow mode exposes a Preview button and Back to list, reusing `render_stage_pane`. No-folder startup hides the preview and has one visible primary Open action.
- Preview no-selection now uses `StageState::Idle`; Loading starts only after a selected image has started asynchronous decode. Decode failure and dimensions continue to use existing states.
- No new dependency, persistence, file scan, per-frame I/O, or decode was added.


## U2 implementation checkpoint

- `cargo check --bin rust-feh`: passed with no warnings.
- `cargo test --test unit_ui_logic --test feature_016_actions --test feature_016_roundtrip`: passed (41 passed, 0 failed). The new pure policy test establishes that process actions require `StageState::Ready`; Loading and Failed do not enable them.
- Open in feh and More are visible in the wide browser and compact preview. More reuses the existing six actions. Save/Move/Copy path remain available for unreadable originals; Resize/Convert/Copy image require the current selected original to decode successfully. The same rendering gate applies to a nonselected row context menu without decoding rows.
- Native interaction review of U2 controls: root confirmed the visible menu exposes all six prior actions. Copy path succeeded; resize converted an 800×600 fixture to 400×300 (independently checked); Open in feh opened the selected file and Escape returned to the same selection. Corrupt JPEG and unsupported GIF disabled Resize/Convert/Copy image while filesystem actions remained available. The selected-original readiness gate also checks that the ready decode belongs to the current selected path. A GIF original with a processed sibling, and a valid TIFF marked converted because of a sibling artifact, still had processing disabled for the selected original. This preserves the decoded-original rule rather than trusting inventory status or a sibling.

## U3 implementation checkpoint

- `cargo check --bin rust-feh`: passed after moving existing tool bodies under the unified Tools menu and removing drawer state.
- Existing tool bodies remain single instances opened on demand: Scan settings, Image Tools and pinning, Saved feh launches, Session status and inventory, Activity log, Format support, and Installed tools. The Details drawer, per-section fold/auto-expand state, and redundant preview collapse control were removed.
- Browser count/status remains visible; scan warnings link to Activity log and the indicator can be dismissed while the log retains the details. Missing-feh recovery is visible on the empty start screen and reachable from Installed tools without opening diagnostics automatically.
- Root native review verified all seven Tools surfaces open on demand. Pinning remained associated with the red image while the live selection moved to blue; pinned Open in feh started at red and round-trip returned to red. Folder-scoped actions showed the existing pinned-target restriction.
- Missing-feh native recovery was checked with an isolated empty PATH: the no-folder state showed a warning and Installed tools button without opening a window. Recheck with only the installed feh binary added to the child PATH changed feh from missing to available and cleared the startup warning. The app log reported feh=true, magick=false.
- A permission-denied folder produced a scan-warning indicator and Activity log details; Dismiss removed the indicator while retaining the log. Nested-folder navigation and Up were checked. Tree disclosure/file glyphs and install-copy labels showed unsupported glyphs in the native font; source now uses ASCII disclosure/separators and stable Copy button IDs. The latest release build and loaded-folder size captures are after these changes, but XTest input stopped reaching the client during post-label interaction retakes. Do not claim the corrected tree or copy labels were manually clicked/verified. Earlier evidence (`compact-preview.png`, `pinned-target.png`, `unsupported-original-actions.png`, `scan-warning.png`) predates these final copy/glyph changes; `loaded-640.png`, `loaded-960.png` and `loaded-1280.png` are release layout captures; `no-folder-640.png` was refreshed after the publication review fix.

## Integrated gate and limits

- A prior full run reported `scanner::probe_tests::identify_binary_omits_magick_subcommand` as failing `(expected (1, 0, false), got (0, 1, false))`. No scanner changes or temporary instrumentation were made and no cause was established. The isolated test passed once, and `cargo test --lib -- --test-threads=16` passed 128 tests with 2 ignored; the later fresh full gate also passed. Treat the earlier result as an unexplained, non-reproduced observation, not a fixed scanner defect.
- Fresh `cargo test --verbose`: passed, exit 0; 227 passed, 3 ignored across targets. Fresh `cargo clippy -- -D warnings`: passed, exit 0. `cargo build --release`: passed, exit 0 (49.62 s).
- Final release layout captures: [loaded at 1280](evidence/loaded-1280.png), [loaded at 960](evidence/loaded-960.png), and [loaded at 640](evidence/loaded-640.png). Earlier no-folder checks covered all three sizes; the retained [no-folder at 640](evidence/no-folder-640.png) capture was refreshed after the publication review fix and confirms Preview is absent. Additional prior interaction captures cover [compact preview](evidence/compact-preview.png), [pinned target](evidence/pinned-target.png), [unsupported original actions](evidence/unsupported-original-actions.png), and [scan warning](evidence/scan-warning.png). The README screenshot was refreshed. The XTest pointer root coordinates remained unchanged despite requested movement during post-label retakes, so these are layout captures, not evidence that the final tree disclosures or Copy buttons were interacted with after their text fixes.
- Same release build and the same 10,000-invalid-JPEG fixture at 1280×960: pre-U1 resident samples were 108,664 KiB (~106.1 MiB); settled post-U3 samples were 111,072 KiB (~108.5 MiB), a +2,408 KiB / +2.22% difference, below the inherited 150 MB target. This is a load-only invalid-image workload without actual preview decode; it does not establish image-preview memory cost. The screenshot is [10k memory](evidence/10k-memory.png).
- The existing 10k filter test passed. A final wheel/scroll attempt did not move the pointer or list, so there is no post-change manual scroll-performance result. No subjective scroll threshold or preview-decoding performance claim is made.
- Prepublication review found the compact Preview control could replace the no-folder welcome with an idle preview. The affordance and compact-body route now require a loaded folder, and stale compact-preview state is cleared when no folder is loaded. Focused `cargo check --bin rust-feh` passed after this guard; final suite remains with root.

- Live network-folder responsiveness and cancellation were not manually exercised in this environment. Existing cancellation/probe tests and code inspection cover the preserved scan path; no live NAS pass is claimed.

- Publication gate after the compact-preview correction: fresh full tests passed (227 passed, 3 ignored), clippy passed, and release build passed. One preceding run failed `inherited_stdout_does_not_delay_probe_completion` with the same rejected-probe tuple as the earlier scanner observation. It passed in isolation and the complete rerun passed; scanner code remains unchanged and the cause is unresolved.

## Startup tool discovery — 2026-10-05

Verified on main at `721d88f`, using the configured terminal's inherited PATH. The earlier missing-tool messages came from the isolated-PATH recovery test described under U3; they do not describe this normal-environment result.

| Tool | Resolved executable | Actual app startup field |
|------|---------------------|--------------------------|
| feh | `/usr/bin/feh` | `feh_available: true` |
| ImageMagick | `/usr/bin/magick` | `magick_available: true` |
| magick-cache | `/usr/local/bin/magick-cache` | `magick_cache_available: true` |

Evidence: shell lookup found the executables above. Release and debug builds succeeded. The release GUI launched without a missing-feh warning. Because desktop input injection did not open Installed tools, the stronger three-tool check used the unchanged debug app under GDB: stop immediately after `ToolCapabilities::detect()`, inspect the actual startup object, then continue into the GUI.

```bash
env -u RUST_FEH_START_FOLDER gdb -q --batch \
  -ex 'set pagination off' -ex 'break src/main.rs:304' \
  -ex run -ex 'print tool_caps' -ex continue \
  --args target/debug/rust-feh
```

All three availability fields were `true`; continuing produced `App started. Use Open folder to load images. Open Activity log from Tools for details.` The breakpoint line applies to the recorded revision. PATH was not overridden, source and settings were unchanged, and the owned test processes were closed afterward.

Limits: this proves startup discovery in that launch environment, not a fresh click-through of Installed tools or discovery from a desktop launcher with a different PATH. It does not verify external-tool operations or a configured magick-cache directory/passkey. The startup `magick_cache_ready` flag also appeared true, but currently only reflects executable availability; it is not evidence of a usable cache.
