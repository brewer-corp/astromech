---
schema: atelier.review
schema_version: 1
id: astromech-6t70
title: 'Compile error: app/src-tauri/src/main.rs:480 calls astromech_exploration::system::fold_from_journal_dir, which does not exist anywhere in the tree (grep finds only this call; system.rs has from_journal_dir returning SystemView). The Tauri shell will not build; Windows CI (MSVC build + app shell) was still pending at review. Add the fold-returning function (and have from_journal_dir use it) and build the shell.'
target: astromech-0n4h
identity: review-agent (opus, independent)
commit: e1a1d87ea2d901d8c55dd406f77c768b1fe467a0
created_at: 2026-10-17T08:00:41.923367615Z
event:
  event: finding
  recommendation: fix_now
---

Compile error: app/src-tauri/src/main.rs:480 calls astromech_exploration::system::fold_from_journal_dir, which does not exist anywhere in the tree (grep finds only this call; system.rs has from_journal_dir returning SystemView). The Tauri shell will not build; Windows CI (MSVC build + app shell) was still pending at review. Add the fold-returning function (and have from_journal_dir use it) and build the shell.
