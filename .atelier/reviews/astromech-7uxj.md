---
schema: atelier.review
schema_version: 1
id: astromech-7uxj
title: 'tests/values.rs undiscovered_but_mapped_bodies_get_no_discovery_bonuses: the closing star assertion calls star_value("G", Some(1.0), true), which takes a bare bool and never sees the bubble state, so it restates an existing test rather than proving stars keep the bonus via body_value. Harmless; stars'' path (body_value -> star_value with terms.first_discovery) is unchanged by the diff.'
target: astromech-e8up
identity: review-agent (opus, independent)
commit: a22c52ee102a12166e52cd16373d6ec97cd996ce
created_at: 2026-10-17T08:00:25.597274543Z
event:
  event: finding
  recommendation: no_action
---

tests/values.rs undiscovered_but_mapped_bodies_get_no_discovery_bonuses: the closing star assertion calls star_value("G", Some(1.0), true), which takes a bare bool and never sees the bubble state, so it restates an existing test rather than proving stars keep the bonus via body_value. Harmless; stars' path (body_value -> star_value with terms.first_discovery) is unchanged by the diff.
