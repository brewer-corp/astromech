---
schema: atelier.review
schema_version: 1
id: astromech-eony
title: finding on astromech-54k4 @1e88a1bb
target: astromech-54k4
identity: review-agent-54k4-1
commit: 1e88a1bb5948024f1f778d1d98128a42be2be4de
created_at: 2026-11-04T21:48:03.441627221Z
event:
  event: finding
  recommendation: defer
---

Low: crates/astromech-intent/src/lib.rs:467-470 with FILLER at :510: because 'my' is a filler word, 'how much is my ship worth' and 'what is my ship worth' normalise to 'how much is ship worth' / 'what is ship worth', which score 0.900 against 'how much is this worth' and 0.875 against 'what is this worth'. Both used to ask 'Did you mean what is this system worth?' (0.833 / 0.800) and now get 'Say again.'. Said in a system, these are plausible near misses for system value, not unrelated speech, so the everyday list now swallows them. Not covered by the Done-when list either way. Suggest deciding the intended behaviour and pinning it in everyday_questions_ask_no_exploration; if they should keep asking, keep only the ship phrases that say 'ship' next to 'my' (e.g. drop the 'what is my ship worth' form, or require the noun).
