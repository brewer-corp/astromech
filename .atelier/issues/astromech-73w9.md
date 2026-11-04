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
status: "in_progress"
title: "Journal following: cache each journal's header time"
updated_at: "2026-11-04T23:19:23.808683130+00:00"
---

## Goal

Choosing which journal to follow doesn't re-read journal headers several times a second while a touched older journal stays newest by modified time.

## Done when

follow_newest caches each journal's Fileheader time by path (refreshed when the file is replaced), with a test that a refused touched journal's header is read once.

## Context

Review finding R2 (astromech-nbcj) on bug/astromech-ki3j: crates/astromech-core/src/pipeline.rs follow_newest reads the first line of the newest and followed journals on every call. On Windows the game's open journal's listing lags, so this can last the session.

## Notes

- 2026-11-04 [decision] Cache each journal's Fileheader time in the Tailer by path, with the file's creation time and length when read; read again when the creation time differs or the file is shorter. One reader (Tailer::header_time) serves start-time ordering and the header-agrees-with-name check, whose cached answer is dropped when the header is re-read. Stats.header_reads counts reads.
  Reason: The game only appends, so a journal's header never changes in place; a replacement (restore by rename, truncate and rewrite) shows as a new creation time or a shorter file. A stat per look is far cheaper than opening and reading the first line. Creation time is unsupported on some filesystems (None == None), and NTFS tunnelling can keep it for a recreated file, so a longer replacement on such a filesystem is not noticed; journals keep their name's session, so the header is the same in practice.
