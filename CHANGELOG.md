## [2026-11-23] — Documentation and repository cleanup

Cleanup after a multi-architect staleness audit (#178–#193). Behaviour is unchanged. One tool label was renamed, and the in-app install hints are now distro-neutral (#190).

### Changed

- Stale images and chat text were removed from the docs. Feature 019 is marked shipped, and feh key `1` is documented (#178). Key `1` works only in a feh window opened with Open in feh, not in Saved feh launches or Launch feh on optimized.
- The pre-019 UI/UX baseline moved to `docs/UI-UX-BASELINE-2026-10-05.md` (#184).
- Stale comments were refreshed, and the stale Quick resize label was renamed (#180).
- Agent files were refreshed. The global Codex block was removed from `AGENTS.md`, and the stale board, backlog and three unused skills were removed (#182).
- Repo sudo-askpass helpers and the env secret-loader examples were removed (#183). `.envrc.example` now uses `source_up_if_exists`.
- A root MIT `LICENSE` was added and linked from the README (#185).
- CI now runs `cargo test --locked` and `cargo clippy --locked --all-targets -- -D warnings` with read-only token permissions, and 17 clippy lints in test code were fixed (#190).
- The "no active feature" status in `AGENTS.md` moved outside the SPECKIT markers so the agent-context script cannot overwrite it. `GUARDRAILS.md` lost its retired campaign sections (#191).
- README, CHANGELOG, PROJECT-HISTORY and the UI/UX docs were aligned with the current code (#192).
- Constitution 1.0.3 (owner approved): feature branches are named `YYYYMMDD-HHMMSS-short-name`, and Principle III names `rust_feh::<module>` paths instead of crate-root re-exports (#193).
- RHEL 10 `dnf` install steps were verified and added to the README (#186). Item G001 is closed. The README apt package list was aligned to CI (#186).
- Stale archive wording was fixed, and `.claude/worktrees/` is now ignored (#189).
- The constitution and docs links to issue #35 were repointed (#187).
- Follow-up consistency PRs after #189 are not listed: none were found in `git log` or `gh pr list` when this entry was updated.

### Removed

- Local Spec Kit state was untracked. The stale hermes manifest and orphan scripts were removed (#179).
- The direct `egui` dependency was dropped (#180).
- Unused `RenamePattern`, `StagedImage`, pre-cache and materialize APIs, and unused `lib.rs` re-exports were removed (#181).
- `archive/original-nfeh` was removed (#187). The code is preserved at tag `archive-original-nfeh-final`. The constitution Archive paragraph was amended to 1.0.2.

## [2026-11-22] — feh key 1 copies to picked

### Added

- In the feh viewer, key `1` copies the shown image to `~/Pictures/picked` (#177). The app passes `--action1` with `mkdir -p … && cp -n %F …`, so the folder is created if missing and existing files are never overwritten.

## [2026-10-11] — Website artifact intent

- Added docs/website-artifact-intent.md with the current implementation, adoption tasks, source authority and acceptance checks.
- This proposal does not change application code, dependencies, data or publishing.

## [2026-10-05] — Browser-first UX

### Changed

- The central workspace is now the folder browser, with a resizable right Preview (feature 019, #168). Narrow windows show the preview with a Back to list control.
- Open in feh and More are visible for the current selection. Open in feh is a round-trip that returns the viewed image to the preview.
- Scan settings, Image Tools and pinning, saved feh launches, status and inventory, and the activity log moved to the Tools menu.
- Requirements and history were merged into `docs/UI-UX-REDESIGN.md` and `docs/PROJECT-HISTORY.md`.

## [2026-10-05] — Bash replaces fish

### Changed

- README and docs shell blocks now use bash syntax (`source ~/.cargo/env`, `make -j"$(nproc)"`).
- `build-and-place.sh` now runs under bash (it used fish syntax before).
- `scripts/sudo-askpass.sh` now installs only the askpass helper. The 000-0-dotfiles repo owns shell config in `~/.bashrc.d/`.

### Removed

- Removed `scripts/sudo-askpass.fish` (fish shell config).

## [2026-09-13] — GitHub delivery cleanup

### Changed

- Reviewed GitHub delivery configuration for this repository.
- Added minimal protection to the default branch. Direct commits and pushes remain allowed; pull requests, reviews, status checks, and agent approvals are not required.
- Preserved project-specific validation and intentional publishing workflows.

### Removed

- Removed the stale required ci status gate while retaining the actual CI workflow.
