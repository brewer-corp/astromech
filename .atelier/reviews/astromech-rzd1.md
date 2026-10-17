---
schema: atelier.review
schema_version: 1
id: astromech-rzd1
title: 'R2 [low] 56 of the 338 conditions in data/catalog/exobiology.json come from one sighting, and some are implausible data noise: Bacterium Aurasus has seen=1 rules for Ammonia/Rocky, ArgonRich/Icy and Methane/Icy beside CarbonDioxide rules seen 3365 and 689 times. They add false candidates. On the held-out fixture, requiring seen>=2 keeps recall at 1139/1148 = 0.992 and lowers candidates per body from 11.6 to 11.2. Fix later: a minimum of 2 sightings per (atmosphere, body class) group in derive(), or drop groups below a small share of the species'' sightings. Also trivial NOTICE.md nits: a missing full stop after ''(24,756 bodies)'', and ''Exobiology values'' says the main sources are ''below'' when they are above.'
target: astromech-znhx
identity: review-agent (opus, independent)
commit: 2715d6c82f423cb1539c5efbafa06a86644264b8
created_at: 2026-10-17T07:27:07.990817312Z
event:
  event: finding
  recommendation: defer
---

R2 [low] 56 of the 338 conditions in data/catalog/exobiology.json come from one sighting, and some are implausible data noise: Bacterium Aurasus has seen=1 rules for Ammonia/Rocky, ArgonRich/Icy and Methane/Icy beside CarbonDioxide rules seen 3365 and 689 times. They add false candidates. On the held-out fixture, requiring seen>=2 keeps recall at 1139/1148 = 0.992 and lowers candidates per body from 11.6 to 11.2. Fix later: a minimum of 2 sightings per (atmosphere, body class) group in derive(), or drop groups below a small share of the species' sightings. Also trivial NOTICE.md nits: a missing full stop after '(24,756 bodies)', and 'Exobiology values' says the main sources are 'below' when they are above.
