---
schema: atelier.review
schema_version: 1
id: astromech-0ohq
title: 'R3 is only partly one shared rule: astromech_store::data_dir shares the final override/portable/app-dir choice, but the shell''s core_data_dir (app/src-tauri/src/main.rs:918-930) re-implements Config::portable(), Config::default_path() and exe_dir, and the shell''s app_dir() (main.rs:31) already differs from Config::app_dir() (no XDG_DATA_HOME; no-HOME fallback ./astromech/data vs exe_dir/astromech-data). On Windows LOCALAPPDATA is always set, so no current path diverges, but the inputs can drift. R3''s done-when check covers only the shared helper; the shell side is untested and not compiled on Linux (Windows CI was still pending at review). Suggest moving exe/app dir, portable detection and config location into one shared helper used by both.'
target: astromech-n1ij
identity: review-agent (opus, independent)
commit: df2c4fb438a4f201a3a05fc02770c5536db43657
created_at: 2026-10-17T05:05:43.909380329Z
event:
  event: finding
  recommendation: defer
---

R3 is only partly one shared rule: astromech_store::data_dir shares the final override/portable/app-dir choice, but the shell's core_data_dir (app/src-tauri/src/main.rs:918-930) re-implements Config::portable(), Config::default_path() and exe_dir, and the shell's app_dir() (main.rs:31) already differs from Config::app_dir() (no XDG_DATA_HOME; no-HOME fallback ./astromech/data vs exe_dir/astromech-data). On Windows LOCALAPPDATA is always set, so no current path diverges, but the inputs can drift. R3's done-when check covers only the shared helper; the shell side is untested and not compiled on Linux (Windows CI was still pending at review). Suggest moving exe/app dir, portable detection and config location into one shared helper used by both.
