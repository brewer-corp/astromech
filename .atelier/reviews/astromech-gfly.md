---
schema: atelier.review
schema_version: 1
id: astromech-gfly
title: 'low: crates/astromech-data/src/spansh.rs:209 the doc line ''Systems controlled by any of powers.'' was left stranded above ring_signal, so ring_signal''s rustdoc is wrong and controlling_power lost its doc. Move the line back down.'
target: astromech-mcxc
identity: review-agent (sonnet, independent)
commit: ef20ef1e2014af6bbac9e86137c2465510972504
created_at: 2026-10-17T02:55:37.565379148Z
event:
  event: finding
  recommendation: defer
---

low: crates/astromech-data/src/spansh.rs:209 the doc line 'Systems controlled by any of powers.' was left stranded above ring_signal, so ring_signal's rustdoc is wrong and controlling_power lost its doc. Move the line back down.
