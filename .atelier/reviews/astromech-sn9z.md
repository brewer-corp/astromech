---
schema: atelier.review
schema_version: 1
id: astromech-sn9z
title: approval on astromech-aiwv @e5138f36
target: astromech-aiwv
identity: reviewer-sonnet-aiwv
commit: e5138f3688f8efaeb8c79a0c4f432126bdb293fc
created_at: 2026-11-05T06:12:33.686988807Z
event:
  event: approval
---

Cause confirmed in code: parse_fresh rejects Market.json whose timestamp is earlier than the event's (1s resolution); old test read ts(0) twice so a second boundary made the file Stale. Reproduced: pre-fix + 1.1s sleep fails with left [] right [Market.]; fixed + same sleep passes. cargo test -p astromech-core passes (204 lib + others). Other tests checked: file/event pairs at pipeline.rs ~1483,1497,1522,1635 use one stamp or write event stamp first (file >= event), no stale comparison in ts(0)-twice sites elsewhere. CI on PR not run.
