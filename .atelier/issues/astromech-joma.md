---
created_at: "2026-10-17T18:47:50.693699371+00:00"
id: "astromech-joma"
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
title: "Route follower callout: long procedural names break the 8-word callout budget"
updated_at: "2026-10-22T21:29:46.532799361+00:00"
---

## Goal

The arrival callout "Next waypoint. <system>." stays within the copy guide for any system name, or the guide says how a name-only sentence is counted.

## Done when

`crates/astromech-core/tests/copy.rs` checks the route callout and the "next waypoint" answer (16-word budget, with the longest plotter label "neutron boost and fuel") on a long sector name (e.g. "Hyades Sector DB-W b2-6", which reads as 9 words, or "Col 285 Sector AB-C d14-123") and passes, with the rule written down in docs/V1-DESIGN.md §6.5.

## Context

Found while adding the Spansh plotters (astromech-94j4): plotted routes are mostly procedural names, so the follower will often speak a name sentence longer than the 8-word callout budget. The name is the whole content and cannot be shortened; this is a copy-policy question (exempt or count names differently), not a plotter bug. The copy test currently uses "Synuefe BY-F d12-39" (8 words).
