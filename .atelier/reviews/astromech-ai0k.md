---
schema: atelier.review
schema_version: 1
id: astromech-ai0k
title: approval on astromech-jp36 @2b3396a4
target: astromech-jp36
identity: reviewer-sonnet-jp36
commit: 2b3396a467007cd21a1dad3bcc0e86aae8f0dd3d
created_at: 2026-11-05T08:26:46.886873399Z
event:
  event: approval
---

Reviewed f88f472e..e2a7b6bb plus fix 509d84e3 (head 2b3396a4; no app/crates/DESIGN changes after it). akom: hidden_stale gone, kit counts only rows that came back past the filter and says N older reports hidden only when above zero, DESIGN.md States, ARCHITECTURE.md and jp36 Done when agree; keeping the client-side count is right (cheap, tested, honest about its limit). qwzi: Allegiance.svelte deleted, no references left. nnj6: DESIGN.md records text-column truncation, the compact panel row and fetch-time staleness. Reran in app/: lint clean, svelte-check 0/0, 427 tests pass; check-guide 0 problems; earlier build and a 14-case overflow check at 1100/1600 px passed. a400 deferred, d8w6 and wxh1 no_action.
