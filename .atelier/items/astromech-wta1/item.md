---
schema: atelier.item
schema_version: 3
id: astromech-wta1
title: Compare the unsold data estimate with real sales
status: ready
mission: astromech-7q0z
labels:
- needs-human
- validation
priority: medium
blocked_by: []
implementers: []
ready:
  by: conversion
  at: 2026-10-17T05:19:41.315725512Z
trivial: null
caused_by: null
created_at: 2026-10-17T05:19:41.315725512Z
---

## Goal

The Exploration page's unsold cartographic estimate has never been compared with real sales; it may overcount the arrival star scanned again on each return to an already-sold system, and nav beacon scans (finding astromech-lopq on astromech-tzc2). Sumeet: after an exploration trip, sell the data and compare the page's estimate per system with the sale's BaseValue plus Bonus in the journal.

## Done when

- [ ] The estimate is within a few percent of the sale for each sold system, or the differences are filed as bugs.
