---
schema: atelier.review
schema_version: 2
id: astromech-p78l
title: finding on astromech-vucf @48020588
target: astromech-vucf
identity: claude-reviewer-u8ge
commit: 48020588e8a83951c98586acd4823565276c9ee9
created_at: 2026-11-07T14:48:46.344994609Z
event:
  event: finding
  recommendation: fix_now
---

low: crates/astromech-proto/src/clock.rs:496-502 Manual::sleep checks 'elapsed >= until' under one lock and registers its Deadline::At(until) under another. An advance that lands between them raises elapsed past until and scans the waits before this one exists, so the sleep is parked with a deadline already passed and never fired; if the thread is registered (busy, so advance waits for it) advance then returns with that thread asleep past its deadline until the next advance. That is a lost wakeup that breaks Done when 2 ('wakes every clock wait whose deadline passed') and would show as a rare world flake. Shown in a scratch copy of clock.rs with a 200 ms pause injected between the check and begin: a registered thread's sleep(1s) was still asleep after advance(1s) returned with elapsed 1s; without the pause it passes. Fix: do the check and the insert under one lock (or have State::begin treat a deadline <= elapsed as fired at once). Related, smaller: callers that compute 'end - now' from an earlier clock.now() (hub wait_flag/wait_gui, policy wait_foreground) get an After(d) measured from the later elapsed, so a concurrent advance pushes their timeout one advance late; worth a comment or an absolute-deadline wait_until when i1ev drives these concurrently.
