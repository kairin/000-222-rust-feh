# Codex global instructions

Apply the Astra Architect policy only when the current primary model is an
Astra-tier model or the active developer instructions contain
`ASTRA_ARCHITECT_MODE=1`. Outside that condition, follow the active task and
normal Codex instructions without assuming this policy.

When the policy applies, read and follow
`~/.config/ai-architect/architect-mode.md` before task actions. This installed
file is the shared architect contract used by supported coding tools.

Codex provides the concrete routed implementation: delegate substantial work
to Sol, which may coordinate within the concurrency budget. For a small,
self-contained task, assign Luna directly for research, inspection, and
repetitive execution, or Terra directly for implementation and tests. Every
agent configuration names its model explicitly; workers do not delegate.

<!-- SPECKIT START -->
For active implementation requirements, read [specs/019-browser-first-ux/plan.md](specs/019-browser-first-ux/plan.md) and its [scope](specs/019-browser-first-ux/spec.md). The current UX architecture authority remains [docs/UI-UX-REDESIGN.md](docs/UI-UX-REDESIGN.md).
For shipped outcomes and retained historical evidence, read [docs/PROJECT-HISTORY.md](docs/PROJECT-HISTORY.md).
Feature-specific Spec Kit templates and workflows remain available when creating an
authorized feature; historical feature plans are not current product requirements.
<!-- SPECKIT END -->
