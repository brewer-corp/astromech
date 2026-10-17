---
schema: atelier.review
schema_version: 1
id: astromech-faj9
title: '[medium] crates/astromech-history/src/exploration.rs landings_before: the query reads every Touchdown/Disembark row of the whole history (events_event_ts only narrows by event; SystemAddress is a json_extract per row, no index), and runs on the pipeline thread at every live FSDJump (HistoryVisits::landings, 50 ms busy timeout) and in add_landings every 3 s poll of the System page. Synthetic 60k Disembark rows took about 40 ms; a long Odyssey history (SRV/on-foot Disembarks) would be several times that, delaying the arrival briefing which the visits.rs docs say must never block. Fix: filter by system in an indexed column (a landings ledger or index on json_extract SystemAddress), or cache per system. Not wrong today, so not merge-blocking.'
target: astromech-ce4s
identity: 'review-agent (sonnet, independent, part B: callouts, history, shell, UI)'
commit: abce3096a915e5a11d15fce7f95785d6761e0b09
created_at: 2026-10-17T05:14:59.298288004Z
event:
  event: finding
  recommendation: defer
---

[medium] crates/astromech-history/src/exploration.rs landings_before: the query reads every Touchdown/Disembark row of the whole history (events_event_ts only narrows by event; SystemAddress is a json_extract per row, no index), and runs on the pipeline thread at every live FSDJump (HistoryVisits::landings, 50 ms busy timeout) and in add_landings every 3 s poll of the System page. Synthetic 60k Disembark rows took about 40 ms; a long Odyssey history (SRV/on-foot Disembarks) would be several times that, delaying the arrival briefing which the visits.rs docs say must never block. Fix: filter by system in an indexed column (a landings ledger or index on json_extract SystemAddress), or cache per system. Not wrong today, so not merge-blocking.
