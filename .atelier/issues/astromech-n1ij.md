---
created_at: "2026-10-16T19:27:17.482285552+00:00"
id: "astromech-n1ij"
issue_type: "task"
labels: []
workflow_branch:
  branch_base: main
  work_branch: task/astromech-n1ij
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
  - kind: "issue"
    id: "astromech-qz9u"
    type: "related"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-17T05:00:39.268729455+00:00"
status: "done"
title: "Search kit follow-ups: shared settings path, safe migrations, origin Enter, live pad, age filter"
updated_at: "2026-10-17T05:00:39.268729455+00:00"
---

## Goal

Close the gaps review found in the search kit's filters and origin (astromech-qz9u):

- R3: the shell's `settings()` opens `astromech.sqlite` in its own `app_dir()`. It should open the core's data dir (`Config::data_dir()`, which knows about portable mode and a `data_dir` override), so filters land in the same store the core uses.
- R4: `astromech-store` runs migrations outside a transaction. If the shell and core open a brand-new file at the same moment, one can fail with "table settings already exists". Run each migration in `BEGIN IMMEDIATE` and re-read `user_version` inside it.
- R6: Enter in `OriginInput` picks the first suggestion even when the typed text exactly matches another one (Spansh's autocomplete is fuzzy). Prefer an exact, case-insensitive match, else the typed text.
- R7: the pad filter is resolved for the current ship only when a lookup's filters load, so a ship change shows the old pad until the lookup is re-selected. Resolve the pad server-side in `galaxy_nearby` (saved filters plus current ship) on every search.
- R8: check the pad size for `mediumtransport01` (Lynx Highliner) in `astromech_galaxy::filters::Pad::for_ship` against the wiki, and the other entries added from memory (Caspian Explorer, Type-11 Prospector, Kestrel Mk II).
- R9: the data-age filter is applied after fetching the 50 nearest systems, so a strict age can return nothing even when fresher matches exist farther out. Send the age to Spansh (if it has a verified `updated_at` filter) or page until enough hits pass.

## Done when

Each item has a test or check that fails before the fix: a concurrent first-open test for R4, an exact-match Enter check for R6, a ship change without re-selecting the lookup for R7, and a strict-age lookup that still returns farther matches for R9. R3 is checked against `Config::data_dir()` in portable mode. R8's sizes are confirmed against the wiki.

## Context

Found in the independent review of feature/astromech-qz9u at 1d1921b (astromech-pxy2, astromech-1imw).

## Notes

- 2026-10-16 [note] Also (review of qz9u round 3, N1): two overlapping filter changes can clobber each other in SearchKit when the first save fails; use a per-change counter like the search's asked counter.
- 2026-10-16 [note] R4 (migrations outside a transaction) is fixed on feature/astromech-ipb5 by the astromech-ipb5 review fix: astromech_store::migrate now runs each migration and its user_version bump in one BEGIN IMMEDIATE transaction and re-reads user_version inside it (test failed_migration_rolls_back_whole). It reaches main when astromech-ipb5 merges.
- 2026-10-16 [note] R7 fixed on feature/astromech-m30a: galaxy_nearby and stations_nearby both re-resolve the pad from the current ship (Filters::for_ship) on every search. The shell isn't unit-testable on Linux; the rule itself is covered by filters::tests::pad_follows_the_ship_unless_the_commander_set_it. The filter box's 'Ship's (X)' label still shows the pad from when the lookup's filters loaded.
- 2026-10-17 [note] Implemented on task/astromech-n1ij at 2a32d75. R3: shell settings() uses astromech_store::data_dir (shared with Config::data_dir). R4: migrations were already transactional on main; added a concurrent first-open test, which exposed a real 'database is locked' on the WAL pragma, now retried. R6: enterChoice. R7: already server-side on main (search_origin resolves the pad on every search); no new code. R8: pads confirmed against Coriolis data (the wiki is behind a bot wall): Lynx M, Caspian L, Type-11 M, Kestrel S, no change needed. R9: updated_at sent to Spansh for systems, bodies, colony candidates, powerplay targets. N1: per-change counter in SearchKit. Shell main.rs is not compiled on Linux (inspection only).

## Close Reason

All items done on task/astromech-n1ij; awaiting independent review and merge by the orchestrator
