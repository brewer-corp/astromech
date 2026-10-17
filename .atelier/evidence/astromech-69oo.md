---
created_at: "2026-10-17T21:32:09.783705491+00:00"
id: "astromech-69oo"
evidence_type: "validation"
commit: "bf22f72f46ca659a3c3955faa4d9bb3c1a8179f7"
result: "pass"
captured_at: "2026-10-17T21:32:09.783645449+00:00"
path: "crates/astromech-core/tests/real_journals.rs"
agent_identity: "claude (astromech-yin0 replay)"
target:
  kind: "issue"
  id: "astromech-tqdh"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-tqdh"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full logbook import of Sumeet's journal folder (copy of ~/edjournal: 22 journals, 3-16 Oct 2026, 35,955 lines, 15.5 MB) timed with the yin0 harness on this Linux dev box, release build: History::import 0.34-0.41 s over four runs (35,955 events, 0 malformed), a second import 0.000 s (0 new events), rebuild_ledgers 0.14-0.17 s, logbook query 0.002-0.003 s, session 0.000 s; history.sqlite 21.4 MB. The core's startup catch-up (pipeline::start_with_eddn on the newest journal, 369 lines) took 0.06 s; on a 3,467-line journal 0.19 s. Limits: two weeks of play, not years; Linux, not Sumeet's Windows PC; command: ASTROMECH_REAL_JOURNALS=~/edjournal cargo test -p astromech-core --release --test real_journals -- --nocapture (exits non-zero for the separate findings it reports, not for timing)."
updated_at: "2026-10-17T21:32:09.787701473+00:00"
---

Full logbook import of Sumeet's journal folder (copy of ~/edjournal: 22 journals, 3-16 Oct 2026, 35,955 lines, 15.5 MB) timed with the yin0 harness on this Linux dev box, release build: History::import 0.34-0.41 s over four runs (35,955 events, 0 malformed), a second import 0.000 s (0 new events), rebuild_ledgers 0.14-0.17 s, logbook query 0.002-0.003 s, session 0.000 s; history.sqlite 21.4 MB. The core's startup catch-up (pipeline::start_with_eddn on the newest journal, 369 lines) took 0.06 s; on a 3,467-line journal 0.19 s. Limits: two weeks of play, not years; Linux, not Sumeet's Windows PC; command: ASTROMECH_REAL_JOURNALS=~/edjournal cargo test -p astromech-core --release --test real_journals -- --nocapture (exits non-zero for the separate findings it reports, not for timing).
