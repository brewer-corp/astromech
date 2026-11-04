---
schema: atelier.review
schema_version: 1
id: astromech-of26
title: finding on astromech-ki3j @3057c961
target: astromech-ki3j
identity: review-agent-ki3j-1
commit: 3057c9616ce7d77b96589d0aee330e194166aa5d
created_at: 2026-11-04T21:17:14.980392582Z
event:
  event: finding
  recommendation: defer
---

medium: crates/astromech-core/src/pipeline.rs:95-106 (start_time/begins_before) and 669-673 (catch_up) R1's pin remains after a clock correction once real time passes the old future header, and at startup it needs no tool touch at all. This PC's clock ran 1 h fast for session J1 (named and stamped by the fast clock), was corrected back, and the game started J2 within the hour. If astromech starts (or restarts) after real time passes J1's header while J2 is still being written, catch_up finds J1 latest by name and begins_before(J2, J1) true, so it follows J1. follow_newest then never switches to J2 because it began more than 120 s earlier, so the game's journal is not read for the rest of that game session (state, callouts, routes, EDDN), with no error. Before ki3j the newest by mtime (J2) was followed. The 3057c961 note's limits mention only the touched-at-runtime form. It fails closed and needs a >120 s backwards correction plus a game restart inside the skew window, so it is rare. A follow-up could let a journal that proves itself the game's (fresh lines in the newest-by-mtime journal, or classify == Game/Gated-fresh) take the session regardless of header order, which would also cover the runtime form and the <120 s-ahead foreign case.
