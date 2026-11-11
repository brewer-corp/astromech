---
schema: atelier.item
schema_version: 3
id: astromech-kdca
title: 'Voice: ''Okay, sir'' and ''Yeah, sir'' should confirm like ''Okay'' and ''Yeah'''
status: draft
mission: null
labels:
- voice
priority: low
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-18T00:11:31.847492784Z
---

## Goal

Make "Okay, sir." and "Yeah, sir." confirm a pending question, as "Okay." and "Yeah." alone already do.

## Done when

- [ ] With a "Did you mean …?" or confirmation pending, "Okay sir" and "Yeah sir" resolve as yes; without one they are ignored or "Say again." as today; tests cover both.

## Context

Review finding R3 of astromech-bztt. "sir" is a filler word in the grammar, but the noise-word path (`astromech_intent::is_noise_only`, used by `Assistant::handle`) does not know it, so "Okay sir" skips the noise-word yes and the grammar reduces it to nothing. Fails safe today.
