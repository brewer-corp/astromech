---
schema: atelier.item
schema_version: 3
id: astromech-0wmn
title: 'Button: a flush variant instead of three negative-margin copies of its padding'
status: ready
mission: null
labels:
- size-s
priority: low
blocked_by: []
implementers: []
ready:
  by: conversion
  at: 2026-11-05T06:47:09.663113633Z
trivial: null
caused_by: null
created_at: 2026-11-05T06:47:09.663113633Z
---

## Goal

Pages that need a Button aligned flush with text do it through the component, not by undoing its padding.

## Done when

- [ ] - About, presets and NeutronFile no longer use `calc(-1 * var(--sp-2))` to cancel Button padding; Button offers the needed alignment once (a prop or class), using tokens only. - Before/after screenshots of the three places show no layout change; `pnpm lint && pnpm check && pnpm test` pass.

## Context

Review finding astromech-5yze (reviewer-fable-pey5) on astromech-pey5, deferred.

## Links

Relationships of the old record that the new format has no field for.

- discovered_from: astromech-pey5 (Shared controls and icons per DESIGN.md: input, dropdown, choice, buttons, badge, icon set)
