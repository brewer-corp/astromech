---
created_at: "2026-10-16T19:05:32.762403539+00:00"
id: "astromech-12uv"
issue_type: "task"
labels:
- "alpha"
- "size-s"
workflow_branch:
  branch_base: main
  work_branch: task/astromech-12uv
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-610m"
    type: "advances"
  - kind: "issue"
    id: "astromech-fa3g"
    type: "related"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Make voice tester test mode visible and fix it at push-to-talk release"
updated_at: "2026-10-17T19:03:21.752311264+00:00"
---

## Goal

When the Voice tester holds push-to-talk in test mode, the player can tell without looking at the window, and a phrase is tested or run according to the mode at the moment push-to-talk was released.

## Done when

- The tray status reads "Voice tester open · push-to-talk testing only" while test mode is on (tested on the status line). Consider a short spoken "test only" cue on push-to-talk in test mode, since the game HUD can't show this state; decide and record it.
- The clip job records whether test mode was on at push-to-talk release, and the speech-to-text thread uses that flag, so leaving the page during transcription neither runs a tested phrase for real nor tests a live one (tested with the routing helper).

## Context

Review round of astromech-fa3g (findings R3b and R4). Today `tester::on_heard` checks `TESTER.armed` when transcription finishes, in the Windows-only stt thread in `crates/astromech-core/src/session.rs`; the tray status line is `Core::status_line` in `app.rs`.
