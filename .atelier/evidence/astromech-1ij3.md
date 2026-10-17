---
created_at: "2026-10-17T04:58:19.904983870+00:00"
id: "astromech-1ij3"
evidence_type: "test"
commit: "20f51b4ed25a84c766d4570c22332834c162d09d"
result: "pass"
captured_at: "2026-10-17T04:58:19.904914179+00:00"
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
title: "At 20f51b4 on task/astromech-ah0i: /tmp/landq/gates.sh passed fresh (cargo fmt --check, clippy -D warnings, cargo test --workspace incl. astromech-eddn 126 tests, check-deps, src-tauri metadata --locked, pnpm lint/check/build/test 168 tests); cargo deny check ok at b449243 (deps unchanged since). Claim proof: edsm::tests::fixture_session_requests_exclude_everything_on_the_discard_list feeds the synthetic session (tests/fixtures/edsm/session.jsonl, 10 listed + 10 unlisted events) with the list unknown at queue time; the courier fetches EDSM's discard list (fixture of the live list, 141 names) before posting and the one request carries exactly the 10 unlisted events in order with commanderName/apiKey/fromSoftware/fromGameVersion/fromGameBuild and the transient fields; the test fails when the send-time filter is removed (checked). Also tested: listed events not queued once the list is known, no key in the queue, nothing sent without the list, off/no key discards the queue, replay not sent, other commander/beta/crew held back, 203 and 503 keep events, per-event codes (101 sent, 304 drops and refetches list, 500 retries, 451 rejects), batches by session and size, key saved only to the keyring (core set_edsm/edsm_status), flight log read (get-logs), pipeline live-only, Settings UI (vitest + headless Chrome screenshot). Live upload to EDSM not done: needs Sumeet's commander and key, filed as astromech-shi5."
updated_at: "2026-10-17T04:58:19.905417601+00:00"
---

At 20f51b4 on task/astromech-ah0i: /tmp/landq/gates.sh passed fresh (cargo fmt --check, clippy -D warnings, cargo test --workspace incl. astromech-eddn 126 tests, check-deps, src-tauri metadata --locked, pnpm lint/check/build/test 168 tests); cargo deny check ok at b449243 (deps unchanged since). Claim proof: edsm::tests::fixture_session_requests_exclude_everything_on_the_discard_list feeds the synthetic session (tests/fixtures/edsm/session.jsonl, 10 listed + 10 unlisted events) with the list unknown at queue time; the courier fetches EDSM's discard list (fixture of the live list, 141 names) before posting and the one request carries exactly the 10 unlisted events in order with commanderName/apiKey/fromSoftware/fromGameVersion/fromGameBuild and the transient fields; the test fails when the send-time filter is removed (checked). Also tested: listed events not queued once the list is known, no key in the queue, nothing sent without the list, off/no key discards the queue, replay not sent, other commander/beta/crew held back, 203 and 503 keep events, per-event codes (101 sent, 304 drops and refetches list, 500 retries, 451 rejects), batches by session and size, key saved only to the keyring (core set_edsm/edsm_status), flight log read (get-logs), pipeline live-only, Settings UI (vitest + headless Chrome screenshot). Live upload to EDSM not done: needs Sumeet's commander and key, filed as astromech-shi5.
