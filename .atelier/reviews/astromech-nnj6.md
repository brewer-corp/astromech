---
schema: atelier.review
schema_version: 1
id: astromech-nnj6
title: finding on astromech-jp36 @e2a7b6bb
target: astromech-jp36
identity: reviewer-sonnet-jp36
commit: e2a7b6bb7dfdbdef096bb343d095fa592db019c8
created_at: 2026-11-05T08:24:34.981348126Z
event:
  event: finding
  recommendation: fix_now
---

low: DESIGN.md Lookup result row / Aligned Columns Rule: the branch adds behaviour DESIGN.md (binding) does not state: a lookup in a panel too narrow for its facts (SearchKit.svelte compact) shows only ranked value, place and actions, contradicting 'pad (always shown for stations)', and text columns (faction, module, bodies) are flexible, share leftover width with the place and truncate. Decision: record both now in two or three sentences under Lookup result row (narrow panels follow the Aligned Columns Rule: facts go, the actions stay; text columns truncate, the place first), instead of waiting for Places (nxz9), which builds full-width finders and does not change the panel rule. Also record there that staleness is judged as of the answer's fetch time (see the decision on item 5, keep).
