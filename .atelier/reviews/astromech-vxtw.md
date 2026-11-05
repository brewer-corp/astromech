---
schema: atelier.review
schema_version: 1
id: astromech-vxtw
title: disposition on astromech-zftk @36f1203f
target: astromech-zftk
identity: orchestrator
commit: 36f1203fed380522464400dae7e384a5756fc4f3
created_at: 2026-11-04T22:57:02.857827534Z
event:
  event: disposition
  finding: astromech-p58u
  decision: fix_now
  supersedes: null
  resolution_commit: 36f1203fed380522464400dae7e384a5756fc4f3
  follow_up: null
---

open() leaves an unopenable journal for the next look and keeps its startup length, so it is split later (36f1203f)
