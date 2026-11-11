---
schema: atelier.item
schema_version: 3
id: astromech-zwmo
title: 'Colonies lookup warning: reword and cover with copy.rs when spoken'
status: draft
mission: null
labels:
- alpha
priority: low
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-17T19:08:37.726927309Z
---

## Goal

Cover the colonies lookup warnings with the copy guide before a voice consumer exists.

## Done when

- [ ] astromech-galaxy colonies.rs warnings ("couldn't be checked") are reworded without contractions and added to tests/copy.rs when a voice or tool consumer is added.

## Context

Review finding R1 on the fix commit of PR #120 (astromech-yerd): the warning does not reach the speaker today.
