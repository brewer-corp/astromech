---
schema: atelier.review
schema_version: 2
id: astromech-38fs
title: finding on astromech-kjfs @5d19a292
target: astromech-kjfs
identity: claude-reviewer-kjfs
commit: 5d19a29220e8c49fd861ed2ad6541ef56f24eeb8
created_at: 2026-11-07T04:54:25.944451394Z
event:
  event: finding
  recommendation: fix_now
---

low: astromech-xil2 Context Two Svelte components import previewQuery directly (app/src/modules/keybinds/KeybindsView.svelte:15, app/src/modules/controller/View.svelte:20), and fakecore.contract.test.ts reads app/src/lib/core-reply-shapes.json, which a Rust test in crates/astromech-core/src/app.rs:1995 writes by path; the issue says the Rust side doesn't change. Fix: list the two components in Context and say core-reply-shapes.json stays where it is (or that the Rust test path changes with it).
