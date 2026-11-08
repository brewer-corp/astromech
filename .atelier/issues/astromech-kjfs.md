---
created_at: "2026-11-07T04:50:34.669875805+00:00"
id: "astromech-kjfs"
issue_type: "mission"
labels: []
strategy:
  path: docs/strategies/one-tested-process.md
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "One tested process"
updated_at: "2026-11-07T22:55:39.284832217+00:00"
---

## Outcome

astromech is one Tauri process whose single command layer serves the window, the control API and a headless mode, and its tests and previews exercise the real core from a scripted game world, with one transport seam for the window and a small journey suite in CI. The strategy is docs/strategies/one-tested-process.md.

## Notes

- 2026-11-07 [note] Mission close (journeys-in-ci, from the yis2 plan review): a workflow_dispatch run of ci.yml on main's last commit at close passes, journey suite included; link it from an evidence receipt on astromech-kjfs with --criterion journeys-in-ci.
