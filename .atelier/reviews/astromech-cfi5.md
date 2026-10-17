---
schema: atelier.review
schema_version: 1
id: astromech-cfi5
title: 'Test strength: mutations tried and reverted. Removing the SEEN dedupe fails the new test (2 messages); sending the message to eprintln instead of the callback fails it (0 messages). sample() passes an eprintln callback; it is only used by tests, not the shell, so it doesn''t reintroduce a global sink.'
target: astromech-arra
identity: review-agent (opus, independent)
commit: 69b89465f6cf049ee883b88cf8c38f81c05bc9aa
created_at: 2026-10-17T07:46:32.531284873Z
event:
  event: finding
  recommendation: no_action
---

Test strength: mutations tried and reverted. Removing the SEEN dedupe fails the new test (2 messages); sending the message to eprintln instead of the callback fails it (0 messages). sample() passes an eprintln callback; it is only used by tests, not the shell, so it doesn't reintroduce a global sink.
