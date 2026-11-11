---
schema: atelier.item
schema_version: 3
id: astromech-8i7a
title: 'Guide: quote the unreadable-config message in settings.md'
status: draft
mission: null
labels: []
priority: low
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-17T20:10:39.000723827Z
---

## Goal

Settings → Files shows "The config file can't be read, so every setting is at its default: ..." when astromech.toml is not valid TOML; docs/guide/settings.md quotes only the ignored-values message and the setup line.

## Done when

- [ ] settings.md names the unreadable-file message.
