---
created_at: "2026-10-17T04:47:38.769467977+00:00"
id: "astromech-y7g3"
evidence_type: "test"
commit: "f20ba0ff4f5caa34b45678ae9535771105444a4c"
result: "pass"
captured_at: "2026-10-17T04:47:38.769414848+00:00"
agent_identity: "implementer (Claude Sonnet)"
target:
  kind: "issue"
  id: "astromech-8ozh"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-8ozh"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Failing test first: without AUTOINCREMENT new_messages_never_take_a_purged_messages_id posted the purged b and c, and migrating_a_v1_outbox_keeps_its_rows failed; with the v2 migration both pass. Full gates (/tmp/landq/gates.sh) pass at f20ba0f"
updated_at: "2026-10-17T04:47:38.769893732+00:00"
---

Failing test first: without AUTOINCREMENT new_messages_never_take_a_purged_messages_id posted the purged b and c, and migrating_a_v1_outbox_keeps_its_rows failed; with the v2 migration both pass. Full gates (/tmp/landq/gates.sh) pass at f20ba0f
