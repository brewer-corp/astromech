---
created_at: "2026-10-23T00:08:31.259829293+00:00"
id: "astromech-b8js"
issue_type: "task"
labels:
- "size-s"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Outfitting to EDDN: settle Arx-ship armour and unknown-sku modules with EDCD"
updated_at: "2026-10-23T00:08:31.259829293+00:00"
---

## Goal

astromech's outfitting/2 messages include every module a station sells to anyone, no more and no less, in the cases where FDevIDs' sku data is ambiguous.

## Done when

EDCD (EDDN maintainers) has answered whether armour for ships first sold through the Arx store (Cobra Mk V, Type-8, Mandalay, Corsair, Python Mk II, Kestrel, Panther Mk II, Type-11, Caspian, Lynx Highliner) and modules with an unknown sku ('?' in FDevIDs: Caustic Sink Launcher, Thargoid Pulse Neutraliser) count as commander-specific; data/catalog/eddn-outfitting.json follows the answer; the test labels the '?' modules as unknown sku, not tech broker; docs/guide/data-sharing.md says the list leaves out modules whose sku is unknown.

## Context

Review findings R1 (astromech-akqt) and R2 (astromech-32yq) on task/astromech-dlrb. Today they are dropped (the cautious choice): 42 armour items at Jameson Memorial, and the Thargoid Pulse Neutraliser.
