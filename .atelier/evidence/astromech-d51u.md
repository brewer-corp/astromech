---
created_at: "2026-10-16T22:10:51.517989096+00:00"
id: "astromech-d51u"
evidence_type: "test"
commit: "864fd9460546ed9f3348dbc43371fcbf5c48e04b"
result: "pass"
captured_at: "2026-10-16T22:10:51.517946437+00:00"
target:
  kind: "issue"
  id: "astromech-2hml"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-2hml"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "confirmation_is_logged_with_latency passed 20 consecutive runs under 3 busy-loop CPU load; fmt, clippy -D warnings, cargo test --workspace, check-deps all pass. Root cause: Runner starts t0 after the key press returns while the fake delay starts inside the press, so measured latency = delay - press time; the 550 ms floor was unsound. Test now asserts 300..1900 ms and that the log line contains the exact returned ms."
updated_at: "2026-10-16T22:10:51.518417605+00:00"
---

confirmation_is_logged_with_latency passed 20 consecutive runs under 3 busy-loop CPU load; fmt, clippy -D warnings, cargo test --workspace, check-deps all pass. Root cause: Runner starts t0 after the key press returns while the fake delay starts inside the press, so measured latency = delay - press time; the 550 ms floor was unsound. Test now asserts 300..1900 ms and that the log line contains the exact returned ms.
