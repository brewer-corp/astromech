---
schema: atelier.review
schema_version: 1
id: astromech-tc9s
title: finding on astromech-kwoi @b59bfe06
target: astromech-kwoi
identity: reviewer-fable-kwoi
commit: b59bfe06cbc77259f52c02a6a4adae596a989658
created_at: 2026-11-05T15:10:06.261973262Z
event:
  event: finding
  recommendation: defer
---

low: app/src/modules/engineering/Build.svelte:425,451 the build plan's side panel still embeds <Nearby> (compact rows, no facts), the last page that lists faction-state systems itself; DESIGN.md Lookup result row says side-panel lookups 'become links into their finder later (decision astromech-g8d5, item 5)' and docs/guide/places.md now names it under Narrow panels. Outside kwoi's Done when (Trade, Materials, Exploration only), so not this branch: file a follow-up under astromech-51ue or astromech-os70 to replace it with a 'Find the nearest systems' link like Materials, then delete lib/ui/Nearby.svelte.
