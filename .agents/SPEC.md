# Class-B definition of done

This file applies to cross-cutting work not governed by an explicitly authorized feature plan. Active product and UX requirements are in [docs/UI-UX-REDESIGN.md](../docs/UI-UX-REDESIGN.md); shipped outcomes and deferrals are in [docs/PROJECT-HISTORY.md](../docs/PROJECT-HISTORY.md).

## Product yardstick

- rust-feh is a Linux-first, lightweight, no-network feh orchestrator. The GUI owns browsing, filtering, sorting, selection and launch; feh owns full viewing.
- Preserve existing behavior and architecture constraints in the active UX plan.
- For code changes, run relevant tests, `cargo clippy -- -D warnings` and a release build as appropriate to the change. Report any unavailable check.
- Optional B-5 validation-report parity for 005/008/009/013/014 remains deferred; do not create reports unless specifically requested.
