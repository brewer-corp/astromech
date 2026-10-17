---
schema: atelier.review
schema_version: 1
id: astromech-h4v0
title: 'medium crates/astromech-eddn/src/outbox.rs:~268-290 (deliver_due): the pass snapshots the due rows (body included) into memory, then re-checks only the sharing flag before each post. If the user switches off (core purges the queue) and back on while a pass is in flight (e.g. several due messages, gateway slow up to the post timeout), stop() reads true again and the worker posts the purged, snapshotted messages; the ''UPDATE outbox SET attempts'' for the deleted row is a silent no-op and remove() likewise, and a ''sent'' row is logged. This defeats R3 for that window. Fix: check rows-affected of the attempts UPDATE and skip the message if 0 (row was purged); add a test with a transport that purges mid-pass.'
target: astromech-6xyo
identity: review-agent (6xyo review, sonnet)
commit: 65f81c752d6d2f419643e6b4a75786507ea51c6b
created_at: 2026-10-17T00:07:27.550761454Z
event:
  event: finding
  recommendation: fix_now
---

medium crates/astromech-eddn/src/outbox.rs:~268-290 (deliver_due): the pass snapshots the due rows (body included) into memory, then re-checks only the sharing flag before each post. If the user switches off (core purges the queue) and back on while a pass is in flight (e.g. several due messages, gateway slow up to the post timeout), stop() reads true again and the worker posts the purged, snapshotted messages; the 'UPDATE outbox SET attempts' for the deleted row is a silent no-op and remove() likewise, and a 'sent' row is logged. This defeats R3 for that window. Fix: check rows-affected of the attempts UPDATE and skip the message if 0 (row was purged); add a test with a transport that purges mid-pass.
