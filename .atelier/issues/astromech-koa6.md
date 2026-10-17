---
created_at: "2026-10-17T17:29:23.095776462+00:00"
id: "astromech-koa6"
issue_type: "bug"
labels:
- "alpha"
- "docs-found"
workflow_branch:
  branch_base: main
  work_branch: bug/astromech-koa6
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-17T19:45:31.833848123+00:00"
status: "done"
title: "Data sharing page says 30 minutes; the sender uses 5"
updated_at: "2026-10-17T19:45:31.833848123+00:00"
---

## Goal

app/src/modules/datasharing/View.svelte says nothing older than 30 minutes is sent. crates/astromech-eddn/src/feed.rs MAX_EVENT_AGE_SECS is 5 * 60, and the EDSM uploader follows the same notion of live. Make the page text match the code (or the code the text, if 30 minutes was intended).

## Done when

The page and the sender agree, and docs/guide/data-sharing.md says the same number. Found while writing the user guide.

## Close Reason

Fixed in PR 136; guide and page match the sender (two limits, EDDN-only 30 min)
