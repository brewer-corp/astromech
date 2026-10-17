---
schema: atelier.review
schema_version: 1
id: astromech-foo8
title: 'R1 medium crates/astromech-market/src/here.rs compare_here: the 12-lookup cap is not enforced when the hold has more than 12 sellable commodity types. plan() saturates room to 0 but compare_here still asks for every load symbol, so a large hold makes more than 12 requests, and the tests only cap the for-sale side. Fix: truncate the load symbols to MAX_LOOKUPS (and plan/compare must agree on which carried lines are unlooked), plus a test with 13+ types.'
target: astromech-qw2m
identity: review-agent (sonnet, independent)
commit: 1452c58046afbabb4365f5793a780d885ac320ca
created_at: 2026-10-17T02:45:26.564622188Z
event:
  event: finding
  recommendation: fix_now
---

R1 medium crates/astromech-market/src/here.rs compare_here: the 12-lookup cap is not enforced when the hold has more than 12 sellable commodity types. plan() saturates room to 0 but compare_here still asks for every load symbol, so a large hold makes more than 12 requests, and the tests only cap the for-sale side. Fix: truncate the load symbols to MAX_LOOKUPS (and plan/compare must agree on which carried lines are unlooked), plus a test with 13+ types.
