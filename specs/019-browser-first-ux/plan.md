# 019 Implementation Plan

## Outcome

Move the current virtualized browser into the central workspace, pair it with a resizable right preview, and expose existing selection actions clearly. One Luna owner executes U0-U3 in order; U4 integrated verification remains pending.

## Constraints and ownership

- Read docs/UI-UX-REDESIGN.md before work. It is the active architecture/design input; this feature records implementation status and evidence.
- One Luna implementation owner holds src/main.rs through U1-U3. Do not run parallel main.rs writers.
- Keep pure helpers in existing ui_logic.rs/types.rs only when needed; add no new module/dependency or presentation framework.
- Reuse navigation, cache, row, selection, stage decode, and feh behavior. Preserve show_rows and bounded scroll.
- No new file I/O, list walks, or image decoding in a per-frame render path.
- Narrow mode reuses the same preview/actions body and preserves selection/state; no second preview window or persistent split preference.

## U0

Captured test and 10k filter baseline before U1; create this feature record and update AGENTS.md/.specify/feature.json together. Record release and GUI baseline when accessible without replacing historical result files.

## U1 ordered work

1. Add a pure idle preview-state representation/test if required; ensure no selection is idle and decode Loading begins only when a decode is requested.
2. Restructure frame layout so the central workspace owns visible Open folder, Up/current path, Filter, List/Tree, existing Sort, subfolder drill-down, and flat/tree rows.
3. Preserve the existing cache and row renderers; keep list virtualization bounded.
4. Add the resizable right preview and a narrow-window control that reveals the same body while maximizing the list.
5. Add no-folder, empty, no-match, scanning and decode-failure messages from existing state/warnings. Scanner currently returns ScanResult with warnings rather than Result; do not introduce error plumbing/framework without evidence.
6. Remove only obsolete stage-only/Inspector layout assumptions in code comments and the retired layout static validator check. Leave tool sections temporarily until U3.

## U2 ordered work

1. Add visible Open in feh and More controls to the browser and compact preview using the current feh selection resolver and existing six context actions.
2. Keep filesystem actions available independently; gate decode-dependent resize/convert/copy-image on the selected original's successful StageState decode, never inventory status or a processed sibling.
3. Reuse the shared action-menu handlers for More and context menus. Do not add action backends, capability frameworks, or per-row decoding.
4. Check live versus pinned target handling and preserve current round-trip and filter semantics.

## U3 ordered work

1. Inventory each existing tool body and expose it from one Tools menu: Scan settings, Image Tools and pinning, Saved feh launches, Session status and inventory, Activity log, Format support, and Installed tools.
2. Reuse the existing detached-window instances and body handlers; align their titles with the menu. Do not extract modules, add a capability framework, or add a new pane.
3. Remove the retired Details drawer, per-section fold/auto-expand policy, and redundant preview collapse control only after the replacement paths are present.
4. Keep count/status compact in the browser and expose scan-warning details through Activity log with a dismissible warning indicator. Missing-feh recovery stays visible on no-folder startup and reachable without automatically opening a window.
5. Keep pinning, saved launches, status/log, format information, and dependency recovery reachable. Avoid filesystem checks or processing in frame rendering.

## U4 integration

Root owns the final native acceptance captures and release comparison; Luna runs and records focused/full checks, updates README usage/captions, and adjusts the legacy feature-001 validator so it cannot overwrite retained historical results.

## Verification

Focused regression checks include `cargo test --test unit_ui_logic --test feature_001_validation --test feature_005_list --test feature_016_actions --test feature_016_roundtrip`. Existing 10k check: `cargo test sc003_filter_10k_under_200ms -- --exact --nocapture`. Final integration gate: `cargo test --verbose`, `cargo clippy -- -D warnings`, and `cargo build --release`. Root captures GUI outcome evidence at 640/960/1280 widths for no-folder, loaded folder, picker/navigation/filter/list-tree/resize/narrow-preview, long names, warning/recovery and tools reachability. Compare the same release build/workload before and after where available; report workload and limits rather than inventing a threshold.
