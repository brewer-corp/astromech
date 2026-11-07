---
schema: atelier.review
schema_version: 1
id: astromech-j896
title: finding on astromech-jdks @f8fc601a
target: astromech-jdks
identity: claude-reviewer-jdks
commit: f8fc601a97c50c538f8fdc4a44d42a163063544d
created_at: 2026-11-07T00:47:12.840299024Z
event:
  event: finding
  recommendation: no_action
---

low: Cargo.toml:63 The window's own release profile (opt-level "s", lto, codegen-units 1, strip) is gone; it now inherits the root's opt-level 3. Intended consequence of joining the workspace, but the shell (and the tauri/wry tree) is now optimised for speed, not size: compare the PR's installer size and the webview-cost numbers with alpha.4. If size matters, [profile.release.package.astromech-shell] opt-level = "s" restores it for the shell crate only (deps stay at 3).
