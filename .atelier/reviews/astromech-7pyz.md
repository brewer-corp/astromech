---
schema: atelier.review
schema_version: 1
id: astromech-7pyz
title: 'Grammar near-misses (assistant::grammar, probed at 2647cb3): existing phrases are unaffected. ''what''s my powerplay rank'', ''who controls this system'' and ''where can I buy steel/a Krait'' still Execute, and the test covers the Powerplay status phrases. ''nearest system to refuel'', not a grammar phrase, now asks ''Did you mean nearest system to reinforce?'' (0.833) where it asked the refuel lookup (0.792). ''where can I acquire a ship'' asks the acquire lookup. Both are Ask only, on phrasings the grammar never had.'
target: astromech-8joi
identity: review-agent (opus, independent)
commit: 2647cb30c796b6226a8243e764cf1993a1e0c71b
created_at: 2026-10-17T08:17:48.487636528Z
event:
  event: finding
  recommendation: no_action
---

Grammar near-misses (assistant::grammar, probed at 2647cb3): existing phrases are unaffected. 'what's my powerplay rank', 'who controls this system' and 'where can I buy steel/a Krait' still Execute, and the test covers the Powerplay status phrases. 'nearest system to refuel', not a grammar phrase, now asks 'Did you mean nearest system to reinforce?' (0.833) where it asked the refuel lookup (0.792). 'where can I acquire a ship' asks the acquire lookup. Both are Ask only, on phrasings the grammar never had.
