---
created_at: "2026-10-17T19:20:53.387179643+00:00"
id: "astromech-xtfj"
evidence_type: "validation"
commit: "274db91e27cb227724291c92e04e2554d8681889"
result: "pass"
captured_at: "2026-10-17T19:20:53.387137895+00:00"
agent_identity: "claude"
target:
  kind: "issue"
  id: "astromech-znvi"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-znvi"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Placement: the Voice tester page (where a phrase is heard and matched), not Settings, so the list sits next to the tests that teach it. Screenshot via scripts/ui at http://127.0.0.1:5173/#/voicetester (docs/guide/img/learned-phrases.png). Tests: grammar remove_alias, store forget_alias, core learned list/forget and Assistant::forget (asks again after forgetting), component tests for list, remove and error. Pre-PR checks pass: fmt, clippy, cargo test --workspace, check-deps, check-guide, pnpm lint/check/build/test. cargo deny not installed on host."
updated_at: "2026-10-17T19:20:53.387430551+00:00"
---

Placement: the Voice tester page (where a phrase is heard and matched), not Settings, so the list sits next to the tests that teach it. Screenshot via scripts/ui at http://127.0.0.1:5173/#/voicetester (docs/guide/img/learned-phrases.png). Tests: grammar remove_alias, store forget_alias, core learned list/forget and Assistant::forget (asks again after forgetting), component tests for list, remove and error. Pre-PR checks pass: fmt, clippy, cargo test --workspace, check-deps, check-guide, pnpm lint/check/build/test. cargo deny not installed on host.
