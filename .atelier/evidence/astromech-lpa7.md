---
created_at: "2026-10-24T23:26:02.596067022+00:00"
id: "astromech-lpa7"
evidence_type: "test"
commit: "320366a080454026ae646dbcd67f1d616aa2eaee"
result: "pass"
captured_at: "2026-10-24T23:25:59.100137673+00:00"
command: "sh spikes/q1-linked-places/replay.sh"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-yn6y"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-yn6y"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Both recipes, the fan-out comparisons and the hotspot coverage replay from the recorded cache with --offline (no live requests); join unit tests pass"
updated_at: "2026-10-24T23:26:02.596358996+00:00"
---

## Summary

Both recipes, the fan-out comparisons and the hotspot coverage replay from the recorded cache with --offline (no live requests); join unit tests pass

## Command

```console
sh spikes/q1-linked-places/replay.sh
```

Exit status: 0

## Stdout

Bytes: 35210
SHA-256: 8906d03dd8abef72dcc3c3969adc27fcfb258bcd00cd8d4528a82a0b368703f5
Test summary: test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

## Stderr

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
