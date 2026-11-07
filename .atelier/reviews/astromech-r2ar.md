---
schema: atelier.review
schema_version: 2
id: astromech-r2ar
title: finding on astromech-vucf @48020588
target: astromech-vucf
identity: claude-reviewer-u8ge
commit: 48020588e8a83951c98586acd4823565276c9ee9
created_at: 2026-11-07T14:48:38.126903268Z
event:
  event: finding
  recommendation: fix_now
---

medium: crates/astromech-core/src/history.rs:224 and :128 (u8ge Done when 3, rule 'whatever stops a clock-waiting thread wakes its wait'; Report open item 3). Under the manual clock a re-pointed history follower deadlocks advance. Follower::stop doesn't wake the old thread's clock.sleep(settle) (2 s), and the replacement follower is registered and blocks in a plain join of it, so it is busy while the old one is parked: advance(d) with d shorter than the settle left (including advance(ZERO), which i1ev's steps use to settle) waits for the new thread forever, and nothing else moves time. The report says 'advance still settles and nothing hangs'; that is false. Reproduced against this commit's astromech-proto in a scratch crate: a registered thread in clock.sleep(2s), a second registered thread joining it, advance(500ms) still blocked after 5 s. Reached in a world by a journal-folder change (set_folders or a config edit, Following::follow) within 2 s of world time after a journal notification: plausible in gllh's every-command test and i1ev worlds. SystemClock is unaffected. Fix: make the settle a wait the stop ends, e.g. recv_timeout on the follower's clock channel until the settle deadline, returning on stop (under SystemClock this changes only a re-point/stop, a teardown like the pipeline's, so record it as a third teardown exception for vucf's plan to accept); or keep it and correct the Report, untick or narrow Done when 3, and file the hazard against i1ev/gllh. Either way the false claim must go.
