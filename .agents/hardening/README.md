# Cline / Luna implementation hardening

> Historical campaign snapshot: its generated Cline manifest and board roots reflect a separate checkout and do not define current UX requirements or current repository status. Use docs/UI-UX-REDESIGN.md for active product/UX authority and docs/PROJECT-HISTORY.md for shipped outcomes. Do not regenerate external boards from this checkout.

Approved by the maintainer on 2026-09-27. This campaign reviews existing rust-feh
behavior; it does not authorize new features, product edits, publishing, or agents
starting other agents merely because a board exists.

## Authority and boundaries

- Cline kanban is the campaign task-status source of truth. Its generated cards describe
  source/test coverage for pinned baseline `6c1823a`; old feature-spec checks are historical
  baseline criteria, not current layout requirements. Current product/UX authority is
  `docs/UI-UX-REDESIGN.md`; shipped outcomes and evidence are in `docs/PROJECT-HISTORY.md`.
  This directory holds campaign inventory and evidence references, not product requirements.
- Campaign workers use **Cline / openai-codex / gpt-6-luna**, explicitly pinned per
  card. This maintainer-selected routing replaces the historical Haiku/Sonnet/Opus
  dispatch roles for this campaign only; all architectural, safety, and publishing
  restrictions remain. Do not amend the project constitution.
- There is no worker fan-out during board creation. Qualification comes first.
  Start with one worker; at most two non-overlapping review tasks thereafter unless
  the coordinator explicitly expands capacity. Budget and time-box are established
  before dispatch, not invented by workers.
- No commits, pushes, PRs, native dependency links, automatic reviews/commits, or
  product changes are authorized by seeding these boards. Reviews start in plan
  mode. Execution needing fixtures/reports/tests requires Act-mode authorization.
- An implementation task is created only for a confirmed defect or measured
  bottleneck, with an explicit write scope and acceptance criteria. Do not create
  speculative fixes or require every implementation to be changed.

## Native board topology

The project board belongs to `/home/kkk/Apps/rust-feh`. Each implementation board
uses a separate detached Git worktree under
`/home/kkk/.cline/kanban-review-workspaces/rust-feh/`. They share Git objects, not
working files. They are persistent board roots, NOT disposable task worktrees.
Do not delete them when a task completes.

All board roots are pinned to the baseline recorded in `manifest.json`. Cline may
create another task checkout when Play is clicked: resolve that checkout with
`git rev-parse --show-toplevel`. Canonical source paths in prompts are references;
read and test the corresponding files in the assigned checkout. Never edit the
main checkout or another board root. Refreshing baselines is a coordinator action.

The user's existing tracked and untracked edits remain in the original checkout.
They are NOT silently copied, committed, reset, stashed, or assumed to be in a
worker checkout. The main setup task must disposition that delta before reviews
claim to cover the live working tree. The initial review baseline is committed
code; the uncommitted aspect-ratio helper is a separately identified candidate.

## Navigation and relationships

- `INDEX.md` and project group cards enumerate every implementation board.
- Each board has four bounded cards: contract/coverage, normal behavior, adverse
  behavior and interactions, and measured performance/closure.
- Group membership is informational. `related`, `depends-on`, and `conflicts-with`
  are different relationships. No native kanban dependency arrows are seeded:
  those can auto-start workers. Prerequisites are named in the card and checked
  by its worker/coordinator. A blocked task must stop without proceeding.
- Shared symbols have one primary review owner. Journey tasks verify seams instead
  of duplicating implementation reviews. A coverage audit reconciles all active
  entry points, helpers, requirements, and tests against the initial inventory.
- Features 002 and 004 are traced through their successors; 015 remains deferred.
  The archived nfeh application is not executable campaign scope.

## Worker acceptance contract

Read the card, this file, the assigned spec, and the repository instructions. Check
the actual provider/model/session mode, project and checkout paths, baseline, dirty
state, prerequisites, and allowed writes. A model string stored on a card is NOT
proof the provider accepted it; never silently fall back to Astra or another model.

