---
schema: atelier.review
schema_version: 1
id: astromech-olfl
title: 'Checked clean: filter sides are right (age, carriers and volume go to Ardent and are re-checked locally; pad, Odyssey and arrival are local only; the radius is dropped). The Near rank_offers refactor is behaviour-identical (with_defaults then best_first). Remembered filters are separate (lookup key trade.<side>.anywhere). The fixtures are trimmed real answers with synthetic FCX-8xx carrier names and ids. The tests prove the Done-when (price order plus updated_at). trade_anywhere (main.rs:715) mirrors trade_search and type-checks by inspection, but cannot be compiled on Linux.'
target: astromech-d1lo
identity: review-agent (d1lo review, sonnet)
commit: 9b7d288da903b76e2d6df0de91c4f4ba1de60958
created_at: 2026-10-17T00:31:04.719143084Z
event:
  event: finding
  recommendation: no_action
---

Checked clean: filter sides are right (age, carriers and volume go to Ardent and are re-checked locally; pad, Odyssey and arrival are local only; the radius is dropped). The Near rank_offers refactor is behaviour-identical (with_defaults then best_first). Remembered filters are separate (lookup key trade.<side>.anywhere). The fixtures are trimmed real answers with synthetic FCX-8xx carrier names and ids. The tests prove the Done-when (price order plus updated_at). trade_anywhere (main.rs:715) mirrors trade_search and type-checks by inspection, but cannot be compiled on Linux.
