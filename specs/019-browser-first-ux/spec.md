# 019 — Browser-first workspace

**Status:** Implemented; verification evidence and limitations are recorded in validation-results.md.
**Design authority:** docs/UI-UX-REDESIGN.md.
**Historical outcomes/evidence:** docs/PROJECT-HISTORY.md.

## Problem and outcome

On startup, users see a mostly blank center and a cramped right Inspector; the Open folder action and file list are buried. Make the central workspace a virtualized browser with visible folder open/navigation/filter controls and a resizable right preview. Keep existing browsing, action-target, feh round-trip, pinned-target, and tool recovery behavior.

## Requirements

- R1: With no folder loaded, show clear next-step text and visible Open folder. Do not show an empty table or false preview Loading state.
- R2: With a folder loaded, central area holds the existing bounded, virtualized flat/tree file list and subfolder drill-down.
- R3: Keep Open folder, Up/current path, Filter, current List/Tree choice, and current sort choice reachable with the browser.
- R4: Right preview is resizable, starts at a usable compact width, and has a narrow-window way to reveal the same preview/actions body while the list takes available width. Only one preview instance is active.
- R5: Preview idle/no-selection differs from active decode Loading, decode failure, no-folder, empty folder, and no filter matches.
- R6: Preserve selection identity, list caches, row behavior, filter/sort semantics, scan cancellation, lazy metadata, no per-frame disk walks or image decode, feh round-trip landing, pinned-image safety, and existing preference behavior.
- R7: No third pane, mandatory menu suite, new app icon, new shortcut suite, new sort mode, new dependency, persisted split, command/capability framework, or module extraction. Keep current secondary tools reachable for the later U3 packet.

## Superseded layout requirements

This feature supersedes 017/018 placement requirements for fixed Inspector width, Inspector-owned browsing/list, stage-only center, Browse-exclusive folder actions, and the Details drawer. These historical rules are not acceptance criteria. All protected engineering invariants remain as stated in docs/UI-UX-REDESIGN.md.

## U0 baseline

Before U1, `cargo test --verbose` passed across all targets in the configured environment when run outside the restricted sandbox (library 139/0/2 ignored; binary 8/0/0; three ignored tests total). The 10k filter test `sc003_filter_10k_under_200ms` passed. This is automated filter evidence, not a numeric scroll baseline. A pre-U1 Large-preset GUI capture is retained at `evidence/baseline-large-window.png` (1280×960 content; screenshot 1330×1047 including decorations); it showed hidden Open/Browse controls, a mostly blank center and false Loading. Release GUI session exited cleanly.
