---
schema: atelier.item
schema_version: 3
id: astromech-ahga
title: Add a circuit breaker to astromech-data
status: draft
mission: null
labels:
- size-s
priority: low
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-16T19:03:05.123603349Z
---

## Goal

Stop hammering a service that is down: after repeated failures, astromech-data answers at once with a plain-words error for a while instead of sending more requests (ARCHITECTURE §5.8: 5 failures, 60 s open).

## Done when

- [ ] Tests on a recorded transport show: after 5 consecutive connection or 5xx failures to one service, further requests to it fail immediately without reaching the transport for 60 s, other services are unaffected, a fresh cached answer is still served, and one success closes the breaker.

## Context

Split out of astromech-jsb9. `Client::send` in `crates/astromech-data/src/lib.rs` is the single place requests go out.

## Links

Relationships of the old record that the new format has no field for.

- related: astromech-h74l (Build the astromech-data crate for Spansh, Ardent and EDSM)
