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
closed_at: "2026-10-16T18:39:19.464606661+00:00"
status: "done"
title: "System names as chips with a copy button"
updated_at: "2026-10-16T18:39:19.464606661+00:00"
---

## Goal

Every system name in the app should be a chip with a one-click copy-to-clipboard button, so the user can paste it into the galaxy map.

## Done when

One shared chip component is used on every page that shows a system name: Nearby lookups, engineer trips, Materials and HGE results. The ship cards are built on feature/astromech-z4jb and adopt the chip there (astromech-3njr). A component test shows that clicking copy puts the exact system name on the clipboard.

## Context

The route-clipboard branch adds `crates/astromech-core/src/clipboard.rs`. Use it if it has landed; otherwise use the webview clipboard API.

## Notes

- 2026-10-16 [note] Chip lives at app/src/lib/ui/SystemChip.svelte (prop: name). Ship cards on feature/astromech-z4jb still need to adopt it: tracked by astromech-3njr. Uses navigator.clipboard.writeText until clipboard.rs lands.

## Close Reason

Shared SystemChip (app/src/lib/ui/SystemChip.svelte) is used for every system name on main's pages: Nearby (covers Materials and HGE results), the Materials location line, and the Builds page's stored-at, trip origin and engineer stops. SystemChip.test.ts asserts the copy writes the exact name. Ship cards adopt it under astromech-3njr.
