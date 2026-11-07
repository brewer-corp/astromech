---
schema: atelier.review
schema_version: 2
id: astromech-dj7j
title: finding on astromech-vucf @168fc814
target: astromech-vucf
identity: claude-reviewer-zgoo
commit: 168fc8140dcef80ec71579e18900b09ec4ba7d2c
created_at: 2026-11-07T21:46:40.313587994Z
event:
  event: finding
  recommendation: defer
---

crates/astromech-core/src/assistant.rs:423-426: zgoo puts the confirm window on the policy's clock (Assistant::now reads runner.policy.clock()) while the Assistant keeps its own clock field for data age (assistant.rs:121, SystemClock by default; session.rs:375 never calls with_clock). The Assistant now reads time from two clocks that differ in world mode. Correct today for the confirm window, and the installed app is unaffected. Fold into astromech-2ebj: once Session::start passes the session clock with with_clock, have Assistant::now read that one clock (and check tests that pin data age with with_clock don't then move the confirm window).
