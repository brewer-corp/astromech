---
schema: atelier.review
schema_version: 1
id: astromech-o1kb
title: 'R2 [low] paths.rs:122: if hard_link succeeds but remove_file(tmp) fails (e.g. AV holding the handle on Windows), the hidden .device-overrides.json.carry-over.tmp stays forever because later calls return early once the target exists. Harmless (hidden, small, never read); note only.'
target: astromech-pb3o
identity: review-agent (opus, independent)
commit: 4298bfb2a7d7baf0bcab20f3ab9534e470434f1c
created_at: 2026-10-17T08:42:08.939621843Z
event:
  event: finding
  recommendation: no_action
---

R2 [low] paths.rs:122: if hard_link succeeds but remove_file(tmp) fails (e.g. AV holding the handle on Windows), the hidden .device-overrides.json.carry-over.tmp stays forever because later calls return early once the target exists. Harmless (hidden, small, never read); note only.
