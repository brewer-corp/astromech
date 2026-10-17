---
schema: atelier.review
schema_version: 1
id: astromech-d5br
title: '[low] exploration_unsold returns Err(''no journal history yet...'') when history.sqlite is absent, which View.svelte renders as a red error rather than the ''No commander yet'' empty state; a first-run state looks like a fault. Also View.test.ts covers only the fixture happy path: no test for the empty/null unsold state, the error state, or the Sell picker (service ids do exist in lib/stations.ts). Return Ok(null) for a missing db.'
target: astromech-tzc2
identity: 'review-agent (sonnet, independent, part B: callouts, history, shell, UI)'
commit: abce3096a915e5a11d15fce7f95785d6761e0b09
created_at: 2026-10-17T05:14:59.440118611Z
event:
  event: finding
  recommendation: defer
---

[low] exploration_unsold returns Err('no journal history yet...') when history.sqlite is absent, which View.svelte renders as a red error rather than the 'No commander yet' empty state; a first-run state looks like a fault. Also View.test.ts covers only the fixture happy path: no test for the empty/null unsold state, the error state, or the Sell picker (service ids do exist in lib/stations.ts). Return Ok(null) for a missing db.
