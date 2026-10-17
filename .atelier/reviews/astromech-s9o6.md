---
schema: atelier.review
schema_version: 1
id: astromech-s9o6
title: 'Reviewed at 918807c (PR #96 head). Not approved: one FIX NOW (astromech-t8ud, committed .pyc). Verified: re-ran scripts/gen-exobiology.py at HEAD on /tmp/ys47-cache (system-stars.json included, no new fetches), EDR 198ebf5 and cached wiki text; catalogue and held-out fixture are byte-identical (source line aside) with held-out recall 17810/18047 = 0.987, 10.2 candidates per body, nearest star for 5607/6189 (91 %), 282 conditions, 0 under two sightings (MIN_SEEN applied in derive before any rule is built). Held-out split by sha256(id64) unchanged and excluded from by_species; star lists are features, not labels. The nearest-star rule matches the runtime''s first Star in Parents for moons, brown dwarfs and barycentres; only named-catalogue systems lose a star (astromech-0nz3, DEFER). New tests most_held_out_bodies_have_their_nearest_star and every_condition_has_at_least_two_sightings exercise the claims; CANDIDATES 11.0. NOTICE slips fixed. cargo test -p astromech-exobiology -p astromech-exploration -p astromech-announce -p astromech-core: all pass. gh pr checks 96 at review: Linux pass, Windows MSVC and Windows installer pending. Merge with h5gs: astromech-b83h.'
target: astromech-ys47
identity: review-agent (opus, independent)
commit: 918807c043016e7baad156f8bd7982eddeb732a5
created_at: 2026-10-17T08:13:39.319563298Z
event:
  event: review
---

Reviewed at 918807c (PR #96 head). Not approved: one FIX NOW (astromech-t8ud, committed .pyc). Verified: re-ran scripts/gen-exobiology.py at HEAD on /tmp/ys47-cache (system-stars.json included, no new fetches), EDR 198ebf5 and cached wiki text; catalogue and held-out fixture are byte-identical (source line aside) with held-out recall 17810/18047 = 0.987, 10.2 candidates per body, nearest star for 5607/6189 (91 %), 282 conditions, 0 under two sightings (MIN_SEEN applied in derive before any rule is built). Held-out split by sha256(id64) unchanged and excluded from by_species; star lists are features, not labels. The nearest-star rule matches the runtime's first Star in Parents for moons, brown dwarfs and barycentres; only named-catalogue systems lose a star (astromech-0nz3, DEFER). New tests most_held_out_bodies_have_their_nearest_star and every_condition_has_at_least_two_sightings exercise the claims; CANDIDATES 11.0. NOTICE slips fixed. cargo test -p astromech-exobiology -p astromech-exploration -p astromech-announce -p astromech-core: all pass. gh pr checks 96 at review: Linux pass, Windows MSVC and Windows installer pending. Merge with h5gs: astromech-b83h.
