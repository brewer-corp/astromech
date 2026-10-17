---
schema: atelier.review
schema_version: 1
id: astromech-unpa
title: 'R7 [low] crates/astromech-exploration/src/system.rs keeps a sampled species as Species_Localised (falling back to the symbol), and biology.rs resolves it with catalog::species(name), which matches English names only. On a non-English client the sampled species is not pinned and the genus falls back to its predicted candidates. The unsold total is unaffected (it uses symbols). Fix later: keep the Species symbol and resolve with species_by_symbol first, the localised name second.'
target: astromech-a63r
identity: 'review-agent (opus, independent, part A: data and prediction)'
commit: e5f94c22467a3514e2f7f88d5e0f7a58780e5631
created_at: 2026-10-17T05:57:53.694991049Z
event:
  event: finding
  recommendation: defer
---

R7 [low] crates/astromech-exploration/src/system.rs keeps a sampled species as Species_Localised (falling back to the symbol), and biology.rs resolves it with catalog::species(name), which matches English names only. On a non-English client the sampled species is not pinned and the genus falls back to its predicted candidates. The unsold total is unaffected (it uses symbols). Fix later: keep the Species symbol and resolve with species_by_symbol first, the localised name second.
