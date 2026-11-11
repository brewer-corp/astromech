---
schema: atelier.item
schema_version: 3
id: astromech-gah6
title: 'Voice origins: procedural-name shapes Parakeet really returns, and real systems in the {system} slot'
status: draft
mission: null
labels:
- size-s
priority: low
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-05T00:37:17.784210853Z
---

## Goal

A spoken procedural or NOT_A_NAME-looking system name searches from that system in the live app, in the forms Parakeet actually returns.

## Done when

- [ ] - The TTS reading rules read the shapes Parakeet returned on astromech-3n7q's synthetic transcripts: a split mass-code letter with an undashed pair ('ABC D 14-123', 'C 22-17'), merged blocks ('ABCD', 'XOPC'), and a lone lowercase 'a' next to a block ('a B-C'), with test rows (R1, astromech-z1fi). - The live `{system}` slot holds more than 'Sol' (`lookups::grammar(&["Sol"])` in crates/astromech-core/src/assistant.rs), for example recent and nearby systems, so the astromech-3n7q check ('near Any Na') works outside tests, or it is decided that it shouldn't.

## Context

Transcripts: astromech-3n7q's implementer note. Parakeet hears 'A Bootis' as 'a booties' and 'MY Apodis' as 'my apotes', so an exact slot match won't catch those; clju (n-best rescoring) is the larger fix.

## Links

Relationships of the old record that the new format has no field for.

- discovered_from review: astromech-z1fi
