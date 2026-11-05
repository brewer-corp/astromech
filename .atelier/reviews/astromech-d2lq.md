---
schema: atelier.review
schema_version: 1
id: astromech-d2lq
title: finding on astromech-51ue @0b6806b9
target: astromech-51ue
identity: reviewer-sonnet-f7i0
commit: 0b6806b99ee217651e9f848144dc2f608e9b5db2
created_at: 2026-11-05T09:13:09.408243065Z
event:
  event: finding
  recommendation: fix_now
---

low: astromech-f7i0 (code of nxz9) app/src/modules/places/{Services,Outfitting,Shipyard,Traders}.svelte:19-23 the author's report is verified. here() reads finder.id from the prop; after opening a finder from a Places home tile, leaving by the Places crumb makes View's hashchange listener (registered first) set finder to null before the page's own listener runs, so every one of the four throws 'TypeError: Cannot read properties of null (reading id)' on leaving. Reproduced with a temporary vitest probe (tile in, crumb out: Services, Outfitting, Shipyard, Material traders each 1 uncaught error, Mining hotspots 0; probe deleted). Opening a finder by URL first does not reproduce, which is why the existing tests pass. Page still changes (error is isolated to the listener) and there is no global error handler, so impact is a console TypeError on the main navigation path, but it is a one-line fix in code the branch owns and Hotspots already carries the fixed pattern. Fix in this branch: read the id from the constant (parsePlaces(location.hash)?.finder?.id === 'services'), ideally one shared helper in lib/places.ts used by all five pages, with a test that enters from the home tile and leaves via the crumb and expects no error.
