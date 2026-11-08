---
created_at: "2026-11-08T15:52:46.472286357+00:00"
id: "astromech-h95t"
issue_type: "task"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Release notes for v1.0.0-alpha.5, the first one-app release"
updated_at: "2026-11-08T15:52:46.472286357+00:00"
---

## Goal

Players updating from v1.0.0-alpha.4 learn that astromech is now one app, what else changed, and what still needs checking in the game.

## Done when

- [ ] `docs/releases/v1.0.0-alpha.5.md` follows alpha.4's layout (intro, Installing, What's new since alpha.4, What has not been checked in the game, Known limits) and covers every user-visible change on main since the v1.0.0-alpha.4 tag.
- [ ] README's release-notes link points at it.
- [ ] `python3 scripts/check-release-notes.py docs/releases/v1.0.0-alpha.5.md` passes.
