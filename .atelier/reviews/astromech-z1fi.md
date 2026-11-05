---
schema: atelier.review
schema_version: 1
id: astromech-z1fi
title: finding on astromech-3n7q @d64c832c
target: astromech-3n7q
identity: review-agent (sonnet, independent)
commit: d64c832c1a8e90a5042ad7a4e32cd4a25afcb4b1
created_at: 2026-11-05T00:36:14.223818530Z
event:
  event: finding
  recommendation: defer
---

low: crates/astromech-voice/src/speakable.rs:322 the real Parakeet shapes of procedural names are pinned as unchanged, not read: 'ABC D 14-123' / 'C 22-17' (split mass-code letter, undashed number pair) and merged blocks 'ABCD'/'XOPC' (transcripts.jsonl bf_emma/am_michael/af_heart rows), and 'a B-C' keeps a lone lowercase 'a' (espeak can read it as a word). The new rules cover only 'D14-123' and fully lowercase 'ab-c d14-123', which Parakeet rarely returned (0 of 13 phrases gave 'ab-c'). Done-when is met as written, so not blocking. Suggest a follow-up to read a lone letter then 'NN-NNN' after a 'sector' block as a mass code, and to upper-case a lone 'a' next to a block; production gain is small while heard names rarely match Spansh anyway (Col heard as call/coal).
