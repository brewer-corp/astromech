---
schema: atelier.review
schema_version: 1
id: astromech-7b7k
title: finding on astromech-1o77 @66eb8d47
target: astromech-1o77
identity: review-agent-1o77-1
commit: 66eb8d47ec68d2519ae308605f23070b68573c3f
created_at: 2026-11-04T23:50:37.161845202Z
event:
  event: finding
  recommendation: defer
---

low: crates/astromech-tools/src/lookup.rs:24 capitalising only the first letter is right for the sentence start, and for 'col 285 sector ...' it helps: 'Col' before a number is the case-sensitive reading-rule prefix, so speakable now says 'Collinder 285'. But the rest stays as heard, and the reading rules match case-sensitively (crates/astromech-voice/src/speakable.rs:31 PREFIXES, :216-223 the 'AB-C' block needs capitals): a lowercase heard 'col 285 sector ab-c d1-2' reads 'ab-c' as a word, and 'hip 1234'/'hd 1234'/'lhs 3447' become 'Hip'/'Hd'/'Lhs', unmatched. Pre-existing beyond the first word and outside 9fwe's Done when; worth checking real Parakeet transcripts of procedural names and, if they come lowercase, upper-casing two-letter sector blocks and catalogue prefixes in the heard name (or matching them case-insensitively before a number).
