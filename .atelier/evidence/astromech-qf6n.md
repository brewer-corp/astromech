---
created_at: "2026-10-18T01:17:13.242014246+00:00"
id: "astromech-qf6n"
evidence_type: "test"
commit: "a61f15a212e2d93449cb1efa95c6237238a82b63"
result: "fail"
captured_at: "2026-10-18T01:17:10.168357973+00:00"
command: "cargo test -p astromech-intent -p astromech-core -p astromech-store --locked -q"
exit_status: "101"
agent_identity: "claude (author agent)"
target:
  kind: "issue"
  id: "astromech-1c66"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-1c66"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "intent, core and store tests incl. ask_only_near_misses_are_never_learned, genuine_phrases_are_learned_key_presses_after_two_yes, a_learned_neighbour_does_not_unlock_an_ask_only_phrase, all_stop_fuzzy_near_misses_never_execute"
updated_at: "2026-10-18T01:17:13.242277077+00:00"
---

## Summary

intent, core and store tests incl. ask_only_near_misses_are_never_learned, genuine_phrases_are_learned_key_presses_after_two_yes, a_learned_neighbour_does_not_unlock_an_ask_only_phrase, all_stop_fuzzy_near_misses_never_execute

## Command

```console
cargo test -p astromech-intent -p astromech-core -p astromech-store --locked -q
```

Exit status: 101

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 920
Truncated: no

```text
error[E0425]: cannot find function `is_ask_only` in crate `astromech_intent`
  --> crates/astromech-core/src/assistant.rs:77:24
   |
77 |     !astromech_intent::is_ask_only(heard) && count >= confirmations_needed(command)
   |                        ^^^^^^^^^^^ not found in `astromech_intent`

error[E0425]: cannot find function `is_ask_only` in crate `astromech_intent`
   --> crates/astromech-core/src/assistant.rs:175:97
    |
175 |         if heard.is_empty() || command == "confirm" || command == "cancel" || astromech_intent::is_ask_only(heard) {
    |                                                                                                 ^^^^^^^^^^^ not found in `astromech_intent`

For more information about this error, try `rustc --explain E0425`.
error: could not compile `astromech-core` (lib) due to 2 previous errors
error: could not compile `astromech-core` (lib test) due to 2 previous errors
```

