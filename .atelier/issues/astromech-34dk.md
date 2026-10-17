---
created_at: "2026-10-17T19:46:37.537080741+00:00"
id: "astromech-34dk"
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
title: "Tray Check for updates: say the result when nothing is found or the check fails"
updated_at: "2026-10-17T19:46:37.537080741+00:00"
---

## Goal

A check started from the tray (astromech-naa3) only reverts the menu line to "Check for updates"; with no update, or on an error, nothing says so. Settings shows the result.

## Done when

The tray (or a toast) reports up to date or the error after a requested check.
