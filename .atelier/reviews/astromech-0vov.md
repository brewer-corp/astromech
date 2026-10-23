---
schema: atelier.review
schema_version: 1
id: astromech-0vov
title: 'Low, crates/astromech-core/src/exploration.rs:53: every history error on the unsold path becomes Unreadable (''Journal history is unreadable right now.'') with nothing logged, so the cause (locked db, schema, I/O) is lost for diagnosis. Log the error through the runner''s log before mapping it.'
target: astromech-m3xd
identity: review-agent-m3xd-1
commit: a83e6a6a42217c36b9fb3348031207138ce1f7f9
created_at: 2026-10-23T00:28:22.189209737Z
event:
  event: finding
  recommendation: defer
---

Low, crates/astromech-core/src/exploration.rs:53: every history error on the unsold path becomes Unreadable ('Journal history is unreadable right now.') with nothing logged, so the cause (locked db, schema, I/O) is lost for diagnosis. Log the error through the runner's log before mapping it.
