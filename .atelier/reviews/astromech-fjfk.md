---
schema: atelier.review
schema_version: 1
id: astromech-fjfk
title: finding on astromech-t34c @60dc9bf4
target: astromech-t34c
identity: review-agent-t34c-1
commit: 60dc9bf4bac92cb3eb296e3749f57bdc6cfc4761
created_at: 2026-11-04T23:39:50.353700691Z
event:
  event: finding
  recommendation: no_action
---

low: crates/astromech-core/src/pipeline.rs:502-520 (73w9) The header cache spots a replaced file by creation time or a shorter length. On Windows, NTFS file-name tunneling keeps the old creation time when a file is deleted or renamed away and a new one takes its name within 15 s, which is how sync tools replace files. So a replacement that is not shorter keeps the old header time, and the cached header_agrees result too. On Linux filesystems without birth time, created() is None on both sides. The test's rename makes the file shorter, so it can't tell which check fired. The impact is small: a same-named replacement normally has the same Fileheader, and a stale header only affects ordering or the Gated/Foreign class, where freshness still decides what is live. A modified-time check alongside length would also catch it if this is revisited.
