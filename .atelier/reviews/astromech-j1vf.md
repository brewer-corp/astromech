---
schema: atelier.review
schema_version: 1
id: astromech-j1vf
title: finding on astromech-yxtz @600f7bc2
target: astromech-yxtz
identity: reviewer-sonnet-yxtz
commit: 600f7bc2defbf5380266e7b900fe73a01c26c517
created_at: 2026-11-05T16:34:35.535515762Z
event:
  event: finding
  recommendation: fix_now
---

high: app/src/modules/settings/ProspectorThresholds.svelte:61,76 the always-rendered p.hint.warn-text now has ::before content '! ' in --alarm, so Settings > General shows a permanent red '!' under Prospector callouts when there is no error (confirmed with scripts/ui ?core=ready#/settings: empty text, ::before '! ', rgb(240,65,46), 20px tall; visible in render). Breaks The Narrow Red Rule. Fix: show the marker only when error is non-empty (class:warn-text={error} as Callouts.svelte does), keeping the aria-live region always present.
