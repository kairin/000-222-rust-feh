# rust-feh UI/UX redesign

**Status:** Active UX authority. The bounded redesign is implemented in [feature 019](../specs/019-browser-first-ux/plan.md); its [validation record](../specs/019-browser-first-ux/validation-results.md) distinguishes verified outcomes from remaining evidence limitations.
**Date:** 2026-10-05. Source references below describe the inspected baseline at commit ad233f0.

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
| K10 | Keep functions within the Codacy limits for length and complexity. | Spec 018 plan |

## 2. Current UI baseline

### 2.1 Frame order

`RustFehApp::update` (`src/main.rs:4326`) draws these parts in this order on each frame:

1. `render_top_menu_bar` (`src/main.rs:3453`). A top bar with one menu.
2. `render_inspector_side_panel` (`src/main.rs:3461`). The right panel.
3. `render_central_image_panel` (`src/main.rs:3894`). The center.
4. `render_detached_inspector_windows` (`src/main.rs:3303`). Floating windows.

There is no bottom panel. The code never calls `TopBottomPanel::bottom`.

### 2.2 Menu bar

- One menu: **View** (`src/main.rs:3456`).
- View holds a "Window size" submenu with three presets and a "Resizable window" check box (`src/main.rs:3430-3451`).
- Preset labels: "Compact (720 × 540)", "Default (960 × 720)", "Large (1280 × 960)" (`src/ui_logic.rs:35-41`).
- There is no File, Tools or Help menu. Spec 018 Batch 3 removed the File menu on 2026-07-12.

### 2.3 Inspector (right panel)

The Inspector is a `SidePanel::right("inspector")` with `.resizable(false)` and `.exact_width(...)` (`src/main.rs:3463-3465`).

Width rule (`inspector_width`, `src/main.rs:3093-3190`):

- The code measures a fixed list of text labels (`STATIC_LABELS`). The widest is about 252 px. It adds 48 px of chrome.
- It clamps the result between a floor of 440 px and a cap of half the window width (`src/main.rs:3184-3189`).
- In practice the width is 440 px, or half the window if the window is narrower than 880 px. The user cannot drag it.
- The reason (017 Phase 5): a long network path must never make the panel wider, and the width must not move as text changes.

The Inspector has four zones from top to bottom (spec 018 plan):

| Zone | Content | Code |
|------|---------|------|
| A. Navigation strip | "⬆ Up", "Flat list", "Folder tree", a spinner, the current path as one truncated label | `render_inspector_nav_strip`, `src/main.rs:3483-3536` |
| B. Subfolders | One row per subfolder ("📁 name"), max 120 px tall. Hidden when there are no subfolders. | `src/main.rs:3542-3564` |
| C. File list | Flat list (Folder, Filename, Status) or folder tree, virtualized. Minimum height is four rows. | `src/main.rs:3573-3622`, flat `3706`, tree `3816` |
| D. Details drawer | The "▶ Details" toggle, a faint label "Browse · actions · session · log · deps · formats", then seven sections in a scroll area | `src/main.rs:2856-2884`, body `2889-3028` |

Zone C column widths: Folder 35%, Status 25%, Filename the rest (`src/main.rs:3593-3605`). Row height 18 px (`src/main.rs:3579`). The column headers are plain bold text (`src/main.rs:3714-3721`). You cannot click them.

Zone D drawer height: 40% of the panel height, clamped to 180-360 px when open. 24 px when closed (`src/ui_logic.rs:1664-1680`).

### 2.4 The seven sections in the Details drawer

