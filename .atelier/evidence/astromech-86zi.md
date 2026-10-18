---
created_at: "2026-10-17T21:41:27.994354696+00:00"
id: "astromech-86zi"
evidence_type: "test"
commit: "b87eecfd7f3311b7b5ff7fcfd19645d2580014bc"
result: "pass"
captured_at: "2026-10-17T21:41:27.994271672+00:00"
agent_identity: "implementer"
target:
  kind: "issue"
  id: "astromech-yil0"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-yil0"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Can fail: copied the 3200 sidecars, changed af_heart.clean.landing_gear.down-1 label to landing_gear.up, ran astromech-core --eval: report went to 3104/3136 with wrong execute 1, ship family 461/464 (99.4%), and the row listed 'WrongExecute label=landing_gear.up got=landing_gear.down'. Unit test eval::tests::groups_by_family_and_condition_and_lists_misses covers the same in the Markdown report; tests/voice_eval_phrases.rs failed (missing command) when srv_drive_assist.off was removed from the set."
updated_at: "2026-10-17T21:41:27.994763878+00:00"
---

Can fail: copied the 3200 sidecars, changed af_heart.clean.landing_gear.down-1 label to landing_gear.up, ran astromech-core --eval: report went to 3104/3136 with wrong execute 1, ship family 461/464 (99.4%), and the row listed 'WrongExecute label=landing_gear.up got=landing_gear.down'. Unit test eval::tests::groups_by_family_and_condition_and_lists_misses covers the same in the Markdown report; tests/voice_eval_phrases.rs failed (missing command) when srv_drive_assist.off was removed from the set.
