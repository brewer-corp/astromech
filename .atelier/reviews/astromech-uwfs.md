---
schema: atelier.review
schema_version: 1
id: astromech-uwfs
title: 'Not approved: R1 and R2 are FIX NOW. What I checked: the diff main...7d5fa6d against the done-when. cargo test -p astromech-galaxy passes. Mutation-checked: removing updated_since in sites.rs:98 fails the_data_age_is_sent_with_the_bodies_search, removing it in colonies.rs:164 fails the_data_age_is_sent_with_the_unpopulated_search, and removing followShip in SearchKit.svelte fails the new SearchKit label test (vitest 3/3 otherwise). The age-in-request clauses are met. The ship-change clause is not: nothing fails if search_origin stops re-resolving the pad (R1). The label is fixed for SearchKit but not for the trade Route page (R2). The shell is checked by inspection only (not compiled on Linux). CI on PR #77: Linux and Windows build/tests pass; the Windows installer is pending.'
target: astromech-ssyo
identity: review-agent (opus, independent)
commit: 7d5fa6d67b38f916be0bf4bc76d95f87e7d884d0
created_at: 2026-10-17T06:09:29.057563140Z
event:
  event: review
---

Not approved: R1 and R2 are FIX NOW. What I checked: the diff main...7d5fa6d against the done-when. cargo test -p astromech-galaxy passes. Mutation-checked: removing updated_since in sites.rs:98 fails the_data_age_is_sent_with_the_bodies_search, removing it in colonies.rs:164 fails the_data_age_is_sent_with_the_unpopulated_search, and removing followShip in SearchKit.svelte fails the new SearchKit label test (vitest 3/3 otherwise). The age-in-request clauses are met. The ship-change clause is not: nothing fails if search_origin stops re-resolving the pad (R1). The label is fixed for SearchKit but not for the trade Route page (R2). The shell is checked by inspection only (not compiled on Linux). CI on PR #77: Linux and Windows build/tests pass; the Windows installer is pending.
