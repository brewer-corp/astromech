---
created_at: "2026-10-17T19:34:52.254784417+00:00"
id: "astromech-nhyb"
issue_type: "task"
labels: []
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-22T21:44:45.282959088+00:00"
status: "done"
title: "Settings controls: keyboard focus ring and announced errors"
updated_at: "2026-10-22T21:44:45.282959088+00:00"
---

## Goal

The Settings page controls built with `button { all: unset }` (callout switch and Reset in Callouts.svelte, ProspectorThresholds.svelte) show no keyboard focus ring, and their error and "Saved" lines are not announced to screen readers. After a rejected cooldown the box keeps the rejected text while the saved value is unchanged.

## Done when

Focus-visible outlines on those buttons (as SystemChip.svelte), aria-live="polite" on the note and error lines, and the cooldown box reset to the saved value on a rejected entry; a vitest covers the reset.

## Close Reason

Done and independently reviewed on feature/astromech-jo3l (approval astromech-15yv).
