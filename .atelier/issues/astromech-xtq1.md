---
created_at: "2026-10-16T18:58:51.991298875+00:00"
id: "astromech-xtq1"
issue_type: "bug"
labels:
- "alpha"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-610m"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Count Parakeet as installed only when its download finished"
updated_at: "2026-10-17T17:01:40.862760732+00:00"
---

## Goal

An interrupted first-time Parakeet (speech-to-text) extraction can leave all four files with a truncated encoder. `models::missing()` judges Parakeet by its files, so it skips the download, Settings shows Installed, the worker fails to load and nothing repairs it. The Kokoro model already requires its download marker (astromech-a50f).

## Done when

A test with a Parakeet folder that has its files but no marker shows it as missing, and the download list offers it again.

## Context

`crates/astromech-voice/src/models.rs` (`missing`, around line 83). Copied S9 folders carry markers; only a manual install without one would re-download about 480 MB. Found in review of astromech-a50f (round 2, N1).
