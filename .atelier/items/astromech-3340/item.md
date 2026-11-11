---
schema: atelier.item
schema_version: 3
id: astromech-3340
title: Check the height scale and the new orange in the installed app on Windows
status: ready
mission: null
labels:
- needs-human
- loose_end
priority: medium
blocked_by:
- astromech-k6pl
implementers: []
ready:
  by: conversion
  at: 2026-11-10T05:55:46.920933807Z
trivial: null
caused_by: null
created_at: 2026-11-10T05:55:46.920933807Z
---

## Goal

The scale rule (one root zoom from window height) and the stronger orange were built and checked in Chrome on Linux. WebView2 on Sumeet's monitor is the real target.

## Done when

- [ ] On Sumeet's PC, in a release that has astromech-k6pl: text is comfortably readable at his usual window size, the Text size setting changes it, nothing is clipped at the smallest window, and the orange no longer looks washed out.
