# .agents/ENV.md — Environment and tooling

Re-verify before relying on anything time-sensitive.

## Repo
- Remote: `origin = https://github.com/kairin/000-222-rust-feh.git`. Default branch: `main`.
- Branch names are `YYYYMMDD-HHMMSS-short-name`. Never delete branches.

## Spec Kit
- Version and integration come from `.specify/init-options.json` (`speckit_version`,
  `integration`) and `.specify/integration.json`. Read those files for current values.
- `.specify/feature.json` is local state. It is ignored by `.specify/.gitignore` and not
  tracked. It can be missing or differ between machines.
- Spec Kit skills use the hyphen form (`speckit-specify`, `speckit-plan`, and so on).

## CI and local checks
- CI (`.github/workflows/ci.yml`) runs `cargo test --verbose` and
  `cargo clippy -- -D warnings`. There is no `cargo fmt` step.
- `rustfmt` and `clippy` may not be installed on a given machine. If a tool is missing,
  report the check as not run.
- Toolchain: Rust stable, edition 2021.

## Dependabot
- There is no `.github/dependabot.yml`. Dependabot security alerts formerly came
  only from `archive/original-nfeh`, the retired nfeh code, which was removed on 2026-11-23
  (preserved at tag `archive-original-nfeh-final`). Those alerts should close; treat any
  remaining alert as a current-app issue.
