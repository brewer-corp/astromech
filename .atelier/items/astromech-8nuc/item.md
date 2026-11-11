---
schema: atelier.item
schema_version: 3
id: astromech-8nuc
title: 'Preview neutron plot: the Spansh sample''s jump total matches its legs'
status: ready
mission: null
labels: []
priority: medium
blocked_by: []
implementers: []
ready:
  by: conversion
  at: 2026-11-07T10:46:10.366041869Z
trivial: null
caused_by: null
created_at: 2026-11-07T10:46:10.366041869Z
---

## Goal

A neutron plot in the preview shows a jump total that matches the route strip.

## Done when

- [ ] app/src/preview/fixtures/plot.json's Spansh neutron sample is a whole recorded reply (or a trimmed one re-dumped so its total and its legs agree), and the preview's plot page and route strip show the same number of jumps (`scripts/ui` screenshot).

## Context

Finding astromech-qcbo from the astromech-96dv review: the trimmed sample says 128 jumps, its legs give 11. Fixtures are recorded raw inputs (decision astromech-1zhg).

## Links

Relationships of the old record that the new format has no field for.

- discovered_from review: astromech-qcbo
