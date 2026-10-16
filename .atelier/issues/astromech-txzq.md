---
created_at: "2026-10-16T18:12:15.204408970+00:00"
id: "astromech-txzq"
issue_type: "bug"
labels:
- "size-m"
workflow_branch:
  branch_base: main
  work_branch: bug/astromech-txzq
priority: "P0"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-610m"
    type: "advances"
  - kind: "issue"
    id: "astromech-mh3m"
    type: "derived_from"
  - kind: "issue"
    id: "astromech-rggm"
    type: "derived_from"
  - kind: "issue"
    id: "astromech-ztrb"
    type: "derived_from"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-16T18:41:17.492512760+00:00"
status: "done"
title: "Journal changes take 30 s to a minute to reach the UI"
updated_at: "2026-10-16T18:41:17.492512760+00:00"
---

## Goal

The app feels laggy. Changes to engineering materials often take 30 s to a minute to show in the UI after the game writes them, and journal updates in general are not always picked up quickly. Find where the time goes and get journal changes to the UI within about 2 s.

## Done when

A timed check (appending fixture events to a live journal file) shows each change on the Materials page within 2 s, and a regression test or diagnostic timer guards it. Record where the delay came from as a note.

## Context

The UI polls every 2 s (Materials) and every 5 s (Builds). The core watches the journal in `crates/astromech-core/src/app.rs` (around lines 540 to 580). Some material changes may only show up as a full `Materials` event rather than the incremental events. Diagnose before changing anything.

## Notes

- 2026-10-16 [note] Diagnosis: the Materials page never goes through the core. The app window (app/src-tauri/src/main.rs materials_inventory) replays the journal folder itself and caches the result keyed on journal_folder_sig, which reads DirEntry::metadata. On Windows that comes straight from FindNextFileW, so it is the size and last-write time NTFS stored in the directory entry (std docs: no extra system calls on Windows), and NTFS updates that entry lazily while the game holds the journal open. The key stays the same, the page keeps getting the cached inventory, and it only refreshes when NTFS happens to update the entry, which fits the 30 s to a minute. Facts: (1) code path and cache key (read in code); (2) every incremental material event (collect, discard, trade, engineer craft, synthesis, broker, mission reward) is parsed and applied: on Linux each reaches the shell's output on the first poll in 1 to 2 ms, so the full Materials event isn't needed; (3) recompute is 157 ms for an 8 MB session journal in a release build, about 0.1 ms on a cache hit. Inference, Windows-only and not reproducible on Linux, where stat is always current: that the stale directory entry is the decisive delay with the real game; astromech-rggm checks it. Fix: key the Materials cache on astromech_materials::journal_version, which reads the newest journal's length through an open handle (GetFileInformationByHandle on Windows, always current), and poll every 1 s, so the worst case is 1 s plus recompute. Guarded by crates/astromech-materials/tests/live_journal.rs, which also runs on the Windows CI job. Not changed, filed instead: the Builds page uses the same stale key, but a correct key would re-read all journal history on every append (astromech-ztrb); the core pipeline reads the journal only on file notifications, which Windows may also defer, so voice state can lag while Status.json is idle (astromech-mh3m; the app pages don't use it).

## Close Reason

Materials cache now keys on the newest journal's length read through an open handle, and the page polls every 1 s. Timed check: every material event type showed within 966 ms worst case (evidence astromech-emzr). Regression test live_journal.rs also runs on Windows CI (evidence astromech-76xv). Delay cause recorded as a note. Real-game Windows confirmation is astromech-rggm (needs-human).
