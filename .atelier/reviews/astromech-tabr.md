---
schema: atelier.review
schema_version: 2
id: astromech-tabr
title: finding on astromech-bp23 @36ac2e58
target: astromech-bp23
identity: claude-reviewer-ndzx
commit: 36ac2e58e02be587b011992816354b14f2ca84fd
created_at: 2026-11-07T07:42:03.401794571Z
event:
  event: finding
  recommendation: defer
---

medium: crates/astromech-api/src/lookups.rs:538 and app/src/modules/diagnostics/View.svelte:29,49-51 shell_info is now answered by the core, but Diagnostics uses it as the WINDOW's version to compare against status.version ('window X, background Y' for a half-applied update). Both now come from the core, so the mismatch can never show, and with the core not running shellInfo() rejects (unhandled) and the page shows '… (background process not running)' instead of the window's version. Fix: give the page its own version without asking the core (a build-time constant baked into the bundle, which ships inside the shell exe), or drop the comparison if y6d6's one process makes it moot; decide before bp23 merges. Defer to astromech-i8vr (same branch, it rewrites the shell side and transport), so it never reaches main.
