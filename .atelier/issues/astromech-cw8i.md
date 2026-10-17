---
created_at: "2026-10-16T21:52:34.817082482+00:00"
id: "astromech-cw8i"
issue_type: "decision"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-17T16:18:34.521258214+00:00"
status: "done"
title: "What does \"Crystalline Structures\" mean for the material lookups?"
updated_at: "2026-10-17T16:18:34.521258214+00:00"
---

## Context

Picked crystalline shards as the most likely reading so the work was not blocked (Sumeet named "crystalline shards" in the parent Goal). Confirm, or name the other POI.

## Question

The parent feature (astromech-sfys) asks for the nearest "Crystalline Structures". Which point of interest is meant?

## Options

- Crystalline shard sites (the surface POI with grade 4 raw shards; Spansh codex subtype "Crystalline Shards"). Built in astromech-osjr: the "Crystalline shards" kind on the Materials page and the voice query "nearest crystalline shards"; "nearest crystalline structures" is a phrase for the same lookup.
- Something else: Odyssey exobiology "crystal" organics (Ice / Silicate / Metallic Crystals, Mineral Spheres), or Lagrange clouds with crystalline fragments. Any of these is another Spansh `landmark_subtype` kind in `astromech_galaxy::sites::SITE_KINDS`, a small change.

## Decision

Crystalline shard sites. Sumeet confirmed on 17 Oct 2026 that "Crystalline Structures" just means crystalline shards, which is what astromech-osjr built. No code change is needed.

## Close Reason

Crystalline structures are crystalline shards (Sumeet, 17 Oct); already built in astromech-osjr
