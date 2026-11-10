---
created_at: "2026-11-07T03:00:10.883768802+00:00"
id: "astromech-q79q"
issue_type: "feature"
labels:
- "size-s"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-xajc"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Status bar: the ship and the app's health on every page"
updated_at: "2026-11-07T03:00:10.883768802+00:00"
---

## Goal

The ship (name, jump range, fuel, cargo) and the app's health (Elite running, Voice ready) sit in one bar at the foot of every page, so no page owns the ship and the header has room.

## Done when

- [ ] A 32 px status bar on every page with the ship from the latest Loadout and Status, and the status line moved out of the header; both update as you play.
- [ ] DESIGN.md's frame has it, with the header's second-row breakpoint remeasured.
- [ ] Checked in the preview at 1100 px and 2560 px wide.
