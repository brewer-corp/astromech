---
created_at: "2026-10-17T05:06:49.418327279+00:00"
id: "astromech-ssyo"
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
status: "todo"
title: "Search kit: live pad label and tests for the pad and age paths"
updated_at: "2026-10-17T05:06:49.418327279+00:00"
---

## Goal

The "Ship's (X)" label in SearchFilters.svelte shows the pad from when the filters loaded, not the current ship's; nothing fails if `search_origin` stops re-resolving the pad on a ship change (finding astromech-lh67). Sending the data age to Spansh is tested only for systems searches; the bodies search (sites.rs) and colony candidates (colonies.rs) are checked by inspection (finding astromech-1zi5).

## Done when

A ship-change test without re-selecting the lookup fails if the pad isn't re-resolved, the label follows the ship, and the bodies and colony searches have age-in-request tests.
