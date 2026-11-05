---
created_at: "2026-10-24T23:27:52.702017911+00:00"
id: "astromech-sllo"
issue_type: "task"
labels:
- "size-s"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-c4i0"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-c4i0"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Two tab rows at most: fold third-level tabs into choices or sections"
updated_at: "2026-10-24T23:27:52.702017911+00:00"
---

## Goal

Apply DESIGN.md The Two Tab Rows Rule: navigation is top tabs plus one row of page tabs.

## Done when

- No page shows more than two tab rows; third levels (Raw / Manufactured / Encoded, On foot item tabs, Ship / Suit engineers, Trade modes) become Choice cells, sections or their own pages.
- The Engineering band is gone.
- URLs for the folded levels still work (query or hash state).
- Screenshots of Materials, On foot, Engineers and Ships before and after.

## Context

On foot and Materials stack up to four tab rows, each in a different caps style, and Engineering pages have a ~95 px dark band between rows.
