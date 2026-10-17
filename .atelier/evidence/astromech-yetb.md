---
created_at: "2026-10-17T19:08:20.347826451+00:00"
id: "astromech-yetb"
evidence_type: "validation"
commit: "4b7b36756fc3db2833cf407840413af802713d7c"
result: "pass"
captured_at: "2026-10-17T19:08:20.347771208+00:00"
agent_identity: "claude"
target:
  kind: "issue"
  id: "astromech-12uv"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-12uv"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Decision: a short spoken cue 'Test only. Nothing pressed.' follows each tested phrase (the HUD cannot show test mode; a phrase that does nothing otherwise looks like a failure); recorded here and in the voice tester guide page. Tests: status line test (app.rs), routing test a_phrase_follows_the_mode_at_release_not_when_transcription_ends. The session.rs change is Windows-only and is not compiled on this Linux host (CI builds it). Pre-PR checks pass: fmt, clippy, cargo test --workspace, check-deps, check-guide, pnpm lint/check. cargo deny not installed on host."
updated_at: "2026-10-17T19:08:20.348137751+00:00"
---

Decision: a short spoken cue 'Test only. Nothing pressed.' follows each tested phrase (the HUD cannot show test mode; a phrase that does nothing otherwise looks like a failure); recorded here and in the voice tester guide page. Tests: status line test (app.rs), routing test a_phrase_follows_the_mode_at_release_not_when_transcription_ends. The session.rs change is Windows-only and is not compiled on this Linux host (CI builds it). Pre-PR checks pass: fmt, clippy, cargo test --workspace, check-deps, check-guide, pnpm lint/check. cargo deny not installed on host.
