---
schema: atelier.review
schema_version: 1
id: astromech-qbcf
title: 'Per-poll full scan of signal lines: History::system_lines'' second arm (crates/astromech-history/src/exploration.rs) filters FSSBodySignals/SAASignalsFound by json_extract(json,''$.SystemAddress'') with no index on that expression. EXPLAIN QUERY PLAN: SEARCH events USING INDEX events_event_ts (event=?) + TEMP B-TREE, i.e. every signal row is read and json-parsed; measured 0.13 s at 60k signal rows on a synthetic 900k-row history. It runs on every 3 s System page poll, reintroducing the cost astromech-dt5h removes for landings. Fix: index those events the same way (a migration after dt5h''s v5 extending the partial SystemAddress index to FSSBodySignals and SAASignalsFound), or key the lookup off an indexed column.'
target: astromech-0n4h
identity: review-agent (opus, independent)
commit: e1a1d87ea2d901d8c55dd406f77c768b1fe467a0
created_at: 2026-10-17T08:00:42.016225491Z
event:
  event: finding
  recommendation: fix_now
---

Per-poll full scan of signal lines: History::system_lines' second arm (crates/astromech-history/src/exploration.rs) filters FSSBodySignals/SAASignalsFound by json_extract(json,'$.SystemAddress') with no index on that expression. EXPLAIN QUERY PLAN: SEARCH events USING INDEX events_event_ts (event=?) + TEMP B-TREE, i.e. every signal row is read and json-parsed; measured 0.13 s at 60k signal rows on a synthetic 900k-row history. It runs on every 3 s System page poll, reintroducing the cost astromech-dt5h removes for landings. Fix: index those events the same way (a migration after dt5h's v5 extending the partial SystemAddress index to FSSBodySignals and SAASignalsFound), or key the lookup off an indexed column.
