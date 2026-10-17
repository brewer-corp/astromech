---
schema: atelier.review
schema_version: 1
id: astromech-qn02
title: 'R2 [medium] Fonticulua Fluctus is valued at 16,777,215 (2^24-1), regressing p4ls''s 20,000,000 in the unsold organic total (crates/astromech-exploration/src/exobiology.rs test and tests/unsold.rs changed to 12 x 16,777,215). The only justification is the generator''s ''wiki wins'' rule, but every other source disagrees: EDR 20,000,000, ExplorerLite 20,000,000 with the comment ''corrected from 16,777,215 -- multiple sources (incl. BioScan) agree on 20M'' (explorer/valuation/exobiology_data.py at 6a5d65a, the very file the generator reads), and (reference only) BioScan 20,000,000. The wiki''s Fluctus cell is the odd 24-bit max copied from the Concha Biconcavis row next to it (which BioScan and the wiki agree is 16,777,215). Impact: the highest-value species, and the ''risky haul'' total, are 16% low. Fix: let the generator override a wiki value when the other sources agree against it (or hard-code Fluctus 20,000,000 with a note), keep it under uncertain, and restore the 20 M expectations. (Correction to R1: 90 species carry rules, not 102; the 224 and 955 counts stand.)'
target: astromech-znhx
identity: 'review-agent (opus, independent, part A: data and prediction)'
commit: e5f94c22467a3514e2f7f88d5e0f7a58780e5631
created_at: 2026-10-17T05:57:53.234165917Z
event:
  event: finding
  recommendation: fix_now
---

R2 [medium] Fonticulua Fluctus is valued at 16,777,215 (2^24-1), regressing p4ls's 20,000,000 in the unsold organic total (crates/astromech-exploration/src/exobiology.rs test and tests/unsold.rs changed to 12 x 16,777,215). The only justification is the generator's 'wiki wins' rule, but every other source disagrees: EDR 20,000,000, ExplorerLite 20,000,000 with the comment 'corrected from 16,777,215 -- multiple sources (incl. BioScan) agree on 20M' (explorer/valuation/exobiology_data.py at 6a5d65a, the very file the generator reads), and (reference only) BioScan 20,000,000. The wiki's Fluctus cell is the odd 24-bit max copied from the Concha Biconcavis row next to it (which BioScan and the wiki agree is 16,777,215). Impact: the highest-value species, and the 'risky haul' total, are 16% low. Fix: let the generator override a wiki value when the other sources agree against it (or hard-code Fluctus 20,000,000 with a note), keep it under uncertain, and restore the 20 M expectations. (Correction to R1: 90 species carry rules, not 102; the 224 and 955 counts stand.)
