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
closed_at: "2026-10-22T21:53:39.453485026+00:00"
status: "done"
title: "Logbook: Unicode search, busy timeout and data folder"
updated_at: "2026-10-22T21:53:39.453485026+00:00"
---

## Goal

Logbook search lowercases the query with Unicode rules but SQLite's lower() folds ASCII only, so names with non-ASCII capitals are never found; the window's read-only history connection has no busy timeout, so a brief SQLITE_BUSY replaces the logbook with an error until the next poll; and the window opens history.sqlite from the default folder, not Config::data_dir(), so a custom or portable data folder shows no history.

## Done when

Search folds both sides the same way (test with a non-ASCII name); the read-only connection sets a busy timeout and the page keeps the last good result on error; the window finds history.sqlite wherever the core writes it.

## Context

`crates/astromech-history/src/logbook.rs`, `lib.rs` (open_read_only), `app/src-tauri/src/main.rs`. Found in review of astromech-fxp0 (R1–R3); the data-folder part is the same fix as the one filed for the shell settings database (astromech-n1ij on main).

## Notes

- 2026-10-22 [note] 2026-10-22: The data-folder part was already fixed on main by 4df831fe (the window opens history.sqlite via Paths::data_dir(config), the same rule as Config::data_dir()), and the page already keeps its last good result on a failed poll (astromech-spxx, app/src/lib/ui/stale.test.ts). rusqlite already defaulted to a 5 s busy timeout; open_read_only now sets 2 s explicitly. No shell change was needed.

## Close Reason

Done and independently reviewed on feature/astromech-fdh3 (approval astromech-o2y7).
