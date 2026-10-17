---
schema: atelier.review
schema_version: 1
id: astromech-hcqc
title: '[info] value.rs applies the 500 cr planet floor before the mapping multiplier, as EDDiscovery''s EstimatedValues.cs PlanetValue33 does. EDDI''s Body.cs applies it after the multiplier. The sources disagree, and it only changes tiny mapped bodies, by under 1000 cr. Every other constant and step matches both sources: k values, terraform bonuses, q, 66.25, multipliers 3.699622554/8.0956/10/3, Odyssey max(30%,555), x1.25 efficiency, x2.6 first discovery.'
target: astromech-wu30
identity: 'review-agent (opus, independent, part A: values and data)'
commit: 8bfc4e716e943e3ce43a826fbe938883ac639b0a
created_at: 2026-10-17T05:18:01.809740533Z
event:
  event: finding
  recommendation: no_action
---

[info] value.rs applies the 500 cr planet floor before the mapping multiplier, as EDDiscovery's EstimatedValues.cs PlanetValue33 does. EDDI's Body.cs applies it after the multiplier. The sources disagree, and it only changes tiny mapped bodies, by under 1000 cr. Every other constant and step matches both sources: k values, terraform bonuses, q, 66.25, multipliers 3.699622554/8.0956/10/3, Odyssey max(30%,555), x1.25 efficiency, x2.6 first discovery.
