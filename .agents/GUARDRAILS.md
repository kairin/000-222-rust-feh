# .agents/GUARDRAILS.md — Operational guardrails (bind every phase, every agent)

Violating these is a stop-the-line event. Read alongside `.agents/CONSTITUTION.md` and your
task file before doing anything.

## Cost governance
- Before ANY fan-out: lead announces model tier + agent count + rationale.
- Default to cheapest tier that fits: **Haiku** = reading, mechanical coding; **Sonnet** =
  orchestration, review, search; **Opus** = ONLY on a cited escalation trigger (see
  Delegation model below). Never an Opus fleet reflexively.
- Concurrent-agent cap: ≤ 8 live agents unless the maintainer approves more.
- Surface running spend at each phase boundary.

## Git & commit discipline
- ONLY orchestrators (Sonnet) and the lead commit. Haiku implementers return diffs; they do
  not commit or push.
- No push / no PR without maintainer approval. Never delete branches.
- Branch names are `YYYYMMDD-HHMMSS-short-name`.
- Commit a GREEN checkpoint (compiles + tests pass) between task sets.

## Parallel isolation
- Concurrent task sets that could touch overlapping files run in separate git worktrees
  (Agent-tool `isolation: "worktree"`). If two sets share files, serialize instead.
- Never two live agents writing the same file. `tasks.md` files have a single writer: the
  owning orchestrator.

## Security checkpoint (non-negotiable for the integration workstreams)
- rust-feh exists because nfeh was retired over CVEs (issue #35). Both integrations expand
  attack surface: caption spawns Python/GPU subprocesses (argument injection, path handling,
  `direnv exec` trust, HF_TOKEN handling); image-tools dlopens libheif at runtime.
- `/security-review` runs on each workstream's diff BEFORE merge, with explicit attention to
  subprocess argument construction, untrusted path handling, and any dlopen loader.
  A failed security review is a hard block.
- Additional standing rule: **caption's `batch_caption.py` ships images to a remote HF Space**
  — any integration of that path is a network-egress + privacy event and must be explicit,
  opt-in, and documented in the feature spec. rust-feh itself stays no-network.

## Repo constraints for all implementers
- Keep logic OUT of `main.rs` (`src/main.rs` is the largest file; check `wc -l src/main.rs` — biggest liability). Testable homes: `ui_logic.rs`,
  `image_proc.rs`, `tool_caps.rs`, `scanner.rs`, `types.rs`.
- 50-line functions, complexity caps. Clippy `-D warnings` (CI runs it). Run `cargo fmt` when available; no workflow step checks it.
- Small branches, green CI; PR discipline as per repo history.

## Delegation model (Phase 3)
- **Sonnet orchestrator** owns one task set / one class-A feature: decompose, dispatch Haiku,
  integrate, keep the feature's tasks.md checkboxes current.
- **Haiku implementers**: one well-specified subtask each, small blast radius.
- **Opus specialist** only on cited triggers: (a) Haiku fails acceptance twice; (b) cross-
  cutting / architectural / concurrency / security-sensitive change (image-tools merge design,
  any main.rs restructuring); (c) genuine ambiguity after re-reading the spec; (d) subtask
  touches > 5 files or breaks a public contract.

## Resume durability
- `docs/UI-UX-REDESIGN.md` is the active product/UX authority and `docs/PROJECT-HISTORY.md`
  is the shipped-outcome and evidence ledger. `.agents` files contain cross-cutting rules
  and routing; feature task files are used only for authorized new implementation.

## UX authority
The active findings, decisions, and review outcomes are maintained in `docs/UI-UX-REDESIGN.md`; shipped decisions are in `docs/PROJECT-HISTORY.md`. Future changes to accepted UX scope require explicit maintainer authorization and an update to the active authority. Do not reopen retired feature-spec wording or add another generic reviewer loop.