| Order | Section | Header text | What it holds | Type |
|-------|---------|-------------|---------------|------|
| 1 | Browse | "Browse — No folder loaded" or "Browse — name" | "Choose folder", path, Filter box, "Include subfolders", "Detect exotic formats (slow)", "Rescan", Sort combo box (`src/main.rs:2747-2827`) | User task |
| 2 | Image actions | "Image actions — no selection" or the file name | "Open in feh", Image Tools (Single, Batch, Rename, Cache tabs) (`src/main.rs:1330-1420`, `2692`) | User task |
| 3 | Feh instances | "Feh instances" | Saved feh launch entries with label, folder, launch button (feature 014) (`src/main.rs:1781-1830`) | User task (advanced) |
| 4 | Session status | "Session status — Showing N / M images" | Status text, scan inventory, "Copy status", speed tips (`src/main.rs:3191-3260`) | Diagnostic |
| 5 | Activity log | "Activity log — N events" | Text log, "Copy log", "Clear logs" (`src/main.rs:1005-1210`) | Diagnostic |
| 6 | Dependencies | "✅ Dependencies — all required tools OK" | Tool status, install hints, "Recheck tools on PATH" (`src/main.rs:1213-1240`) | Diagnostic |
| 7 | Format discovery | "Format discovery — N groups" | Format routing per format (`src/main.rs:1241-1270`) | Diagnostic |

Each section has a "Detach window" button. A detached section opens as an `egui::Window` (`src/main.rs:3324`).

### 2.5 Auto-open rules (feature 018 FIX-1)

A pure state machine, `ui_logic::AutoExpandState` (`src/ui_logic.rs:1696-1790`), opens and closes sections:

- No folder loaded → Browse opens and the drawer opens. When a folder loads, Browse closes again.
- A scan starts → Session status opens. When the scan ends, it closes.
- A required tool is missing → Dependencies opens. When a recheck finds all tools, it closes.
- If the user closes a section, the machine does not open it again in the same scope.
- If the user opens a section, the machine does not close it.
- `sync_auto_expand_folder_edge` (`src/main.rs:697-719`) runs each frame. It acts only when the "folder present" state changes. It starts with `prior_folder_present = true`, so the first frame with no folder opens Browse.

### 2.6 Center (stage)

- A `CentralPanel` holds one group frame with the stage pane (`src/main.rs:3894-3902`).
- The stage pane has a "▶ Stage" / "▼ Stage" toggle that hides the image, and a status line (`src/main.rs:3994-4029`).
- Status line text: "Loading…", "Cannot preview NAME: REASON", or "W×H".
- The image scales to fit and never grows above 1:1 (`src/main.rs:4031-4053`).
- Right-click on the image opens the shared context menu.

### 2.7 Right-click context menu

`render_image_context_menu` (`src/main.rs:4059-4102`) is shared by the stage and by list rows (`src/main.rs:3698`, `3811`). It has six items: "Save a copy…", "Move to…", "Resize copy", "Convert format" (jpg, png, webp), "Copy path", "Copy image". "Resize copy", "Convert format" and "Copy image" are off for files that the app cannot decode.

There is no visible hint that this menu exists. The only hint is the status text after a selection: "Use Image actions to open in feh, or right-click for Resize/Convert." (`src/main.rs:4351-4360`). That text is inside the closed Session status section.

### 2.8 Keyboard

The code has no keyboard shortcuts. `Key::`, `KeyboardShortcut` and `consume_shortcut` do not occur in `src/`.

### 2.9 Screenshot at start (2026-10-05, observed)

What the screenshot shows, top to bottom:

- The menu bar shows only "View".
- The center (about 72% of the width) is empty except for a small "☐ Stage  Loading…" chip in the top-left corner.
- The right panel shows "↑ Up", "Flat list" (selected) and "Folder tree", then an empty table with the headers "Folder Filename Status" and no message.
- Below the table: "☐ Details" and the faint label "Browse · actions · session · log · deps · formats".
- Seven closed sections: "Browse — No folder loaded", "Image actions — no selection", "Feh instances", "Session status — Showing 0 / 0 images", "Activity log — 4 events", "✓ Dependencies — all required tools OK", "Format discovery — 5 groups".
- A large empty area below the sections.
- No "Choose folder" button is visible.

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

