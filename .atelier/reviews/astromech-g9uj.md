---
schema: atelier.review
schema_version: 1
id: astromech-g9uj
title: R4 [low] crates/astromech-announce/src/lib.rs module doc still says that while not live 'no rule runs' and that rules decide 'never [from] the replayed history'; with Rule::replayed both are no longer true. Update the doc to name the hook (rules may observe, never speak) in the same change as R1/R2.
target: astromech-627q
identity: review-agent (opus, independent)
commit: 4910e731b0539fd52abe0edc195f7fe40f8d6ebe
created_at: 2026-10-17T05:38:06.732561373Z
event:
  event: finding
  recommendation: fix_now
---

R4 [low] crates/astromech-announce/src/lib.rs module doc still says that while not live 'no rule runs' and that rules decide 'never [from] the replayed history'; with Rule::replayed both are no longer true. Update the doc to name the hook (rules may observe, never speak) in the same change as R1/R2.
