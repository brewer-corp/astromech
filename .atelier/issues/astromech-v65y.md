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
status: "todo"
title: "Route: a poll erases an action's error within two seconds"
updated_at: "2026-10-22T20:41:35.272146187+00:00"
---

## Goal

A failed Skip, Back, Clear or Copy on the Route page keeps its error until the commander acts again.

## Done when

modules/route/View.svelte no longer clears an action's error when the next 2 s poll succeeds (run() sets err = null on every success). A test covers it.

## Context

Found in the independent review of astromech-spxx; predates it.