These findings describe the dated baseline, not the partially implemented feature 019. Each has an ID (F-NN), evidence, user effect and current packet disposition. The packet references below replace the earlier P0/P1/P2 proposal labels; deferred items are not implementation obligations.

### F-01 The main action is hidden three levels deep

- **Status:** Verified placement; observed absence in the supplied screenshot.
- **Evidence:** "Choose folder" is a button at `src/main.rs:2751`. It is inside the Browse section (`src/main.rs:2898`), inside the Details drawer (`src/main.rs:2856-2884`), at the bottom of the Inspector.
- **Effect:** A new user sees no way to start. The auto-open rule opens Browse at start, but one click on "Details" or "Browse" hides the button again. Then the only way to load a folder is to find it again in a closed section.
- **Fix:** U1 visible Open folder and explanatory startup.

### F-02 The large area holds the least useful content

- **Status:** Verified.
- **Evidence:** The center holds only the stage (`src/main.rs:3894-3902`). The list, navigation, filter and all actions are in a panel of fixed width (440 px at 960 or 1280 wide) that you cannot resize (`src/main.rs:3463-3465`).
- **Effect:** In the supplied no-selection screenshot, about 72% of the window is empty; exact proportions vary with window size and scale. The list is cramped: the Filename column gets about 40% of 440 px, so long names cut off.
- **Fix:** U1 central browser and resizable preview.

### F-03 The stage shows "Loading…" when nothing is loading

- **Status:** Verified.
- **Evidence:** `stage_state` starts as `StageState::Loading` (`src/main.rs:137`). `kick_stage_decode_if_selection_changed` (`src/main.rs:3917-3925`) returns at once when `selected` and `stage_requested_path` are both `None`. Thus the `Failed { "No image selected" }` branch never runs at start, and the label stays "Loading…" until the user selects an image.
- **Effect:** The user thinks that the app is busy.
- **Fix:** U1 honest Idle/Loading state.

### F-04 User tasks and diagnostics are in one list

- **Status:** Verified.
- **Evidence:** Seven sections in one list (`src/main.rs:2889-3028`). Four are diagnostic (Session status, Activity log, Dependencies, Format discovery). They have the same weight as Browse and Image actions.
- **Effect:** The user must read seven headings to find the two that they need.
- **Fix:** U3 reachable secondary tools and compact status.

### F-05 Empty areas show no message

- **Status:** Verified, Observed.
- **Evidence:** With zero rows, `show_rows` draws nothing (`src/main.rs:3729`, tree `3870`). The flat list shows its headers above an empty box. When a filter matches nothing, the list is also empty with no message.
- **Effect:** The user cannot tell whether the app failed, is still loading, or has no data.
- **Fix:** U1 distinct empty, scanning and no-match states.

### F-06 The window lacks standard parts

- **Status:** Verified.
- **Evidence:** One menu, "View", with only size settings (`src/main.rs:3430-3459`). No File or Help menu. No bottom status bar. No keyboard shortcut suite was present.
- **Effect:** Users of other desktop apps look for File → Open and Ctrl+O first. Neither exists.
- **Fix:** U1 visible Open folder; U3 compact status. Additional menus and shortcuts are deferred.

### F-07 Image actions are only in a right-click menu

- **Status:** Verified.
- **Evidence:** Six actions exist only in `Response::context_menu` (`src/main.rs:4059-4102`). The only hint was in a closed Image actions section.
- **Effect:** Users who do not know to right-click do not find these actions.
- **Fix:** U2 visible Open in feh and More actions.

### F-08 Labels use developer words

- **Status:** Verified.
- **Evidence:** "Stage", "Inspector" (in README), "Session status", "Format discovery", "Feh instances", "Detect exotic formats (slow)" (`src/main.rs:2798`), "Recheck tools on PATH", status tag "magick · awaiting convert" (`src/ui_logic.rs:659`), "native".
- **Effect:** A user who is not a developer cannot tell what these controls do.
- **Fix:** U3 plain user-facing labels where controls move.

