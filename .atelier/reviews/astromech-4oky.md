---
schema: atelier.review
schema_version: 1
id: astromech-4oky
title: 'R5 low crates/astromech-powerplay/src/lib.rs:146-150: a newly listed journal that is not the newest and fails to open (JournalTail::read_new returns empty on error) is still pushed to ''read'' and never re-read until the file list changes or the app restarts. Rust opens with full share mode, so a transient failure is unlikely; noted for completeness.'
target: astromech-fdh3
identity: review-agent-fdh3-1
commit: 63470471264d143f5c0e854f4641ee757634d5a8
created_at: 2026-10-22T21:45:44.715447989Z
event:
  event: finding
  recommendation: no_action
---

R5 low crates/astromech-powerplay/src/lib.rs:146-150: a newly listed journal that is not the newest and fails to open (JournalTail::read_new returns empty on error) is still pushed to 'read' and never re-read until the file list changes or the app restarts. Rust opens with full share mode, so a transient failure is unlikely; noted for completeness.
