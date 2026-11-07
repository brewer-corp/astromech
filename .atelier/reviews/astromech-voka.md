---
schema: atelier.review
schema_version: 1
id: astromech-voka
title: finding on astromech-jdks @f8fc601a
target: astromech-jdks
identity: claude-reviewer-jdks
commit: f8fc601a97c50c538f8fdc4a44d42a163063544d
created_at: 2026-11-07T00:47:13.055674928Z
event:
  event: finding
  recommendation: defer
---

low: .github/workflows/webview-cost.yml:10 The trigger paths still assume the window has its own lockfile: a tauri/wry/webview bump now only changes the root Cargo.lock (app/src-tauri/Cargo.lock is gone), so the App window cost workflow no longer runs for it. Fix: add Cargo.lock (and Cargo.toml) to its paths.
