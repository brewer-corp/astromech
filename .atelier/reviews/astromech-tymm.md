---
schema: atelier.review
schema_version: 1
id: astromech-tymm
title: 'R1 [medium] crates/astromech-announce/src/biology.rs (SpanshBiology in crates/astromech-core/src/biology.rs): a failed Spansh lookup (network error, 5xx) is logged then stored as Lookup::Done(vec![]) for the session. The arrival line and the late-answer path then say nothing for that system for good, and the Biology page shows known: [] instead of null (looks like ''nothing reported'', not ''unknown''). One transient failure silently loses the spec''d ''known biology here'' line. Suggest: do not cache errors (drop the entry, maybe retry after a back-off inside the one-minute window) or store Failed and report null.'
target: astromech-a63r
identity: 'review-agent (sonnet, independent, part B: callouts, state, UI)'
commit: dcd988bc1ffbf9a6c4171078e4b65c1ee60fa48f
created_at: 2026-10-17T05:53:46.815447975Z
event:
  event: finding
  recommendation: defer
---

R1 [medium] crates/astromech-announce/src/biology.rs (SpanshBiology in crates/astromech-core/src/biology.rs): a failed Spansh lookup (network error, 5xx) is logged then stored as Lookup::Done(vec![]) for the session. The arrival line and the late-answer path then say nothing for that system for good, and the Biology page shows known: [] instead of null (looks like 'nothing reported', not 'unknown'). One transient failure silently loses the spec'd 'known biology here' line. Suggest: do not cache errors (drop the entry, maybe retry after a back-off inside the one-minute window) or store Failed and report null.
