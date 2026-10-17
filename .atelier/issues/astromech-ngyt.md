---
created_at: "2026-10-15T20:36:13.651930072+00:00"
id: "astromech-ngyt"
issue_type: "task"
labels:
- "size-m"
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-0zcu"
  - kind: "issue"
    id: "astromech-4zeh"
  - kind: "issue"
    id: "astromech-6920"
  - kind: "issue"
    id: "astromech-a63r"
  - kind: "issue"
    id: "astromech-drku"
  - kind: "issue"
    id: "astromech-ew3x"
  - kind: "issue"
    id: "astromech-gpee"
  - kind: "issue"
    id: "astromech-kv9k"
  - kind: "issue"
    id: "astromech-omi1"
  - kind: "issue"
    id: "astromech-rpsb"
  - kind: "issue"
    id: "astromech-wq3f"
  - kind: "issue"
    id: "astromech-ze13"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-16T19:48:10.626237432+00:00"
status: "done"
title: "Event → reaction engine"
updated_at: "2026-10-16T19:48:10.626237432+00:00"
---

## Goal

Rules subscribe to journal and Status events and output speech, a toast in the app or overlay, or a Windows notification. Priority queue, per-rule toggle and cooldown, quiet during docking and combat, silent while replaying the journal.

## Done when

Replay tests show: a rule fires once per event within its cooldown; a disabled rule never fires; nothing is spoken during journal catch-up at startup.

## Context

Today only route progress speaks. Every callout uses this.

## Notes

- 2026-10-16 [note] Built in bdcef37. New crate astromech-announce (Engine, Rule trait, Settings, SpeechQueue); core callouts.rs delivers speech (one at a time, waits for Speaker::busy), app toasts (status.toasts, shown by app/src/lib/ui/Toasts.svelte) and Windows notifications (PowerShell WinRT toast, cfg(windows), compile- and run-unverified). Live point: the pipeline's catch_up() (current journal from the top + first Status.json) is silent; everything read afterwards, including new journal files, is live. Quiet: after DockingGranted until docked/cancelled/denied/timeout/left; combat = Status InDanger or BeingInterdicted, or UnderAttack within 30 s; only Urgent speaks, toasts/notifications still go. Settings: [callouts.<id>] enabled / cooldown_s in astromech.toml; no Settings UI (follow-up astromech-s0gz). No rules registered yet (builtin() is empty). Route progress: there is no event-driven route speech on main; the only route speech is the T0 'jumps left' reply to a voice question (astromech-tools Query::Jumps), which stays on the assistant path because it answers a question rather than reacting to an event. Nothing was migrated.

## Close Reason

Replay tests show a rule fires once per event within its cooldown, a disabled rule never fires, and nothing fires during the startup catch-up; quiet in docking and combat tested. Review outstanding.
