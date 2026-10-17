---
schema: atelier.review
schema_version: 1
id: astromech-hq3s
title: R5 [low] edsm/api.rs FLIGHT_LOG_TIMEOUT (10 s) is checked against a literal 15 s in a test, duplicating app/src-tauri/src/main.rs set_read_timeout(15 s) (a per-read socket timeout). Margin is ample and the doc comment points at the shell; timeout text carries no URL or key. Note only.
target: astromech-ky3d
identity: review-agent (opus, independent)
commit: 769a9dea096cd5fb96e00ce173c87c92fc0134b2
created_at: 2026-10-17T05:37:39.682018707Z
event:
  event: finding
  recommendation: no_action
---

R5 [low] edsm/api.rs FLIGHT_LOG_TIMEOUT (10 s) is checked against a literal 15 s in a test, duplicating app/src-tauri/src/main.rs set_read_timeout(15 s) (a per-read socket timeout). Margin is ample and the doc comment points at the shell; timeout text carries no URL or key. Note only.
