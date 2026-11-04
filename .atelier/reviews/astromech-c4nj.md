---
schema: atelier.review
schema_version: 1
id: astromech-c4nj
title: finding on astromech-j75v @9f516d3e
target: astromech-j75v
identity: review-agent-j75v-1
commit: 9f516d3e479fae0aa610989b7b29db820e240589
created_at: 2026-11-04T21:25:05.845292754Z
event:
  event: finding
  recommendation: defer
---

minor: crates/astromech-intent/src/lib.rs:472 Only a trailing 'near' or 'from' is split off. 'where can I sell gold in Sol' -> trade.sell Coltan (Ask) and 'around Sol' -> Food Cartridges (Ask) are the same misreading the issue reports, with another preposition. 'near the Sol system' / 'near Sol system' keeps 'Sol system' as the name, which Spansh does not know, so the player hears 'Spansh does not know a system called Sol system.' Any non-system words after 'near' or 'from' are also taken as a name ('from a station' -> 'Spansh does not know a system called a station.'), which is plain but odd. Suggest a follow-up: strip a trailing 'system', and consider 'in' and 'around' once voice-eval data shows how people say it.
