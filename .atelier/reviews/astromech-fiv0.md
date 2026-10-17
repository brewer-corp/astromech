---
schema: atelier.review
schema_version: 1
id: astromech-fiv0
title: 'low: crates/astromech-powerplay/src/logistics.rs version() and the file''s ''fetched'' are used only by a unit test and nothing outside; either surface them in Logistics (rules version, like rules_version for activities) or remove. Also the reinforce/undermine ''no limit'' rules are only flagged uncertain in the data, not checked against the game; fine for now, but needs-human in-game check should be filed.'
target: astromech-bf25
identity: review-agent (sonnet, independent)
commit: 31a3ed261d1ce12acde54b1b84d01ae4ac96e790
created_at: 2026-10-17T03:02:53.820704544Z
event:
  event: finding
  recommendation: defer
---

low: crates/astromech-powerplay/src/logistics.rs version() and the file's 'fetched' are used only by a unit test and nothing outside; either surface them in Logistics (rules version, like rules_version for activities) or remove. Also the reinforce/undermine 'no limit' rules are only flagged uncertain in the data, not checked against the game; fine for now, but needs-human in-game check should be filed.
