---
schema: atelier.item
schema_version: 3
id: astromech-w0if
title: 'Header: what to say when the core doesn''t answer a status read'
status: ready
mission: astromech-7q0z
labels:
- decision
- needs-human
priority: medium
blocked_by: []
implementers: []
ready:
  by: conversion
  at: 2026-10-22T21:34:32.198775344Z
trivial: null
caused_by: null
created_at: 2026-10-22T21:34:32.198775344Z
---

## Goal

When the window's status read fails, the header says "Not running" while Settings (since astromech-cvxf) keeps the last status with "Could not refresh: the core did not answer." The window can't tell a stopped core from one that didn't answer, so the two disagree about one read. What should the header say?

## Done when

- [ ] The question is answered and the answer is written in this item's Report.

## Context

Review finding R3 (astromech-lbth) on feature/astromech-jo3l; App.svelte:104 versus settings/View.svelte:77,111.

## Options

- Keep "Not running": simplest; it is the one place that shows whether the core is alive, and a core that doesn't answer is, to the user, not working.
- "Not answering" (or similar) after a failed read, "Not running" only when the window knows the core is stopped (no endpoint file, or its process gone).
- Keep the last status with a stale marker, as Settings does.
