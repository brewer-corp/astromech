---
schema: atelier.item
schema_version: 3
id: astromech-6230
title: Sign Windows releases
status: draft
mission: astromech-610m
labels:
- size-m
priority: low
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-16T18:20:01.765922192Z
---

## Goal

Sign the installer, the core and the voice worker in `release.yml`, most likely through Azure Artifact Signing (see the decision astromech-rqyv). Deferred until there is evidence people use astromech.

## Done when

- [ ] A release built in CI passes `signtool verify /pa` on each signed binary, and the README and V1-DESIGN §7.4 no longer describe the SmartScreen click-through.

## Links

Relationships of the old record that the new format has no field for.

- parent: astromech-huab (M5: v1.0 release)
