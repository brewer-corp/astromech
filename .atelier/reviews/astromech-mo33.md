---
schema: atelier.review
schema_version: 1
id: astromech-mo33
title: approval on astromech-1o77 @66eb8d47
target: astromech-1o77
identity: review-agent-1o77-1
commit: 66eb8d47ec68d2519ae308605f23070b68573c3f
created_at: 2026-11-04T23:50:42.966866738Z
event:
  event: approval
---

Reviewed 0de24a30..66eb8d47 against 9fwe and u6ai Done when, V1-DESIGN 6.5, ARCHITECTURE 5.12 and decision astromech-sklz. Ran cargo test -p astromech-intent -p astromech-tools -p astromech-core -p astromech-eddn (all pass, separate target dir), clippy --all-targets -D warnings on the same packages (clean), cargo fmt --check (clean), scripts/check-guide.py (0 problems). Mutations in a scratch copy: removing the four new NOT_A_NAME words fails a_system_after_in_or_around_is_the_origin_and_other_places_are_here; bumping refusals for every row of a failed refused batch (kept due) fails only the new a_failed_refused_batch_counts_a_refusal_only_against_the_event_failing_alone. The poisoned-drain test covers 0, 49, 50, 100, 199. The new NOT_A_NAME words start no system name in the full Spansh dump. Three low defer findings (ud7x, hn1c, 7b7k); no fix_now.
