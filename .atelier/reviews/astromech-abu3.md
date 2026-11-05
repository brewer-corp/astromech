---
schema: atelier.review
schema_version: 1
id: astromech-abu3
title: finding on astromech-kwoi @b59bfe06
target: astromech-kwoi
identity: reviewer-fable-kwoi
commit: b59bfe06cbc77259f52c02a6a4adae596a989658
created_at: 2026-11-05T15:10:05.778694624Z
event:
  event: finding
  recommendation: defer
---

low: app/src/lib/galaxy.ts:147 FACTION_STATES hand-copies astromech_galaxy::remote::QUERIES (ids, and new labels/about lines) with no drift test; the answer already carries r.query.label. A query id added in Rust (and to a material's source query) is unreachable from the finder, and Materials' 'Find the nearest systems' link to #/places/faction-states/<new id> opens on Boom silently (parseFactionRoute null -> DEFAULT). Same shape as astromech-otpp (ORES): defer to that follow-up, e.g. a test comparing FACTION_STATES ids with a recorded list of QUERIES ids, or a shell command that returns QUERIES.
