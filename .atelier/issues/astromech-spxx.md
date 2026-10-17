---
created_at: "2026-10-17T22:48:16.486252976+00:00"
id: "astromech-spxx"
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
title: "Polling pages hide a failure after the first successful read"
updated_at: "2026-10-17T22:48:16.486252976+00:00"
---

## Goal

Ships, Materials, Powerplay and Station show a later failed poll, not only a failed first read.

## Done when

When a poll fails after data was shown, the page keeps the data and says it is stale (or shows the error beside it). A test per page.

## Context

Found in the review of the astromech-x81p PR: these pages show their error only while they hold no data (!fleet, err && !ov), so a core that goes away mid-session leaves old numbers on screen with no sign.
