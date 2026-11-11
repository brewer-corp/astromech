---
schema: atelier.item
schema_version: 3
id: astromech-irrv
title: 'Route page: the title is the plotter'
status: ready
mission: astromech-xajc
labels:
- size-m
priority: high
blocked_by:
- astromech-k6pl
- astromech-mwh5
- astromech-woqv
- astromech-z5oh
implementers: []
ready:
  by: conversion
  at: 2026-11-07T03:00:11.210775156Z
trivial: null
caused_by: null
created_at: 2026-11-07T03:00:11.210775156Z
---

## Goal

The Route page's title is where you plot: from, the destination editable in place, Plotter and Worked out dropdowns (the neutron star file's status, download and check inside), Replot from here, the range-changed line, and the ? and ⋯ menu. The plot side panel, the import box and the on-page help go.

## Done when

- [ ] Plotting, replotting, changing the engine and downloading the neutron star file work from the title in the preview, with their busy and error states.
- [ ] Import, Open on Spansh and Clear work from the ⋯ menu; import opens a dialog.
- [ ] The header's plot popover and the page agree (one plot setup).
- [ ] Route tests updated.

## Links

Relationships of the old record that the new format has no field for.

- parent: astromech-ok29 (Route page redesign: the title is the plotter, the route a timeline)
