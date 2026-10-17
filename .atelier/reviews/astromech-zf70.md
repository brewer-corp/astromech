---
schema: atelier.review
schema_version: 1
id: astromech-zf70
title: 'R1 [medium] crates/astromech-core/src/summary.rs tests: the done-when asks for a test with a locked history database, but neither test locks the database. The held Mutex covers the in-process try_lock path and the corrupt file fails inside open_read_only (PRAGMA user_version), so the read() Err branch, which is where SQLITE_BUSY on an already-open connection surfaces (the issue''s ''busy database'' case), is untested: mutating that branch from Err(Unreadable) back to Ok(None) leaves every test green (checked). WAL does not stop a real lock: a writer connection with PRAGMA locking_mode=EXCLUSIVE that has committed a write makes readers fail with ''database is locked'' (confirmed with sqlite). Fix: create a real history file with History::open, make one successful session() call so the read-only connection is cached, then hold an exclusive-locking-mode writer and assert Err(Unreadable) (and Ok again once released).'
target: astromech-wzol
identity: review-agent (opus, independent)
commit: ddb07e2113b4f35429b4528808622c209f42b9be
created_at: 2026-10-17T05:35:24.127433746Z
event:
  event: finding
  recommendation: fix_now
---

R1 [medium] crates/astromech-core/src/summary.rs tests: the done-when asks for a test with a locked history database, but neither test locks the database. The held Mutex covers the in-process try_lock path and the corrupt file fails inside open_read_only (PRAGMA user_version), so the read() Err branch, which is where SQLITE_BUSY on an already-open connection surfaces (the issue's 'busy database' case), is untested: mutating that branch from Err(Unreadable) back to Ok(None) leaves every test green (checked). WAL does not stop a real lock: a writer connection with PRAGMA locking_mode=EXCLUSIVE that has committed a write makes readers fail with 'database is locked' (confirmed with sqlite). Fix: create a real history file with History::open, make one successful session() call so the read-only connection is cached, then hold an exclusive-locking-mode writer and assert Err(Unreadable) (and Ok again once released).
