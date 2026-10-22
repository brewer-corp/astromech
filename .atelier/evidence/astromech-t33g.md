---
created_at: "2026-10-22T23:49:40.387283090+00:00"
id: "astromech-t33g"
evidence_type: "validation"
commit: "327c059301ee2d1a145df8472e7f626f461ac933"
result: "fail"
captured_at: "2026-10-22T23:49:40.387243867+00:00"
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
title: "S6 Data sharing: fail (defect in the mission's delivered work, untracked until now). Claim 'outfitting is sent': does not hold. crates/astromech-eddn/src/companion.rs:26 SEND_OUTFITTING = false, and feed.rs:154 turns every Outfitting visit into a skip 'outfitting/2 is off until PowerPlay and tech-broker modules are filtered' (gated in astromech-xvjs after review R1; no open issue). Reproduce: ASTROMECH_REAL_JOURNALS=<copy of ~/edjournal> CARGO_BUILD_JOBS=4 cargo test -p astromech-core --release --test real_journals -- --nocapture; expected outfitting-v2.0.json messages; actual 0 sent and 111 skipped with that reason. The Data sharing page (#/datasharing preview) honestly omits outfitting from 'What gets sent'. Filed astromech-dlrb. Holds in code: the same dry run builds 2948 messages for commodity/3, shipyard/2, journal/1 and approachsettlement, codexentry, dockinggranted/denied, fssallbodiesfound, fssbodysignals, fssdiscoveryscan, fsssignaldiscovered, navbeaconscan, scanbarycentre (fcmaterials_journal and navroute built but none sent here), nothing during catch-up; eddn tests (145 unit) pass against the live schema fixtures; EDSM upload dry run queues the journal with the discard list applied and 0 problems. Blocked: acceptance by EDDN from the live game (test schemas, then a play session with no rejections) needs astromech-uxyq; a live EDSM upload needs astromech-shi5 (both needs-human). Not sent to EDDN/EDSM from here: it would send data on Sumeet's behalf."
updated_at: "2026-10-22T23:49:40.387581135+00:00"
---

S6 Data sharing: fail (defect in the mission's delivered work, untracked until now). Claim 'outfitting is sent': does not hold. crates/astromech-eddn/src/companion.rs:26 SEND_OUTFITTING = false, and feed.rs:154 turns every Outfitting visit into a skip 'outfitting/2 is off until PowerPlay and tech-broker modules are filtered' (gated in astromech-xvjs after review R1; no open issue). Reproduce: ASTROMECH_REAL_JOURNALS=<copy of ~/edjournal> CARGO_BUILD_JOBS=4 cargo test -p astromech-core --release --test real_journals -- --nocapture; expected outfitting-v2.0.json messages; actual 0 sent and 111 skipped with that reason. The Data sharing page (#/datasharing preview) honestly omits outfitting from 'What gets sent'. Filed astromech-dlrb. Holds in code: the same dry run builds 2948 messages for commodity/3, shipyard/2, journal/1 and approachsettlement, codexentry, dockinggranted/denied, fssallbodiesfound, fssbodysignals, fssdiscoveryscan, fsssignaldiscovered, navbeaconscan, scanbarycentre (fcmaterials_journal and navroute built but none sent here), nothing during catch-up; eddn tests (145 unit) pass against the live schema fixtures; EDSM upload dry run queues the journal with the discard list applied and 0 problems. Blocked: acceptance by EDDN from the live game (test schemas, then a play session with no rejections) needs astromech-uxyq; a live EDSM upload needs astromech-shi5 (both needs-human). Not sent to EDDN/EDSM from here: it would send data on Sumeet's behalf.
