---
created_at: "2026-10-15T20:36:13.651680346+00:00"
id: "astromech-h74l"
issue_type: "task"
labels:
- "size-m"
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-20dy"
  - kind: "issue"
    id: "astromech-4c6q"
  - kind: "issue"
    id: "astromech-7ix7"
  - kind: "issue"
    id: "astromech-a63r"
  - kind: "issue"
    id: "astromech-h89f"
  - kind: "issue"
    id: "astromech-mljg"
  - kind: "issue"
    id: "astromech-n0qb"
  - kind: "issue"
    id: "astromech-pxy2"
  - kind: "issue"
    id: "astromech-tw2y"
  - kind: "issue"
    id: "astromech-xqdl"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-16T18:56:24.987431487+00:00"
status: "done"
title: "Build the astromech-data crate for Spansh, Ardent and EDSM"
updated_at: "2026-10-16T18:56:24.987431487+00:00"
---

## Goal

Every lookup needs remote data. Wrap Spansh (systems, stations and bodies search; route jobs), Ardent Insight (commodity and nearest service) and EDSM (system, bodies, traffic) in one crate with a shared User-Agent, a TTL cache in SQLite, schema validation, rate limiting and offline errors in plain words.

## Done when

Tests on recorded responses show: a cached answer is served without a request until its TTL ends; a response that fails the schema check is an error naming the service, not a panic or a wrong answer; no network gives a plain-words error.

## Context

- Seed: `crates/astromech-galaxy/src/remote.rs` (Spansh systems search for faction states,
  EDSM sphere).
- **Spansh**: rich filters (stations by module, ship, commodity or service; systems by state,
  Powerplay or population) and the route plotters. Undocumented JSON APIs (REPORT §4): check
  the request shape in the site's network traffic, and validate every response.
- **Ardent Insight**: documented REST API.
  `/v2/system/name/{s}/commodity/name/{c}/nearby/imports|exports`, `/nearest/{service}`, and
  per-system markets. Filters: `maxDistance` ≤ 500 ly, `maxDaysAgo`, `minLandingPadSize`,
  `fleetCarriers`, `minVolume`.
- **EDSM**: star positions (already used for map fill), bodies, traffic, flight log.

## Close Reason

astromech-data crate built: Spansh search + neutron route jobs, Ardent nearby/nearest/markets, EDSM system/bodies/traffic/sphere behind one client with User-Agent, SQLite TTL cache, typed schema checks, per-service rate limit and plain-words errors. Tests on recorded responses cover the three Done-when clauses (cargo test -p astromech-data, evidence recorded). Galaxy's Spansh/EDSM calls go through it. Remaining endpoints in astromech-jsb9. Independent review outstanding.
