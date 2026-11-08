---
schema: atelier.review
schema_version: 2
id: astromech-4r9s
title: finding on astromech-y6d6 @916c97c3
target: astromech-y6d6
identity: claude-reviewer-y6d6-merge
commit: 916c97c3d378b97645c35f5e0085e265eb6510ee
created_at: 2026-11-08T15:37:17.290672363Z
event:
  event: finding
  recommendation: defer
---

low: app/src/lib/proxy.browser.test.ts:6-7 and crates/astromech-api/src/lib.rs:10 still name the removed shell: 'the shell's one proxy ... (app/src-tauri/src/window.rs)' and 'the shell's proxy stamps origin: app_window on what it forwards'. window.rs is gone (the merge kept it deleted) and the window now dispatches in process through host.rs's call and control::window_request. Not from these four commits (both predate 571fddcd), and comments only, so nothing fails; a reader following them looks for a file that doesn't exist. Fix: point both at app/src-tauri/src/host.rs's call (in process, control::window_request).
