---
schema: atelier.review
schema_version: 1
id: astromech-lcr4
title: 'Author''s claim that a real old-layout file is already in WAL holds: every Cache::open since the crate was added (a2be550) ran PRAGMA journal_mode=WAL before creating the table, and the expires-less layout was file-backed in production only between 8f1e901 and f02beb5 (both 17 Oct, after the v0.1.5 tag b662341, which has no astromech-data crate). So the ''v0.1.5'' label means dev builds at workspace version 0.1.5, not the v0.1.5 release; the comment/doc wording predates this branch. Noted for accuracy only.'
target: astromech-kktx
identity: review-agent (opus, independent)
commit: f16bc18e58ad90dc15652114d1603b029e6a8423
created_at: 2026-10-17T06:23:32.116936545Z
event:
  event: finding
  recommendation: no_action
---

Author's claim that a real old-layout file is already in WAL holds: every Cache::open since the crate was added (a2be550) ran PRAGMA journal_mode=WAL before creating the table, and the expires-less layout was file-backed in production only between 8f1e901 and f02beb5 (both 17 Oct, after the v0.1.5 tag b662341, which has no astromech-data crate). So the 'v0.1.5' label means dev builds at workspace version 0.1.5, not the v0.1.5 release; the comment/doc wording predates this branch. Noted for accuracy only.
