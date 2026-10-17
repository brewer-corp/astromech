---
schema: atelier.review
schema_version: 1
id: astromech-nhhq
title: 'R4 [low] crates/astromech-eddn/src/edsm/api.rs:38 vs app/src-tauri/src/main.rs:55: the flight-log request runs synchronously in the control handler with a 30 s ureq timeout, but the shell''s control socket read timeout is 15 s. When EDSM is slow, the page shows a generic socket read timeout instead of EDSM''s answer. Align the timeouts (for example, 10 s for get-logs).'
target: astromech-ah0i
identity: review-agent (opus, independent)
commit: aa58b47beb073e878c106e16878529bca17d43f1
created_at: 2026-10-17T05:04:00.445327780Z
event:
  event: finding
  recommendation: defer
---

R4 [low] crates/astromech-eddn/src/edsm/api.rs:38 vs app/src-tauri/src/main.rs:55: the flight-log request runs synchronously in the control handler with a 30 s ureq timeout, but the shell's control socket read timeout is 15 s. When EDSM is slow, the page shows a generic socket read timeout instead of EDSM's answer. Align the timeouts (for example, 10 s for get-logs).
