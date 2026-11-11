---
schema: atelier.item
schema_version: 3
id: astromech-5skm
title: 'Release builds start with a cold cache: save the release cache from main'
status: ready
mission: null
labels:
- size-s
priority: medium
blocked_by: []
implementers: []
ready:
  by: conversion
  at: 2026-11-07T00:48:43.854501772Z
trivial: null
caused_by: null
created_at: 2026-11-07T00:48:43.854501772Z
---

## Goal

A tag build can restore only caches saved on its own ref or on main, and release.yml's `release` cache is saved only from PR branches, so every release compiles from scratch (the alpha.4 baseline's 17 minutes). Let tag builds start warm.

## Done when

- [ ] A tag build restores a cache saved from main (its Swatinem/rust-cache step reports a hit), and the next release's build time is recorded against astromech-jdks's measurements.

## Context

Options: a release-profile build on pushes to main that touch Cargo files (costs Windows minutes per push), or a scheduled or manual cache-warming run. Finding astromech-swp9 on astromech-jdks.

## Links

Relationships of the old record that the new format has no field for.

- discovered_from review: astromech-swp9
