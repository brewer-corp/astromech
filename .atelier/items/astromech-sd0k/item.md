---
schema: atelier.item
schema_version: 3
id: astromech-sd0k
title: Which exobiology value wins where sources disagree?
status: ready
mission: astromech-7q0z
labels:
- decision
- needs-human
priority: low
blocked_by: []
implementers: []
ready:
  by: conversion
  at: 2026-10-17T06:33:31.615337226Z
trivial: null
caused_by: null
created_at: 2026-10-17T06:33:31.615337226Z
---

## Goal

data/catalog/exobiology.json lists value disagreements under `uncertain`. Fonticulua Fluctus was settled at 20,000,000 (the wiki's 16,777,215 is 2^24-1). Still open: Concha Biconcavis (wiki 16,777,215, also 2^24-1; EDR 19,010,800) and Bacterium Nebulus / Scopulum (wiki 9.1 M / 8.6 M; EDR and Spansh about 5.3 M / 4.9 M).

## Done when

- [ ] The question is answered and the answer is written in this item's Report.

## Context

The values feed the unsold organic total and the biology summary; a wrong value misleads which body to sample.

## Options

- Wiki values throughout, except obvious placeholders such as 2^24-1.
- Check in game: sell a sample of each and read the journal's SellOrganicData value.
