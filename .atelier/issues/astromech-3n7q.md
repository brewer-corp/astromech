---
created_at: "2026-11-04T23:51:08.425564470+00:00"
id: "astromech-3n7q"
issue_type: "task"
labels:
- "size-s"
workflow_branch:
  branch_base: main
  work_branch: task/astromech-3n7q
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-05T00:57:30.561325096+00:00"
status: "done"
title: "Voice origins: real systems beside NOT_A_NAME, and lowercase procedural names"
updated_at: "2026-11-05T00:57:30.561325096+00:00"
---

## Goal

A spoken origin place is read as a system only when it is one, and procedural names read right whatever case speech recognition returns.

## Done when

- A place containing 'bulk', 'quantity' or 'quantities' anywhere ('in small quantities') is no system name, with a test (R1, astromech-ud7x).
- A place is checked against the known-system slot values before NOT_A_NAME applies, so 'near Any Na', 'A Bootis' and 'MY Apodis' search from those systems, with a test (R2, astromech-hn1c).
- The TTS reading rules handle a lowercase procedural name ('col 285 sector ab-c d14-123', 'hip 12345'), checked against real Parakeet transcripts first (R3, astromech-7b7k).

## Context

Deferred review findings on task/astromech-1o77: crates/astromech-intent/src/lib.rs:495 and :535, crates/astromech-tools/src/lookup.rs:24, crates/astromech-voice/src/speakable.rs:31 and :216-223.

## Notes

- 2026-11-04 [note] Implementer report (554e42c7). R1: QUANTITY words (bulk, quantity, quantities) anywhere in the place mean here; 'large' stays a first word in NOT_A_NAME. Spansh dump scan: no name contains those words. R2: split_origin takes a known-system check (exact {system} slot phrasing) that overrides NOT_A_NAME and QUANTITY; names_origin is now Grammar::names_origin (assistant.rs learn uses it). R3: speakable PREFIXES match any case before a number; the 'AB-C' block and the mass code (a-h or A-H) match any case and are written upper-case. Docs: ARCHITECTURE.md search-origin bullet, guide lookups.md. Proof: new test a_known_system_is_the_origin_even_when_it_starts_like_no_name and new rows in a_system_after_in_or_around_... and reading_rules_table; all three failed before the change (seen) and pass after. cargo fmt --check, clippy -D warnings (intent, voice, core), cargo test -p astromech-intent -p astromech-voice -p astromech-core, check-deps.py, check-guide.py all pass. Parakeet transcripts (synthetic Kokoro, voices af_heart/am_michael/bf_emma/bm_george, clean audio, 4 Nov 2026): 'Col 285 Sector AB-C d14-123' -> 'call 285 sector ABC D 14 dash 123', 'Col 285 Sector, ABCD 14-123', 'coal 285 sector, ABC D 14-123', 'coal 285 sector ABC D 14 dash 123'; spelled 'A B dash C, D fourteen dash one twenty-three' -> 'a b dash c d 14 dash 123', 'a B-C, D14-123', 'A B dash C D 14 dash 123', 'Col 285 sector a B-C D14-123'; 'Collinder 285' -> 'colander/Colinder/Colander 285'; 'HIP 12345' and 'H I P one two three four five' -> 'HIP 12345' (8/8, once 'HIP 12,345'); 'HD 1234' -> 'HD 1234' or 'HD one thousand two hundred thirty four'; 'LHS 3447' -> 'LHS 3447' or 'LHS three thousand four hundred (and) forty seven'; 'Synuefe XO-P c22-17' -> 'Sinoof, XOP, C 22-17', 'Synuf XOPC 22-17'; 'Any Na' -> 'any Na', 'any knob', 'any na', 'any nah'; 'A Bootis' -> 'a booties' (4/4); 'MY Apodis' -> 'my apotes/apothees/apodes/apodies'; 'in small quantities' as said. So Parakeet returns catalogue prefixes upper-case and never gave 'ab-c'; the lowercase forms it does give are 'a B-C' / 'a b dash c' and an upper-case mass code 'D14-123', which the old rules missed. Open: production {system} slot is only 'Sol' (assistant.rs grammar()), so R2 helps only once those systems are in the slot; Parakeet hears 'A Bootis'/'MY Apodis' as 'a booties'/'my apotes', not exact, so they still mean here; Parakeet mishears 'Col' as 'call'/'coal' and merges the mass-code letter into the block ('ABCD 14-123', 'XOPC 22-17'), so heard procedural names rarely match Spansh exactly; number words ('three thousand four hundred forty seven') are not folded to digits. Review outstanding.

## Close Reason

Implementer report. Changed:
- R1 (ud7x): 'bulk', 'quantity' or 'quantities' anywhere in a place marks it as an amount, not a system.
- R2 (hn1c): a place that exactly matches a {system} slot value is a system even when it starts with a NOT_A_NAME word. names_origin is now a Grammar method, and assistant.rs uses it.
- R3 (7b7k): the TTS reading rules match catalogue prefixes, the sector block and the mass code in any case, and write them in capitals.
- ARCHITECTURE.md and the lookups guide page are updated.

Proof:
- cargo test for intent, voice and core, plus fmt, clippy, check-deps and check-guide, all clean. Evidence astromech-wetd after merging main.
- Parakeet transcripts of 13 synthetic Kokoro phrases are in the implementer note. Parakeet returns catalogue prefixes in capitals, and never 'ab-c' or 'hip'.
- PR #203 CI green.

Review: approval astromech-x4av. Finding astromech-z1fi is deferred to astromech-gah6.

Deviations: none. Surprise: the real procedural-name shapes differ from those Done when names.

Open items (in astromech-gah6): the live {system} slot holds only Sol, so R2 has no effect in the live app yet; Parakeet turns 'A Bootis' into 'a booties'.
