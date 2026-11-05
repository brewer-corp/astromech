---
created_at: "2026-11-04T21:22:13.591250196+00:00"
id: "astromech-7v7h"
evidence_type: "test"
commit: "15f6a0bf3b255968ead4ef16f66c72122f9b4ec1"
result: "pass"
captured_at: "2026-11-04T21:22:11.730445279+00:00"
command: "cargo test -p astromech-eddn"
exit_status: "0"
agent_identity: "implement-agent-az6g"
target:
  kind: "issue"
  id: "astromech-az6g"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-az6g"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "cargo test -p astromech-eddn at 15f6a0bf: 151+2 pass. after_an_hour_of_503s_the_queue_drains_in_batches: 201 events after a simulated 60-min 503 outage drain in 10 requests (168 before the fix; the test fails without it). no_request_goes_while_edsms_rate_limit_is_used_up: with X-Rate-Limit-Remaining 0 and Reset 600 nothing is posted until 600 s later (fails without the fix: unexpected request)."
updated_at: "2026-11-04T21:22:13.591641597+00:00"
---

## Summary

cargo test -p astromech-eddn at 15f6a0bf: 151+2 pass. after_an_hour_of_503s_the_queue_drains_in_batches: 201 events after a simulated 60-min 503 outage drain in 10 requests (168 before the fix; the test fails without it). no_request_goes_while_edsms_rate_limit_is_used_up: with X-Rate-Limit-Remaining 0 and Reset 600 nothing is posted until 600 s later (fails without the fix: unexpected request).

## Command

```console
cargo test -p astromech-eddn
```

Exit status: 0

## Stdout

Bytes: 11755
SHA-256: b046930c504051dbf7231d7d2c30f3ae19eb52fecdf72a0745160006ac2a6b53
Test summary: test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

## Stderr

Bytes: 350
SHA-256: 43fde53e2a7b3cf7255fc4bb6e0dccb7985d75b87e786afa80df9242c262943a
