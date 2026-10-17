---
schema: atelier.review
schema_version: 1
id: astromech-3i2u
title: '[low] app/src-tauri/src/main.rs exploration_system/exploration_unsold + View.svelte polling: every 3 s the shell re-parses the whole of the newest two journal files and opens the history; every 15 s unsold folds all Scan/ScanOrganic lines since the last death (JSON parse each). Runs in spawn_blocking so the runtime is not blocked, but cost grows with a long explorer''s session; consider caching by file length/mtime or a longer interval when the window is hidden. Shell was read for compile errors only (not built on Linux); none found. No allow-list is needed (no per-command permissions in capabilities).'
target: astromech-tzc2
identity: 'review-agent (sonnet, independent, part B: callouts, history, shell, UI)'
commit: abce3096a915e5a11d15fce7f95785d6761e0b09
created_at: 2026-10-17T05:14:59.373331341Z
event:
  event: finding
  recommendation: defer
---

[low] app/src-tauri/src/main.rs exploration_system/exploration_unsold + View.svelte polling: every 3 s the shell re-parses the whole of the newest two journal files and opens the history; every 15 s unsold folds all Scan/ScanOrganic lines since the last death (JSON parse each). Runs in spawn_blocking so the runtime is not blocked, but cost grows with a long explorer's session; consider caching by file length/mtime or a longer interval when the window is hidden. Shell was read for compile errors only (not built on Linux); none found. No allow-list is needed (no per-command permissions in capabilities).
