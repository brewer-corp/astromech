---
created_at: "2026-10-17T18:51:24.694431618+00:00"
id: "astromech-bi0n"
evidence_type: "validation"
commit: "00b899f72d164b787b946f89530fad341a26cc5b"
result: "pass"
captured_at: "2026-10-17T18:51:24.694382867+00:00"
agent_identity: "claude"
target:
  kind: "issue"
  id: "astromech-yerd"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-yerd"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Fail-before: with three old-copy lines restored (policy NeverAutomated, session Voice isn't loaded, health I can't switch) the new copy tests failed, 3 of them; restored, all pass. Pre-PR checks pass: fmt, clippy -D warnings, cargo test --workspace, check-deps, check-guide, pnpm check/build/test. cargo deny not installed on host."
updated_at: "2026-10-17T18:51:24.694772705+00:00"
---

Fail-before: with three old-copy lines restored (policy NeverAutomated, session Voice isn't loaded, health I can't switch) the new copy tests failed, 3 of them; restored, all pass. Pre-PR checks pass: fmt, clippy -D warnings, cargo test --workspace, check-deps, check-guide, pnpm check/build/test. cargo deny not installed on host.
