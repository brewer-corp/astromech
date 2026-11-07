---
created_at: "2026-11-07T00:01:47.475936108+00:00"
id: "astromech-ja5a"
issue_type: "task"
labels:
- "alpha"
- "size-s"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Ship v1.0.0-alpha.4 with the galaxy rework"
updated_at: "2026-11-07T00:01:47.475936108+00:00"
---

## Goal

Sumeet is still on alpha.3 (23 Oct), so the galaxy rework, the Find or plot box (no From field, no self-closing plot panel) and the rest of main since then haven't reached him. Publish the next alpha through the updater so he can play-check it (astromech-cxgq).

## Done when

- Version 1.0.0-alpha.4 in Cargo.toml, both Cargo.locks, app/package.json, app/src-tauri/Cargo.toml and tauri.conf.json; README links its notes.
- docs/releases/v1.0.0-alpha.4.md covers what changed since alpha.3 and passes scripts/check-release-notes.py.
- The tag is pushed and v1.0.0-alpha.4 is published on brewer-corp/astromech with latest.json.

## Context

Same steps as alpha.3 (astromech-rcui). Sumeet approved cutting it on 2026-11-06.
