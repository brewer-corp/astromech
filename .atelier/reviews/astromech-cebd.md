---
schema: atelier.review
schema_version: 1
id: astromech-cebd
title: load_dir keeps empty-transcript clips that have a condition (synthetic); they score as misses. Test empty_synthetic_transcripts_count_as_misses (seen failing without the fix).
target: astromech-yil0
identity: implementer
commit: d5dbc2af33e5961df155f9b6ed8f5a82126a4da3
created_at: 2026-10-17T21:45:01.457936016Z
event:
  event: disposition
  finding: astromech-v2pv
  decision: fix_now
  supersedes: null
  resolution_commit: d5dbc2af33e5961df155f9b6ed8f5a82126a4da3
  follow_up: null
---

load_dir keeps empty-transcript clips that have a condition (synthetic); they score as misses. Test empty_synthetic_transcripts_count_as_misses (seen failing without the fix).
