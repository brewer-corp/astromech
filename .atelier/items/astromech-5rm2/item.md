---
schema: atelier.item
schema_version: 3
id: astromech-5rm2
title: Ask Spansh's maintainer about linked-place query volume
status: ready
mission: null
labels:
- needs-human
- size-s
priority: medium
blocked_by: []
implementers: []
ready:
  by: conversion
  at: 2026-11-05T06:19:46.350866500Z
trivial: null
caused_by: null
created_at: 2026-11-05T06:19:46.350866500Z
---

## Goal

Ask Spansh's maintainer whether astromech may run linked-place queries against Spansh's search API at the volume the engine needs (spike astromech-yn6y open item). Asked in parallel: the recipes ship within the spike's budget without waiting for the answer (decision astromech-g8d5, item 9). Each recipe costs 2 to 7 Spansh requests per query at 1 request/s, and recipe 1 moved 14 to 27 MB of station records per query; a local station file built from the public dumps would bring that to about 1 request.

## Done when

- [ ] Spansh's maintainer has answered (or a stated time has passed with no answer), and the answer is recorded as a note here with what it means for the recipes (astromech-sgwi) and the local station file (astromech-ibmy).

## Context

Spike close reason in astromech-yn6y (request counts per recipe, User-Agent, rate limit); the engine, astromech-xlbz, in astromech-os70.

## Links

Relationships of the old record that the new format has no field for.

- related: astromech-os70 (Linked places: engine, recipes, Find near this, Find along route)
- discovered_from: astromech-yn6y (Compound place queries: test two linked-place recipes on real data)
