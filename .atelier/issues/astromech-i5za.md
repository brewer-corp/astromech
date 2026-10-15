---
created_at: "2026-10-15T20:21:44.872648805+00:00"
id: "astromech-i5za"
issue_type: "task"
labels:
- "size-m"
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-axzc"
  - kind: "issue"
    id: "astromech-hprt"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Find a module near me"
updated_at: "2026-10-15T20:21:44.872648805+00:00"
---

## Goal

Search for a module by group, class, rating and mount ("6A FSD", "3C gimballed multi-cannon") and list the nearest stations selling it, with pad size, arrival distance, data age and price where known. Voice: "where can I buy a 6A FSD?"

## Done when

For a recorded Spansh response, the list is in distance order and honours pad size; the voice phrase "nearest 6A frame shift drive" resolves to the same query.

## Context

Spansh stations search with a module filter. Unverified: the request shape; check it in the site's network traffic first.
