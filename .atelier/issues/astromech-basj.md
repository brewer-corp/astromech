---
created_at: "2026-10-17T16:54:24.108790358+00:00"
id: "astromech-basj"
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
    id: "astromech-stxg"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-17T17:28:49.160067401+00:00"
status: "done"
title: "Link each app page to its guide page"
updated_at: "2026-10-17T17:28:49.160067401+00:00"
---

## Goal

Each app page has a small help link that opens its guide page on GitHub (through the existing open_external path).

## Done when

Every page with a guide page links to it, and a test checks every link target exists in `docs/guide/`.

## Close Reason

Help link on every page and the setup flow; test checks every target file and heading in docs/guide; lint, check, build and 203 tests pass.
