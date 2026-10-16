---
schema: atelier.review
schema_version: 1
id: astromech-7jv7
title: '[medium] crates/astromech-core/src/actions.rs:58 and :102 (Runner::nearest_site via Plan::Lookup): the voice site lookup runs Galaxy::sites synchronously, a blocking Spansh bodies search (30 s timeout, 10 s connect) plus a possible EDSM sphere fill (ensure_covered, the same timeouts), on the ''stt'' thread (crates/astromech-core/src/session.rs:365), with no spoken acknowledgement. Failure scenario: the commander says ''nearest crystalline shards'' while Spansh is slow or the connection drops, then says ''landing gear down'' on approach. The gear command waits behind the lookup for up to about a minute and then runs late, and nothing is said in the meantime. Fix: run the lookup on its own thread (or a worker) and speak the answer when it arrives, so the stt thread goes straight back to the queue. Optionally say ''Looking that up'' first. Add a test with a slow Transport that checks a later command is handled while the lookup is still running.'
target: astromech-sfys
identity: review-agent (sfys review)
commit: 8c44864ffb8b65cc78872de4fd3c564004816b0a
created_at: 2026-10-16T22:31:57.886931807Z
event:
  event: finding
  recommendation: fix_now
---

[medium] crates/astromech-core/src/actions.rs:58 and :102 (Runner::nearest_site via Plan::Lookup): the voice site lookup runs Galaxy::sites synchronously, a blocking Spansh bodies search (30 s timeout, 10 s connect) plus a possible EDSM sphere fill (ensure_covered, the same timeouts), on the 'stt' thread (crates/astromech-core/src/session.rs:365), with no spoken acknowledgement. Failure scenario: the commander says 'nearest crystalline shards' while Spansh is slow or the connection drops, then says 'landing gear down' on approach. The gear command waits behind the lookup for up to about a minute and then runs late, and nothing is said in the meantime. Fix: run the lookup on its own thread (or a worker) and speak the answer when it arrives, so the stt thread goes straight back to the queue. Optionally say 'Looking that up' first. Add a test with a slow Transport that checks a later command is handled while the lookup is still running.