For each required scenario record **executed-pass / executed-fail / skipped /
blocked / not-applicable-with-reason**. Early-return tests are not behavior passes.
Count the selected tests; a filter matching zero tests is a failed verification.
Do not weaken assertions or ignore errors to obtain green output.

Use synthetic images and disposable directories. Scope HOME/XDG/cache and output
paths to the task. Preserve the absolute RUSTUP_HOME/CARGO_HOME if HOME is isolated.
Do not source or copy credential-bearing environment files. A private clipboard
session is required before clipboard mutation tests; DISPLAY alone is insufficient.
No personal image folders, production caches, personal feh config, system installs,
network mounts, or system-wide process killing. Terminate only child PIDs you own.
The legacy resource script uses `pkill`; it is not safe to run unmodified.

Test-selection commands in cards are discovery/baseline commands, not proof every
acceptance criterion has a test. Inventory selected names and inspect skip paths
first. Set up a reproducible harness for missing cases in an authorized isolated
checkout; record a blocker if the environment cannot run it. Do not treat an
argument-vector unit test as a GUI journey or a mock as proof of an external tool.

Measure the actual build at the recorded revision, not a stale root executable.
Metadata-only fake JPEGs cannot validate decoding. Use separate real-image fixtures.
Performance comparisons need the same fixture/build mode, multiple repetitions,
reported spread, and cold/warm distinctions. Establish required thresholds before
changing code; a speedup must preserve outputs, safety, and relevant resource budgets.

Persist exact commands, exit codes, assertions, logs, baseline/candidate hashes,
fixture recipe, findings, and remaining uncertainty in the task's assigned external
evidence directory. A plan-mode task returns its report in-session until report
writes are authorized. Evidence must survive task-worktree cleanup.

## Closure

Findings are verified behavior, confirmed defect, measured optimization opportunity,
documentation mismatch, or blocked/unverified. "No change justified" is valid.
Blocked GUI/cache/network checks remain visible and do not become successful by
waiver or because adjacent unit tests passed. The reviewer records any explicit
maintainer risk acceptance separately from verified behavior.

A board is verified only for its recorded candidate and environment, after required
checks and relevant journey evidence are complete. A worker finishing a report is
not board closure. Security-sensitive file operations and asynchronous-state fixes
receive an independent session review at the integrated change boundary.

## Setup artifacts

- `catalog.py`: initial implementation inventory (definitions, not task status).
- `manifest.json`: generated exact source/test/case mapping and baseline.
- `kanban_setup.py`: validates and seeds through the supported kanban CLI; never
  directly writes the live board JSON, starts tasks, or creates dependency links.
- `INDEX.md`: generated native board navigation and stable task IDs.
- External control/evidence directory:
  `/home/kkk/.cline/kanban-review-workspaces/rust-feh-control/`.

Run the seeder with no arguments to validate only. `--seed` creates missing boards
and cards idempotently; `--verify` checks persisted cards through the CLI. Never
delete existing cards to make a rerun succeed.

## Open and inspect

```sh
cline --cwd /home/kkk/Apps/rust-feh --kanban
```

Use the master board's `GROUP-*` index cards or the Projects sidebar to enter a
`rust-feh-<implementation-id>` board. Each card begins with its stable `Review-ID`;
the generated index also records its native Cline card ID. The 15 groups are
navigation relationships, not a claim that the UI provides nested native boards.

Do **not** use **Start all backlog tasks**: prerequisite relationships are prompt
checks, not an enforced scheduler. Start `SETUP-01` individually, then qualify one
Luna session with `SETUP-02`. A board card records model selection, not proof of
model availability or a guarantee of successful execution.

Auxiliary board-root worktrees intentionally include no shared ignored directories
or symlinked credentials. Before executing a master-board task, inspect Cline's
generated task checkout for automatically inherited ignored-path symlinks and
use isolated build/config paths. Never source an inherited `.envrc` or `.env`.

The checked-out board roots consume local disk and Git worktree registrations.
Keep them while their boards are in use. Any future removal must first preserve
board data, evidence, and local changes, then use Git's worktree management; do
not recursively delete them as ordinary temporary directories.
