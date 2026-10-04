# rust-feh — Cline hardening boards

> Historical campaign snapshot: its generated Cline manifest and board roots reflect a separate checkout and do not define current UX requirements or current repository status. Use docs/UI-UX-REDESIGN.md for active product/UX authority and docs/PROJECT-HISTORY.md for shipped outcomes. Do not regenerate external boards from this checkout.

Task status lives in Cline; this is generated navigation, not a status board.

Baseline: `6c1823abe2f3835f7d74f630357e6610e9a3ac4b`. All cards explicitly select Cline / openai-codex / gpt-6-luna.
Cards begin in plan mode, auto-review disabled. No workers or native dependency links are started.

Open master: `cline --cwd /home/kkk/Apps/rust-feh --kanban`

Browser: http://127.0.0.1:3484/rust-feh

Start with SETUP-01, then a single SETUP-02 qualification; all implementation execution requires SETUP-01..04.

## Implementation boards

| ID | Group | Implementation | Native workspace |
|---|---|---|---|
| BOOT-01 | Startup and lifecycle | Startup detection and GUI failure recovery | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-boot-01` |
| BOOT-02 | Startup and lifecycle | Startup folder and first-frame behavior | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-boot-02` |
| LIFE-01 | Startup and lifecycle | Shutdown and owned resource lifetime | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-life-01` |
| PREF-01 | Preferences and persistence | Window size, lock and persistence | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-pref-01` |
| PREF-02 | Preferences and persistence | Launch-entry persistence and identity | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-pref-02` |
| PREF-03 | Preferences and persistence | Action destinations and safe configuration writes | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-pref-03` |
| SCAN-01 | Discovery | Native classification without pixel decoding | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-scan-01` |
| SCAN-02 | Discovery | Traversal, recursion and filesystem boundaries | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-scan-02` |
| SCAN-03 | Discovery | Warning classification and bounded summaries | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-scan-03` |
| SCAN-04 | Discovery | External format probing, budget and cancellation | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-scan-04` |
| SCAN-05 | Discovery | Network-path policy and expensive metadata | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-scan-05` |
| SCAN-06 | Scan lifecycle | Superseded worker cancellation | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-scan-06` |
| SCAN-07 | Scan lifecycle | Partial/completion ordering and generation rejection | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-scan-07` |
| SCAN-08 | Scan lifecycle | Converted merge and live inventory preservation | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-scan-08` |
| NAV-01 | Folder navigation | Unified picker, Up and breadcrumb navigation | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-nav-01` |
| NAV-02 | Folder navigation | Asynchronous subfolder discovery | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-nav-02` |
| NAV-03 | Folder navigation | Rescan selection and recursive-mode transitions | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-nav-03` |
| LIST-01 | List data and interaction | Path-aware search filtering | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-list-01` |
| LIST-02 | List data and interaction | Sort order and viewer-filelist parity | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-list-02` |
| LIST-03 | List data and interaction | Memoization and mutation invalidation | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-list-03` |
| LIST-04 | List data and interaction | Tree building, expansion and inventory counts | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-list-04` |
| LIST-05 | List data and interaction | Selection, scrolling and row targeting | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-list-05` |
| LIST-06 | List data and interaction | Flat/tree virtualization and long-row layout | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-list-06` |
| INSP-01 | Inspector | Inspector width and four-zone height budget | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-insp-01` |
| INSP-02 | Inspector | Drawer auto-expand state machine | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-insp-02` |
| INSP-03 | Inspector | Section detach and reattach lifecycle | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-insp-03` |
| INSP-04 | Inspector | Pinned image context and safe action scope | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-insp-04` |
| STAGE-01 | Selected-image stage | Decode, dimension bounds and aspect correctness | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-stage-01` |
| STAGE-02 | Selected-image stage | Decode supersession and texture lifecycle | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-stage-02` |
| STAGE-03 | Selected-image stage | Stage empty/error state and action gating | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-stage-03` |
| VIEW-01 | Viewer integration | Filelist, spawn arguments and isolated profile | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-view-01` |
| VIEW-02 | Viewer integration | Launch entries, folders and multiple viewers | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-view-02` |
| VIEW-03 | Viewer integration | Handoff validation and deleted-image fallback | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-view-03` |
| VIEW-04 | Viewer integration | Return selection, cross-folder landing and exit order | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-view-04` |
| VIEW-05 | Viewer integration | Viewer child ownership and handoff cleanup | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-view-05` |
| ACT-01 | File actions and clipboard | Save-copy destination and collision safety | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-act-01` |
| ACT-02 | File actions and clipboard | Loss-proof moves and post-move selection | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-act-02` |
| ACT-03 | File actions and clipboard | Output policies, backups and write boundaries | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-act-03` |
| ACT-04 | File actions and clipboard | Context-menu action dispatch and outcomes | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-act-04` |
| CLIP-01 | File actions and clipboard | Copy path, pixels and clipboard availability | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-clip-01` |
| IMAGE-01 | Image transformations | Resize dimensions, fit and filter semantics | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-image-01` |
| IMAGE-02 | Image transformations | Crop parsing, clamping and preview parity | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-image-02` |
| IMAGE-03 | Image transformations | Encoding, conversion and external fallback | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-image-03` |
| BATCH-01 | Batch and rename | Batch target snapshot, progress and cancellation | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-batch-01` |
| BATCH-02 | Batch and rename | Rename token expansion and collision preview | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-batch-02` |
| BATCH-03 | Batch and rename | Rename execution and rollback safety | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-batch-03` |
| BATCH-04 | Batch and rename | Processed assets, summaries and inventory integration | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-batch-04` |
| CACHE-01 | Cache and prepared viewing | Cache readiness, configuration and disabled mode | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-cache-01` |
| CACHE-02 | Cache and prepared viewing | Cache identity, freshness and hit/miss correctness | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-cache-02` |
| CACHE-03 | Cache and prepared viewing | Cache-hit output policy and backup preservation | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-cache-03` |
| CACHE-04 | Cache and prepared viewing | Precache jobs, progress and cancellation | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-cache-04` |
| CACHE-05 | Cache and prepared viewing | Prepare-fast materialization, launch and cleanup | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-cache-05` |
| CAP-01 | Capabilities and feedback | Tool detection, recheck and mid-session loss | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-cap-01` |
| CAP-02 | Capabilities and feedback | Format routes and advertised processing capability | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-cap-02` |
| CAP-03 | Capabilities and feedback | Dependency guidance and format UI | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-cap-03` |
| OBS-01 | Capabilities and feedback | Progress, status precedence and busy repaint | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-obs-01` |
| OBS-02 | Capabilities and feedback | Activity log retention, copy and error fidelity | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-obs-02` |
| VERIFY-01 | Verification infrastructure | Test isolation, fixtures and honest skip accounting | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-verify-01` |
| VERIFY-02 | Verification infrastructure | Benchmark fixtures, provenance and process ownership | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-verify-02` |
| VERIFY-03 | Verification infrastructure | Build, CI and packaging verification | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-verify-03` |
| VERIFY-04 | Verification infrastructure | Requirement and source coverage reconciliation | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-verify-04` |

## Exact persisted card IDs

| Review ID | Cline card ID | Native project |
|---|---|---|
| BOOT-01-01 | b70fb | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-boot-01` |
| BOOT-01-02 | 734a3 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-boot-01` |
| BOOT-01-03 | 21e6c | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-boot-01` |
| BOOT-01-04 | 5139a | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-boot-01` |
| BOOT-02-01 | b2709 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-boot-02` |
| BOOT-02-02 | 20d88 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-boot-02` |
| BOOT-02-03 | 89c01 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-boot-02` |
| BOOT-02-04 | 064bf | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-boot-02` |
| LIFE-01-01 | d3da5 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-life-01` |
| LIFE-01-02 | 6febe | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-life-01` |
| LIFE-01-03 | 7618e | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-life-01` |
| LIFE-01-04 | 64944 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-life-01` |
| PREF-01-01 | c965d | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-pref-01` |
| PREF-01-02 | 00b7e | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-pref-01` |
| PREF-01-03 | 678e9 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-pref-01` |
| PREF-01-04 | a8989 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-pref-01` |
| PREF-02-01 | 29168 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-pref-02` |
| PREF-02-02 | 32f0f | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-pref-02` |
| PREF-02-03 | 97d09 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-pref-02` |
| PREF-02-04 | df2a3 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-pref-02` |
| PREF-03-01 | 23e02 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-pref-03` |
| PREF-03-02 | 6c070 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-pref-03` |
| PREF-03-03 | acfe7 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-pref-03` |
| PREF-03-04 | 8b245 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-pref-03` |
| SCAN-01-01 | 8daa3 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-scan-01` |
| SCAN-01-02 | 118f8 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-scan-01` |
| SCAN-01-03 | 9334a | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-scan-01` |
| SCAN-01-04 | 6da40 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-scan-01` |
| SCAN-02-01 | c3c51 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-scan-02` |
| SCAN-02-02 | cbfef | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-scan-02` |
| SCAN-02-03 | bcb12 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-scan-02` |
| SCAN-02-04 | 5d41f | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-scan-02` |
| SCAN-03-01 | f3339 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-scan-03` |
| SCAN-03-02 | 3d664 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-scan-03` |
| SCAN-03-03 | 365f2 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-scan-03` |
| SCAN-03-04 | b8b01 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-scan-03` |
| SCAN-04-01 | 88690 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-scan-04` |
| SCAN-04-02 | 3cfb9 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-scan-04` |
| SCAN-04-03 | 2d7bf | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-scan-04` |
| SCAN-04-04 | c5300 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-scan-04` |
| SCAN-05-01 | 5630e | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-scan-05` |
| SCAN-05-02 | 6327f | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-scan-05` |
| SCAN-05-03 | ad807 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-scan-05` |
| SCAN-05-04 | eb86e | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-scan-05` |
| SCAN-06-01 | a59cb | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-scan-06` |
| SCAN-06-02 | e2a75 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-scan-06` |
| SCAN-06-03 | d6beb | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-scan-06` |
| SCAN-06-04 | 27049 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-scan-06` |
| SCAN-07-01 | 4b08e | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-scan-07` |
| SCAN-07-02 | 7e193 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-scan-07` |
| SCAN-07-03 | f7a6b | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-scan-07` |
| SCAN-07-04 | 23f1d | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-scan-07` |
| SCAN-08-01 | 84a0a | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-scan-08` |
| SCAN-08-02 | add7e | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-scan-08` |
| SCAN-08-03 | 4339b | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-scan-08` |
| SCAN-08-04 | 81b34 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-scan-08` |
| NAV-01-01 | 25b83 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-nav-01` |
| NAV-01-02 | 3f8a8 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-nav-01` |
| NAV-01-03 | 3bad7 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-nav-01` |
| NAV-01-04 | f4127 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-nav-01` |
| NAV-02-01 | 3b1ae | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-nav-02` |
| NAV-02-02 | c1f1e | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-nav-02` |
| NAV-02-03 | 7311f | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-nav-02` |
| NAV-02-04 | d2bd0 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-nav-02` |
| NAV-03-01 | 18e67 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-nav-03` |
| NAV-03-02 | d4d28 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-nav-03` |
| NAV-03-03 | 950cd | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-nav-03` |
| NAV-03-04 | 25050 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-nav-03` |
| LIST-01-01 | 7b19c | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-list-01` |
| LIST-01-02 | 1337f | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-list-01` |
| LIST-01-03 | 56274 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-list-01` |
| LIST-01-04 | dd86f | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-list-01` |
| LIST-02-01 | 97400 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-list-02` |
| LIST-02-02 | 5acf9 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-list-02` |
| LIST-02-03 | 5ac92 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-list-02` |
| LIST-02-04 | 98737 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-list-02` |
| LIST-03-01 | 2b5f8 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-list-03` |
| LIST-03-02 | ca222 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-list-03` |
| LIST-03-03 | 38ae4 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-list-03` |
| LIST-03-04 | ecc0e | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-list-03` |
| LIST-04-01 | e62d1 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-list-04` |
| LIST-04-02 | e87cb | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-list-04` |
| LIST-04-03 | 7ac91 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-list-04` |
| LIST-04-04 | 0bc10 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-list-04` |
| LIST-05-01 | c68be | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-list-05` |
| LIST-05-02 | be838 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-list-05` |
| LIST-05-03 | 707c9 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-list-05` |
| LIST-05-04 | 0ebe0 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-list-05` |
| LIST-06-01 | d5a5b | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-list-06` |
| LIST-06-02 | 9fff0 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-list-06` |
| LIST-06-03 | d2888 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-list-06` |
| LIST-06-04 | f333b | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-list-06` |
| INSP-01-01 | f905a | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-insp-01` |
| INSP-01-02 | 83713 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-insp-01` |
| INSP-01-03 | 36126 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-insp-01` |
| INSP-01-04 | ae88f | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-insp-01` |
| INSP-02-01 | 5e986 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-insp-02` |
| INSP-02-02 | 8cdee | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-insp-02` |
| INSP-02-03 | 68e7c | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-insp-02` |
| INSP-02-04 | 48be9 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-insp-02` |
| INSP-03-01 | 4a108 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-insp-03` |
| INSP-03-02 | 82890 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-insp-03` |
| INSP-03-03 | 9c6df | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-insp-03` |
| INSP-03-04 | 6290c | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-insp-03` |
| INSP-04-01 | c0b58 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-insp-04` |
| INSP-04-02 | ef052 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-insp-04` |
| INSP-04-03 | edfbf | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-insp-04` |
| INSP-04-04 | 6b43c | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-insp-04` |
| STAGE-01-01 | 85b30 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-stage-01` |
| STAGE-01-02 | 3e5da | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-stage-01` |
| STAGE-01-03 | 92e1a | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-stage-01` |
| STAGE-01-04 | 461e6 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-stage-01` |
| STAGE-02-01 | 2448e | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-stage-02` |
| STAGE-02-02 | 8df7d | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-stage-02` |
| STAGE-02-03 | f08c5 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-stage-02` |
| STAGE-02-04 | 82cfc | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-stage-02` |
| STAGE-03-01 | 0c903 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-stage-03` |
| STAGE-03-02 | d7b03 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-stage-03` |
| STAGE-03-03 | 854b9 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-stage-03` |
| STAGE-03-04 | 7ff21 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-stage-03` |
| VIEW-01-01 | d9501 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-view-01` |
| VIEW-01-02 | 91881 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-view-01` |
| VIEW-01-03 | fc41e | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-view-01` |
| VIEW-01-04 | d523b | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-view-01` |
| VIEW-02-01 | bbc34 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-view-02` |
| VIEW-02-02 | 942b5 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-view-02` |
| VIEW-02-03 | 9311c | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-view-02` |
| VIEW-02-04 | 57ae8 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-view-02` |
| VIEW-03-01 | 2ce30 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-view-03` |
| VIEW-03-02 | 16252 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-view-03` |
| VIEW-03-03 | c5564 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-view-03` |
| VIEW-03-04 | 29770 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-view-03` |
| VIEW-04-01 | ab647 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-view-04` |
| VIEW-04-02 | baff1 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-view-04` |
| VIEW-04-03 | 691e8 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-view-04` |
| VIEW-04-04 | 3c9f7 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-view-04` |
| VIEW-05-01 | c1b4a | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-view-05` |
| VIEW-05-02 | 9c0ad | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-view-05` |
| VIEW-05-03 | c2b32 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-view-05` |
| VIEW-05-04 | d715a | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-view-05` |
| ACT-01-01 | 6788f | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-act-01` |
| ACT-01-02 | 968c2 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-act-01` |
| ACT-01-03 | 7a063 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-act-01` |
| ACT-01-04 | 454a9 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-act-01` |
| ACT-02-01 | 05ecf | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-act-02` |
| ACT-02-02 | a8023 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-act-02` |
| ACT-02-03 | 1b4ce | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-act-02` |
| ACT-02-04 | 45928 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-act-02` |
| ACT-03-01 | 985b2 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-act-03` |
| ACT-03-02 | 2368d | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-act-03` |
| ACT-03-03 | f2cc4 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-act-03` |
| ACT-03-04 | ca512 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-act-03` |
| ACT-04-01 | a244b | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-act-04` |
| ACT-04-02 | 12951 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-act-04` |
| ACT-04-03 | 0a065 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-act-04` |
| ACT-04-04 | a78a1 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-act-04` |
| CLIP-01-01 | 8b729 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-clip-01` |
| CLIP-01-02 | d04fa | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-clip-01` |
| CLIP-01-03 | e88cb | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-clip-01` |
| CLIP-01-04 | d75a4 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-clip-01` |
| IMAGE-01-01 | 13eaa | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-image-01` |
| IMAGE-01-02 | a7205 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-image-01` |
| IMAGE-01-03 | 4fd4e | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-image-01` |
| IMAGE-01-04 | 11d9f | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-image-01` |
| IMAGE-02-01 | 1816d | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-image-02` |
| IMAGE-02-02 | 94f72 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-image-02` |
| IMAGE-02-03 | 194ee | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-image-02` |
| IMAGE-02-04 | 87fba | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-image-02` |
| IMAGE-03-01 | fd9e0 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-image-03` |
| IMAGE-03-02 | b66e9 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-image-03` |
| IMAGE-03-03 | be213 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-image-03` |
| IMAGE-03-04 | f5c1e | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-image-03` |
| BATCH-01-01 | 825aa | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-batch-01` |
| BATCH-01-02 | 8d085 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-batch-01` |
| BATCH-01-03 | fa818 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-batch-01` |
| BATCH-01-04 | 35a6f | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-batch-01` |
| BATCH-02-01 | f016c | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-batch-02` |
| BATCH-02-02 | 7ce92 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-batch-02` |
| BATCH-02-03 | d6f2e | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-batch-02` |
| BATCH-02-04 | 99c72 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-batch-02` |
| BATCH-03-01 | 516dc | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-batch-03` |
| BATCH-03-02 | 4b421 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-batch-03` |
| BATCH-03-03 | f7647 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-batch-03` |
| BATCH-03-04 | c7754 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-batch-03` |
| BATCH-04-01 | c301f | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-batch-04` |
| BATCH-04-02 | f98c2 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-batch-04` |
| BATCH-04-03 | 6eda4 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-batch-04` |
| BATCH-04-04 | fde24 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-batch-04` |
| CACHE-01-01 | c133a | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-cache-01` |
| CACHE-01-02 | 82d82 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-cache-01` |
| CACHE-01-03 | 2c936 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-cache-01` |
| CACHE-01-04 | dbf2f | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-cache-01` |
| CACHE-02-01 | a79b0 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-cache-02` |
| CACHE-02-02 | e30dc | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-cache-02` |
| CACHE-02-03 | edf31 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-cache-02` |
| CACHE-02-04 | 482cc | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-cache-02` |
| CACHE-03-01 | e2804 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-cache-03` |
| CACHE-03-02 | a03f0 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-cache-03` |
| CACHE-03-03 | f36e0 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-cache-03` |
| CACHE-03-04 | 3388e | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-cache-03` |
| CACHE-04-01 | b6c0d | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-cache-04` |
| CACHE-04-02 | e9e13 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-cache-04` |
| CACHE-04-03 | 66712 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-cache-04` |
| CACHE-04-04 | edbf8 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-cache-04` |
| CACHE-05-01 | 7958d | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-cache-05` |
| CACHE-05-02 | f6359 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-cache-05` |
| CACHE-05-03 | 1cafe | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-cache-05` |
| CACHE-05-04 | 956fa | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-cache-05` |
| CAP-01-01 | 566bb | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-cap-01` |
| CAP-01-02 | 5dcbf | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-cap-01` |
| CAP-01-03 | 29382 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-cap-01` |
| CAP-01-04 | c0730 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-cap-01` |
| CAP-02-01 | 26fa4 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-cap-02` |
| CAP-02-02 | 5d7b8 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-cap-02` |
| CAP-02-03 | 90c96 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-cap-02` |
| CAP-02-04 | 39d0d | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-cap-02` |
| CAP-03-01 | 0ab78 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-cap-03` |
| CAP-03-02 | 3c323 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-cap-03` |
| CAP-03-03 | fc18c | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-cap-03` |
| CAP-03-04 | df836 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-cap-03` |
| OBS-01-01 | 5de93 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-obs-01` |
| OBS-01-02 | 5b6a9 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-obs-01` |
| OBS-01-03 | b0743 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-obs-01` |
| OBS-01-04 | 1b8b7 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-obs-01` |
| OBS-02-01 | 79c7d | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-obs-02` |
| OBS-02-02 | caba6 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-obs-02` |
| OBS-02-03 | b5cb7 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-obs-02` |
| OBS-02-04 | 70b60 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-obs-02` |
| VERIFY-01-01 | 0f709 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-verify-01` |
| VERIFY-01-02 | 4cda4 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-verify-01` |
| VERIFY-01-03 | b6237 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-verify-01` |
| VERIFY-01-04 | 0d859 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-verify-01` |
| VERIFY-02-01 | 1c7ba | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-verify-02` |
| VERIFY-02-02 | fb175 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-verify-02` |
| VERIFY-02-03 | d3571 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-verify-02` |
| VERIFY-02-04 | 98230 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-verify-02` |
| VERIFY-03-01 | f1be9 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-verify-03` |
| VERIFY-03-02 | 1c1ac | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-verify-03` |
| VERIFY-03-03 | aa670 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-verify-03` |
| VERIFY-03-04 | e0dd7 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-verify-03` |
| VERIFY-04-01 | 87952 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-verify-04` |
| VERIFY-04-02 | 3617f | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-verify-04` |
| VERIFY-04-03 | 47b1e | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-verify-04` |
| VERIFY-04-04 | 89383 | `/home/kkk/.cline/kanban-review-workspaces/rust-feh/rust-feh-verify-04` |
| SETUP-01 | 01cf5 | `/home/kkk/Apps/rust-feh` |
| SETUP-02 | 94acc | `/home/kkk/Apps/rust-feh` |
| SETUP-03 | 3cf8e | `/home/kkk/Apps/rust-feh` |
| SETUP-04 | 06508 | `/home/kkk/Apps/rust-feh` |
| SETUP-05 | 12e36 | `/home/kkk/Apps/rust-feh` |
| GROUP-01 | cc829 | `/home/kkk/Apps/rust-feh` |
| GROUP-02 | 4490a | `/home/kkk/Apps/rust-feh` |
| GROUP-03 | a3696 | `/home/kkk/Apps/rust-feh` |
| GROUP-04 | ae142 | `/home/kkk/Apps/rust-feh` |
| GROUP-05 | 654a9 | `/home/kkk/Apps/rust-feh` |
| GROUP-06 | 201b7 | `/home/kkk/Apps/rust-feh` |
| GROUP-07 | 3a34e | `/home/kkk/Apps/rust-feh` |
| GROUP-08 | a175a | `/home/kkk/Apps/rust-feh` |
| GROUP-09 | 5e9b6 | `/home/kkk/Apps/rust-feh` |
| GROUP-10 | dcb74 | `/home/kkk/Apps/rust-feh` |
| GROUP-11 | 64687 | `/home/kkk/Apps/rust-feh` |
| GROUP-12 | 5a7e9 | `/home/kkk/Apps/rust-feh` |
| GROUP-13 | eddff | `/home/kkk/Apps/rust-feh` |
| GROUP-14 | e9a58 | `/home/kkk/Apps/rust-feh` |
| GROUP-15 | 16d43 | `/home/kkk/Apps/rust-feh` |
| JOURNEY-01 | 98871 | `/home/kkk/Apps/rust-feh` |
| JOURNEY-02 | f112c | `/home/kkk/Apps/rust-feh` |
| JOURNEY-03 | e7589 | `/home/kkk/Apps/rust-feh` |
| JOURNEY-04 | 94f06 | `/home/kkk/Apps/rust-feh` |
| JOURNEY-05 | 4a3dd | `/home/kkk/Apps/rust-feh` |
| JOURNEY-06 | bfc0e | `/home/kkk/Apps/rust-feh` |
