---
schema: atelier.review
schema_version: 2
id: astromech-43sv
title: finding on astromech-y6d6 @107e5c5d
target: astromech-y6d6
identity: claude-reviewer-v3u4
commit: 107e5c5d56fe9802323304beffa0ca83e69e3698
created_at: 2026-11-07T19:15:21.177925040Z
event:
  event: finding
  recommendation: defer
---

low: crates/astromech-core/src/pipeline.rs:758-763, 816-833 (107e5c5d). status_seen is set only when read_status_retry returns Some, and parse_status returns None for a whole, valid Status.json with no Flags (astromech-status status_file.rs:64-68: 'e.g. at the main menu'). So while Elite sits at the main menu, every idle pass (4 Hz) re-reads Status.json five times with 1 ms sleeps (~4 ms of the pipeline thread) and then on_companions re-reads and parses Cargo.json and NavRoute.json, indefinitely. Correctness is fine (StateHub::update ignores unchanged state, the reaction engine ignores an unchanged status and route), and the cost is small, but it is a permanent poll the commit message doesn't intend ('when it changed'). Fix: record the stamp after any complete read (valid JSON, Flags or not), retrying only on a parse or IO failure; or cap re-reads of the same stamp.
