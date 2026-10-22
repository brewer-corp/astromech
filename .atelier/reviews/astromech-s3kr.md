---
schema: atelier.review
schema_version: 1
id: astromech-s3kr
title: 'R3 low crates/astromech-history/src/lib.rs:602-626 (yrfv judgement): a name:X key joined to the only FID named X is moved for good on the incremental path (events.cmdr rewritten), but a later commanders recompute (pending ''commanders'') re-derives events.cmdr from lines and, with a second FID named X present, leaves name:X apart. So incremental and recompute disagree once a second FID with the name appears, and no test covers the ''join stays'' case. Rare (two commanders sharing a name on one PC). Sumeet to decide whether the join should stay or be undone; then add a test for the chosen behaviour.'
target: astromech-fdh3
identity: review-agent-fdh3-1
commit: 63470471264d143f5c0e854f4641ee757634d5a8
created_at: 2026-10-22T21:45:44.410511893Z
event:
  event: finding
  recommendation: defer
---

R3 low crates/astromech-history/src/lib.rs:602-626 (yrfv judgement): a name:X key joined to the only FID named X is moved for good on the incremental path (events.cmdr rewritten), but a later commanders recompute (pending 'commanders') re-derives events.cmdr from lines and, with a second FID named X present, leaves name:X apart. So incremental and recompute disagree once a second FID with the name appears, and no test covers the 'join stays' case. Rare (two commanders sharing a name on one PC). Sumeet to decide whether the join should stay or be undone; then add a test for the chosen behaviour.
