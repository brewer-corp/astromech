---
created_at: "2026-10-17T06:30:54.143156106+00:00"
id: "astromech-71mc"
evidence_type: "test"
commit: "0cffa0ada4af6a7b1a80b819f5f0e4380ae3fcd6"
result: "pass"
captured_at: "2026-10-17T06:30:54.143083580+00:00"
agent_identity: "implementer (Claude Opus)"
target:
  kind: "issue"
  id: "astromech-4kw5"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-4kw5"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "At 0cffa0a: astromech-log test the_shells_lines_land_in_its_log_file (init with SHELL, log a cache-open line, read astromech-shell.log: line present and redacted, the core's file untouched) passes, and fails when the sink writes to the core's file name; rolls_over_and_prunes passes; /tmp/landq/gates.sh passed in full (fmt, clippy -D warnings, cargo test --workspace, check-deps with astromech-log as a leaf, shell Cargo.lock, pnpm lint/check/build/test); shell clippy-clean for x86_64-pc-windows-msvc with stub C tools (not built or run on Windows). The cache-open (Client::shared log), unknown-mod (astromech-odyssey) and core-start messages call astromech_log::log."
updated_at: "2026-10-17T06:30:54.144128640+00:00"
---

At 0cffa0a: astromech-log test the_shells_lines_land_in_its_log_file (init with SHELL, log a cache-open line, read astromech-shell.log: line present and redacted, the core's file untouched) passes, and fails when the sink writes to the core's file name; rolls_over_and_prunes passes; /tmp/landq/gates.sh passed in full (fmt, clippy -D warnings, cargo test --workspace, check-deps with astromech-log as a leaf, shell Cargo.lock, pnpm lint/check/build/test); shell clippy-clean for x86_64-pc-windows-msvc with stub C tools (not built or run on Windows). The cache-open (Client::shared log), unknown-mod (astromech-odyssey) and core-start messages call astromech_log::log.
