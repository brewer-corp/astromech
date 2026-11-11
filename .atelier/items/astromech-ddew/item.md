---
schema: atelier.item
schema_version: 3
id: astromech-ddew
title: Listen to the TTS name reading rules
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
  at: 2026-10-16T19:40:10.037540786Z
trivial: null
caused_by: null
created_at: 2026-10-16T19:40:10.037540786Z
---

## Goal

The name reading rules (astromech-w8ai) choose TTS input text, but nobody has heard Kokoro/espeak read it: spelled sector letters (does a lone "A" read as the letter or the article "uh"?), "D 12 dash 39", the ore respellings (Grandidierite, Serendibite, Rhodplumsite, Musgravite, Benitoite), "Gleeza", "Collinder".

## Done when

- [ ] Sumeet has listened to a rendered set of the table examples and each reads right, or the misreads are filed.

## Context

Render with `crates/astromech-voice/examples/render-voices.rs` or the speaking path. Found in review of astromech-w8ai (R1, R2).
