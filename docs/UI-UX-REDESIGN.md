# rust-feh UI/UX redesign

**Status:** Active UX authority. The bounded redesign is implemented in [feature 019](../specs/019-browser-first-ux/plan.md); its [validation record](../specs/019-browser-first-ux/validation-results.md) distinguishes verified outcomes from remaining evidence limitations.
**Date:** 2026-10-05. Source references describe the inspected baseline at commit ad233f0; see [UI-UX-BASELINE-2026-10-05.md](UI-UX-BASELINE-2026-10-05.md).

This is the active UX authority. It records the current findings, accepted target, protected behavior, implementation packets and evidence. Historical shipped outcomes and retained measurements live in [PROJECT-HISTORY.md](PROJECT-HISTORY.md). Old feature plans and task lists are not current visual requirements.

**Current disposition:** Feature 019 implements the central browser, resizable/compact preview, visible selection actions, on-demand Tools menu and warning recovery. Native interactions confirmed these outcomes, including pinned targets and unreadable originals with processed siblings. Fresh full tests, clippy and release build pass; final release captures cover the three supported sizes. The same 10k fixture measured 111,072 KiB RSS (~108.5 MiB), below the inherited target. Final scroll input and post-correction tree/menu interactions could not be repeated because desktop input injection stopped delivering events; the last plain-label/glyph edits have code/build coverage. See the validation record for exact evidence and limitations. Richer breadcrumbs, sortable headers and a shortcut suite remain deferred. Semantic LLM access is a separate bounded follow-up in Section 7.

## Contents

