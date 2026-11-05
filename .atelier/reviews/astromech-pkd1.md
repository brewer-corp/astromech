---
schema: atelier.review
schema_version: 1
id: astromech-pkd1
title: finding on astromech-sllo @1fd71219
target: astromech-sllo
identity: reviewer-fable-sllo
commit: 1fd712195d1b0ebd90f65ccdd3d7f695e3f16e83
created_at: 2026-11-05T18:50:34.359478032Z
event:
  event: finding
  recommendation: defer
---

low: app/src/modules/odyssey/View.svelte:356 On foot > Engineers shows the same suit engineers as Engineers > Suit engineers (#/unlocks?kind=suit): name, status, system, referrals and invitation; only the per-engineer mod list is unique to On foot. Decision Q1 keeps it as a Choice for now (the reversible option, nothing is lost). Follow-up: move the mod list onto the Engineers page's suit cards and drop the On foot view, so one page owns suit engineers.
