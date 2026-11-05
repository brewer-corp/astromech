---
schema: atelier.review
schema_version: 1
id: astromech-temh
title: finding on astromech-j75v @f94f9050
target: astromech-j75v
identity: review-agent-j75v-1
commit: f94f9050f0eecc4fc316abec010ce89e766b81dc
created_at: 2026-11-04T22:26:59.038225877Z
event:
  event: finding
  recommendation: defer
---

minor: crates/astromech-intent/src/lib.rs:989 Matching the head with 'near me' appended lets short non-lookup sentences followed by 'near/from <system>' borrow the shared 'near me' words of 'famine near me' and clear the 0.60 ask threshold. I swept every non-lookup MVP phrase with ' near Sol', ' from Sol' and ' near mi' against assistant::grammar(), at 9f516d3e and at f94f9050. About 60 sentences went from NoMatch ('Say again') to a 'Did you mean famine near Sol?' question at 0.60-0.79: 'paste from Sol' 0.78, 'negative near Sol' 0.79, 'affirmative near Sol' 0.79, 'fuel near Sol', 'cancel near Sol', 'supercruise near Sol', 'raise gear near Sol'. 'where am i near Sol' and 'where are we from Sol' now ask about acquire and war. None reached Execute, and the lookups are read-only, so the cost is a wrong question rather than a wrong action. Suggest, under astromech-1n9b: take the near-me variant only when the bare head already matches the same command above the ask threshold, or score it without the appended words; add the sweep's worst cases as NoMatch tests.
