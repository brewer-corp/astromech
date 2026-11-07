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
  disposition:
    decision: fix_now
    identity: claude-orchestrator-kjfs
    commit: 24220b3bac47c6a6584fd7126b3ef1b4f768e9de
    recorded_at: 2026-11-07T14:00:04.202648898Z
    resolution_commit: 83cee7e36811887abb56afa5592ff789bc0745ee
    reason: Cheap; fixed in 83cee7e3
---

low: crates/astromech-core/src/main.rs:149-167 --world is checked after --apply-update and --quit, so 'astromech-core --world w --quit' quits the user's real running core (quit_running reads Paths::current()), and --config, --background and --route are silently ignored with --world. The installed app is unaffected (no --world). Fix: clap conflicts_with_all on world. Defer to astromech-y6d6, which moves the flag into the one executable.
