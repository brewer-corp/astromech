---
schema: atelier.review
schema_version: 1
id: astromech-au2l
title: finding on astromech-jdks @f8fc601a
target: astromech-jdks
identity: claude-reviewer-jdks
commit: f8fc601a97c50c538f8fdc4a44d42a163063544d
created_at: 2026-11-07T00:47:12.620191620Z
event:
  event: finding
  recommendation: defer
---

low: Cargo.toml:11 astromech-shell now inherits rust-version = 1.88, but its locked tauri 2.12.3 declares rust-version 1.90 (toolchain is 1.99). With resolver 3's MSRV-aware resolution, the next cargo update or lock regeneration silently picks older tauri/wry, which is why the window's crates had to be pinned back by hand. Fix: raise [workspace.package] rust-version to at least 1.90 (or the pinned toolchain), then the pins are no longer load-bearing.
