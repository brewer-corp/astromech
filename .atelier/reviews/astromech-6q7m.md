---
schema: atelier.review
schema_version: 1
id: astromech-6q7m
title: finding on astromech-kwoi @b59bfe06
target: astromech-kwoi
identity: reviewer-fable-kwoi
commit: b59bfe06cbc77259f52c02a6a4adae596a989658
created_at: 2026-11-05T15:10:07.016370297Z
event:
  event: finding
  recommendation: defer
---

low: app/src/lib/palette.ts:5 and app/src/lib/ui/Palette.svelte:4 comments still say a material 'opens its nearby search' (it now opens with a link to the Faction states finder; materials/palette.ts and ARCHITECTURE were updated). Also lib/places.ts:42 FINDERS sell-cargo input 'Hold' has no reader: FinderPage renders finder.input only with an input snippet, so it is a placeholder until astromech-2hbw's matcher reads FINDERS. Residue only; tidy with astromech-2hbw, which rewrites the palette.
