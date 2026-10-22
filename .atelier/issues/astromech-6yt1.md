---
created_at: "2026-10-16T19:20:28.657350702+00:00"
id: "astromech-6yt1"
issue_type: "task"
labels: []
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-9mb9"
    type: "related"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Apply the URL character filter to every open_external target"
updated_at: "2026-10-22T20:41:39.919410557+00:00"
---

## Goal

`open_external` in `app/src-tauri/src/main.rs` lets `https://github.com/brewer-corp/astromech/…` through with a bare prefix check, skipping the refusal of spaces, control characters, quotes and commas that `astromech_engineering::links::is_openable_url` applies (explorer.exe parses its own command line). A compromised webview could pass switches after that prefix.

## Done when

Every URL `open_external` accepts goes through one shared predicate that refuses those characters, with a test for the GitHub prefix.

## Context

Found in review of astromech-9mb9 (round 3, R1). The only caller today is the hard-coded release-notes button.
