---
created_at: "2026-10-16T18:12:14.949841375+00:00"
id: "astromech-vmot"
issue_type: "task"
labels:
- "size-m"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-snqt"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "System names as chips with a copy button"
updated_at: "2026-10-16T18:12:14.949841375+00:00"
---

## Goal

Every system name in the app should be a chip with a one-click copy-to-clipboard button, so the user can paste it into the galaxy map.

## Done when

One shared chip component is used on every page that shows a system name: Nearby lookups, engineer trips, Materials and HGE results, and the ship cards. A component test shows that clicking copy puts the exact system name on the clipboard.

## Context

The route-clipboard branch adds `crates/astromech-core/src/clipboard.rs`. Use it if it has landed; otherwise use the webview clipboard API.
