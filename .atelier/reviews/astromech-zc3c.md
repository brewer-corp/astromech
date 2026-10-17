---
schema: atelier.review
schema_version: 1
id: astromech-zc3c
title: Re-reviewed fix 0ff7611 for astromech-zf70. The new test opens a real history file, makes one successful read that caches the read-only connection, holds a BEGIN EXCLUSIVE writer, asserts Err(Unreadable), then asserts Ok again after release. Mutating the read() Err arm to Ok(None) makes it fail (checked; restored). Both summary tests pass. Production is WAL, where an open reader is rarely locked out. The test switches to rollback-journal mode to force the lock, but it still drives the same read() error branch that any SQLITE_BUSY or other read error on an open connection reaches, so the path is the production path. Rusqlite is dev-only.
target: astromech-wzol
identity: review-agent (sonnet, independent)
commit: 5b2eab66b0b99549f31883639314999599d1b426
created_at: 2026-10-17T05:47:32.145807443Z
event:
  event: approval
---

Re-reviewed fix 0ff7611 for astromech-zf70. The new test opens a real history file, makes one successful read that caches the read-only connection, holds a BEGIN EXCLUSIVE writer, asserts Err(Unreadable), then asserts Ok again after release. Mutating the read() Err arm to Ok(None) makes it fail (checked; restored). Both summary tests pass. Production is WAL, where an open reader is rarely locked out. The test switches to rollback-journal mode to force the lock, but it still drives the same read() error branch that any SQLITE_BUSY or other read error on an open connection reaches, so the path is the production path. Rusqlite is dev-only.
