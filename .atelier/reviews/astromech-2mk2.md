---
schema: atelier.review
schema_version: 1
id: astromech-2mk2
title: approval on astromech-yn6y @3b26427e
target: astromech-yn6y
identity: claude-reviewer
commit: 3b26427ea71d841b74c9d22bdc866581ab24ee41
created_at: 2026-10-24T23:30:01.030164021Z
event:
  event: approval
---

Approved at 3b26427e. The spike meets every Done-when item: both recipes run end to end through astromech_data::Client over a recording 1 req/s-per-service transport; requests, wall time, A/B candidate sizes, spot checks (EDSM, raw Spansh, Spansh vs Ardent, Inara 503 noted), failures, hotspot filterability and coverage, faction freshness, local-vs-live split with a reproduced 90 MB / 30 MB gz estimate, and a go with an engine shape. Join and re-check logic are correct; replay is offline and reproducible; tests, clippy and fmt are clean; hygiene is fine. Seven non-blocking precision notes are in the accompanying review record (fan-out 'same answer' overclaim for recipe 2, Ardent 1000-row cap, recipe 2 ignoring permits, replay.sh not covering compare_spansh/edsm, FINDINGS.md references).
