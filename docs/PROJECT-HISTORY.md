# rust-feh project history

This is the compact record of shipped feature outcomes, decisions that still constrain
the product, deferred work, and evidence worth retaining. The active product and UX
authority is [UI-UX-REDESIGN.md](UI-UX-REDESIGN.md); old feature plans and task lists are
not current UI requirements. Source code, tests, preserved validation reports and images
remain the evidence for shipped behavior.

## Feature ledger

| Feature | Outcome and surviving decision | Preserved evidence |
|---------|--------------------------------|--------------------|
| 001 | Persistent virtual browsing and large-list behavior. Keep bounded, virtualized listing and lazy metadata. | [Validation](../specs/001-persistent-ui-virtual-browsing/validation-results.md); [5k controls](../specs/001-persistent-ui-virtual-browsing/evidence/20260705-v1-controls-5k.png); [missing-feh hint](../specs/001-persistent-ui-virtual-browsing/evidence/20260705-v8-feh-missing-hint.png) |
| 002 | Runtime detection was superseded by 009. | History only; do not reopen. |
| 003 | Performance protocol and historical results. Old scroll evidence was subjective; 10k filter and RSS observations are not a current redesign baseline. | [Results](../specs/003-gui-performance-validation/validation-results.md); repeatable current checks are owned by U4 in the active plan. |
| 004 | Scanner resilience was absorbed by 011. | History only; do not reopen. |
| 005 | List/tree, inventory and status presentation. Preserve current sort/tree behavior; do not infer decode capability from inventory status. | Current tests and U2 capability acceptance; historical reports were not independently reproducible in this ledger. |
| 006 | Window size preferences and stability shipped; preserve existing preference behavior. | [T009 screenshot](../specs/006-window-viewer-stability/evidence/20260705-t009-feh-1280x960-tiny-zoomed.png) |
| 007 | Former roadmap/index is replaced by this ledger plus the active redesign plan. | No separate implementation. |
| 008–009 | Tool capability detection, graceful missing-tool handling, install/recheck behavior. Keep ImageMagick optional and correctly described. | Existing tests and source; old panel/menu locations are retired. |
| 011–012 | Background browsing, scan feedback, NAS policy and activity log. Preserve cancellation, stale-result rejection and recovery. | [011 results](../specs/011-browsing-experience-round/validation-results.md); [012 results](../specs/012-ui-feedback-polish/validation-results.md) |
| 013 | Image Tools and magick-cache integration shipped. Preserve current operations and setup guidance. | [Setup guide](MAGICK-CACHE-SETUP.md); code and integration tests. |
| 014 | Saved feh launches and clipboard actions shipped. Preserve existing behavior and target rules. | Code and integration tests. |
| 015 | Caption integration was specified and deferred. Revisit only if upstream provides local batch operation plus a usable license/tag; remote inference/network egress is outside this project’s no-network rule. | No implementation commitment. |
| 016 | Stage preview, image actions and feh round-trip shipped. Preserve target identity, action safety and round-trip landing. | [Results](../specs/016-feh-viewer-actions/validation-results.md); integration tests. |
| 017 | Lazy, non-recursive folder scanning and drill-down shipped in the 018 line. Preserve scan cancellation and navigation. Per-entry launch image-count cache remains deferred. | Source/tests; no separate result report. |
| 018 | Inspector implementation shipped its state, detach and pin safety fixes. Its Inspector-first layout, fixed width, stage-only center, Details drawer and menu exclusivity are superseded by the active redesign. | [Aspect-ratio measurement](../specs/018-inspector-ux-rework/aspect-ratio-evidence.md); source/tests. |
| 019 | Browser workspace, resizable/compact preview, visible image actions and on-demand Tools implemented on 2026-10-05. Full tests, clippy and release pass; same-fixture load-only RSS is ~108.5 MiB. Final input-injection and network-folder limitations remain explicit. | [Current validation and screenshots](../specs/019-browser-first-ux/validation-results.md). |

## Decisions retained

- rust-feh orchestrates browsing and actions; feh owns full viewing, navigation and
  slideshow. rust-feh is Linux-first, lightweight and no-network.
- No wallpaper feature; historical feh --bg-fill references do not describe current
  behavior.
- Keep the existing egui/eframe 0.30 glow stack and minimal dependencies.
- Keep listing virtualized and bounded; metadata stays lazy; scans remain cancellable.
- Preserve feh filelist ordering, round-trip landing, operation-specific target checks,
  pinned-image safety, existing window preferences and graceful missing-tool behavior.
- Do not add thumbnails, metadata sorting, full preferences, Recent folders, full viewer
  or editor, imported GPL code, a new image backend, or a new async runtime as part of the
  UI redesign.
- Historical 150 MB RSS is an inherited target to re-evaluate on a comparable existing
  workload. No numeric scroll threshold is retained because the historical scroll result
  was subjective.

## Deferred items

These are recorded for traceability and are not redesign blockers:

- 017 per-entry launch image-count cache.
- 005 per-folder skipped counts.
- 009 executable-permission edge case.
- Optional B-5 validation-results parity for 005/008/009/013/014.
- Broader keyboard navigation, new sort modes, Back/Forward history, workspace
  persistence, thumbnails and metadata sorting.
- imtools external integration remains a separate proposal. The maintainer ruling against
  importing its GPL code remains in force.

## Consolidated predecessors

The former POSITIONING, NFEH-COMPARISON-AND-MIGRATION, OUTSTANDING-ISSUES-ROADMAP,
NEXT-ROUND-CONSOLIDATED and SESSION-2026-06-22-TRACEABILITY records were consolidated into
the active redesign plan and this ledger. Keep their dated history in Git. Do not copy
their old closed-task chronology or retired UI constraints back into active instructions.
