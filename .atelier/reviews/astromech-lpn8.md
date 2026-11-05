---
schema: atelier.review
schema_version: 1
id: astromech-lpn8
title: finding on astromech-az6g @32ef569f
target: astromech-az6g
identity: review-agent-az6g-1
commit: 32ef569f95400a7642fd26e6baa8d5c17e458182
created_at: 2026-11-04T21:29:18.484743658Z
event:
  event: finding
  recommendation: no_action
---

info: crates/astromech-eddn/src/edsm/mod.rs:366 limited_until lives in memory, so a restart during a rate-limit window forgets it. The first post after restart then most likely gets HTTP 429, which answer() maps to Hold (no event is lost or charged a refusal), and that answer's Remaining 0 sets the limit again. Cost: one request per restart. Not worth persisting.
