---
schema: atelier.review
schema_version: 1
id: astromech-5zxz
title: 'R2 [medium] crates/astromech-data/src/lib.rs Client::shared + cache.rs put: the remote cache is now a persistent file (remote_cache.sqlite) shared across runs, but Cache::put never deletes, so every keyed body (Ardent market answers up to 1000 rows each, Spansh/EDSM answers) accumulates on disk forever; before this branch the cache was in memory and died with the process. Pruning expired rows on put is already a done-when of astromech-zs2q; take zs2q next (or prune here) so the file stays bounded.'
target: astromech-proz
identity: review-agent (opus, independent)
commit: 72df8be9943607bf04534ac08f1161785b65f852
created_at: 2026-10-17T05:23:23.878537307Z
event:
  event: finding
  recommendation: defer
---

R2 [medium] crates/astromech-data/src/lib.rs Client::shared + cache.rs put: the remote cache is now a persistent file (remote_cache.sqlite) shared across runs, but Cache::put never deletes, so every keyed body (Ardent market answers up to 1000 rows each, Spansh/EDSM answers) accumulates on disk forever; before this branch the cache was in memory and died with the process. Pruning expired rows on put is already a done-when of astromech-zs2q; take zs2q next (or prune here) so the file stays bounded.
