---
schema: atelier.review
schema_version: 1
id: astromech-v12q
title: 'R3 low: scripts/gen-odyssey.py:128-131 adds an uncertain entry only when a cell has a ''CR'' part that will not parse. A cost with no CR line at all gets credits null and no entry. data/catalog/odyssey.json:1912 (Added Melee Damage) has credits null but is missing from ''uncertain'', so the page''s ''N values are unsure'' count is too low and the pick shows ''0+ CR''. Fix: after the loop in parse_cost, flag when cr is None. A catalogue test should also assert that every null credits value has a matching uncertain entry.'
target: astromech-j4c9
identity: review-agent (j4c9 review)
commit: 7094a73373653b239990f7e278c845bcc2afc096
created_at: 2026-10-16T22:28:48.822348486Z
event:
  event: finding
  recommendation: defer
---

R3 low: scripts/gen-odyssey.py:128-131 adds an uncertain entry only when a cell has a 'CR' part that will not parse. A cost with no CR line at all gets credits null and no entry. data/catalog/odyssey.json:1912 (Added Melee Damage) has credits null but is missing from 'uncertain', so the page's 'N values are unsure' count is too low and the pick shows '0+ CR'. Fix: after the loop in parse_cost, flag when cr is None. A catalogue test should also assert that every null credits value has a matching uncertain entry.
