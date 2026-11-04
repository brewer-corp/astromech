---
created_at: "2026-11-04T22:18:35.919525847+00:00"
id: "astromech-fcle"
evidence_type: "test"
commit: "854f6b3954261cde186c07bdf8c8e75b15f10690"
result: "pass"
captured_at: "2026-11-04T22:18:33.619720460+00:00"
command: "cargo test -p astromech-eddn"
exit_status: "0"
agent_identity: "implement-agent-7ew2"
target:
  kind: "issue"
  id: "astromech-7ew2"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-7ew2"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "cargo test -p astromech-eddn at 854f6b39: 153+2 pass. a_poisoned_event_in_an_outages_backlog_still_drains_in_batches: 60-min 503 outage, 201 events, one poisoned: 29 recovery requests (77 before the fix, test failed then), every other event stored exactly once, poison given up. after_an_hour_of_503s_the_queue_drains_in_batches: 10 requests, all 201 stored once. edsms_rate_limit_headers_wait_at_most_an_hour: missing/unreadable/negative/huge/Unix-timestamp headers, waits 0/60/3600 s (fails with the 3600 s cap removed). dxv1 tests pass."
updated_at: "2026-11-04T22:18:35.919824575+00:00"
---

## Summary

cargo test -p astromech-eddn at 854f6b39: 153+2 pass. a_poisoned_event_in_an_outages_backlog_still_drains_in_batches: 60-min 503 outage, 201 events, one poisoned: 29 recovery requests (77 before the fix, test failed then), every other event stored exactly once, poison given up. after_an_hour_of_503s_the_queue_drains_in_batches: 10 requests, all 201 stored once. edsms_rate_limit_headers_wait_at_most_an_hour: missing/unreadable/negative/huge/Unix-timestamp headers, waits 0/60/3600 s (fails with the 3600 s cap removed). dxv1 tests pass.

## Command

```console
cargo test -p astromech-eddn
```

Exit status: 0

## Stdout

Bytes: 11914
SHA-256: c0a9d0f44fdff06310601e5a4c6fd44437c71239e470f40eb2646847404865e3
Test summary: test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

## Stderr

Bytes: 455
SHA-256: 8e0544927ce89da7629ad3ea4bec03e53242d7949c72408c88856161ab65a1ac
