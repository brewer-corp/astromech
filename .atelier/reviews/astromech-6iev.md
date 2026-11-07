---
schema: atelier.review
schema_version: 2
id: astromech-6iev
title: finding on astromech-vucf @2d3ab0e7
target: astromech-vucf
identity: claude-reviewer-gllh
commit: 2d3ab0e758ec4c08140b5901e5032d4c9ded6d97
created_at: 2026-11-07T19:42:09.607530124Z
event:
  event: finding
  recommendation: fix_now
---

low: crates/astromech-core/tests/every_window_command.rs set_voice entry (n5cr fix, 2d3ab0e7) The read-back is vacuous: set_voice sets speaker bf_emma, speed 1.0, colour cockpit, which are exactly Config::default()'s voice (catalog::DEFAULT_VOICE is bf_emma, config.rs:160-163), and the world's astromech.toml sets no [voice]. So voice_settings already shows bf_emma/cockpit before the call, and a set_voice emptied to Done::empty() still passes, which is the defect n5cr asked to fix, and set_voice is not among the four listed exceptions. Fix: set a non-default voice, speed and colour (for example bm_george, 1.2, and a colour other than cockpit) and read those back. I checked every other read-back against the world's start state, and each one shows a change.
