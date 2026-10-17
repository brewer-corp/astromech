---
schema: atelier.review
schema_version: 1
id: astromech-0ga5
title: VoiceTurn holds callouts from PTT down until the reply is queued (60 s cap); tests.
target: astromech-cmhv
identity: orchestrator (Claude, not a code author)
commit: a0752a9097e2e7ad5d2028ec7a7f1ab2aea0a83e
created_at: 2026-10-16T19:58:21.913666211Z
event:
  event: disposition
  finding: astromech-r5bm
  decision: fix_now
  supersedes: null
  resolution_commit: 0627e015c2e8de7b303757deff80f609c5cc975d
  follow_up: null
---

VoiceTurn holds callouts from PTT down until the reply is queued (60 s cap); tests.
