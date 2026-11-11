---
schema: atelier.item
schema_version: 3
id: astromech-dbza
title: A/B a British Kokoro voice with the British lexicon
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
created_at: 2026-10-16T18:54:29.597211948Z
---

## Goal

Find out whether passing the shipped `lexicon-gb-en.txt` makes a British Kokoro v1.0 voice sound better than espeak-ng-only phonemisation.

## Done when

- [ ] - One British voice (e.g. bf_emma) renders the same few callout lines with and without `lexicon-gb-en.txt`, attached as evidence. - The result (and whether the worker should pass the lexicon) is noted in ARCHITECTURE.md next to the Kokoro TTS worker notes.

## Context

From review of astromech-a50f. The worker passes a language (`en` / `en-us`) and no lexicon, so English is phonemised by espeak-ng alone, unlike Kokoro's own misaki pipeline. Note that in sherpa-onnx 1.13.8 a non-empty language makes the multi-lang front end skip the lexicon entirely (`kokoro-multi-lang-lexicon.cc`, `ConvertNonChineseToTokenIDs`), and with no language set it falls back to the model's `voice` metadata (`en-us`), so the lexicon may be unreachable for English in 1.13.8 (`offline-tts-kokoro-impl.h`, `Generate`). Check that first; if so, record that and what it would take. `examples/render-voices.rs` renders through the real path.

## Links

Relationships of the old record that the new format has no field for.

- related: astromech-a50f (Switch text-to-speech to Kokoro v1.0)
