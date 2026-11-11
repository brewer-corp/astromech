---
schema: atelier.item
schema_version: 3
id: astromech-snqt
title: Walk every screen for v1.0
status: ready
mission: astromech-610m
labels:
- needs-human
- size-s
- validation
priority: high
blocked_by: []
implementers: []
ready:
  by: conversion
  at: 2026-10-17T17:41:24.831155178Z
trivial: null
caused_by: null
created_at: 2026-10-16T18:12:14.997047472Z
---

## Goal

Sumeet walks every screen of the built app and either signs it off or files what still looks rough as new children of the v1.0 UI refinement feature.

## Done when

- [ ] Every page is listed in a note with "ok" or the IDs of the issues filed for it.

## Notes

- 2026-10-17 [note] Windows: the Help link opens an https URL with a #fragment through open_external, which runs explorer.exe <url>. Check on Windows that the browser opens the page at the right heading (PR #113 review R8).
