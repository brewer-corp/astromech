---
schema: atelier.review
schema_version: 1
id: astromech-gpw4
title: 'R4 [low] Plot.svelte: the ship/range line is fetched once on mount, so it can be stale after a ship swap or cargo change (the plot itself recomputes). Recommended LATER: refetch on click or show plotted.range in the summary.'
target: astromech-94j4
identity: review-agent (sonnet, independent)
commit: 562d5f3a6dd08cda92d4aad45434cf0bbbf03b62
created_at: 2026-10-17T19:07:59.392082973Z
event:
  event: finding
  recommendation: defer
---

R4 [low] Plot.svelte: the ship/range line is fetched once on mount, so it can be stale after a ship swap or cargo change (the plot itself recomputes). Recommended LATER: refetch on click or show plotted.range in the summary.
