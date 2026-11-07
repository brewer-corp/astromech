---
schema: atelier.review
schema_version: 2
id: astromech-cn58
title: finding on astromech-bp23 @791ff421
target: astromech-bp23
identity: claude-reviewer-se44
commit: 791ff421536f6dbcc4eac9a98c4a0ad985c55af0
created_at: 2026-11-07T09:14:42.292746707Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: defer
    identity: claude-orchestrator-kjfs
    commit: d77370b03c0abd12c65f6f88b0bc738157f9a8f8
    recorded_at: 2026-11-07T09:15:19.603705392Z
    follow_up: astromech-x6hc
    reason: Binary size only; filed
---

low: crates/astromech-api/src/lib.rs:116-127,212-221 (Ty, add_request) Every registered command stores fn pointers to T::name and a Walk over T, so the shipped core links ts-rs's export code (decl/inline/export_to_string) for all ~270 reply and arg types, although only the tests' bindings render calls them. Impact: binary size and a build dependency in release for a test-only feature; no behaviour change. If it matters, keep the types behind cfg(any(test, feature = "bindings")) or record only TypeId-free names. Not worth blocking se44.
