---
schema: atelier.review
schema_version: 1
id: astromech-z8k3
title: 'F1 low real_journals.rs dump guard: a ''..'' after a missing folder passes the nearest-existing-ancestor check and create_dir_all then writes elsewhere (probe wrote the dump into the repo worktree). Refuse paths with a parent-dir component.'
target: astromech-yin0
identity: reviewer agent a6ff3cfa (independent)
commit: 668f80c860a8fd56313c6ca1bb4b140dfca2b374
created_at: 2026-10-17T22:24:20.163722361Z
event:
  event: finding
  recommendation: fix_now
---

F1 low real_journals.rs dump guard: a '..' after a missing folder passes the nearest-existing-ancestor check and create_dir_all then writes elsewhere (probe wrote the dump into the repo worktree). Refuse paths with a parent-dir component.
