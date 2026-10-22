---
created_at: "2026-10-22T22:11:51.946246878+00:00"
id: "astromech-l5r3"
evidence_type: "test"
commit: "21e7add372f503d5323f41c13760c817f4f152c9"
result: "pass"
captured_at: "2026-10-22T22:11:20.656310460+00:00"
command: "cargo test -p astromech-voice speakable"
exit_status: "0"
agent_identity: "claude-implementer-uuqt"
target:
  kind: "issue"
  id: "astromech-sdg7"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-sdg7"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "speakable reading_rules_table passes with the new negative rows (2:30, x2, Krait Mk II, T-10, Type-9, FSD, OK, bare Cr, 5 m, 1 t, Deck 2 K, Col. Smith, HD video, Prua Phoe XX-Y e1-2345, dates, non-ASCII) and ore rows; 10 of these rows fail against the old rules; copy.rs still passes. Ore respellings chosen by a Kokoro->Parakeet round trip (bf_emma, bm_george)."
updated_at: "2026-10-22T22:11:51.949525668+00:00"
---

## Summary

speakable reading_rules_table passes with the new negative rows (2:30, x2, Krait Mk II, T-10, Type-9, FSD, OK, bare Cr, 5 m, 1 t, Deck 2 K, Col. Smith, HD video, Prua Phoe XX-Y e1-2345, dates, non-ASCII) and ore rows; 10 of these rows fail against the old rules; copy.rs still passes. Ore respellings chosen by a Kokoro->Parakeet round trip (bf_emma, bm_george).

## Command

```console
cargo test -p astromech-voice speakable
```

Exit status: 0

## Stdout

Bytes: 163
Truncated: no

```text

running 1 test
test speakable::tests::reading_rules_table ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 29 filtered out; finished in 0.00s
```

## Stderr

Bytes: 341
Truncated: no

```text
cargo-gate: 2 builds already running, waiting for a slot...
   Compiling astromech-voice v1.0.0-alpha.2 (/home/supadava/projects/astromech-wt/astromech-uuqt/crates/astromech-voice)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.77s
     Running unittests src/lib.rs (target/debug/deps/astromech_voice-cd717c78e37ccda7)
```

