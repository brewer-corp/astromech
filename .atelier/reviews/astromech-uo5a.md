---
schema: atelier.review
schema_version: 1
id: astromech-uo5a
title: 'Pre-existing, not worsened: two processes opening a brand-new remote_cache.sqlite at once still leaves one on the in-memory cache. Cache::open''s PRAGMA journal_mode=WAL on a rollback-journal file returns SQLITE_BUSY (''database is locked'') immediately under contention despite the 5 s busy timeout (busy handler is not used for that lock upgrade). Reproduced by removing the WAL pre-step from two_openers_of_a_v015_file_both_get_the_file_backed_cache: fails 4/4 at the WAL stage, both with no file and with a non-WAL old file. Real path: first launch, if the core and the app window start together. Suggest a follow-up issue: retry the WAL switch (or run it under BEGIN IMMEDIATE-free retry loop) and extend the two-opener test to a fresh file.'
target: astromech-kktx
identity: review-agent (opus, independent)
commit: f16bc18e58ad90dc15652114d1603b029e6a8423
created_at: 2026-10-17T06:23:30.796869244Z
event:
  event: finding
  recommendation: defer
---

Pre-existing, not worsened: two processes opening a brand-new remote_cache.sqlite at once still leaves one on the in-memory cache. Cache::open's PRAGMA journal_mode=WAL on a rollback-journal file returns SQLITE_BUSY ('database is locked') immediately under contention despite the 5 s busy timeout (busy handler is not used for that lock upgrade). Reproduced by removing the WAL pre-step from two_openers_of_a_v015_file_both_get_the_file_backed_cache: fails 4/4 at the WAL stage, both with no file and with a non-WAL old file. Real path: first launch, if the core and the app window start together. Suggest a follow-up issue: retry the WAL switch (or run it under BEGIN IMMEDIATE-free retry loop) and extend the two-opener test to a fresh file.
