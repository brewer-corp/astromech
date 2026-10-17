---
created_at: "2026-10-17T17:41:24.732781627+00:00"
id: "astromech-daex"
issue_type: "task"
labels:
- "docs-found"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Guide polish: help links should follow the installed version; check-guide edge cases"
updated_at: "2026-10-17T17:41:24.732781627+00:00"
---

## Goal

(1) Help links pin blob/main (app/src/lib/help.ts). Point at blob/v<installed version>/ once a tagged release contains docs/guide, falling back to main for dev builds. (2) scripts/check-guide.py skips silently: underscores in headings, closing ## in headings, setext headings, ~~~ fences, titled, reference-style and HTML links. Make it handle or reject them.

## Done when

Help links match the installed version's guide, and check-guide handles or flags those cases.