### F-09 Arrow symbols show as empty boxes

- **Status:** Observed; exact rendering cause needs reproduction.
- **Evidence:** The code draws "▶ Details", "▼ Details" (`src/main.rs:2858-2862`) and "▶ Stage", "▼ Stage" (`src/main.rs:3996-4000`). The screenshot shows "☐ Details" and "☐ Stage". The rendered disclosure symbols appear as empty boxes; font coverage or rendering is a likely cause, not independently reproduced here. Other symbols, for example "⬆" and "✅", show correctly.
- **Effect:** The toggles look like check boxes, not like expand controls.
- **Fix:** U1/U3 remove obsolete toggles and unsupported disclosure glyphs.

### F-10 Browse was closed at start in the screenshot

- **Status:** Observed, not reproduced.
- **Evidence:** The baseline auto-expand rule opens Browse on the first frame with no folder. The screenshot shows the drawer open but Browse closed. Possible causes: the user clicked Browse before the screenshot, or a bug in the edge logic.
- **Effect:** If it is a bug, a new user sees no "Choose folder" button at all.
- **Fix:** U1 keeps Open folder outside collapsible content; no need to reconstruct the screenshot’s unknown interaction history.

### F-11 The path is one label that you cannot click

- **Status:** Verified.
- **Evidence:** The path is one truncated label (`src/main.rs:3523-3531`). "⬆ Up" is the only way to go to a parent folder. There is no back or forward history.
- **Effect:** To go up three levels, the user clicks Up three times. A long path cuts off at the right, so the user sees the start of the path, not the current folder name.
- **Fix:** U1 preserves visible Up/current path with full-path hover. Richer breadcrumbs and history are deferred.

### F-12 Sort is far from the list

- **Status:** Verified.
- **Evidence:** Sort is a combo box in Browse (`src/main.rs:2815-2825`). The column headers are static text (`src/main.rs:3714-3721`).
- **Effect:** Users expect to click a column header to sort.
- **Fix:** U1 makes existing Sort visible near the list. Clickable sort headers are deferred.

### F-13 The "Details" breadcrumb looks like links

- **Status:** Verified.
- **Evidence:** "Browse · actions · session · log · deps · formats" is a weak `Label` (`src/main.rs:2866-2871`). It does nothing when clicked.
- **Effect:** Users click it and nothing happens.
- **Fix:** U3 removes the obsolete Details label.

### F-14 In-app text about ImageMagick is out of date

- **Status:** Verified.
- **Evidence:** `src/tool_caps.rs` says "ImageMagick (detection only)" and "may use ImageMagick (no convert yet)" (`src/tool_caps.rs:218`, `:231`). But Image Tools runs `magick convert` (`src/image_proc.rs:528`, `:556`). `tool_caps.rs:204` still lists "Quick resize (jpg/png/webp)", but spec 018 Batch 3 removed the "Quick resize 50% (demo)" button.
- **Effect:** The app gives wrong information about what it can do.
- **Fix:** U3 corrects existing capability descriptions.

### F-15 The window-size menu is in the only menu

- **Status:** Verified.
- **Evidence:** The View menu holds only size presets and a resizable check box (`src/main.rs:3430-3451`).
- **Effect:** The window manager already does this job. Real view options (list or tree, show preview, show log) are not in View.
- **Fix:** Keep existing window preferences. A broader View-menu redesign is deferred.

### F-16 The scan inventory is a multi-line block in a closed section

- **Status:** Verified.
- **Evidence:** `render_scan_inventory_banner` (`src/main.rs:3635-3657`) runs only inside Session status (`src/main.rs:3250`).
- **Effect:** The count of images and skipped files is hard to find.
- **Fix:** U3 compact count/status with reachable inventory details.

### F-17 Listing status is incorrectly used as decode capability

