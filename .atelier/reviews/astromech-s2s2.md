---
schema: atelier.review
schema_version: 1
id: astromech-s2s2
title: review on astromech-yn6y @3b26427e
target: astromech-yn6y
identity: claude-reviewer
commit: 3b26427ea71d841b74c9d22bdc866581ab24ee41
created_at: 2026-10-24T23:30:00.847640824Z
event:
  event: review
---

Checked: Done-when items against the Close Reason; disk.rs (cache keying, 1 req/s per-service spacing reserved before send, 400 ledger cap, offline refusal), join.rs (grid nearest-within, brute-force test), main.rs recipe 1 (Spansh state/allegiance/pop filters re-checked locally, permit set excluded on both sides, origin->A->B sort) and recipe 2 (ring_signals A side, Ardent B side re-checked for pad/placeholder demand/price, per-ring best by cr/h, Pareto price-vs-jumps). replay.sh exits 0 offline with ledger unchanged at 148 (output 35210 bytes, same as evidence lpa7; hash differs only because ages are relative to now). cargo test 2/2, clippy and fmt --check clean. compare_spansh (55-day median, 1333 h) and edsm-factions (Epsilon Indi, Wolf 1453 disagreements) reproduce offline from cache. sample_dump.py rerun reproduces 141.8k systems / 292.9k stations / 267.7k hotspot rings / 92 MB (31 MB gz). Hygiene: cache/, dumps/, target/ gitignored and untracked; diff touches only spikes/ and .atelier/; spikes is excluded from the root workspace; no personal or game files. Non-blocking notes for the Close Reason / 51ue: (1) 'One search per candidate costs 8 to 164 requests for the same answer' overstates: recipe 2 hotspots-first found 13 of 14 ring systems with a buyer vs 12 for the set join (Ardent rounds distances), buyers-first compared no answer, and 164 was estimated, not run. (2) 'replay.sh reruns every query below' omits compare_spansh=on and edsm-factions, which reproduce from cache but are not in replay.sh. (3) Ardent's 1000-row cap was hit in 6 of 7 recipe-2 runs; harmless for min_price because Ardent sorts price-descending (verified in cache), but the Painite 719 / LTD 709 buyer counts are cap-bound, and the engine must treat the cap as truncation. (4) Recipe 2 does not exclude permit-locked systems (main.rs:487-490, 432), unlike recipe 1. (5) coverage.sh's denominator requires reserve_level and the numerator does not, so 83-88% is not strictly a lower bound. (6) replay.sh:2 and README refer to FINDINGS.md, which does not exist. (7) '4M hotspot rings galaxy-wide / 500 MB' has no script behind it. None changes the go verdict or engine shape.
