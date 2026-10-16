---
created_at: "2026-10-16T20:02:05.604643587+00:00"
id: "astromech-04bk"
evidence_type: "validation"
commit: "06e57df93c762e3446fcf1301685994c036625c7"
result: "pass"
captured_at: "2026-10-16T20:02:05.604512843+00:00"
target:
  kind: "issue"
  id: "astromech-xvjs"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-xvjs"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "At 06e57df: cargo fmt --all (clean), cargo clippy --workspace --all-targets -D warnings (clean), cargo test --workspace (all pass), scripts/check-deps.py OK, app/src-tauri cargo metadata --locked OK, app pnpm lint/check/build pass. Mutation checks: holding a transaction across the post makes queuing_never_waits_for_a_post_in_progress fail (10 s); the ApproachBody fixture with the real shape fails without the Body fix (63b35d3). Diagnostics screenshot (vite sample data) shows dates per row, e.g. 'Oct 16, 12:01:00 PM'."
updated_at: "2026-10-16T20:02:05.605238908+00:00"
---

At 06e57df: cargo fmt --all (clean), cargo clippy --workspace --all-targets -D warnings (clean), cargo test --workspace (all pass), scripts/check-deps.py OK, app/src-tauri cargo metadata --locked OK, app pnpm lint/check/build pass. Mutation checks: holding a transaction across the post makes queuing_never_waits_for_a_post_in_progress fail (10 s); the ApproachBody fixture with the real shape fails without the Body fix (63b35d3). Diagnostics screenshot (vite sample data) shows dates per row, e.g. 'Oct 16, 12:01:00 PM'.
