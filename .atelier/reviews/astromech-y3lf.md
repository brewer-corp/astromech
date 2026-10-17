---
schema: atelier.review
schema_version: 1
id: astromech-y3lf
title: 'R1 [medium] The ship-change test does not guard search_origin. crates/astromech-galaxy/src/filters.rs a_search_resolves_the_pad_from_the_current_ship_not_the_filters_sent tests Filters::for_ship_type, which is for_ship plus Pad::for_ship, both already covered by pad_follows_the_ship_unless_the_commander_set_it. Mutation: change app/src-tauri/src/main.rs:660 to ''let filters = filters.unwrap_or_default();'' and nothing fails, on Linux or in CI: the Windows job only builds and clippies app/src-tauri (ci.yml:118-123) and the shell has no tests. That is the same gap finding astromech-lh67 named (''nothing fails if search_origin stops calling it''), so this done-when clause is not met. Fix: split search_origin''s resolution into a function that takes the engineering state rather than the AppHandle, test a ship change through it, and run cargo test for app/src-tauri in the Windows CI job. Or the orchestrator rescopes the clause explicitly to ''rule tested in galaxy; shell call checked by inspection''.'
target: astromech-ssyo
identity: review-agent (opus, independent)
commit: 7d5fa6d67b38f916be0bf4bc76d95f87e7d884d0
created_at: 2026-10-17T06:09:22.664718119Z
event:
  event: finding
  recommendation: fix_now
---

R1 [medium] The ship-change test does not guard search_origin. crates/astromech-galaxy/src/filters.rs a_search_resolves_the_pad_from_the_current_ship_not_the_filters_sent tests Filters::for_ship_type, which is for_ship plus Pad::for_ship, both already covered by pad_follows_the_ship_unless_the_commander_set_it. Mutation: change app/src-tauri/src/main.rs:660 to 'let filters = filters.unwrap_or_default();' and nothing fails, on Linux or in CI: the Windows job only builds and clippies app/src-tauri (ci.yml:118-123) and the shell has no tests. That is the same gap finding astromech-lh67 named ('nothing fails if search_origin stops calling it'), so this done-when clause is not met. Fix: split search_origin's resolution into a function that takes the engineering state rather than the AppHandle, test a ship change through it, and run cargo test for app/src-tauri in the Windows CI job. Or the orchestrator rescopes the clause explicitly to 'rule tested in galaxy; shell call checked by inspection'.
