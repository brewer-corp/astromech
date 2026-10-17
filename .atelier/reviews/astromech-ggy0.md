---
schema: atelier.review
schema_version: 1
id: astromech-ggy0
title: 'low crates/astromech-eddn/src/feed.rs:184 (companion_file): the 5 min budget is measured from the event timestamp to file arrival, but journal() already admits a live event up to 5 min old (feed.rs:127). An event accepted at age 4:50 whose companion file lands 15 s later is dropped silently with no log row, though it was live when received. Normal Market/Shipyard/NavRoute files arrive within seconds of the event, so legitimate loss is rare. Consider measuring from when the event was received, and/or logging the drop as a skipped row.'
target: astromech-6xyo
identity: review-agent (6xyo review, sonnet)
commit: 65f81c752d6d2f419643e6b4a75786507ea51c6b
created_at: 2026-10-17T00:07:27.593131593Z
event:
  event: finding
  recommendation: defer
---

low crates/astromech-eddn/src/feed.rs:184 (companion_file): the 5 min budget is measured from the event timestamp to file arrival, but journal() already admits a live event up to 5 min old (feed.rs:127). An event accepted at age 4:50 whose companion file lands 15 s later is dropped silently with no log row, though it was live when received. Normal Market/Shipyard/NavRoute files arrive within seconds of the event, so legitimate loss is rare. Consider measuring from when the event was received, and/or logging the drop as a skipped row.
