---
created_at: "2026-11-07T05:34:15.184796431+00:00"
id: "astromech-7yb5"
issue_type: "spike"
labels:
- "size-m"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Phonemiser without espeak-ng: misaki-rs with an Elite-name fallback"
updated_at: "2026-11-07T05:34:15.184796431+00:00"
---

## Goal

Show that Kokoro can be phonemised without espeak-ng (the GPL-3.0 part of sherpa-onnx) and still say Elite's words properly.

## Done when

- A prototype turns text into Kokoro phonemes with misaki-rs (espeak feature off), `speakable.rs` first, and a fallback for words misaki doesn't know: a pronunciation list for named systems, stations and factions built from data/, and a rule or small-model guess for the rest.
- A report under docs/evidence/<this id>/ compares it with espeak-ng on the voice-eval phrases plus a list of at least 200 system and station names: how many words fall to the fallback, and Kokoro audio for a listening sample. The listening judgement is Sumeet's (after merge); the stand-in before merge is the eval: Kokoro renders through the new phonemiser, transcribed by Parakeet, scored by `voice-eval`, no worse than with espeak.
- The licence of misaki's dictionary data (us_gold, gb_gold and the silver lists) is found and recorded; if it can't be found, say so and treat it as a blocker.
