---
created_at: "2026-10-15T20:36:13.663495866+00:00"
id: "astromech-orel"
issue_type: "task"
labels:
- "size-s"
priority: "P0"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-0474"
    type: "related"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-16T19:55:54.147107120+00:00"
status: "done"
title: "Settings → Voice responses"
updated_at: "2026-10-16T19:55:54.147107120+00:00"
---

## Goal

Colour, speaker with a preview button, and speed in Settings (config file only today).

## Done when

Changing the speaker in Settings changes the next reply, and survives a restart.

## Close Reason

Settings → Voice responses sets speaker (English Kokoro v1.0 voices, British first, with Preview), colour and speed via set_voice through the locked config edit. The session's speaker switches at once, so the next reply uses the new voice (set_voice_changes_the_next_reply), and the setting is in astromech.toml so it survives a restart (voice_settings_persist). Real playback on Windows unverified here.
