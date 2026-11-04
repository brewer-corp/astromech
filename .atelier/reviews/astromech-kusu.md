---
schema: atelier.review
schema_version: 1
id: astromech-kusu
title: finding on astromech-az6g @32ef569f
target: astromech-az6g
identity: review-agent-az6g-1
commit: 32ef569f95400a7642fd26e6baa8d5c17e458182
created_at: 2026-11-04T21:29:18.272256403Z
event:
  event: finding
  recommendation: defer
---

low: crates/astromech-eddn/src/edsm/tests.rs:715 The drain test proves the queue is empty, not that the events were delivered. Events given up during recovery would also empty it, and the request-count bound would still pass. Suggested: assert up.stored has events entries with no duplicates, so the test also proves no loss or duplication under the new batching.
