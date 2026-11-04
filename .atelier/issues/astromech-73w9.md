---
created_at: "2026-11-04T20:56:30.688754673+00:00"
id: "astromech-73w9"
issue_type: "task"
labels:
- "size-s"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
  - kind: "review"
    id: "astromech-nbcj"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Journal following: cache each journal's header time"
updated_at: "2026-11-04T20:56:30.688754673+00:00"
---

## Goal

Choosing which journal to follow doesn't re-read journal headers several times a second while a touched older journal stays newest by modified time.

## Done when

follow_newest caches each journal's Fileheader time by path (refreshed when the file is replaced), with a test that a refused touched journal's header is read once.

## Context

Review finding R2 (astromech-nbcj) on bug/astromech-ki3j: crates/astromech-core/src/pipeline.rs follow_newest reads the first line of the newest and followed journals on every call. On Windows the game's open journal's listing lags, so this can last the session.
