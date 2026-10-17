---
created_at: "2026-10-17T05:27:12.314263471+00:00"
id: "astromech-243w"
issue_type: "task"
labels: []
workflow_branch:
  branch_base: main
  work_branch: task/astromech-243w
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
title: "Trade lookups: blank age and distance selects, placeholder rows, shorter partial-buyer voice"
updated_at: "2026-10-17T05:46:43.437766035+00:00"
---

## Goal

Left over from the Trade near me review (astromech-zf3s, R6 and R7, carried in astromech-proz's notes) and the astromech-proz review:
- R6: the age and distance dropdowns show blank.
- R7: placeholder rows can push real markets out of Ardent's 1000-row answer (the truncation check runs before placeholders are dropped).
- astromech-2myn: voice adds a ~20-word partial-buyer sentence for every commodity no market takes whole, with no limit, often repeating the main answer's station. Limit it to the top one or two and drop the station when it repeats.

## Done when

Component test: the selects show their saved values; recorded-response test: placeholders don't count toward truncation; copy test: a hold of five such commodities stays within the copy guide's budget.
