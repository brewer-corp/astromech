---
schema: atelier.item
schema_version: 3
id: astromech-7yb5
title: 'Phonemiser without espeak-ng: misaki-rs with an Elite-name fallback'
status: ready
mission: null
labels:
- size-m
- spike
priority: medium
blocked_by: []
implementers: []
ready:
  by: conversion
  at: 2026-11-07T05:34:15.184796431Z
trivial: null
caused_by: null
created_at: 2026-11-07T05:34:15.184796431Z
---

## Goal

Show that Kokoro can be phonemised without espeak-ng (the GPL-3.0 part of sherpa-onnx) and still say Elite's words properly.

## Done when

- [ ] - A prototype turns text into Kokoro phonemes with misaki-rs (espeak feature off), `speakable.rs` first, and a fallback for words misaki doesn't know: a pronunciation list for named systems, stations and factions built from data/, and a rule or small-model guess for the rest. - A report under docs/evidence/<this id>/ compares it with espeak-ng on the voice-eval phrases plus a list of at least 200 system and station names: how many words fall to the fallback, and Kokoro audio for a listening sample. The listening judgement is Sumeet's (after merge); the stand-in before merge is the eval: Kokoro renders through the new phonemiser, transcribed by Parakeet, scored by `voice-eval`, no worse than with espeak. - The licence of misaki's dictionary data (us_gold, gb_gold and the silver lists) is found and recorded; if it can't be found, say so and treat it as a blocker.

## Links

Relationships of the old record that the new format has no field for.

- parent: astromech-v6r3 (Voice stack without sherpa-onnx)
