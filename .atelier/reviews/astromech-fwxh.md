---
schema: atelier.review
schema_version: 1
id: astromech-fwxh
title: finding on astromech-54k4 @1e88a1bb
target: astromech-54k4
identity: review-agent-54k4-1
commit: 1e88a1bb5948024f1f778d1d98128a42be2be4de
created_at: 2026-11-04T21:48:03.812329005Z
event:
  event: finding
  recommendation: no_action
---

Info: crates/astromech-announce/src/credits.rs:9-16: unit boundaries are right for every caller (checked 999,499 -> 999 thousand; 999,500 -> 1 million; 999,949,999 -> 999.9 million; 999,950,000 -> 1 billion; 99,950,000 -> 100 million; 1 -> 1 credit; discovery amount() and lookup prices use the same function, so none can say '1000 thousand' or '1000 million'). Exact half-tenths round by f64 formatting (half to even on the binary value): 9,950,000 -> 9.9 million and 1,250,000 -> 1.2 million, but 1,050,000 -> 1.1 million and 13,500 -> 14 thousand; the old discovery formatter used round() (half away). A 0.05 difference in a spoken figure; no fix needed. Discovery's sub-million cutoff moving from 950 thousand to 999.5 thousand is an improvement. The formatter lives in astromech-announce, not astromech-tools as Done-when says; check-deps.py confirms tools depends on announce, so the recorded deviation is correct.
