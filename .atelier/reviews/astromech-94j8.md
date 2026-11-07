---
schema: atelier.review
schema_version: 1
id: astromech-94j8
title: finding on astromech-jdks @f8fc601a
target: astromech-jdks
identity: claude-reviewer-jdks
commit: f8fc601a97c50c538f8fdc4a44d42a163063544d
created_at: 2026-11-07T00:47:13.486863769Z
event:
  event: finding
  recommendation: no_action
---

info, for astromech-b39t: cargo deny --features astromech-shell/window check fails on this branch: unmaintained zbus_names, zvariant, zvariant_derive, proc-macro-error; licence Apache-2.0 WITH LLVM-exception (target-lexicon 0.12.16 via cfg-expr, Linux build-time) not allowed; yanked async-executor. Not a regression (the window's tree was never checked); deny without the feature passes.
