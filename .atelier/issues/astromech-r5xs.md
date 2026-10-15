---
created_at: "2026-10-15T20:30:24.596365958+00:00"
id: "astromech-r5xs"
issue_type: "epic"
labels: []
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-egtp"
  - kind: "issue"
    id: "astromech-rxv2"
  children:
  - kind: "issue"
    id: "astromech-0tuy"
  - kind: "issue"
    id: "astromech-ggct"
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-hw3c"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Engineering material lookups"
updated_at: "2026-10-15T20:30:24.659137763+00:00"
---

## Scope

Already built: builds, shopping list, trader maths, engineer trips (#11–#14); HGE pools by faction and state, and Boom/Famine/War/Outbreak/Civil Unrest system lookups. The HGE signal callout is in the arrival-callouts epic.

## Outcome

The user can find the nearest raw-material sites, including crystalline shards, with a jump count, and sees how fresh the Boom/Famine/War state behind an HGE suggestion is. This replaces the Inara and wiki lookups for material farming.

## Proof

Lookup tests from fixed positions and a check in the real game. Independent review.

## Delivers

S3

## Unknowns

Which point of interest the user means by "Crystalline Structures" (crystalline shard sites, or something else).
