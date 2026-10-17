---
schema: atelier.review
schema_version: 1
id: astromech-v12n
title: 'R1 [low] The star-class gate is learned from about a quarter of the sightings, and the recall test barely exercises it. Spansh''s bodies search returns only a body''s immediate parent (checked live: parents=[{type: Null}] for 36 Ophiuchi A 3), so scripts/gen-exobiology.py nearest_star() is None for every moon and every body around a barycentre: 292 of the 400 held-out fixture bodies have no star, and those skip the star gate. In the game, predict.rs always knows the nearest star from the journal Parents chain, so moons are gated by star lists learned from planets around stars. Measured on the fixture with the script''s fits(): bodies with a known star 284/288 = 0.986 recall, 7.7 candidates per body; unknown star 0.994, 13.0. So the gate works where it can be checked, but the stated 99.2% is mostly with it unchecked. Fix later: resolve the nearest star from the system''s bodies (Spansh system dump or dump endpoint) during derivation, and report recall on star-known bodies too.'
target: astromech-znhx
identity: review-agent (opus, independent)
commit: 2715d6c82f423cb1539c5efbafa06a86644264b8
created_at: 2026-10-17T07:27:07.921505010Z
event:
  event: finding
  recommendation: defer
---

R1 [low] The star-class gate is learned from about a quarter of the sightings, and the recall test barely exercises it. Spansh's bodies search returns only a body's immediate parent (checked live: parents=[{type: Null}] for 36 Ophiuchi A 3), so scripts/gen-exobiology.py nearest_star() is None for every moon and every body around a barycentre: 292 of the 400 held-out fixture bodies have no star, and those skip the star gate. In the game, predict.rs always knows the nearest star from the journal Parents chain, so moons are gated by star lists learned from planets around stars. Measured on the fixture with the script's fits(): bodies with a known star 284/288 = 0.986 recall, 7.7 candidates per body; unknown star 0.994, 13.0. So the gate works where it can be checked, but the stated 99.2% is mostly with it unchecked. Fix later: resolve the nearest star from the system's bodies (Spansh system dump or dump endpoint) during derivation, and report recall on star-known bodies too.
