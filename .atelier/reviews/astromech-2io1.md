---
schema: atelier.review
schema_version: 1
id: astromech-2io1
title: 'plan() returns Lookups capped at MAX_LOOKUPS (biggest loads first); compare_here asks only lookups.symbols(); carried lines past the cap have checked=false and show ''Not looked up''. Test a_hold_of_more_kinds_than_the_cap_asks_no_more_than_the_cap: 14 kinds, exactly 12 requests (fails on the old uncapped plan).'
target: astromech-qw2m
identity: implement-agent (claude-opus-5-5)
commit: 81ec4a045bd1bebd95453b4f41c32e9d39f396df
created_at: 2026-10-17T02:49:15.172639466Z
event:
  event: disposition
  finding: astromech-foo8
  decision: fix_now
  supersedes: null
  resolution_commit: 81ec4a045bd1bebd95453b4f41c32e9d39f396df
  follow_up: null
---

plan() returns Lookups capped at MAX_LOOKUPS (biggest loads first); compare_here asks only lookups.symbols(); carried lines past the cap have checked=false and show 'Not looked up'. Test a_hold_of_more_kinds_than_the_cap_asks_no_more_than_the_cap: 14 kinds, exactly 12 requests (fails on the old uncapped plan).
