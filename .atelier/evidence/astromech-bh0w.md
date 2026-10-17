---
created_at: "2026-10-17T03:11:51.258977930+00:00"
id: "astromech-bh0w"
evidence_type: "test"
commit: "49e9c32d13759a0f1a2d6c3e38975d05c0c5c69f"
result: "pass"
captured_at: "2026-10-17T03:11:51.258915183+00:00"
target:
  kind: "issue"
  id: "astromech-ew3x"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-ew3x"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "crates/astromech-announce/tests/signals.rs: each signal type in a fixture honk (conflict zones by intensity, Thargoid CZ/signal/Titan, Powerplay CZ/signal, megaship) gets its line once per system, stations/carriers quiet, resets on FSDJump; fails with the once-per-system check disabled. Catch-up replay silent (replay.rs). fmt, clippy -D warnings, cargo test --workspace, check-deps, tauri cargo metadata all pass."
updated_at: "2026-10-17T03:11:51.259342180+00:00"
---

crates/astromech-announce/tests/signals.rs: each signal type in a fixture honk (conflict zones by intensity, Thargoid CZ/signal/Titan, Powerplay CZ/signal, megaship) gets its line once per system, stations/carriers quiet, resets on FSDJump; fails with the once-per-system check disabled. Catch-up replay silent (replay.rs). fmt, clippy -D warnings, cargo test --workspace, check-deps, tauri cargo metadata all pass.
