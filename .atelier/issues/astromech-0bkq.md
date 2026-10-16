---
created_at: "2026-10-16T19:57:19.874760390+00:00"
id: "astromech-0bkq"
issue_type: "task"
labels:
- "size-s"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-omi1"
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-cmhv"
    type: "related"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Callout delivery: speaker busy without a device, notifications off the pipeline thread"
updated_at: "2026-10-16T19:57:19.874760390+00:00"
---

## Goal

Callout delivery stays correct and cheap in the cases the first engine slice left open:

- `Speaker::busy()` in `speech.rs` is right with no audio device (the Windows fallback `play_wav_async` replaces the current sound, so callouts cut each other off) and cannot stay true forever if the TTS thread panics (every callout would then be dropped as stale, unlogged).
- The pipeline (`pipeline.rs` `apply_lines`) delivers reactions after releasing `self.stats`, not while holding it, and Windows notifications (PowerShell, about 0.5–1 s CPU and 60 MB each) go through a worker with a global rate limit instead of running on the pipeline thread.
- Notification failures are logged: a detached waiter logs the PowerShell exit status.

## Done when

Tests cover busy() with no device and after a TTS thread failure (callouts resume or the failure is logged), delivery outside the stats lock, and the notification rate limit. A Windows run shows a notification appears, its failure is logged when PowerShell fails, and whether Windows 11 Do Not Disturb hides it while Elite is fullscreen (recorded as evidence).

## Context

Review findings R3, R4 and R5 on the reaction engine (astromech-ngyt, feature/astromech-cmhv). The prospector callouts (astromech-omi1) are the first rule to use this delivery path.
