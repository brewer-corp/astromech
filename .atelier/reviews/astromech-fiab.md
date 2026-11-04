---
schema: atelier.review
schema_version: 1
id: astromech-fiab
title: approval on astromech-j75v @f0b3c043
target: astromech-j75v
identity: review-agent-j75v-1
commit: f0b3c043b049eca12671416e984ea34970a40133
created_at: 2026-11-04T22:31:45.044116188Z
event:
  event: approval
---

Re-checked after approval astromech-ei0e went stale. 51120881: astromech-1n9b's two new Done-when bullets match R7 (astromech-temh: non-lookup sentence plus near/from <system> gets Say again, worst cases as tests) and R8 (astromech-5w3d: here-phrasings outside HERE_WORDS search from the current system again). f0b3c043 merge of main: lookup.rs only switches to astromech_announce::credits::credits; ARCHITECTURE 5.12 keeps the Search origin bullet beside the new Everyday questions step 5 (renumbered Say again to 6) with no contradiction: the everyday rule runs inside best_whole, so an everyday head is a no-match that the origin split already discards; voice-commands.md change is the exploration phrase from main. Reran after touching every crate's src/lib.rs: cargo test -p astromech-intent -p astromech-core -p astromech-tools -p astromech-galaxy all pass (a_trailing_system_is_the_lookups_origin, a_named_system_is_the_origin_not_part_of_the_slot, everyday_questions_ask_no_exploration seen running); clippy --all-targets -D warnings clean; check-guide OK.
