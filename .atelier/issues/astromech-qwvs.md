---
created_at: "2026-11-07T03:00:11.378267755+00:00"
id: "astromech-qwvs"
issue_type: "task"
labels:
- "size-m"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-dsyu"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-sixj"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-woqv"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-xajc"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Route page: the timeline"
updated_at: "2026-11-07T03:00:11.378267755+00:00"
---

## Goal

The route shows as the agreed timeline (astromech-ok29's Design): star-type icons as nodes, the current leg one row a jump with your position, other legs as the stars passed with the no-scoop warning, the next stop raised with its copy icon, inhabited stops' second line, Make next on hover. Skip and Back buttons and the 'On your clipboard' line go.

## Done when

- [ ] The page matches the design in the preview at 1440p, and at 1100 px with nothing clipped.
- [ ] Copy turns to a check; a failed copy is the red error; Make next works.
- [ ] The stars' tooltips use Tip.
- [ ] Route tests updated; docs/guide's route page describes it.
