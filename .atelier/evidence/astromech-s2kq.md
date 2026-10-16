---
created_at: "2026-10-16T20:51:38.003725141+00:00"
id: "astromech-s2kq"
evidence_type: "validation"
commit: "23f2a9c4e0905476eb240da37d83a693fc1fdb01"
result: "pass"
captured_at: "2026-10-16T20:51:38.003664628+00:00"
target:
  kind: "issue"
  id: "astromech-3tlm"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-3tlm"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Diagnosis of the Windows CI failure (job 117184854096): imports_at_startup_then_follows_new_lines timed out at history.rs:190 waiting for the appended jump; the startup import passed. Cause, from the code: after its first import the follower only re-imported on a notify event (rx.recv()), and the test keeps its append handle open. On Windows, ReadDirectoryChangesW reports size and last-write changes only when NTFS updates the directory entry, which happens lazily for an open file (the same lag as astromech-txzq), so no event arrived. The live app would also have stopped following play whenever Status.json events didn't happen to cover it. Fix at 23f2a9c: a 2 s recheck compares the journal count and the newest journal's length (read through an open handle) with the last import's, and imports on change. Proof on Linux: follows_appends_without_notifications (watcher off, writer kept open) passes, and fails with the recheck disabled (timed out after 20 s, checked). fmt, clippy -D warnings, cargo test --workspace, check-deps and the shell --locked check pass. Windows CI is the remaining confirmation."
updated_at: "2026-10-16T20:51:38.004026153+00:00"
---

Diagnosis of the Windows CI failure (job 117184854096): imports_at_startup_then_follows_new_lines timed out at history.rs:190 waiting for the appended jump; the startup import passed. Cause, from the code: after its first import the follower only re-imported on a notify event (rx.recv()), and the test keeps its append handle open. On Windows, ReadDirectoryChangesW reports size and last-write changes only when NTFS updates the directory entry, which happens lazily for an open file (the same lag as astromech-txzq), so no event arrived. The live app would also have stopped following play whenever Status.json events didn't happen to cover it. Fix at 23f2a9c: a 2 s recheck compares the journal count and the newest journal's length (read through an open handle) with the last import's, and imports on change. Proof on Linux: follows_appends_without_notifications (watcher off, writer kept open) passes, and fails with the recheck disabled (timed out after 20 s, checked). fmt, clippy -D warnings, cargo test --workspace, check-deps and the shell --locked check pass. Windows CI is the remaining confirmation.
