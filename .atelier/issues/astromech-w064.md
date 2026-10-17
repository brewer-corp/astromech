---
created_at: "2026-10-17T08:01:20.702968957+00:00"
id: "astromech-w064"
issue_type: "bug"
labels: []
workflow_branch:
  branch_base: main
  work_branch: bug/astromech-w064
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
status: "in_progress"
title: "Exploration values: apply the 500 cr planet minimum after the mapping multiplier"
updated_at: "2026-10-17T08:26:22.615461582+00:00"
---

## Goal

`planet_value` applies the 500 cr minimum before the mapping multiplier while EDDI applies it after, so mapped small planets (k=300, under about 2.3 Earth masses) come out high: a 1 EM icy body is 2222 here against 2121 in EDDI (finding astromech-85ni on astromech-e8up; the earlier astromech-hcqc noted EDDiscovery does it the other way). Decide by checking against real sales or EDSM's published estimates for small mapped bodies, then match.

## Done when

A value test for a mapped 1 EM icy body matches the chosen reference, with the source cited.
