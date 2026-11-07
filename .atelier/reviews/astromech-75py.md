---
schema: atelier.review
schema_version: 2
id: astromech-75py
title: finding on astromech-kjfs @5d19a292
target: astromech-kjfs
identity: claude-reviewer-kjfs
commit: 5d19a29220e8c49fd861ed2ad6541ef56f24eeb8
created_at: 2026-11-07T04:54:26.917259188Z
event:
  event: finding
  recommendation: defer
---

low: docs/strategies/one-tested-process.md Why, astromech-6i8s Context, astromech-bp23 Done when 'about 42 commands' undercounts: generate_handler in app/src-tauri/src/window.rs:1354-1406 registers 52 (48 besides core_status, core_call, shell_info, open_external). Fix next time these records are touched; bp23's test lists them anyway.
