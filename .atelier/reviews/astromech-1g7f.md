---
schema: atelier.review
schema_version: 1
id: astromech-1g7f
title: 'R3 [low] Fixed temp name .{name}.carry-over.tmp could be clobbered by a concurrent carry_over, but the only caller (make_api in the window) runs under the Binds mutex and the app is single-instance (tauri_plugin_single_instance), so no current path races. Checked: user file never overwritten (link fails on existing target, AlreadyExists -> Ok(false)), source never written or deleted, test fails if the target is truncated or the temp left behind.'
target: astromech-pb3o
identity: review-agent (opus, independent)
commit: 4298bfb2a7d7baf0bcab20f3ab9534e470434f1c
created_at: 2026-10-17T08:42:09.012757837Z
event:
  event: finding
  recommendation: no_action
---

R3 [low] Fixed temp name .{name}.carry-over.tmp could be clobbered by a concurrent carry_over, but the only caller (make_api in the window) runs under the Binds mutex and the app is single-instance (tauri_plugin_single_instance), so no current path races. Checked: user file never overwritten (link fails on existing target, AlreadyExists -> Ok(false)), source never written or deleted, test fails if the target is truncated or the temp left behind.
