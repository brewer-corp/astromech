---
schema: atelier.review
schema_version: 2
id: astromech-zo5n
title: finding on astromech-y6d6 @26afef19
target: astromech-y6d6
identity: claude-reviewer-v3u4
commit: 26afef19d845b864358994d291ec64c98ed410ca
created_at: 2026-11-07T17:57:42.877141875Z
event:
  event: finding
  recommendation: fix_now
---

low: app/vite.config.ts:7,10-11. Comments still say the bundle 'ships inside the shell's exe, so Diagnostics can tell a window and a core from different releases apart' and 'The Tauri shell loads dist/'. After this change Diagnostics shows one version (decision note on v3u4) and there is no shell, so the comment states the opposite of the code that reads __APP_VERSION__ (app/src/modules/diagnostics/View.svelte:46-47). Fix: reword to 'the app's version, shown in Diagnostics' and 'The app (astromech-core) loads dist/'.
