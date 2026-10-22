---
created_at: "2026-10-17T21:26:01.769924649+00:00"
id: "astromech-ak62"
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
status: "in_progress"
title: "Narrow window polish: header wraps to two rows at 900 px, Powerplay leaves a gap"
updated_at: "2026-10-22T21:14:09.473444291+00:00"
---

## Goal

At about 900 px wide the top tabs wrap to a second row and the Powerplay page leaves a large empty band between the standing panel and Where to work.

## Done when

Header fits one row down to 900 px (or wraps deliberately), and Powerplay columns do not leave a gap at 1400 px.

## Context

Found in the astromech-5qh0 walk. Screenshots: docs/evidence/astromech-5qh0/after/ready-keybinds-900.png (header), powerplay at 1400 in the local full set (ready-powerplay-1400.png).

## Notes

- 2026-10-22 [note] 2026-10-22: The window's minimum width is 800 px (main.rs min_inner_size), and the brand, the eight tabs and the status need about 1000 px on one row, more with Finish setup or a long status ('Waiting for Elite · Voice paused · Kill switch on'). So the header wraps on purpose: when one row would not fit (measured with a ResizeObserver in App.svelte, not a fixed breakpoint), the tabs take their own row under the name, and the name, Finish setup and status stay on the first. The .frame column is minmax(0, 1fr) so the header can no longer push the page wider than the window (before: 1056 px scroll width at 900). Powerplay: in the three-column range (container query on the section) What earns merits spans both rows and Where to work fills the two columns beside it. Screenshots in docs/evidence/astromech-ak62/.
