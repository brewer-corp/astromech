---
schema: atelier.item
schema_version: 3
id: astromech-x6hc
title: Link ts-rs's export code only into tests and the bindings generator
status: draft
mission: null
labels: []
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-07T09:15:19.157272254Z
---

## Goal

The shipped core does not carry ts-rs's export code for every command type.

## Done when

- [ ] `astromech_api::Ty` and `add_request`'s type pointers are behind `cfg(any(test, feature = "bindings"))` (or equivalent), the bindings tests still pass, and a release build of astromech-core no longer links ts-rs's export functions.

## Context

Finding astromech-cn58 from the astromech-se44 review: binary size only, no behaviour change.

## Links

Relationships of the old record that the new format has no field for.

- discovered_from review: astromech-cn58
