---
schema: atelier.item
schema_version: 3
id: astromech-66pq
title: Check the Powerplay collect and deliver rules in the real game
status: ready
mission: null
labels:
- needs-human
priority: medium
blocked_by: []
implementers: []
ready:
  by: conversion
  at: 2026-10-17T03:04:29.982389821Z
trivial: null
caused_by: null
created_at: 2026-10-17T03:04:29.982389821Z
---

## Goal

Confirm in the live game the Powerplay logistics rules in data/catalog/powerplay-logistics.json (astromech-bf25).

## Done when

- [ ] - Each of these has been checked in game and the data file corrected where it is wrong: - supplies for acquisition are collected only from Fortified systems within 20 ly, or Strongholds within 30 ly; - reinforcement and undermining supply have no distance limit; - undermining data is handed in at any system the power controls; - the refcard caveats, which are only warnings so far (no undermining in a stronghold with a carrier; research and industrial data don't count for reinforcement). - Every `uncertain` flag in the data file is either cleared or kept with a note.
