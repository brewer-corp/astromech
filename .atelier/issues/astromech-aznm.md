---
created_at: "2026-10-22T21:34:32.593835606+00:00"
id: "astromech-aznm"
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
    id: "astromech-7q0z"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-04T21:06:02.132058098+00:00"
status: "done"
title: "Settings: repeated save and error messages are not re-announced to screen readers"
updated_at: "2026-11-04T21:06:02.132058098+00:00"
---

## Goal

A second save, or the same error twice, is announced again by screen readers on the Callouts and Prospector threshold settings.

## Done when

The live region's text changes on every save or error (for example by clearing it first, or keying it), with a component test that checks the region is updated on a repeated save.

## Context

Review finding R5 (astromech-axga) on feature/astromech-jo3l: Callouts.svelte:66, ProspectorThresholds.svelte:56.

## Close Reason

New lib/ui/announce.ts empties a live region, awaits Svelte's tick, then sets the text, so a repeated message is a text change. Callouts.svelte now keeps one status string plus a warn flag (replacing error || note); ProspectorThresholds.svelte announces its errors the same way. Test src/modules/settings/announce.test.ts watches the region with a MutationObserver and expects [text, '', text] for a second save and repeated errors; all three fail on the old components. Not checked with a real screen reader (needs Windows/NVDA).
