---
created_at: "2026-11-05T06:19:46.014214722+00:00"
id: "astromech-ibmy"
issue_type: "task"
labels:
- "size-l"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-xlbz"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Local station, ring and hotspot file built in CI from the Spansh galaxy dump"
updated_at: "2026-11-05T06:19:46.014214722+00:00"
---

## Goal

Build a local file of systems, stations (type, pads, arrival distance, services, trader kind), rings with hotspots, and permit flags from Spansh's `galaxy_stations.json.gz` in CI, like the star map preload and the neutron star file, so the engine's station and ring legs stop costing live requests (spike astromech-yn6y: about 1 live request per recipe once stations are local).

## Done when

Provisional: a manual workflow builds and publishes the file on a pre-release with its SHA-256, a script test checks the builder against a fixture, the app downloads it on request, the engine answers station and ring legs from it, and its real size from a full build is recorded (spike estimate about 90 MB SQLite, 30 MB gzipped, plus or minus 2x).

## Unknowns

- Whether it is needed: depends on the live request volume the engine ends up with and on the Spansh maintainer's answer (astromech-5rm2).
- Material trader kind is not in the dumps and has to be derived from the station's economy; permit flags are not in station records.
