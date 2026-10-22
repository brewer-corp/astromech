---
created_at: "2026-10-17T23:30:24.566875711+00:00"
id: "astromech-v65y"
issue_type: "task"
labels:
- "alpha"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-22T21:44:44.881481897+00:00"
status: "done"
title: "Route: a poll erases an action's error within two seconds"
updated_at: "2026-10-22T21:44:44.881481897+00:00"
---

## Goal

A failed Skip, Back, Clear or Copy on the Route page keeps its error until the commander acts again.

## Done when

modules/route/View.svelte no longer clears an action's error when the next 2 s poll succeeds (run() sets err = null on every success). A test covers it.

## Context

Found in the independent review of astromech-spxx; predates it.

## Close Reason

Done and independently reviewed on feature/astromech-jo3l (approval astromech-15yv).
