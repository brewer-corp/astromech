---
created_at: "2026-10-17T18:39:18.121034128+00:00"
id: "astromech-sec7"
evidence_type: "validation"
commit: "0a1613dfbe218c1c6c4f7429ea073809287ec935"
result: "pass"
captured_at: "2026-10-17T18:39:18.120981670+00:00"
agent_identity: "claude"
target:
  kind: "issue"
  id: "astromech-g1ys"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-g1ys"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full pre-PR checks run on Linux: fmt, clippy -D warnings, cargo test --workspace, check-deps, check-guide, pnpm lint/check/build/test all pass. cargo deny not installed on this host (no dependency change except a dev-dependency on an existing workspace crate). Test fails before (Handbrake missing from VOICE_ACTIONS) and passes after."
updated_at: "2026-10-17T18:39:18.121316445+00:00"
---

Full pre-PR checks run on Linux: fmt, clippy -D warnings, cargo test --workspace, check-deps, check-guide, pnpm lint/check/build/test all pass. cargo deny not installed on this host (no dependency change except a dev-dependency on an existing workspace crate). Test fails before (Handbrake missing from VOICE_ACTIONS) and passes after.
