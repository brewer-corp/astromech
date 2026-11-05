---
schema: atelier.review
schema_version: 1
id: astromech-5w3d
title: finding on astromech-j75v @f94f9050
target: astromech-j75v
identity: review-agent-j75v-1
commit: f94f9050f0eecc4fc316abec010ce89e766b81dc
created_at: 2026-11-04T22:26:59.322061057Z
event:
  event: finding
  recommendation: defer
---

minor: crates/astromech-intent/src/lib.rs:467 Now that the near-me phrases split, anything after 'near' that is not in HERE_WORDS becomes a named system and executes at 1.00. 'where is there a famine near mi', 'near you', 'near Nemi' and 'near my current location' executed from the current system at 9f516d3e (0.83-0.92 against 'near me'). Now they answer 'Spansh does not know a system called mi.' (or 'you', 'my current location'). The trade and station lookups already behaved this way. The answer is plain, not a wrong search, but it is a small regression for common here-phrasings. Suggest, under astromech-1n9b: add 'you', 'my current location', 'my ship' and 'by' to HERE_WORDS, and treat a place that sounds like 'me' (the phonetic key) as here.
