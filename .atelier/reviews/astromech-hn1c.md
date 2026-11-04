---
schema: atelier.review
schema_version: 1
id: astromech-hn1c
title: finding on astromech-1o77 @66eb8d47
target: astromech-1o77
identity: review-agent-1o77-1
commit: 66eb8d47ec68d2519ae308605f23070b68573c3f
created_at: 2026-11-04T23:50:36.941346921Z
event:
  event: finding
  recommendation: defer
---

low (pre-existing, not introduced here): crates/astromech-intent/src/lib.rs:535 the older NOT_A_NAME words block real systems. A scan of the full Spansh dump (~/.cache/astromech/spansh/systems.json.gz) for names whose first word is any NOT_A_NAME entry finds 19: A Bootis, A Capricorni, A Carinae, A Cen, a Centauri, a Lupi, a Puppis, A/a Velorum, AN Coronae Borealis, AN Mensae, AN Orionis, AN Sextantis, Any Kutjara, Any Minurok, Any Na, Any Nal, MY Apodis, MY Orionis. 'where can I sell gold near Any Na' searches from the current system and says nothing about it. The four new words (bulk, large, quantity, quantities) start no system name in the dump. Suggest checking the place against the {system} slot's exact phrasings (known systems) before applying NOT_A_NAME, with a test for 'near A Bootis'.
