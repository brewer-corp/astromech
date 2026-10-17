---
created_at: "2026-10-17T05:05:54.956818764+00:00"
id: "astromech-7kfs"
evidence_type: "test"
commit: "d76608aad606e3b406813d96c24be0ef2b08f10b"
result: "pass"
captured_at: "2026-10-17T05:05:54.956780774+00:00"
agent_identity: "implementer (Claude Opus)"
target:
  kind: "issue"
  id: "astromech-ah0i"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-ah0i"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Fix for review finding astromech-yw5v at d76608aad606e3b406813d96c24be0ef2b08f10b: EDSM HTTP errors are built from ureq's error kind and cause, never the URL (the flight-log GET carries apiKey in its query), and flight_log hides the key in anything it returns. edsm::api::tests::a_failed_flight_log_request_never_shows_the_key (closed local port, GET and POST) failed before the fix with the URL and key in the error, passes after; a_flight_log_error_says_what_edsm_said also checks an error quoting the key has it hidden. /tmp/landq/gates.sh passed fresh at that commit (exit 0; cargo test --workspace, vitest 168 tests)."
updated_at: "2026-10-17T05:05:54.957080534+00:00"
---

Fix for review finding astromech-yw5v at d76608aad606e3b406813d96c24be0ef2b08f10b: EDSM HTTP errors are built from ureq's error kind and cause, never the URL (the flight-log GET carries apiKey in its query), and flight_log hides the key in anything it returns. edsm::api::tests::a_failed_flight_log_request_never_shows_the_key (closed local port, GET and POST) failed before the fix with the URL and key in the error, passes after; a_flight_log_error_says_what_edsm_said also checks an error quoting the key has it hidden. /tmp/landq/gates.sh passed fresh at that commit (exit 0; cargo test --workspace, vitest 168 tests).
