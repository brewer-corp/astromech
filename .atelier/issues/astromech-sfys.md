---
created_at: "2026-10-15T20:36:13.646922536+00:00"
id: "astromech-sfys"
issue_type: "feature"
labels: []
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-e8ux"
  - kind: "issue"
    id: "astromech-huab"
  children:
  - kind: "issue"
    id: "astromech-361l"
  - kind: "issue"
    id: "astromech-bwmz"
  - kind: "issue"
    id: "astromech-fipx"
  - kind: "issue"
    id: "astromech-n0qb"
  - kind: "issue"
    id: "astromech-osjr"
  - kind: "issue"
    id: "astromech-qhi4"
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Engineering material lookups"
updated_at: "2026-10-16T18:12:15.175151598+00:00"
---

## Goal

The user can find the nearest raw-material sites, including crystalline shards, with a jump count, and sees how fresh the Boom/Famine/War state behind an HGE suggestion is. This replaces the Inara and wiki lookups for material farming.

## Done when

Lookup tests from fixed positions. The game check (astromech-fipx) is done. Independent review. Delivers S3.

## Scope

Already built: builds, shopping list, trader maths, engineer trips (#11–#14); HGE pools by faction and state, and Boom/Famine/War/Outbreak/Civil Unrest system lookups. The HGE signal callout belongs to the arrival callouts (astromech-77a1).

## Unknowns

Which point of interest the user means by "Crystalline Structures" (crystalline shard sites, or something else).
