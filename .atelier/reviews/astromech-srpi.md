---
schema: atelier.review
schema_version: 2
id: astromech-srpi
title: finding on astromech-bp23 @8197eae3
target: astromech-bp23
identity: claude-reviewer-8s6k
commit: 8197eae3aaea51f8468babfeca01b516675004b2
created_at: 2026-11-07T06:52:31.348354391Z
event:
  event: finding
  recommendation: defer
---

low: .atelier/issues/astromech-l2yy.md item 3 points at the Request doc comment in crates/astromech-core/src/control.rs, which this change deletes; astromech-api's Request no longer lists commands, so the item is moot or should point at the registry. Update the record as a planning change.
