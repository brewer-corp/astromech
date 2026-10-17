---
schema: atelier.review
schema_version: 1
id: astromech-c3le
title: R2 [low] crates/astromech-announce/src/session_summary.rs:150 credits() is a copy of crates/astromech-tools/src/lookup.rs:65, and both say '1000 thousand credits' for 999,500-999,999 (and '1.0 million' at exactly a million). Recommend one shared helper (tools already depends on announce) that rounds up to million at the boundary.
target: astromech-oo8y
identity: review-agent (opus, independent)
commit: e9945e7abf52f8d8b47f45c1bc67e8733379a76b
created_at: 2026-10-17T05:12:58.257565479Z
event:
  event: finding
  recommendation: defer
---

R2 [low] crates/astromech-announce/src/session_summary.rs:150 credits() is a copy of crates/astromech-tools/src/lookup.rs:65, and both say '1000 thousand credits' for 999,500-999,999 (and '1.0 million' at exactly a million). Recommend one shared helper (tools already depends on announce) that rounds up to million at the boundary.
