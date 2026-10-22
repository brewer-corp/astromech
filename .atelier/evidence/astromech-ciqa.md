---
created_at: "2026-10-22T23:49:40.611556847+00:00"
id: "astromech-ciqa"
evidence_type: "validation"
commit: "327c059301ee2d1a145df8472e7f626f461ac933"
result: "fail"
captured_at: "2026-10-22T23:49:40.611515980+00:00"
agent_identity: "validator-e8ux"
target:
  kind: "issue"
  id: "astromech-e8ux"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-e8ux"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "S8 Exploration and exobiology: fail on the strategy preamble's voice rule; the journal-replay part passes. Pass: exploration tests system 3, values 8, unsold 3, biology 4; exobiology predict 5, sampling 2, surface 4; announce biology 9, sampling 6, discovery 4 (all fresh at 327c059); real-journal harness folds the current system view and unsold data with 0 problems; app preview #/exploration shows the live system view with body values (value now and if mapped, worth mapping, first footfall), #/exobiology shows the system biology summary and the sampling helper (distance from the nearest sample). Fail: the preamble says a module isn't done until its voice_exposed queries have phrase templates (ARCHITECTURE §18.7), and §18.7's M7 row lists 'Is this system worth mapping?' and 'which organics haven't I scanned here?'. The grammar has no exploration or exobiology query: through the core Assistant 'is this system worth mapping', 'which organics haven't I scanned here', 'what is this system worth', 'how much is my unsold data worth' and 'what biology is in this system' all return NotUnderstood. The biology and sampling callouts speak on events but are not queries. Cause: defect in the mission's delivered work (the 17 Oct validation passed S8 reading it as a view, not a lookup). Filed astromech-m3xd."
updated_at: "2026-10-22T23:49:40.611821860+00:00"
---

S8 Exploration and exobiology: fail on the strategy preamble's voice rule; the journal-replay part passes. Pass: exploration tests system 3, values 8, unsold 3, biology 4; exobiology predict 5, sampling 2, surface 4; announce biology 9, sampling 6, discovery 4 (all fresh at 327c059); real-journal harness folds the current system view and unsold data with 0 problems; app preview #/exploration shows the live system view with body values (value now and if mapped, worth mapping, first footfall), #/exobiology shows the system biology summary and the sampling helper (distance from the nearest sample). Fail: the preamble says a module isn't done until its voice_exposed queries have phrase templates (ARCHITECTURE §18.7), and §18.7's M7 row lists 'Is this system worth mapping?' and 'which organics haven't I scanned here?'. The grammar has no exploration or exobiology query: through the core Assistant 'is this system worth mapping', 'which organics haven't I scanned here', 'what is this system worth', 'how much is my unsold data worth' and 'what biology is in this system' all return NotUnderstood. The biology and sampling callouts speak on events but are not queries. Cause: defect in the mission's delivered work (the 17 Oct validation passed S8 reading it as a view, not a lookup). Filed astromech-m3xd.