- **Status:** Verified by code reading; runtime cases have not been exercised in this review.
- **Evidence:** The scanner accepts gif/bmp (`src/scanner.rs:220`), but `Cargo.toml:25` enables jpeg/png/webp decoders. `file_status_decodable` accepts every `NativeListed` and `Converted` row (`src/ui_logic.rs:669`). `detect_converted_status` also marks an original file Converted merely because a processed sibling exists (`src/ui_logic.rs:961-997`). A JPEG sibling does not make the original HEIC/TIFF path decodable by `image::open`.
- **Effect:** The preview fails and list-row process actions may be offered for an unsupported target. Renaming “native” to “ready” would make the false promise worse.
- **Fix:** Separate inventory status from the specific target’s process capability. Enable gif/bmp decoders only with a written size/dependency rationale, or keep those process actions unavailable with an explanation. Test unsupported originals with processed siblings as well as GIF/BMP and corrupt supported-format files. Keep feh viewing, copy path, save-copy and move independent of preview support. A supported extension permits an attempt; it never proves valid image data.
- **Owner:** U2 capability work; it must not delay U1's visible browser outcome.

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

**Included:** before U1, record the current behavior and a same-build/same-fixture baseline using an existing workload if available; compare current source with Section 10 and note material conflicts; record retired 017/018 visual rules in new feature clarifications; update AGENTS.md plus .specify/feature.json when the new feature is created.

**Excluded:** application edits, guessing a feature number, changing pointers before feature creation is authorized, replaying historical tasks.

**Owner/prerequisite:** a Sol orchestrator (or another supported non-Astra primary) coordinates this packet before U1 and delegates bounded inspection, implementation, tests and documentation edits to Luna. Astra supplies only judgment on evidence provided; Astra does not inspect the filesystem, research, code or run tests. This document remains the mandatory design input while pointers are stale.

**Acceptance:** the new feature records this architecture, protected behaviors, packets and evidence plan; historical layout rules are explicitly superseded.

**Stop/escalate:** only for a material source conflict with the accepted outcome or unavailable required write; state the exact conflict or boundary.

**Source seams:** frame ordering and menu/panel layout are RustFehApp::update and render_top_menu_bar/render_inspector_side_panel/render_central_image_panel in src/main.rs. Folder handling is navigate_to_folder/pick_folder; list data is compute_list_indices and ImageListMetrics. The baseline source map is Section 10; use feature 019 and current symbols during implementation.

### 6.2 U1 — Make the browser the useful workspace

**Objective:** users can open a folder and find images in the large central area, with an honest preview state.

**Included:** one sequential owner for src/main.rs restructures frame/layout order; places visible Open folder, Up/current path, Filter and existing List/Tree controls by the browser; moves current virtualized flat/tree list and subfolder drill-down to the central workspace; adds a resizable right preview and usability-based narrow fallback with one active preview; distinguishes no-folder, scanning, completed-empty, no-match, scan-error and no-selection-idle states. Reuse current handlers, list/cache logic and stage behavior. Preserve existing navigation and sort choices.

**Excluded:** mandatory menu suite, third pane, new icon/app-icon, richer navigation/history, new sort modes, persistence, dependencies/modules, image decode or filesystem walks during listing.

**State boundary:** use existing scan outcomes and summarized warnings. `ScanMsg::Complete` carries `ScanResult`, not a new error result; show inaccessible-path and scan warnings honestly without adding scanner error plumbing solely to create a separate screen. Loading belongs only to current pending work. New shortcuts and sortable headers remain deferred.

**Owner/prerequisite:** one main.rs owner keeps layout ownership through U2/U3. Pure helper/test work may be separate only with disjoint files and explicit interfaces. U0 source map is the prerequisite.

**Validation/acceptance:** focused state tests when state logic changes. At 640x480, 960x720 and 1280x960, inspect no-folder and loaded states; interact with Open folder, Up/current navigation, Filter, List/Tree and narrow preview reveal. Include long names/paths. Confirm bounded virtualized list remains. Complete when the browser is immediately usable, list space dominates, preview can resize or be revealed, and states are distinct.

