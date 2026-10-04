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
