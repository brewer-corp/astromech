---
schema: atelier.item
schema_version: 3
id: astromech-jsb9
title: Wrap the remaining remote endpoints in astromech-data
status: draft
mission: null
labels:
- size-s
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-16T18:56:18.009741511Z
---

## Goal

astromech-data (astromech-h74l) covers the searches, Ardent commodity/nearest/markets, EDSM system/bodies/traffic/sphere and the Spansh neutron route job. Add the rest when a lookup needs them, so no feature talks to a service directly.

## Done when

- [ ] Each added endpoint has typed results and a recorded-response test in `crates/astromech-data/tests/recorded.rs`, and its request shape is checked against the live site with the date noted.

## Context

- Spansh: system-name autocomplete (`/api/systems/field_values/system_names?q=`), `/api/nearest`, documented `/api/system/{id64}` and `/api/station/{marketId}`; the other plotters (road to riches, exobiology, trade, fleet carrier) belong to astromech-94j4.
- Spansh systems filters for state, Powerplay and population are reachable through `Search::filter` but their shapes are unverified; add typed helpers once a lookup checks them.
- EDSM flight log needs a per-commander API key (settings and keyring).

## Links

Relationships of the old record that the new format has no field for.

- related: astromech-h74l (Build the astromech-data crate for Spansh, Ardent and EDSM)
