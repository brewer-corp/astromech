---
schema: atelier.review
schema_version: 1
id: astromech-ox6s
title: 'R6 low, 83og poll cost: Plot.svelte:29 now calls route_plot_ship every 5 s, and the shell''s plot_ship (app/src-tauri/src/main.rs:941) re-parses the last two journal files on every call (state_from_journal_dir) outside the journal cache. Plot is mounted in the Galaxy rail, so this runs on every Galaxy page while open. Small on typical journals; noted so it is not mistaken for free. No change needed for this branch.'
target: astromech-jo3l
identity: review-agent-jo3l-1
commit: 80085727d888bdaa7d42f5c57a54b04d47b706d1
created_at: 2026-10-22T21:33:59.013657995Z
event:
  event: finding
  recommendation: no_action
---

R6 low, 83og poll cost: Plot.svelte:29 now calls route_plot_ship every 5 s, and the shell's plot_ship (app/src-tauri/src/main.rs:941) re-parses the last two journal files on every call (state_from_journal_dir) outside the journal cache. Plot is mounted in the Galaxy rail, so this runs on every Galaxy page while open. Small on typical journals; noted so it is not mistaken for free. No change needed for this branch.