1. [Protected product and engineering invariants](#1-protected-product-and-engineering-invariants)
2. [Current UI baseline](#2-current-ui-baseline)
3. [Historical visual requirements retired](#3-historical-visual-requirements-retired)
4. [Current review findings](#4-current-review-findings)
5. [Target layout](#5-target-layout)
6. [Implementation work packets](#6-implementation-work-packets)
7. [Implementation constraints and decisions](#7-implementation-constraints-and-decisions)
8. [Acceptance criteria](#8-acceptance-criteria)
9. [Review checklist](#9-review-checklist)
10. [Code location index](#10-code-location-index)

## 1. Protected product and engineering invariants

These preserve responsiveness, correctness and the feh workflow. They do not freeze the old Inspector, menu, drawer or panel arrangement; Section 3 records the retired rules.

| ID | Constraint | Source |
|----|-----------|--------|
| K1 | Do not copy feh features into rust-feh. Give viewing, zoom and slideshow to feh. | Constitution §I |
| K2 | Keep business logic out of `main.rs`. Put new pure logic in `ui_logic.rs` or `types.rs`, with unit tests. | Constitution §I, §III |
| K3 | Add no dependency without a written reason. Prefer the standard library. egui/eframe is the only approved GUI toolkit. | Constitution §II |
| K4 | Do not read full image data during listing. Load metadata lazily. Keep the UI responsive during scans. | Constitution §V |
| K5 | Keep the list virtualized (`ScrollArea::show_rows`) and bounded. Compare scroll behavior on the same existing workload when a repeatable baseline is available; do not invent a numeric threshold if it is not. | Spec 018 FR-012, SC-002; current baseline is subjective |
| K6 | Keep the resident memory (RSS) below 150 MB with 10,000 images loaded. Historical evidence: about 126 MB at 10,000 images (2026-06-22), 142.8 MB during the 2026-06-28 run, and 140.5 MB in feature 016. Re-measure the implementation; these are not current results. | Spec 003 SC-004 |
| K7 | Do not wrap the panel in an outer `ScrollArea` with unbounded height. It breaks list virtualization. | Spec 018 FR-011 |
| K8 | No network access. | README, constitution |
| K9 | Pass `cargo test`, `cargo clippy -- -D warnings` and the release build. CI enforces clippy. | `.agents/SPEC.md`, `.github/workflows/ci.yml` |
| K10 | Keep functions short (about 50 lines, `.agents/GUARDRAILS.md`). No automated gate; Codacy was removed in 5d51ec8. | Spec 018 plan |

## 2. Current UI baseline

Moved to the historical baseline: [UI-UX-BASELINE-2026-10-05.md](UI-UX-BASELINE-2026-10-05.md#2-current-ui-baseline).
Pre-feature-019 content at `ad233f0`; line references are stale.

---

## 3. Historical visual requirements retired

These requirements are dropped from the **new implementation plan** because they recreate the screenshot’s problems. Record their supersession in the new feature spec; do not reimplement them to satisfy old wording or static checks.

| Historical requirement | Replacement and reason |
|------------------------|------------------------|
| 018: central panel contains only the staged image; list and folder navigation live in the Inspector | Central workspace contains the primary browser. Preview is a resizable companion. Most of the window is immediately useful for finding files. |
| 017/018: fixed, non-resizable Inspector width and static-label width floor | User-resizable preview split with bounded defaults and truncation. Long paths must not resize panels, but users may. |
| 018: Browse is the exclusive home of folder actions; remove File menu duplicates | Keep Open folder, navigation and filter visible in the workspace. An additional File menu is optional, not required. |
| 018: all seven sections behind one Details drawer, folded by default | Remove the drawer. Put browsing and image actions in the primary workspace; make secondary existing tools reachable through the smallest fitting presentation. |
| 018: no controls, inventory, or navigation in central panel | Drop this restriction. Allocate workspace according to the current task and make empty states actionable. |
| 012/018: status and diagnostics visibility tied to old bottom-panel/drawer structure | Persistent compact status, contextual warnings and direct Log/Installed tools access. Preserve useful feedback, not its old container. |

**Engineering behavior to preserve:** bounded virtualized list height; cached filter/sort/tree calculations; cancellable background scans; non-recursive default and unified folder navigation; stale-result rejection and by-path converted-status merges; feh filelist order and round-trip landing; explicit action target resolution; pinned image safety; existing window presets/persistence; graceful missing-tool handling. No per-frame disk walks or image decoding during listing. The redesigned preview must have one active instance at a time and reuse existing stage/actions behavior; do not assume a detached-preview mechanism already exists.

The old drawer’s exact auto-open/close layout is not an invariant. Its useful lesson is: show a problem when it appears, respect a user dismissing a notice, and do not reopen windows every frame. The redesign uses an inline warning with an Installed tools action rather than forcing the old Dependencies section open.

---

## 4. Current review findings

Moved to the historical baseline: [UI-UX-BASELINE-2026-10-05.md](UI-UX-BASELINE-2026-10-05.md#4-current-review-findings).
Pre-feature-019 content at `ad233f0`; line references are stale.

---

## 5. Target layout

The target is a browser-first workspace with a resizable preview. The virtualized list occupies the flexible central area; a right pane shows the selected image and actions. Keep Open folder, Up/current path, Filter and the existing List/Tree choice visible. A compact status area shows scan progress, count and current warnings, with inventory and log details reachable. Remove the Details drawer and blank image-only center.

A permanent left folder pane is out of scope. Existing subfolder drill-down and Folder tree already serve navigation. Keep current Up/path behavior; richer breadcrumbs are optional. Do not require File/View/Tools/Help menus, one window per tool, a new application icon, a third pane, a command framework, module extraction, dependencies or persisted split settings.

### 5.1 First run and folder loaded

Before a folder is loaded, show an explanatory empty state and a visible Open folder action. Do not show an empty table, preview box, diagnostic accordion, Recent folders or a false Loading state. If feh is missing, show a recoverable warning while still allowing folder browsing.

After loading, keep the browser dominant, with a bounded subfolder strip and a resizable preview that starts near one third of usable width. Open in feh is the primary selection action. More actions exposes all six existing context actions. Keep status compact and errors discoverable. Secondary tool bodies remain reachable through the smallest fitting use of existing controls and window mechanisms; do not require one new window for each tool.

### 5.2 Narrow windows and visual hierarchy

At narrow widths, let the list use the workspace. A visible Preview control switches that region to the same preview/actions body, with a visible Back to list control. Retain selection, filter and navigation state across the switch. Choose the transition by checking the supported minimum and ordinary window sizes; 800 logical pixels is not a fixed breakpoint. Only one preview instance is active. No new detached-preview window or saved width/split settings are required.

Open folder and Open in feh are the primary actions. Truncate long names and paths with full values available on hover. Preserve fit-to-pane/no-upscale preview behavior and feh delegation. New icons and breadcrumb enhancements are optional polish, not acceptance gates.

---

## 6. Implementation work packets

The user authorized implementation, now recorded as feature 019. These packets remain the design input; its task and validation records track execution. Do not replay completed 017/018 tasks or run generic approval rounds.

### 6.1 U0 — Evidence and implementation setup

**Objective:** establish the implementation baseline before application edits.

**Included:** before U1, record the current behavior and a same-build/same-fixture baseline using an existing workload if available; compare current source with the code location index in [UI-UX-BASELINE-2026-10-05.md](UI-UX-BASELINE-2026-10-05.md) (Section 10) and note material conflicts; record retired 017/018 visual rules in new feature clarifications; update AGENTS.md plus .specify/feature.json when the new feature is created.

**Excluded:** application edits, guessing a feature number, changing pointers before feature creation is authorized, replaying historical tasks.

**Owner/prerequisite:** a Sol orchestrator (or another supported non-Astra primary) coordinates this packet before U1 and delegates bounded inspection, implementation, tests and documentation edits to Luna. Astra supplies only judgment on evidence provided; Astra does not inspect the filesystem, research, code or run tests. This document remains the mandatory design input while pointers are stale.

**Acceptance:** the new feature records this architecture, protected behaviors, packets and evidence plan; historical layout rules are explicitly superseded.

**Stop/escalate:** only for a material source conflict with the accepted outcome or unavailable required write; state the exact conflict or boundary.

**Source seams:** frame ordering and menu/panel layout are RustFehApp::update and render_top_menu_bar/render_inspector_side_panel/render_central_image_panel in src/main.rs. Folder handling is navigate_to_folder/pick_folder; list data is compute_list_indices and ImageListMetrics. The baseline source map is the code location index in [UI-UX-BASELINE-2026-10-05.md](UI-UX-BASELINE-2026-10-05.md) (Section 10); seam names are pre-019; several were removed in 019. Use feature 019 and current symbols during implementation.

### 6.2 U1 — Make the browser the useful workspace

**Objective:** users can open a folder and find images in the large central area, with an honest preview state.

**Included:** one sequential owner for src/main.rs restructures frame/layout order; places visible Open folder, Up/current path, Filter and existing List/Tree controls by the browser; moves current virtualized flat/tree list and subfolder drill-down to the central workspace; adds a resizable right preview and usability-based narrow fallback with one active preview; distinguishes no-folder, scanning, completed-empty, no-match, scan-error and no-selection-idle states. Reuse current handlers, list/cache logic and stage behavior. Preserve existing navigation and sort choices.

**Excluded:** mandatory menu suite, third pane, new icon/app-icon, richer navigation/history, new sort modes, persistence, dependencies/modules, image decode or filesystem walks during listing.

**State boundary:** use existing scan outcomes and summarized warnings. `ScanMsg::Complete` carries `ScanResult`, not a new error result; show inaccessible-path and scan warnings honestly without adding scanner error plumbing solely to create a separate screen. Loading belongs only to current pending work. New shortcuts and sortable headers remain deferred.

**Owner/prerequisite:** one main.rs owner keeps layout ownership through U2/U3. Pure helper/test work may be separate only with disjoint files and explicit interfaces. U0 source map is the prerequisite.

**Validation/acceptance:** focused state tests when state logic changes. At 640x480, 960x720 and 1280x960, inspect no-folder and loaded states; interact with Open folder, Up/current navigation, Filter, List/Tree and narrow preview reveal. Include long names/paths. Confirm bounded virtualized list remains. Complete when the browser is immediately usable, list space dominates, preview can resize or be revealed, and states are distinct.

**Stop/escalate:** only for a measured conflict among virtualization, bounded layout and supported minimum size; report the case and alternatives.

**Ordered steps:** (1) preserve frame ordering constraints and place browser controls, (2) move existing list/subfolder renderers without rewriting cache/row logic, (3) add preview sizing and narrow fallback with one instance, (4) add/resolve state display from existing scan and stage state. **Source seams (pre-019 names; see [baseline](UI-UX-BASELINE-2026-10-05.md)):** src/main.rs render_inspector_nav_strip, render_inspector_subfolder_drilldown, render_inspector_file_list, render_flat_image_list, render_tree_image_list, render_stage_pane, kick_stage_decode_if_selection_changed, poll_stage_decode, and update; src/ui_logic.rs compute/filter/sort/tree helpers. **Worker return:** changed paths, state transitions, screenshots/interactions, tests run and remaining evidence gaps.

### 6.3 U2 — Make selection actions clear and safe

**Objective:** selection visibly leads to opening the intended image in feh or choosing an applicable existing action.

**Included:** prominent Open in feh and More actions with the six existing actions; reuse current handlers/context menu; preserve target identity through filtering, pinned context, rename/move and feh round-trip; correct operation-specific capability checks so inventory status or a processed sibling cannot imply that the original is decodable. Keep filesystem actions available when valid and explain process failures accurately.

**Excluded:** new operations/backends, imported viewer features, generalized capability framework, module extraction, unrelated action-semantic changes.

**Owner/prerequisite:** same main.rs owner; focused pure capability logic stays in existing ui_logic.rs. U1 supplies the preview surface.

**Validation/acceptance:** existing action/round-trip tests plus focused cases for unsupported original with Converted sibling, corrupt supported extension and pinned/live target when affected. Interact with Open in feh and More actions. Labels/enabled actions must describe the actual target, and each current action must be reachable.

**Stop/escalate:** only for unresolved ambiguity over live versus pinned target; identify handler and state path.

**Ordered steps:** (1) map each existing context action to its current handler, (2) expose Open in feh and More actions on preview, (3) audit target resolution at each call site, (4) repair only the capability predicate and add focused cases. **Source seams (pre-019 names; see [baseline](UI-UX-BASELINE-2026-10-05.md)):** src/main.rs render_image_context_menu, render_image_actions_body, render_image_actions_full, action_* methods, open_in_feh/open_in_feh_pinned; src/ui_logic.rs file_status_decodable and PanelContext; tests/integration/feature_016_actions.rs and feature_016_roundtrip.rs, tests/unit/ui_logic.rs. **Worker return:** action-to-handler map, target-safety audit, test output and actual visible-control interaction evidence.

### 6.4 U3 — Keep tools and recovery reachable

**Objective:** remove the crowded drawer without losing current tools, feedback or recovery.

**Included:** map drawer capabilities to the smallest reachable existing controls; keep compact count/progress/warnings and inventory/log access; keep Image Tools, Saved feh launches, Installed tools, Format support, recheck/install and log actions reachable; remove drawer and height allocation after mapping; make warnings dismissible without per-frame reopening. Reuse existing render bodies and current window/control mechanisms where they fit.

**Excluded:** one new window per tool, new app icon, third pane, command/capability framework, new preferences/persistence, duplicate tool implementations.

**Owner/prerequisite:** same main.rs owner after U1/U2. Preserve pinned-image safety and folder-scoped tool restrictions. Complete a capability-location map first.

**Validation/acceptance:** interact with current tool/recovery routes; cover missing feh, recheck, failed/cancelled scan, inventory and log. Confirm no drawer remains, no capability is lost and browsing remains unobscured.

**Stop/escalate:** only if a current capability has no reachable route; report capability, handler and smallest possible presentation.

**Ordered steps:** (1) inventory current Details drawer contents, (2) assign each capability an existing reachable control, (3) relocate/reuse render bodies, (4) remove drawer and obsolete height math, (5) exercise recovery routes. **Source seams (pre-019 names; see [baseline](UI-UX-BASELINE-2026-10-05.md)):** src/main.rs render_inspector_sections_drawer_body, render_session_status_body, render_activity_log_body, render_deps_section_body, render_format_discovery_body, render_inspector_feh_instances, render_inspector_image_tools, detached-window rendering; src/ui_logic.rs AutoExpandState and drawer height helpers. **Worker return:** capability-location map, removed old-layout references, interaction evidence and tests.

### 6.5 U4 — Integrated evidence and current documentation

**Objective:** verify the complete flow and align validator/docs with shipped behavior.

**Included:** update scripts/validate-feature-001.sh to assert browser behavior rather than the retired TopBottomPanel marker; update README and active pointers after feature creation; run relevant tests, clippy and release build; record GUI and performance/memory evidence with an existing repeatable workload where available.

**Excluded:** new benchmark harness, invented numeric scroll threshold, making fresh benchmark tooling block core UX, blanket test additions, unrelated documentation consolidation.

**Owner/prerequisite:** orchestrator coordinates after U1-U3. CI currently runs cargo test --verbose and cargo clippy -- -D warnings.

**Validation/acceptance:** review AC-1–AC-9 with evidence labels (automated, GUI interaction, code inspection or limitation). Run cargo test --verbose, cargo clippy -- -D warnings and release build. Reuse the same existing workload/build/fixture for before/after scroll and RSS when available; report when no numeric baseline can be reproduced. Historical 150 MB RSS remains an inherited engineering target to reassess for reproducibility, not a reason to invent a benchmark system or block UX.

**Existing GUI/performance protocol:** generate the existing 10,000-file fixture with `./scripts/generate-perf-fixture.sh 10000`; use the same release build and fixture for before/after comparisons. During manual review, open the fixture, use the same rapid-scroll interaction, sample RSS with `./scripts/sample-rss.sh` three times, and record build, fixture count, samples, machine and conditions. The automated timing check remains `cargo test sc003_filter_10k_under_200ms`. Do not treat subjective scrolling as a numeric pass; if historical fixture/build conditions cannot be reproduced, state the limitation instead of inventing a new harness or threshold.

**Stop/escalate:** only for required check failures or demonstrated protected-behavior regression; report exact evidence and owner.

**Ordered steps:** (1) update obsolete validator semantics, (2) update README and active pointers after feature creation, (3) run focused tests plus CI checks/release build, (4) compare existing workload evidence to U0 baseline or record why it is not reproducible, (5) provide a final evidence matrix. **Source seams:** scripts/validate-feature-001.sh, README.md, AGENTS.md, .specify/feature.json, .github/workflows/ci.yml, tests/perf/sc_timing.rs. **Worker return:** diff summary, commands/results, GUI evidence, baseline comparison/limitations and unresolved items.

### 6.6 Outcome milestones

The five packets form three reviewable outcomes: browser workspace (U0+U1), selection/actions (U2), and tools/recovery plus integrated closeout (U3+U4). They are coherent user outcomes, not line-edit tasks. Do not assign simultaneous owners to shared main.rs work.

### 6.7 Adversarial architecture review

Two independent Astra judgments considered a simpler three-unit plan and a more explicit five-packet handoff. The synthesis keeps five bounded packets under three outcomes: U0 makes the handoff traceable, U1-U3 deliver behavior, and U4 owns integrated evidence. The review rejected unsupported or unnecessary additions: mandatory four-menu suite, one window per tool, permanent third pane, new application icon, new command/capability framework, module extraction, dependencies, persistence, fixed 800px breakpoint, new benchmark harness, and mandatory header sorting/shortcut suite. Existing sort modes and controls remain; new header affordances, basic shortcuts, richer breadcrumbs and icon polish are optional deferred follow-up with no acceptance or ownership obligation. No repeated generic approval loop is needed.

Completed in feature 019 (see specs/019-browser-first-ux/tasks.md).

---

## 7. Implementation constraints and decisions

Render top/bottom/side panels before CentralPanel. Preserve bounded list height, show_rows, cached list indices/tree rows, cancellable scans and stable widget IDs. Do not turn sorting, filesystem access or decoding into per-frame work. Reuse current egui widgets and existing UI bodies. Explicitly manage focus if future shortcut work is separately approved. A screenshot proves appearance, not click behavior; use actual interaction evidence at outcome boundaries.

### Architecture decisions and scope

| ID | Decision |
|----|----------|
| D0 | Wallpaper remains out of scope; correct current claims while retaining dated history. |
| D1 | Central browser with resizable right preview; no permanent third pane. |
| D2 | Visible Open folder/navigation/filter controls; extra menus are optional. |
| D3 | Resizable preview with usability-based narrow fallback; no fixed 800px rule or persisted split. |
| D4 | Remove Details drawer after every existing capability is mapped to a reachable control. |
| D5 | No thumbnails, metadata sort, full settings or Recent folders. |
| D6 | Preserve current sort modes and navigation. New header sorting, shortcut suite and breadcrumb enhancement are optional deferred work. |
| D7 | Feature 019 records this authorized implementation; update active pointers together. |
| D8 | Operation-specific target capability; inventory labels must not promise decode readiness. |
| D9 | Reuse existing render bodies and handlers; add no dependency/framework/module solely for presentation. |

These decisions govern feature 019. Implementation status and observed results live in its [validation record](../specs/019-browser-first-ux/validation-results.md); baseline descriptions and code references remain dated evidence in [UI-UX-BASELINE-2026-10-05.md](UI-UX-BASELINE-2026-10-05.md).

### Follow-up: semantic access for agents

The user asked whether an LLM can inspect and operate the native UI like Playwright operates a browser. Prefer the existing accessibility stack before adding a separate command interface. The current eframe 0.30 dependency includes AccessKit through default features; its Unix adapter exposes AT-SPI roles, names, states and actions. See the [AccessKit documentation](https://github.com/AccessKit/accesskit). Compiled support is not proof that every current control is usable through this interface.

A bounded follow-up should inspect the live tree, label missing custom controls and inputs with stable identities, then demonstrate opening a folder, filtering, selecting a row and invoking the visible action menu through a local semantic client. Preserve list virtualization: offscreen rows may require scrolling and reacquiring nodes. Native file pickers and feh are separate surfaces and must be checked separately. A small external adapter could expose inspect/click/type/wait operations to an LLM using the same UI actions.

Astra's bounded architecture review accepted this approach. Start with an existing semantic client; add an adapter only after the tree works and a client integration gap is demonstrated. Wait for observable state changes rather than fixed delays. Separate app accessibility defects from client limitations, and label the current browser/preview layout without restoring the retired Inspector structure.

On 2026-10-05, a read-only check returned `IsEnabled=false` and `ScreenReaderEnabled=false` from the local accessibility service. No desktop setting was changed, and no semantic interaction pass is claimed. This investigation does not add a dependency, service, browser migration or acceptance gate to the current UI redesign.

---

## 8. Acceptance criteria

These criteria govern feature 019. Evidence is collected at outcome boundaries, not repeated for every microtask.

| ID | Criterion | Evidence |
|----|-----------|----------|
| AC-1 | Fresh no-folder startup shows Open folder, explanatory empty state and no false Loading or empty diagnostics. | Screenshots at three supported sizes and real picker interaction. |
| AC-2 | Loaded folder shows central virtualized browser, resizable preview and usable narrow fallback. | Three-size screenshots; long-name/path fixture; actual resize/reveal interaction. |
| AC-3 | No selection is idle; Loading is only active decode; empty, filter, scan and error states differ. | Focused state tests and GUI states. |
| AC-4 | Open in feh is prominent and all six existing context actions are reachable from visible controls/More actions. | Actual selection and action-menu interaction. |
| AC-5 | Process actions reflect the specific target; unsupported originals with converted siblings and corrupt supported files are not represented as decodable. | Focused capability/error tests and GUI sample. |
| AC-6 | Details drawer is gone and its useful capabilities remain reachable; primary controls are not clipped. | GUI review, including missing-tool state. |
| AC-7 | Flat/tree, filter/sort choices, round-trip landing, pinned targets and folder-scoped tool rules retain behavior. | Existing targeted tests plus actual relevant interactions. |
| AC-8 | Scan cancellation/navigation remain responsive; no hot-path filesystem walk or decode is introduced. | Targeted tests, code audit and available local/network-folder interaction. |
| AC-9 | cargo test, clippy and release build pass; no unexplained dependency. Performance/RSS are measured on the same existing workload when reproducible and limitations are stated. | Fresh check output and comparable recorded evidence where available. |

Historical scroll evidence was subjective; feature 016’s +3.4% measured scan/background-pass time, not scrolling. Do not invent a numeric scroll threshold or benchmark harness. Retain the inherited 150 MB RSS target as a measurement to reassess for reproducibility. A screenshot cannot prove picker/action behavior. Mark evidence automated, observed interaction, code inspection or limitation; never convert a proxy into a manual pass.

---

## 9. Review checklist

A reviewer should challenge the implementation against the current user problem:

1. Does the first screen visibly tell a user how to begin, without a drawer or unexplained Loading label?
2. Is the file list comfortably readable at the normal preset, with useful preview access at the minimum size?
3. Are Open in feh and every existing image action discoverable, with correct targets and capability messages?
4. Have diagnostics moved out of the primary workflow without losing recovery, log, inventory, install or recheck capabilities?
5. Have scan cancellation, list virtualization/cache behavior, feh round trips and pinned safety survived the move?
6. Do screenshots and actual interaction evidence substantiate the UX, and do current measurements substantiate performance?
7. Does any old requirement or validator inadvertently restore the hidden Browse entry, fixed cramped Inspector, empty center or Details drawer? If so, correct the requirement/validator rather than reintroducing the problem.

---

## 10. Code location index

Moved to the historical baseline: [UI-UX-BASELINE-2026-10-05.md](UI-UX-BASELINE-2026-10-05.md#10-code-location-index).
Pre-feature-019 content at `ad233f0`; line references are stale.
