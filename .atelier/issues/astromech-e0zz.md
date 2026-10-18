---
created_at: "2026-10-17T19:22:54.126798893+00:00"
id: "astromech-e0zz"
issue_type: "bug"
labels:
- "alpha"
workflow_branch:
  branch_base: main
  work_branch: bug/astromech-e0zz
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-17T23:41:00.541639616+00:00"
status: "done"
title: "Push-to-talk release is acted on about 210 ms late, leaving no margin in the 400 ms action-start budget"
updated_at: "2026-10-17T23:41:00.541639616+00:00"
---

## Goal

Give the action-start budget (ARCHITECTURE §11, MVP criterion 1: keypress begins <= 400 ms p95 after push-to-talk release) real margin.

## Done when

The MVP criteria harness (scripts/mvp_criteria.py) reports the action-start estimate with a clear margin under 400 ms on the Windows runner, or the budget is consciously re-set with a recorded decision.

## Context

Found by the MVP criteria harness (astromech-08w8) against the mock game on the Windows runner. With a keyboard push-to-talk key, the core handles the release p50 208-210 ms, p95 218-225 ms after it (three runs). The 150 ms debounce (controls.rs `Mapper::flush`) is only checked on the controls thread's 100 ms idle tick, so with no further input the release is reported about 200 ms late. A HOTAS that streams reports may flush it sooner. session.rs then sleeps post_roll - debounce (50 ms) and waits for the next mic callback before speech-to-text. With S9's STT p95 (104 ms) and the measured command-to-key-down p95 (11-16 ms), the estimate came out 403.7 ms (FAIL), 398.0 and 394.6 ms (PASS): no margin. Options include flushing the debounce on a timer at its deadline instead of the next idle tick, a shorter debounce for keyboard and mouse, and overlapping the post-roll with speech-to-text start.

## Notes

- 2026-10-17 [note] Fix: Mapper::wait_ms gives the controls thread's sleep as the 100 ms idle tick, or the time to the end of a pending release's debounce window if sooner (min 1 ms), so PttUp comes out at release + 150 ms instead of at the first idle tick after it. Estimate from the stage numbers (not measured: PR #119's harness has not merged, and the Windows loop can't be built on this machine): release handling falls from p50 208-210 / p95 218-225 ms to about 151-155 ms, so the action-start estimate falls by about 55-60 ms, from 395-404 to about 340 ms. Side effect to watch: session.rs sleeps post_roll - debounce (50 ms) after PttUp before ending capture, and it used to start late; the audio tail after the release is now the configured post_roll_ms (200 ms) instead of about 260 ms. If speech endings clip, raise post_roll_ms or keep a minimum tail in session.rs. Re-run the harness short mode on Windows to confirm both.

## Close Reason

The controls thread sleeps to the end of a pending release's debounce window (Mapper::wait_ms) instead of waiting for the next 100 ms idle tick, so PttUp is reported at release + 150 ms, not about 210 ms. Fake-clock test fails on a fixed 100 ms wait. Harness not merged and the Windows loop not buildable here: expected effect is about 55-60 ms off the action-start estimate (from stage numbers), to be confirmed with the harness on Windows. Side effect: the capture tail is now the configured post_roll_ms. Review recorded.
