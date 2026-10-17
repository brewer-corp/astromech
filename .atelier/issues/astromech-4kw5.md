---
created_at: "2026-10-17T06:02:43.373276974+00:00"
id: "astromech-4kw5"
issue_type: "task"
labels: []
workflow_branch:
  branch_base: main
  work_branch: task/astromech-4kw5
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-17T06:30:54.241663753+00:00"
status: "done"
title: "Shell: write its log lines to a file"
updated_at: "2026-10-17T06:30:54.241663753+00:00"
---

## Goal

The Tauri shell logs with eprintln!, but release builds use windows_subsystem = "windows", so stderr goes nowhere: a cache file that won't open (finding astromech-cw9m on astromech-zs2q) and unknown Odyssey mod names (astromech-j3sk on astromech-f1us) are invisible. Write the shell's log lines to a log file next to the core's.

## Done when

A shell test shows a logged line lands in the shell log file; the cache-open and unknown-mod messages use it.

## Close Reason

The shell logs to astromech-shell.log next to astromech-core.log through the shared astromech-log crate; the cache-open and unknown-mod messages use it.
