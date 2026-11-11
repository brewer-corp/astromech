---
schema: atelier.item
schema_version: 3
id: astromech-9lv6
title: Does the galaxy map react to UI actions on V or Enter with the search box unfocused?
status: draft
mission: null
labels:
- needs-human
priority: low
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-17T18:43:58.065481324Z
---

## Goal

Find out whether the galaxy map reacts to UI_* (Interface) bindings, such as UI_Select on Enter, while the search box is not focused, and extend the paste_key health warning if it does.

## Done when

- [ ] The answer is recorded in docs/PLOTTING.md (verified in game) and the health rule covers the actions that matter, or states that they are exempt.

## Context

Review finding R2 on PR #116 (astromech-2za5). Needs the game, so likely needs-human.
