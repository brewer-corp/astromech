---
schema: atelier.review
schema_version: 2
id: astromech-s8ri
title: review on astromech-y6d6 @b5af3bc1
target: astromech-y6d6
identity: claude-reviewer-9lh3
commit: b5af3bc135de96d0aa8250e4b14f524c941710d7
created_at: 2026-11-07T19:47:17.579334843Z
event:
  event: review
---

Reviewed astromech-9lh3 (27d6f1cb, f6a048be, b5af3bc1) from 'atelier issue show astromech-9lh3 --full' and the diff; review request refused (Done when lines wait on Windows CI). Verified: Tauri 2.12.2's installer.nsi (extracted from the cli .node in app/node_modules) runs NSIS_HOOK_PREINSTALL/PREUNINSTALL before CheckIfAppIsRunning (RmShutdown RmForceShutdown) and nothing earlier kills the app under /S; Section Install deletes $INSTDIR\$OldMainBinaryName read after the hook, so the PREINSTALL re-register keeps the forwarder; allowDowngrades defaults true so 0.0.0-dev /S over 1.0.0-alpha.4 is allowed; alpha.4 was built with cli 2.12.2. Quit path, spawn_apply's astromech-apply-<pid>.exe rename, relaunch --route/--background, Run value format and the Local\astromech-core mutex are unchanged against main and v1.0.0-alpha.4; LOCKED_EXES is the same list and order. Old window watchdog (alpha.4 main.rs:80-100) acts only when core.json is present, so hook order --quit, shell, forced fallback is sound. y2m1's claim holds (PageLeaveReinstall's default on upgrade runs the old uninstaller without /UPDATE; apply_default never re-sets). Ran: cargo test -p astromech-shell -p astromech-log (6 passed), cargo test -p astromech-core --lib update:: the_window_page (9 passed); a stand-in server with alpha.4's reply shape against scripts/upgrade_smoke.py's call/wait. Not run: makensis, anything on Windows.
