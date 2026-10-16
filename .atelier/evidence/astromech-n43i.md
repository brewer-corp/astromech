---
created_at: "2026-10-16T22:10:05.949853827+00:00"
id: "astromech-n43i"
evidence_type: "validation"
commit: "ce2de6e42a07dc41338407c665bad8cc5984b40a"
result: "pass"
captured_at: "2026-10-16T22:10:05.949775651+00:00"
target:
  kind: "issue"
  id: "astromech-zig3"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-zig3"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Checks run after the last change: cargo fmt --check, clippy --workspace --all-targets --locked -D warnings, cargo test --workspace --locked (0 failures), scripts/check-deps.py, app/src-tauri cargo metadata --locked (shell lock current; the shell itself builds only on Windows CI, its two new commands were type-checked in a scratch crate), app pnpm lint/check/build/test (72 tests). Replay of the docking fixture shows the facts (crates/astromech-station/tests/dock.rs replaying_a_docking_shows_the_facts); a book reopened read-only after the journal is gone returns the station for a module seen there (a_later_offline_query_returns_the_station); component tests in app/src/modules/station/View.test.ts; scripts/ui screenshot of #/station read and checked. Not covered: the live app on Windows, the voice query."
updated_at: "2026-10-16T22:10:05.950302653+00:00"
---

Checks run after the last change: cargo fmt --check, clippy --workspace --all-targets --locked -D warnings, cargo test --workspace --locked (0 failures), scripts/check-deps.py, app/src-tauri cargo metadata --locked (shell lock current; the shell itself builds only on Windows CI, its two new commands were type-checked in a scratch crate), app pnpm lint/check/build/test (72 tests). Replay of the docking fixture shows the facts (crates/astromech-station/tests/dock.rs replaying_a_docking_shows_the_facts); a book reopened read-only after the journal is gone returns the station for a module seen there (a_later_offline_query_returns_the_station); component tests in app/src/modules/station/View.test.ts; scripts/ui screenshot of #/station read and checked. Not covered: the live app on Windows, the voice query.
