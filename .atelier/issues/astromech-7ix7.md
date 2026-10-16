---
created_at: "2026-10-15T20:36:13.655892700+00:00"
id: "astromech-7ix7"
issue_type: "task"
labels:
- "size-m"
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-bf25"
  - kind: "issue"
    id: "astromech-w1q2"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-16T20:49:22.789558095+00:00"
status: "done"
title: "Nearby Powerplay targets"
updated_at: "2026-10-16T20:49:22.789558095+00:00"
---

## Goal

Acquisition, reinforcement and undermining systems for my power within N ly, with state and progress.

## Done when

A recorded Spansh response returns systems for the pledged power in distance order.

## Context

Spansh systems search with Powerplay filters. Unverified: the field names.

## Notes

- 2026-10-16 [note] Done in 948ca35. Spansh filters verified live (polite capture, 6 POSTs ~2 s apart, astromech UA): controlling_power / power / power_state as {"value": [..]} (OR over values); unknown filter names are ignored (count unchanged). Result fields: controlling_power, power[], power_state, power_state_control_progress, power_state_reinforcement, power_state_undermining, power_conflict_progress[{power, progress}]. Acquire = Unoccupied + power contains mine; Reinforce = controlling_power mine; Undermine = Exploited/Fortified/Stronghold controlled by any of the 11 other powers (list from data/catalog/powerplay.json). Shell command powerplay_targets (pledged power from the journal replay; origin typed or current). UI: Where to work panel, kind tabs and a 20/40/80 ly select on the search kit (filters remembered under powerplay.<kind>). Not done: a voice query for nearby targets (no voice lookup over the network exists yet for any search).

## Close Reason

Recorded Spansh responses return the pledged power's systems in distance order (evidence astromech-kefl, astromech-r75c).
