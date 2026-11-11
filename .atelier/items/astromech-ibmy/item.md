---
schema: atelier.item
schema_version: 3
id: astromech-ibmy
title: Local station, ring and hotspot file built in CI from the Spansh galaxy dump
status: draft
mission: astromech-7q0z
labels:
- size-l
priority: medium
blocked_by:
- astromech-xlbz
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-05T06:19:46.014214722Z
---

## Goal

Build a local file of systems, stations (type, pads, arrival distance, services, trader kind), rings with hotspots, and permit flags from Spansh's `galaxy_stations.json.gz` in CI, like the star map preload and the neutron star file, so the engine's station and ring legs stop costing live requests (spike astromech-yn6y: about 1 live request per recipe once stations are local).

## Done when

- [ ] Provisional: a manual workflow builds and publishes the file on a pre-release with its SHA-256, a script test checks the builder against a fixture, the app downloads it on request, the engine answers station and ring legs from it, and its real size from a full build is recorded (spike estimate about 90 MB SQLite, 30 MB gzipped, plus or minus 2x).

## Unknowns

- Whether it is needed: depends on the live request volume the engine ends up with and on the Spansh maintainer's answer (astromech-5rm2). Exit: supersede this item when that answer or the request volume measured on astromech-xlbz and astromech-sgwi says live legs are fine.
- It blocks nothing today: the recipes ship within the spike's budget without it (decision astromech-g8d5, item 9), and astromech-os70's validation does not wait on it. If astromech-wla4's corridor design turns out to need local stations, link astromech-wla4 blocked_by this item then.
- Material trader kind is not in the dumps and has to be derived from the station's economy; permit flags are not in station records.

## Links

Relationships of the old record that the new format has no field for.

- parent: astromech-os70 (Linked places: engine, recipes, Find near this, Find along route)