**Stop/escalate:** only for a measured conflict among virtualization, bounded layout and supported minimum size; report the case and alternatives.

**Ordered steps:** (1) preserve frame ordering constraints and place browser controls, (2) move existing list/subfolder renderers without rewriting cache/row logic, (3) add preview sizing and narrow fallback with one instance, (4) add/resolve state display from existing scan and stage state. **Source seams:** src/main.rs render_inspector_nav_strip, render_inspector_subfolder_drilldown, render_inspector_file_list, render_flat_image_list, render_tree_image_list, render_stage_pane, kick_stage_decode_if_selection_changed, poll_stage_decode, and update; src/ui_logic.rs compute/filter/sort/tree helpers. **Worker return:** changed paths, state transitions, screenshots/interactions, tests run and remaining evidence gaps.

### 6.3 U2 — Make selection actions clear and safe

**Objective:** selection visibly leads to opening the intended image in feh or choosing an applicable existing action.

**Included:** prominent Open in feh and More actions with the six existing actions; reuse current handlers/context menu; preserve target identity through filtering, pinned context, rename/move and feh round-trip; correct operation-specific capability checks so inventory status or a processed sibling cannot imply that the original is decodable. Keep filesystem actions available when valid and explain process failures accurately.

**Excluded:** new operations/backends, imported viewer features, generalized capability framework, module extraction, unrelated action-semantic changes.

**Owner/prerequisite:** same main.rs owner; focused pure capability logic stays in existing ui_logic.rs. U1 supplies the preview surface.

**Validation/acceptance:** existing action/round-trip tests plus focused cases for unsupported original with Converted sibling, corrupt supported extension and pinned/live target when affected. Interact with Open in feh and More actions. Labels/enabled actions must describe the actual target, and each current action must be reachable.

**Stop/escalate:** only for unresolved ambiguity over live versus pinned target; identify handler and state path.

**Ordered steps:** (1) map each existing context action to its current handler, (2) expose Open in feh and More actions on preview, (3) audit target resolution at each call site, (4) repair only the capability predicate and add focused cases. **Source seams:** src/main.rs render_image_context_menu, render_image_actions_body, render_image_actions_full, action_* methods, open_in_feh/open_in_feh_pinned; src/ui_logic.rs file_status_decodable and PanelContext; tests/integration/feature_016_actions.rs and feature_016_roundtrip.rs, tests/unit/ui_logic.rs. **Worker return:** action-to-handler map, target-safety audit, test output and actual visible-control interaction evidence.

### 6.4 U3 — Keep tools and recovery reachable

**Objective:** remove the crowded drawer without losing current tools, feedback or recovery.

**Included:** map drawer capabilities to the smallest reachable existing controls; keep compact count/progress/warnings and inventory/log access; keep Image Tools, Saved feh launches, Installed tools, Format support, recheck/install and log actions reachable; remove drawer and height allocation after mapping; make warnings dismissible without per-frame reopening. Reuse existing render bodies and current window/control mechanisms where they fit.

**Excluded:** one new window per tool, new app icon, third pane, command/capability framework, new preferences/persistence, duplicate tool implementations.

**Owner/prerequisite:** same main.rs owner after U1/U2. Preserve pinned-image safety and folder-scoped tool restrictions. Complete a capability-location map first.

**Validation/acceptance:** interact with current tool/recovery routes; cover missing feh, recheck, failed/cancelled scan, inventory and log. Confirm no drawer remains, no capability is lost and browsing remains unobscured.

**Stop/escalate:** only if a current capability has no reachable route; report capability, handler and smallest possible presentation.

