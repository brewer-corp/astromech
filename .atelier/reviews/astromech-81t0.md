---
schema: atelier.review
schema_version: 2
id: astromech-81t0
title: finding on astromech-y6d6 @b5af3bc1
target: astromech-y6d6
identity: claude-reviewer-9lh3
commit: b5af3bc135de96d0aa8250e4b14f524c941710d7
created_at: 2026-11-07T19:47:19.229461667Z
event:
  event: finding
  recommendation: defer
---

low: crates/astromech-core/src/update.rs:485-488 (LOCKED_EXES doc). 'The helper runs as astromech-apply-<pid>.exe, so this never ends it' holds by image name only. The helper is the core's child (spawn_apply), and stop_locked_exes uses taskkill /F /T /IM astromech-core.exe; when wait_pid's 45 s times out because the core hasn't exited, /T walks the live core's tree and ends the helper before it copies or relaunches, leaving astromech off. Pre-existing (same on main and v1.0.0-alpha.4, whose helper runs on the updater route), not worsened here. Follow-up: kill astromech-core.exe without /T (its WebView2 and worker are already named), and correct the comment.
