---
created_at: "2026-11-05T15:35:43.032049519+00:00"
id: "astromech-4odm"
issue_type: "task"
labels:
- "size-s"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-2hbw"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Find or plot: focus after a row action, live announcements, nested buttons in options"
updated_at: "2026-11-05T15:35:43.032049519+00:00"
---

## Goal

The header Find or plot box works well with the keyboard and a screen reader.

## Done when

- After Add stop or Set destination from a popover row (Tab then Enter), focus returns to the input instead of dropping to the page body (finding astromech-n1xc).
- FollowNote and the "Asking Spansh…" and "Nothing matches" lines are announced (`role="status"` or `aria-live="polite"`).
- The nested Plot, Add stop and Set destination buttons inside listbox options are resolved: a grid pattern, or actions reached by arrow keys from the active option with only the input in the Tab order (finding astromech-hdj3). Tab from the input no longer walks up to about thirty buttons.
- Tests cover focus return and keyboard reach of each action; `pnpm lint && pnpm check && pnpm test` pass.

## Context

Deferred findings from the 2hbw review (reviewer-sonnet-2hbw). Its decision: Add stop and Set destination stay reachable without new chords (Enter means Plot).
