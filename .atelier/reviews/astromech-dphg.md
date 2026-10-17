---
schema: atelier.review
schema_version: 1
id: astromech-dphg
title: 'Residual shell assumptions (low): Paths::current() in the shell uses the shell exe''s folder as exe_dir, so portable detection matches the core only when both exes sit together (true installed; false for a dev ASTROMECH_SHELL pointing elsewhere), and the core''s --config override stays invisible to the window (documented in ARCHITECTURE). The shell wiring (data_dir(), config(), open_external roots) has no test and the shell does not compile on Linux; the Windows CI jobs were still pending at review. Grep of app/src-tauri/src finds no LOCALAPPDATA/APPDATA/XDG/current_exe path logic left (only USERPROFILE/HOME for the game journal folder, which is the game''s, not the core''s, and a current_exe for locating the core exe).'
target: astromech-4yds
identity: review-agent (sonnet, independent)
commit: 971e5efaa926c58970fd440f72b15847d1d6a6cb
created_at: 2026-10-17T06:35:09.453661295Z
event:
  event: finding
  recommendation: defer
---

Residual shell assumptions (low): Paths::current() in the shell uses the shell exe's folder as exe_dir, so portable detection matches the core only when both exes sit together (true installed; false for a dev ASTROMECH_SHELL pointing elsewhere), and the core's --config override stays invisible to the window (documented in ARCHITECTURE). The shell wiring (data_dir(), config(), open_external roots) has no test and the shell does not compile on Linux; the Windows CI jobs were still pending at review. Grep of app/src-tauri/src finds no LOCALAPPDATA/APPDATA/XDG/current_exe path logic left (only USERPROFILE/HOME for the game journal folder, which is the game's, not the core's, and a current_exe for locating the core exe).
