---
schema: atelier.review
schema_version: 1
id: astromech-fs3u
title: finding on astromech-pey5 @5c36e669
target: astromech-pey5
identity: reviewer-fable-pey5
commit: 5c36e66901b0dc61bd103cf1c8b4951d00cecb0c
created_at: 2026-11-05T06:45:12.914749414Z
event:
  event: finding
  recommendation: fix_now
---

low: app/src/lib/ui/Glyph.svelte:49 the new 'clear' glyph (the x that empties the header box and From) is not in DESIGN.md's icon list (DESIGN.md:278-283 names add stop, set destination, open external, chevron), so the set and its binding doc disagree. Decision 2: keep the glyph (an IconButton gives the 28 px hit area, tooltip and label a text x would not) and add 'clear (a cross; empties a box)' to the Icons paragraph and the Icon Or Word examples. While in that file drop pey5 from the pending list in the header comment (DESIGN.md:256), which is stale once this merges.
