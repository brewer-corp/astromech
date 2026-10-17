---
schema: atelier.review
schema_version: 1
id: astromech-ss47
title: '[low] exobiology: species_value returns a name that no production code uses (only unsold.rs, which discards it), and data/catalog/exobiology.json has ''Bacterium Tela '' with a trailing space. Drop the name from the return or trim the data when the catalog is next touched.'
target: astromech-wu30
identity: 'review-agent (opus, independent, part A: values and data)'
commit: 8bfc4e716e943e3ce43a826fbe938883ac639b0a
created_at: 2026-10-17T05:18:01.750348613Z
event:
  event: finding
  recommendation: defer
---

[low] exobiology: species_value returns a name that no production code uses (only unsold.rs, which discards it), and data/catalog/exobiology.json has 'Bacterium Tela ' with a trailing space. Drop the name from the return or trim the data when the catalog is next touched.
