---
schema: atelier.review
schema_version: 1
id: astromech-dewj
title: 'R6 [low] crates/astromech-market/src/cargo.rs plan/best_partial: when Ardent''s answer is truncated (1000 rows, best price first) and the min-demand re-ask finds no whole-lot buyer, the partial buyer is chosen only from the top 1000 by price, so a cheaper market with much more demand that was cut could pay more in all. Rare (the cut rows are the lowest prices) and the same limit applies to the whole-load ranking; no action unless seen in practice.'
target: astromech-proz
identity: review-agent (opus, independent)
commit: 72df8be9943607bf04534ac08f1161785b65f852
created_at: 2026-10-17T05:23:24.217411805Z
event:
  event: finding
  recommendation: no_action
---

R6 [low] crates/astromech-market/src/cargo.rs plan/best_partial: when Ardent's answer is truncated (1000 rows, best price first) and the min-demand re-ask finds no whole-lot buyer, the partial buyer is chosen only from the top 1000 by price, so a cheaper market with much more demand that was cut could pay more in all. Rare (the cut rows are the lowest prices) and the same limit applies to the whole-load ranking; no action unless seen in practice.
