# UI/UX baseline, 2026-10-05 (historical)

This is the historical pre-feature-019 baseline at commit `ad233f0`. Line references are stale and the Inspector/drawer UI is retired. The current authority is [UI-UX-REDESIGN.md](UI-UX-REDESIGN.md) and [specs/019-browser-first-ux/](../specs/019-browser-first-ux/).

---

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
