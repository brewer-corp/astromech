---
created_at: "2026-10-15T20:21:44.862890103+00:00"
id: "astromech-0uao"
issue_type: "task"
labels:
- "size-m"
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-dt5y"
  - kind: "issue"
    id: "astromech-g9hw"
  - kind: "issue"
    id: "astromech-hnwd"
  - kind: "issue"
    id: "astromech-r1pt"
  - kind: "issue"
    id: "astromech-y3bw"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Build the astromech-data crate for Spansh, Ardent and EDSM"
updated_at: "2026-10-15T20:21:44.862890103+00:00"
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
