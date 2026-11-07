---
schema: atelier.review
schema_version: 2
id: astromech-emsi
title: finding on astromech-bp23 @8197eae3
target: astromech-bp23
identity: claude-reviewer-8s6k
commit: 8197eae3aaea51f8468babfeca01b516675004b2
created_at: 2026-11-07T06:52:30.847942029Z
event:
  event: finding
  recommendation: defer
---

medium: crates/astromech-core/src/app.rs:714-980 (Core::commands) The core's own handlers still lock shared state with .lock().unwrap() (49 sites in app.rs: session, crash, ui, game...). catch_unwind now turns a panic into an error reply, but if it happens under c.session's lock every later status/voice_test/exobiology call becomes 'failed with an internal error' until restart, and the session threads that unwrap the same mutex panic too. astromech-api's lib.rs doc ('the next call runs as normal') holds only for state locked via astromech_api::lock. Not a regression (before, the connection dropped). Fix in a later bp23 child or a filed issue: lock state that handlers share with astromech_api::lock, or narrow the lib.rs claim.
