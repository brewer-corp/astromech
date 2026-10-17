---
created_at: "2026-10-16T18:38:18.704215608+00:00"
id: "astromech-ztrb"
issue_type: "bug"
labels:
- "alpha"
- "size-m"
workflow_branch:
  branch_base: main
  work_branch: bug/astromech-ztrb
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-610m"
    type: "advances"
  - kind: "issue"
    id: "astromech-txzq"
    type: "derived_from"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Builds page still lags journal changes on Windows"
updated_at: "2026-10-17T17:12:23.444963066+00:00"
---

## Goal

Get journal changes onto the Engineering (Builds) page within about 2 s on Windows, as astromech-txzq did for Materials.

## Done when

Appending engineering-relevant events (Loadout, EngineerCraft, EngineerProgress) to an open journal changes what `engineering_fleet` and `engineering_engineers` return within 2 s, including on Windows, and a test guards it.

## Context

`engineering_state` in `app/src-tauri/src/main.rs` still caches on `journal_folder_sig`, which reads `DirEntry::metadata`. On Windows that is the NTFS directory entry, which can lag appends to the file the game holds open (astromech-txzq). The stale key happens to throttle a full re-read: `astromech_engineering::from_journal_dir` and `engineers::from_journal_dir` parse every journal file on every miss, so switching to the live `astromech_materials::journal_version` key alone would re-read the whole journal history every few seconds in game. That needs an incremental replay (keep state plus a per-file offset) or a key that only changes on relevant events. The page also polls every 5 s (`app/src/modules/engineering/View.svelte`).

## Notes

- 2026-10-17 [note] Root cause: engineering_state cached on journal_folder_sig (DirEntry metadata: lags appends to the open journal on Windows, astromech-txzq). Switching to journal_version alone would re-read every journal on each append (all five replays rescan all history). Fix: astromech_journal::JournalWatch reads bytes appended to the newest journal through an open handle and bumps a generation only when a complete new line parses to a modelled (non-Other) event; JournalCache rebuilds only on a new generation. All replays use parse_line and none read Other, so unmodelled events (Music, ReceiveText) cannot change the page. Pages poll every 1 s. Unverified: real Windows timing (astromech-rggm covers Materials; Builds needs the same check); app/src-tauri cannot be built here (no webkit), so main.rs changes are checked by inspection and Windows CI build/clippy.
