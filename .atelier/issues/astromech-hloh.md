---
created_at: "2026-10-16T20:25:32.396940512+00:00"
id: "astromech-hloh"
issue_type: "task"
labels: []
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-ipb5"
    type: "related"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Logbook: Unicode search, busy timeout and data folder"
updated_at: "2026-10-22T20:41:39.389745824+00:00"
---

## Goal

Logbook search lowercases the query with Unicode rules but SQLite's lower() folds ASCII only, so names with non-ASCII capitals are never found; the window's read-only history connection has no busy timeout, so a brief SQLITE_BUSY replaces the logbook with an error until the next poll; and the window opens history.sqlite from the default folder, not Config::data_dir(), so a custom or portable data folder shows no history.

## Done when

Search folds both sides the same way (test with a non-ASCII name); the read-only connection sets a busy timeout and the page keeps the last good result on error; the window finds history.sqlite wherever the core writes it.

## Context

`crates/astromech-history/src/logbook.rs`, `lib.rs` (open_read_only), `app/src-tauri/src/main.rs`. Found in review of astromech-fxp0 (R1–R3); the data-folder part is the same fix as the one filed for the shell settings database (astromech-n1ij on main).
