---
created_at: "2026-10-16T22:10:15.120809687+00:00"
id: "astromech-51i6"
evidence_type: "diagnosis"
commit: "f8c900b66d63f1a7cbc6712e74b8ab92dd1b028c"
result: "pass"
captured_at: "2026-10-16T22:10:15.120698911+00:00"
agent_identity: "claude-implementer"
target:
  kind: "issue"
  id: "astromech-qhi4"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-qhi4"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Reproduced before the fix: the new test trip_skips_engineers_capped_below_the_grade_for_this_module (beam laser, Overcharged G4, The Dweller unlocked, Broo Tarquin and Mel Brandon not) failed on commit 24bbeb5's code with 'grades blocked, not sent to The Dweller': the trip planned The Dweller. Cause: scripts/gen-engineering.py unioned coriolis-data modules.json engineers across module groups per blueprint grade (verified: regenerated per-group lists union to exactly the old lists, 0 differences)."
updated_at: "2026-10-16T22:10:15.121322453+00:00"
---

Reproduced before the fix: the new test trip_skips_engineers_capped_below_the_grade_for_this_module (beam laser, Overcharged G4, The Dweller unlocked, Broo Tarquin and Mel Brandon not) failed on commit 24bbeb5's code with 'grades blocked, not sent to The Dweller': the trip planned The Dweller. Cause: scripts/gen-engineering.py unioned coriolis-data modules.json engineers across module groups per blueprint grade (verified: regenerated per-group lists union to exactly the old lists, 0 differences).
