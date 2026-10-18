---
created_at: "2026-10-17T19:23:38.246138599+00:00"
id: "astromech-54q6"
issue_type: "task"
labels: []
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Background lookups (known biology, bodies known): retry cadence and session-long answers"
updated_at: "2026-10-17T19:23:38.246138599+00:00"
---

## Goal

The remote arrival callouts retry and refresh deliberately: a known number of retries while an arrival waits, and an answer that can go stale over a long session refreshed.

## Done when

`astromech_core::background` documents and tests its retry cadence while a callout waits for a late answer, and a Done answer expires (or is refreshed) after a set time, with a test.

## Context

Review findings on PR #129 (astromech-8sx3), both LATER. (1) The rule asks on every tick (about once a second), so with EDSM offline the 15 s back-off lets a new lookup start about every 15 s in the minute window: up to about 3 retries and 3 log lines per arrival, bounded by the client rate limit and untested. (3) Background keeps a Done answer for the whole session, on top of the client cache (EDSM a week), so a system scanned by others later in a long session keeps its old count. Known biology behaves the same way.
