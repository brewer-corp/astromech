---
created_at: "2026-10-16T23:17:08.441537589+00:00"
id: "astromech-z5tn"
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
status: "backlog"
title: "Fold journals incrementally for the colonisation view"
updated_at: "2026-10-16T23:17:08.441537589+00:00"
---

## Goal

The colonisation view re-parses the whole journal folder whenever the newest journal grows (about 0.7 s on a 96 MB folder in release). Fold new lines incrementally instead, like a tailer.

## Done when

A test or measurement shows a journal append costs time proportional to the new lines, not the folder.

## Context

Deferred from the astromech-05bi review (crates/astromech-colonisation/src/lib.rs:362-378, app/src-tauri/src/main.rs:591-604). Same pattern as the Engineering and Powerplay views.
