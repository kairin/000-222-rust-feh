# Aspect-ratio helper evidence

Queue item: `qitem-20260926110950-d9b804dc`

Candidate changes:

- `src/image_proc.rs`: adds `aspect_ratio(width: u32, height: u32) -> Option<f64>`.
- `tests/unit/image_proc.rs`: covers landscape, portrait, zero width, zero height,
  both dimensions zero, and `u32::MAX` width.

TDD evidence: before the implementation, the focused test failed to compile because
`rust_feh::image_proc::aspect_ratio` did not exist. After implementation, it passed.

Verification performed:

- `cargo check` — exit 0.
- `cargo test --test unit_image_proc` — 8 passed, 0 failed.
- `cargo clippy --all-targets` — exit 0; it emitted 11 existing warnings in
  `src/tool_caps.rs`, `src/ui_logic.rs`, and unrelated test files, with none in
  this candidate.
- `git diff --check -- src/image_proc.rs tests/unit/image_proc.rs` — no output.

Limitation: `cargo fmt --check` reports pre-existing formatting drift outside this
candidate, so it is not a clean repository-wide formatting signal.
