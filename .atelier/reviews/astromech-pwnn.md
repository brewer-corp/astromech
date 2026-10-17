---
schema: atelier.review
schema_version: 1
id: astromech-pwnn
title: 'Fix 81ec4a0 reviewed. R1: plan() takes max carried and sizes goods-for-sale room as max minus carried, so symbols() is at most 12 in every case; plan and compare share one Lookups so the unlooked-up carried lines (checked=false) agree. Order is deterministic (sellable sorts by tonnes desc, then symbol). 14-type test asserts 12 requests and the 2 unlooked names. R2: Market.svelte shows ''Market as of <when(seen)> UTC (<ago>)''; formatting checked, component test added. Reran here tests (7 pass) and pnpm test (162 pass). Minor, no action: the new test''s hold is already in descending order so it does not prove the tonnes sort (covered by sellable''s sort); a carried good past the cap that is also for sale here is looked up via the for-sale list yet shows checked=false (harmless, best still shown).'
target: astromech-qw2m
identity: review-agent (sonnet, independent, fix review)
commit: 4a88aee91a0e83e8d1b33a2bf57f93ad599b051d
created_at: 2026-10-17T02:52:43.709997658Z
event:
  event: approval
---

Fix 81ec4a0 reviewed. R1: plan() takes max carried and sizes goods-for-sale room as max minus carried, so symbols() is at most 12 in every case; plan and compare share one Lookups so the unlooked-up carried lines (checked=false) agree. Order is deterministic (sellable sorts by tonnes desc, then symbol). 14-type test asserts 12 requests and the 2 unlooked names. R2: Market.svelte shows 'Market as of <when(seen)> UTC (<ago>)'; formatting checked, component test added. Reran here tests (7 pass) and pnpm test (162 pass). Minor, no action: the new test's hold is already in descending order so it does not prove the tonnes sort (covered by sellable's sort); a carried good past the cap that is also for sale here is looked up via the for-sale list yet shows checked=false (harmless, best still shown).
