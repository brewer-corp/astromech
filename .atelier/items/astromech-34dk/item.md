---
schema: atelier.item
schema_version: 3
id: astromech-34dk
title: 'Tray Check for updates: say the result when nothing is found or the check fails'
status: draft
mission: null
labels: []
priority: low
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-17T19:46:37.537080741Z
---

## Goal

A check started from the tray (astromech-naa3) only reverts the menu line to "Check for updates"; with no update, or on an error, nothing says so. Settings shows the result.

## Done when

- [ ] The tray (or a toast) reports up to date or the error after a requested check.
