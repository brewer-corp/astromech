---
schema: atelier.review
schema_version: 1
id: astromech-udd8
title: '[low] crates/astromech-core/src/route.rs:333-339: Desk::set_here saves route.json (write tmp + rename) on every replayed system change, so the startup catch-up of a long exploration session writes the file once per jump (hundreds of small writes in a burst). Correct and atomic; cost is negligible. Noted only; deferring the save to the end of catch-up is not worth the extra state.'
target: astromech-1135
identity: claude-reviewer-2
commit: 642cafe0a3bc7073503113bcc662608b12300bec
created_at: 2026-10-17T17:39:52.896844471Z
event:
  event: finding
  recommendation: no_action
---

[low] crates/astromech-core/src/route.rs:333-339: Desk::set_here saves route.json (write tmp + rename) on every replayed system change, so the startup catch-up of a long exploration session writes the file once per jump (hundreds of small writes in a burst). Correct and atomic; cost is negligible. Noted only; deferring the save to the end of catch-up is not worth the extra state.
