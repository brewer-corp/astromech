---
schema: atelier.review
schema_version: 1
id: astromech-85ni
title: 'Pre-existing, not introduced here: crates/astromech-exploration/src/value.rs planet_value applies the 500 cr floor before the mapping multiplier (max(500, base) x mult); EDDI Body.cs estimateBodyValue applies it after (Math.Max(500, base x mult)). For mapped planets with base < 500 (k=300 bodies under ~2.3 EM) astromech is high: a 1 EM icy body mapped (already mapped, Odyssey) is 2222 here vs 2121 in EDDI. Scan-only values (the EDSM-checked tests) are unaffected. File a follow-up to settle which order the game uses and align.'
target: astromech-e8up
identity: review-agent (opus, independent)
commit: a22c52ee102a12166e52cd16373d6ec97cd996ce
created_at: 2026-10-17T08:00:25.411591091Z
event:
  event: finding
  recommendation: defer
---

Pre-existing, not introduced here: crates/astromech-exploration/src/value.rs planet_value applies the 500 cr floor before the mapping multiplier (max(500, base) x mult); EDDI Body.cs estimateBodyValue applies it after (Math.Max(500, base x mult)). For mapped planets with base < 500 (k=300 bodies under ~2.3 EM) astromech is high: a 1 EM icy body mapped (already mapped, Odyssey) is 2222 here vs 2121 in EDDI. Scan-only values (the EDSM-checked tests) are unaffected. File a follow-up to settle which order the game uses and align.
