---
schema: atelier.review
schema_version: 2
id: astromech-5bvd
title: finding on astromech-bp23 @ad9ad743
target: astromech-bp23
identity: claude-reviewer-batch1
commit: ad9ad743534d4cc42681c9bdcec10cbf34aad7d1
created_at: 2026-11-07T08:05:03.062033134Z
event:
  event: finding
  recommendation: defer
---

low: crates/astromech-core/src/crash.rs:20-24 (96ci) The hook skips the marker whenever dispatching() is true, including a panic that cannot be caught: a second panic during the first one's unwind (a Drop that panics) aborts the process, and both hook calls see dispatching() true, so the core dies with no crash marker and no crash-last.txt. Rare. Defer: say it in crash.rs's doc, or count panics in the hook (write the marker for the second one on the same thread while dispatching).
