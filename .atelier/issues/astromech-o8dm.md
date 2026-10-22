---
created_at: "2026-10-22T21:26:28.636115803+00:00"
id: "astromech-o8dm"
issue_type: "task"
labels:
- "size-s"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Search results show a failure as a bare reason, without a 'Could not' lead-in"
updated_at: "2026-10-22T21:26:28.636115803+00:00"
---

## Goal

A failed search on any search-kit lookup reads as a sentence with a 'Could not ...' lead-in, as the app's copy rule asks, not the bare reason (for example 'no position in the journal yet').

## Done when

lib/ui/SearchKit.svelte (or the shared error path) shows 'Could not search: <reason>.' or the copy guide's equivalent, with a component test.

## Context

Seen by the astromech-jo3l implementer on 22 Oct 2026; older code, not changed there.
