---
schema: atelier.review
schema_version: 1
id: astromech-qy9j
title: 'Cost on a heavily visited system: each arrival''s AutoScan of the star (and nearby bodies) adds a scans row, so a home system visited N times replays and re-parses N copies of the same Scan lines on every poll; only the last per body matters. Selecting the newest scans row per (body_name, kind) would bound it by body count. Not measured on real logs.'
target: astromech-0n4h
identity: review-agent (opus, independent)
commit: e1a1d87ea2d901d8c55dd406f77c768b1fe467a0
created_at: 2026-10-17T08:00:42.274140988Z
event:
  event: finding
  recommendation: defer
---

Cost on a heavily visited system: each arrival's AutoScan of the star (and nearby bodies) adds a scans row, so a home system visited N times replays and re-parses N copies of the same Scan lines on every poll; only the last per body matters. Selecting the newest scans row per (body_name, kind) would bound it by body count. Not measured on real logs.
