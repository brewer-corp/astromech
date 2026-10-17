---
schema: atelier.review
schema_version: 1
id: astromech-un6b
title: Checked SQL (newest per body/kind, organic all kept, tie-break on event_id, honks by system+commander), EXPLAIN QUERY PLAN on all branches, v7 additive partial index (read-only open requires matching version, same as prior migrations), cargo test -p astromech-history -p astromech-exploration green (29 passed). Mutations (read all scans; drop honk events) make the new/extended tests fail. CI pending at review time.
target: astromech-lelp
identity: review-agent (sonnet, independent)
commit: ee185ff1f9f94135a46db0e554218e19d0cfa189
created_at: 2026-10-17T08:48:05.936702128Z
event:
  event: approval
---

Checked SQL (newest per body/kind, organic all kept, tie-break on event_id, honks by system+commander), EXPLAIN QUERY PLAN on all branches, v7 additive partial index (read-only open requires matching version, same as prior migrations), cargo test -p astromech-history -p astromech-exploration green (29 passed). Mutations (read all scans; drop honk events) make the new/extended tests fail. CI pending at review time.
