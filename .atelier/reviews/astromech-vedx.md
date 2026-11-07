---
schema: atelier.review
schema_version: 2
id: astromech-vedx
title: finding on astromech-bp23 @36ac2e58
target: astromech-bp23
identity: claude-reviewer-ndzx
commit: 36ac2e58e02be587b011992816354b14f2ca84fd
created_at: 2026-11-07T07:42:04.132430099Z
event:
  event: finding
  recommendation: defer
---

low: crates/astromech-store/src/paths.rs:155-180 and crates/astromech-core/src/app.rs:41-50 Paths::publish_config/config_in_use (the config.path note, its pid liveness and windows-sys use) now have no reader. The doc comments say so, but no follow-up issue is filed (the Close Reason calls it one). ARCHITECTURE §5.8 also still says 'the core and the app window opening an old file together' for remote_cache.sqlite, which no longer happens. Fix: file an issue to delete the note (or do it in i8vr/y6d6) and update that sentence.
