---
schema: atelier.review
schema_version: 2
id: astromech-3tli
title: finding on astromech-vucf @50e59010
target: astromech-vucf
identity: claude-reviewer-d2nq
commit: 50e59010c8f51ecae55b994a55df7e4d1680b986
created_at: 2026-11-07T13:50:04.412045106Z
event:
  event: finding
  recommendation: defer
---

low: crates/astromech-core/src/main.rs:149-167 --world is checked after --apply-update and --quit, so 'astromech-core --world w --quit' quits the user's real running core (quit_running reads Paths::current()), and --config, --background and --route are silently ignored with --world. The installed app is unaffected (no --world). Fix: clap conflicts_with_all on world. Defer to astromech-y6d6, which moves the flag into the one executable.
