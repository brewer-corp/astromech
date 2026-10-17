---
created_at: "2026-10-17T19:45:12.253105436+00:00"
id: "astromech-6a7c"
issue_type: "bug"
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
title: "The window can follow a stale config.path after a --config run"
updated_at: "2026-10-17T19:45:12.253105436+00:00"
---

## Goal

Paths::publish_config (astromech-c9em) tells the window which config file the tray core uses, in config.path. It is only written when the tray core starts, so a deleted --config target, a console-mode run (run_console does not publish) or two cores with different --config values can leave the window reading a file the core is not using.

## Done when

The window falls back to the default file when the published path does not exist, console runs publish or clear it, and a test covers each.
