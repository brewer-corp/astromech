---
schema: atelier.review
schema_version: 1
id: astromech-i1bp
title: 'Reviewed git diff main...HEAD (cache.rs init now under TransactionBehavior::Immediate, layout read inside the transaction, DROP + SCHEMA + commit; ARCHITECTURE.md line). busy_timeout is set before the WAL switch and before BEGIN IMMEDIATE, so the second opener waits for the first''s commit and then re-reads the already-new layout; in WAL the read after taking the write lock sees the committed schema. in_memory() path unaffected. cargo test -p astromech-data: 6+18 pass. Mutations: main''s init with the new test fails 6/7 runs with ''no such table: remote_cache'' (matches evidence astromech-veqs); a Deferred transaction instead of Immediate fails 3/3. WAL pre-step in the test is justified by git history. Residual: a fresh (or non-WAL) file opened by both processes at once still fails at the WAL switch, pre-existing and recorded as DEFER.'
target: astromech-kktx
identity: review-agent (opus, independent)
commit: f16bc18e58ad90dc15652114d1603b029e6a8423
created_at: 2026-10-17T06:23:32.511372623Z
event:
  event: approval
---

Reviewed git diff main...HEAD (cache.rs init now under TransactionBehavior::Immediate, layout read inside the transaction, DROP + SCHEMA + commit; ARCHITECTURE.md line). busy_timeout is set before the WAL switch and before BEGIN IMMEDIATE, so the second opener waits for the first's commit and then re-reads the already-new layout; in WAL the read after taking the write lock sees the committed schema. in_memory() path unaffected. cargo test -p astromech-data: 6+18 pass. Mutations: main's init with the new test fails 6/7 runs with 'no such table: remote_cache' (matches evidence astromech-veqs); a Deferred transaction instead of Immediate fails 3/3. WAL pre-step in the test is justified by git history. Residual: a fresh (or non-WAL) file opened by both processes at once still fails at the WAL switch, pre-existing and recorded as DEFER.
