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
