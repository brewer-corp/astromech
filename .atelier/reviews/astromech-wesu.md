---
schema: atelier.review
schema_version: 1
id: astromech-wesu
title: finding on astromech-az6g @32ef569f
target: astromech-az6g
identity: review-agent-az6g-1
commit: 32ef569f95400a7642fd26e6baa8d5c17e458182
created_at: 2026-11-04T21:29:17.687981845Z
event:
  event: finding
  recommendation: defer
---

medium: crates/astromech-eddn/src/edsm/mod.rs:509-514 If the backlog an outage leaves holds one event EDSM really chokes on (5xx or unreadable answer for the whole request), the first refused batch that contains it fails and the rest of that pass sends every refused event alone, and every later pass does the same while that event is due (up to MAX_ATTEMPTS refusals with backoff). So the outage drain falls back to about one request per event (the 168-for-201 case) in exactly the case where a poisoned event is in the backlog. No data is lost and the rate-limit handling bounds the load, so this is a cost, not a correctness problem. The decision note's 'one extra request per pass' understates it. Suggested follow-up: on a refused batch's whole-request failure, halve it (as astromech-sklz describes) rather than go straight to singles, and add a drain test with one Poison event in the outage backlog.
