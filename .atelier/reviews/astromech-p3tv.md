---
schema: atelier.review
schema_version: 2
id: astromech-p3tv
title: finding on astromech-bp23 @ad9ad743
target: astromech-bp23
identity: claude-reviewer-batch1
commit: ad9ad743534d4cc42681c9bdcec10cbf34aad7d1
created_at: 2026-11-07T08:05:02.650852511Z
event:
  event: finding
  recommendation: fix_now
---

low: crates/astromech-core/src/hub.rs:98, :117, speech.rs:444, callouts.rs:468 (96ci) The lock-recovery rewrite missed Condvar waits: cv.wait(g).unwrap() / cv.wait_timeout(..).unwrap() return Err for a poisoned mutex (poison is permanent), so after any panic under StateHub.state, the speech queue or the callouts queue, StateHub::wait_flag/wait_gui (every verified key press, actions.rs:210,219) panics, the speech thread dies (voice goes silent) and the callouts thread dies, though .lock().recover() on the same mutexes now succeeds. Contradicts the crate doc's 'the core's own state and session state lock this way' and Done when's 'session threads they share state with ... the same poison recovery'. Fix: .recover() on those four results (the generic Recover impl on LockResult<G> covers (Guard, WaitTimeoutResult)); a hub test that poisons state then calls wait_flag.
