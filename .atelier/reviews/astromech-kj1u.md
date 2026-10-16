---
schema: atelier.review
schema_version: 1
id: astromech-kj1u
title: A bad saved pick is reported on its own result (error) and left out of the totals; the page renders cards from the saved picks when there is no current list, shows the reason on the card, and removes by pick identity. Crate test a_bad_pick_is_reported_on_its_own_and_the_rest_still_count; component tests for a failed list and for one bad pick (both fail on the old page).
target: astromech-j4c9
identity: orchestrator (Claude, not a code author)
commit: aca815d6622a13dc0d938aa00738704871d7ce70
created_at: 2026-10-16T22:34:09.856883123Z
event:
  event: disposition
  finding: astromech-0rqw
  decision: fix_now
  supersedes: null
  resolution_commit: aca815d6622a13dc0d938aa00738704871d7ce70
  follow_up: null
---

A bad saved pick is reported on its own result (error) and left out of the totals; the page renders cards from the saved picks when there is no current list, shows the reason on the card, and removes by pick identity. Crate test a_bad_pick_is_reported_on_its_own_and_the_rest_still_count; component tests for a failed list and for one bad pick (both fail on the old page).
