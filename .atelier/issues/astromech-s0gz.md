---
created_at: "2026-10-16T19:47:26.525520987+00:00"
id: "astromech-s0gz"
issue_type: "task"
labels:
- "alpha"
- "size-s"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Callout toggles and cooldowns in Settings"
updated_at: "2026-10-17T17:01:41.508366203+00:00"
---

## Goal

The Settings page lists every callout with an on/off switch and its cooldown, so callouts can be tuned without editing `astromech.toml`.

## Done when

Turning a callout off or changing its cooldown in Settings is written to `[callouts.<id>]` in `astromech.toml` and takes effect on the next session (or at once); a test covers the round trip.

## Context

The reaction engine (astromech-ngyt) reads per-rule `enabled` and `cooldown_s` from `[callouts.<id>]` in `astromech.toml`; there is no UI for them yet. Today the window cannot write the config (it opens it in Notepad), so this needs a core command to change it. The prospector thresholds (astromech-omi1) belong on the same page.
