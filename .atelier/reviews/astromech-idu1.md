---
schema: atelier.review
schema_version: 1
id: astromech-idu1
title: 'R4 [low] crates/astromech-market/src/list.rs:151 and app/src/lib/colonisation.ts:140: no test covers the truncated path for the list (Rust asserts truncated is empty; the UI''s ''Too many markets for X: narrow the radius'' warning is never rendered in a test). Ardent sorts exports cheapest first, so a truncated commodity drops dearer markets at any distance, and stocked counts and ranking can be understated; the warning is the only signal. Add a recorded or synthetic NEARBY_LIMIT-row answer to cover it.'
target: astromech-k3wp
identity: review-agent (k3wp review)
commit: f5e62c69c573b451172bf1cd2e16cce3b6519e75
created_at: 2026-10-16T23:10:37.516194049Z
event:
  event: finding
  recommendation: defer
---

R4 [low] crates/astromech-market/src/list.rs:151 and app/src/lib/colonisation.ts:140: no test covers the truncated path for the list (Rust asserts truncated is empty; the UI's 'Too many markets for X: narrow the radius' warning is never rendered in a test). Ardent sorts exports cheapest first, so a truncated commodity drops dearer markets at any distance, and stocked counts and ranking can be understated; the warning is the only signal. Add a recorded or synthetic NEARBY_LIMIT-row answer to cover it.
