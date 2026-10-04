# 019 Tasks

## U0 — evidence and pointers

- [x] Record current test baseline, environment limitation, and existing 10k filter test before U1.
- [x] Record the pre-U1 release GUI baseline when the current binary was available.
- [x] Create 019 scope, plan, tasks, validation record.
- [x] Point AGENTS.md at current plan and set .specify/feature.json to specs/019-browser-first-ux.

## U1 — browser workspace

- [x] Add/test only the pure preview idle/loading state needed to prevent false Loading.
- [x] Move existing visible browser controls, subfolder drill-down, and virtualized list into the central workspace; preserve current cache, sort, filtering, selection and row renderers.
- [x] Add resizable right preview and a narrow reveal/list-dominant mode reusing one preview body.
- [x] Show actionable no-folder and distinct empty/no-match/scanning/decode states using current scanner/stage data.
- [x] Verify actual selection/preview, resize, compact Preview/Back, filtering/no-match/Clear filter, and folder-picker outcome at the UI boundary; keep tools/drawer for U3.
- [x] Record changed paths, screenshots/actions, exact test commands/results and limitations before handing off to U2.

## U2 — selected actions

- [x] Add visible Open in feh and More controls to wide browser and compact preview.
- [x] Reuse the existing six action handlers and keep filesystem actions available for unreadable input.
- [x] Gate Resize/Convert/Copy image on successful decode of current selected original; keep nonselected row processing unavailable without per-row decode.
- [x] Native-check action target, readiness for valid/corrupt/unsupported inputs, and visible menu reachability.

## U3 — tools and status

- [x] Map existing scan settings, image tools/pinning, saved feh launches, session status, activity log, format support, and dependency recovery into one Tools menu opening the existing tool windows.
- [x] Remove the obsolete Details drawer, per-section fold/auto-expand state, and stage collapse control after relocating the existing bodies.
- [x] Keep current scan status and count visible; make scan warnings discoverable via Activity log and dismissible without deleting the log.
- [x] Keep missing-feh recovery visible on the no-folder state and reachable through Tools → Installed tools; do not auto-open diagnostics.
- [x] Replace stale guidance and capability copy that described the retired layout or overstated image-processing availability.
- [x] Root native review covered missing-feh recovery/recheck, all seven tool windows, pinning, scan warnings, nested navigation, narrow preview, and three final window-size captures. The final plain-label changes compile and pass focused checks; a post-label interactive retake was unavailable and is recorded as a limitation.

## U4 — integrated verification and docs

- [x] Run `cargo test --verbose` and `cargo clippy -- -D warnings`; record commands and outcomes.
- [x] Complete release build and same-workload comparison; record limits.
- [x] Root captures final 640/960/1280 GUI evidence and same-fixture release comparison; report limitations honestly.
- [x] Update README usage/captions and legacy validator output path/criterion; preserve historical validation results.
