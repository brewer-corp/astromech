---
created_at: "2026-10-17T06:32:38.489171589+00:00"
id: "astromech-v4ui"
evidence_type: "test"
commit: "1c0409fc98b43747a18f6c45636b457d4b128c3d"
result: "pass"
captured_at: "2026-10-17T06:32:38.489077263+00:00"
agent_identity: "implementer (Claude Opus)"
target:
  kind: "issue"
  id: "astromech-znhx"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-znhx"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Review fixes at 1c0409f. sqqa: conditions measured by scripts/gen-exobiology.py from Spansh bodies search (17 Oct 2026; 90 species x 4 reference points x 100 nearest = 24,756 bodies; 18,567 used, 6,189 held out by id64 hash); no BioScan/ExplorerLite data; script-measured recall on all held-out bodies 17,908/18,047 = 99.2%, 11.5 candidates per body. cargo test -p astromech-exobiology --test predict: recall_on_held_out_bodies (400-body held-out fixture: 1,139/1,148 = 99.2%, 11.3 candidates, asserts >= 0.98 and <= 12.5), each_condition_gates (removing the body-class, star, volcanism check or ignoring pressure each fails it), the_catalogue_knows_names_from_the_journal_and_spansh (Fluctus 20,000,000); lib tests margins_scale_with_the_range_or_the_bound, volcanism_kinds. qn02: astromech-exploration unsold test back to 12 x 20,000,000. rtq0: astromech-announce --test biology each_system_starts_afresh and a_late_answer_awaited_in_one_system_is_not_said_in_the_next both fail with the per-system reset removed. Full gates /tmp/landq/gates.sh exit 0 (108 cargo suites, check-deps, shell lock, pnpm 182 tests)."
updated_at: "2026-10-17T06:32:38.489686389+00:00"
---

Review fixes at 1c0409f. sqqa: conditions measured by scripts/gen-exobiology.py from Spansh bodies search (17 Oct 2026; 90 species x 4 reference points x 100 nearest = 24,756 bodies; 18,567 used, 6,189 held out by id64 hash); no BioScan/ExplorerLite data; script-measured recall on all held-out bodies 17,908/18,047 = 99.2%, 11.5 candidates per body. cargo test -p astromech-exobiology --test predict: recall_on_held_out_bodies (400-body held-out fixture: 1,139/1,148 = 99.2%, 11.3 candidates, asserts >= 0.98 and <= 12.5), each_condition_gates (removing the body-class, star, volcanism check or ignoring pressure each fails it), the_catalogue_knows_names_from_the_journal_and_spansh (Fluctus 20,000,000); lib tests margins_scale_with_the_range_or_the_bound, volcanism_kinds. qn02: astromech-exploration unsold test back to 12 x 20,000,000. rtq0: astromech-announce --test biology each_system_starts_afresh and a_late_answer_awaited_in_one_system_is_not_said_in_the_next both fail with the per-system reset removed. Full gates /tmp/landq/gates.sh exit 0 (108 cargo suites, check-deps, shell lock, pnpm 182 tests).
