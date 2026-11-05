---
created_at: "2026-11-04T20:37:14.159531266+00:00"
id: "astromech-c19a"
evidence_type: "test"
commit: "4ba62832ed2517bf121e92630ccf1e9e3169a2a3"
result: "pass"
captured_at: "2026-11-04T20:37:13.700651529+00:00"
command: "cargo test -p astromech-galaxy --test plot --test plot_route"
exit_status: "0"
agent_identity: "orchestrator"
target:
  kind: "issue"
  id: "astromech-6h7t"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-6h7t"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "astromech-galaxy plot tests: the recorded Sol-Colonia neutron route (128 legs) reports 166 jumps; trimmed fixture reports 11 (10+1), not total_jumps 128"
updated_at: "2026-11-04T20:37:14.159814614+00:00"
---

## Summary

astromech-galaxy plot tests: the recorded Sol-Colonia neutron route (128 legs) reports 166 jumps; trimmed fixture reports 11 (10+1), not total_jumps 128

## Command

```console
cargo test -p astromech-galaxy --test plot --test plot_route
```

Exit status: 0

## Stdout

Bytes: 821
SHA-256: 6ddec3a70ea562a2d0e7639a18fc87a508392303515ec3ffcdd3104d92778a6b
Test summary: test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

## Stderr

Bytes: 291
SHA-256: 80a03a68c881071f940f9a70797dd647fff0ad42c859d43e4b1e910863580b3f