**Ordered steps:** (1) inventory current Details drawer contents, (2) assign each capability an existing reachable control, (3) relocate/reuse render bodies, (4) remove drawer and obsolete height math, (5) exercise recovery routes. **Source seams:** src/main.rs render_inspector_sections_drawer_body, render_session_status_body, render_activity_log_body, render_deps_section_body, render_format_discovery_body, render_inspector_feh_instances, render_inspector_image_tools, detached-window rendering; src/ui_logic.rs AutoExpandState and drawer height helpers. **Worker return:** capability-location map, removed old-layout references, interaction evidence and tests.

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

These decisions govern feature 019. Implementation status and observed results live in its [validation record](../specs/019-browser-first-ux/validation-results.md); baseline descriptions and code references below remain dated evidence.

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

All paths are at commit `ad233f0`.

| Item | Location |
|------|----------|
| App start state, `stage_state: Loading` | `src/main.rs:137` |
| `navigate_to_folder` | `src/main.rs:681` |
| `sync_auto_expand_folder_edge` | `src/main.rs:697-719` |
| `pick_folder` | `src/main.rs:721` |
| Activity log body | `src/main.rs:1005` |
| Dependencies section | `src/main.rs:1213-1240` |
| Format discovery section | `src/main.rs:1241-1270` |
| Browse header label | `src/main.rs:1279-1293` |
| Image actions body, "Open in feh" | `src/main.rs:1330-1345` |
| feh spawn arguments | `src/main.rs:1560`, `src/ui_logic.rs:123` |
| Feh instances body | `src/main.rs:1781` |
| Image Tools panel | `src/main.rs:2692` |
| Browse controls ("Choose folder", Filter, Sort) | `src/main.rs:2747-2827` |
| Inspector layout (zones A to D) | `src/main.rs:2830-2884` |
| Details drawer sections | `src/main.rs:2889-3028` |
| Inspector width rule | `src/main.rs:3093-3190` |
| Session status body | `src/main.rs:3191-3260` |
| Detached window chrome and loop | `src/main.rs:3282-3420` |
| View menu | `src/main.rs:3430-3451` |
| Menu bar | `src/main.rs:3453-3459` |
| Inspector side panel | `src/main.rs:3461-3474` |
| Navigation strip (zone A) | `src/main.rs:3483-3536` |
| Subfolder list (zone B) | `src/main.rs:3542-3564` |
| File list (zone C) | `src/main.rs:3573-3622` |
| Scan inventory block | `src/main.rs:3635-3657` |
| Flat list row and headers | `src/main.rs:3667-3770` |
| Tree list | `src/main.rs:3775-3890` |
| Central panel | `src/main.rs:3894-3902` |
| Stage decode start | `src/main.rs:3917-3990` |
| Stage pane, toggle, status line | `src/main.rs:3994-4029` |
| Stage image | `src/main.rs:4031-4053` |
| Right-click menu | `src/main.rs:4059-4102` |
| `update` (frame order) | `src/main.rs:4326-4346` |
| `select_image` status text | `src/main.rs:4351-4360` |
| No wallpaper code (comment) | `src/main.rs:4669` |
| Window presets | `src/ui_logic.rs:35-49` |
| `showing_count_label`, `file_status_label` | `src/ui_logic.rs:645-662` |
| `format_inventory_bar` | `src/ui_logic.rs:694` |
| `file_status_decodable` | `src/ui_logic.rs:669-671` |
| Native extension list | `src/scanner.rs:220` |
| Stage decode (`decode_stage_rgba`) | `src/image_proc.rs:410-422` |
| `initial_open_sections` | `src/ui_logic.rs:1646` |
| Drawer height math | `src/ui_logic.rs:1664-1680` |
| `AutoExpandState` | `src/ui_logic.rs:1696-1790` |
| Tool timing and routing text | `src/tool_caps.rs:195-235` |
| ImageMagick convert calls | `src/image_proc.rs:528`, `src/image_proc.rs:556` |
| magick-cache calls | `src/image_proc.rs:253`, `:282`, `:327` |
