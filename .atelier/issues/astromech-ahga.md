---
created_at: "2026-10-16T19:03:05.123603349+00:00"
id: "astromech-ahga"
issue_type: "task"
labels:
- "size-s"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-h74l"
    type: "related"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Add a circuit breaker to astromech-data"
updated_at: "2026-10-16T19:03:05.123603349+00:00"
---

## Goal

Stop hammering a service that is down: after repeated failures, astromech-data answers at once with a plain-words error for a while instead of sending more requests (ARCHITECTURE §5.8: 5 failures, 60 s open).

## Done when

Tests on a recorded transport show: after 5 consecutive connection or 5xx failures to one service, further requests to it fail immediately without reaching the transport for 60 s, other services are unaffected, a fresh cached answer is still served, and one success closes the breaker.

## Context

Split out of astromech-jsb9. `Client::send` in `crates/astromech-data/src/lib.rs` is the single place requests go out.
