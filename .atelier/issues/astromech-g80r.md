---
created_at: "2026-10-16T20:06:26.082165862+00:00"
id: "astromech-g80r"
issue_type: "task"
labels:
- "needs-human"
- "size-s"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-cmhv"
    type: "related"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Check callout notifications on Windows"
updated_at: "2026-10-16T20:06:26.082165862+00:00"
---

## Goal

On the Windows PC, a callout notification appears, a failing one is logged, and we know whether Windows 11 Do Not Disturb hides it while Elite runs fullscreen.

## Done when

Evidence from a Windows run: (1) a notification from astromech appears (for example a test callout with the notification channel); (2) with PowerShell blocked or the script broken, the log shows "notification failed (PowerShell …)"; (3) with Elite fullscreen and Do Not Disturb on (the Windows 11 default for full-screen games), whether the notification shows, is held in the notification centre, or is lost.

## Context

Split from astromech-0bkq, whose code (notifications on their own thread, at most one per 15 s, failures logged with the PowerShell exit status and first error line) was built and tested on Linux only. `WindowsNotifier` in `crates/astromech-core/src/callouts.rs` has not been compiled or run on Windows.
