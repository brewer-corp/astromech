---
schema: atelier.review
schema_version: 1
id: astromech-wbxx
title: 'R3 [medium] One-sided pressure bounds are effectively not applied. crates/astromech-exobiology/src/predict.rs range_fit: with only a min or only a max, the span is bound.abs().max(1.0), so for pressure (every bound < 1 atm) the taper margin is a fixed 0.15 atm, 5-100x the bound itself: Aleoida Spica max 0.0135 atm still fits at 0.1 atm (fit ~0.42), Aleoida Arcus min 0.0161 fits a 0.002 atm body at 0.91. 116 of 121 rules with a pressure bound are one-sided. This contradicts the module doc (''15 % of the range beyond either edge'') and is part of why bodies get up to 26 candidates in 11 genera (36 Ophiuchi A 4 a, real: 2). It is load-bearing for the recall claim: scaling the one-sided margin to the bound makes the fixture test fail (Eol Prou KW-L c8-268 1 d: Stratum Limaxus at 0.0147 atm vs its rule''s min 0.05). Fix: decide deliberately (pressure as a hard or tapered gate scaled to the bound, accepting the misses, or pressure dropped as unreliable) and make the code, doc comment and a test say the same thing.'
target: astromech-8e8c
identity: 'review-agent (opus, independent, part A: data and prediction)'
commit: e5f94c22467a3514e2f7f88d5e0f7a58780e5631
created_at: 2026-10-17T05:57:53.326784376Z
event:
  event: finding
  recommendation: fix_now
---

R3 [medium] One-sided pressure bounds are effectively not applied. crates/astromech-exobiology/src/predict.rs range_fit: with only a min or only a max, the span is bound.abs().max(1.0), so for pressure (every bound < 1 atm) the taper margin is a fixed 0.15 atm, 5-100x the bound itself: Aleoida Spica max 0.0135 atm still fits at 0.1 atm (fit ~0.42), Aleoida Arcus min 0.0161 fits a 0.002 atm body at 0.91. 116 of 121 rules with a pressure bound are one-sided. This contradicts the module doc ('15 % of the range beyond either edge') and is part of why bodies get up to 26 candidates in 11 genera (36 Ophiuchi A 4 a, real: 2). It is load-bearing for the recall claim: scaling the one-sided margin to the bound makes the fixture test fail (Eol Prou KW-L c8-268 1 d: Stratum Limaxus at 0.0147 atm vs its rule's min 0.05). Fix: decide deliberately (pressure as a hard or tapered gate scaled to the bound, accepting the misses, or pressure dropped as unreliable) and make the code, doc comment and a test say the same thing.
