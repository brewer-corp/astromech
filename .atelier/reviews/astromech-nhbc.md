---
schema: atelier.review
schema_version: 2
id: astromech-nhbc
title: finding on astromech-bp23 @36531f8c
target: astromech-bp23
identity: claude-reviewer-ro7a
commit: 36531f8c99c5623824057f12726be11a1e8f58ce
created_at: 2026-11-07T11:16:06.256710741Z
event:
  event: finding
  recommendation: fix_now
---

low: crates/astromech-core/src/app.rs:1877-1884 (the_status_line_says_when_push_to_talk_is_only_testing) with app/every_window_command.rs:151-152 This is the first lib test to set the process-wide VOICE_PAUSED (pause_voice, then resume_voice over TCP). status_line_for appends ' · paused' when it is set (app.rs:244), and the status-line test asserts 'Waiting for Elite' exactly without holding CONFIG_EDITS, so if it runs in the paused window the two parallel lib tests race and it fails (a CI flake, Windows too). Inspection only: 300 runs of both together did not hit it, the window is about a millisecond. The CONFIG_EDITS comment now claims tests that set the voice pause hold it, but readers must too. Fix: take CONFIG_EDITS in that test (and any other lib test that reads status_line or voice_paused).
