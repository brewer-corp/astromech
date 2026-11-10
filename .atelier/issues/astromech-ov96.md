---
created_at: "2026-11-10T06:39:42.065916391+00:00"
id: "astromech-ov96"
issue_type: "bug"
labels:
- "size-s"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "cargo-deny fails every PR: bundled SQLite in libsqlite3-sys 0.38.2 has RUSTSEC-2026-0459"
updated_at: "2026-11-10T06:39:42.065916391+00:00"
---

## Goal

CI's cargo-deny step passes again. Since the advisory RUSTSEC-2026-0459 (CVE-2026-119375) was published, the Linux job fails on every branch, including ones that change no Rust: first seen on PR 226 (mockups only) on 10 Nov 2026. SQLite before 3.54.0 can return uninitialised page-cache memory when a database is opened beside a -wal file with a different page size; libsqlite3-sys bundles it, and the workspace has libsqlite3-sys 0.38.2 through rusqlite 0.40.2.

## Done when

- [ ] The workspace resolves libsqlite3-sys 0.39.0 or later (a rusqlite release that depends on it), and cargo deny check passes with no new ignore.
- [ ] cargo test --workspace and cargo clippy --workspace --all-targets -- -D warnings pass.

## Context

Exposure is low: only a process that opens a database and -wal a less trusted party wrote is affected, and astromech opens its own files. The check failing on every PR is the cost. If no rusqlite release takes the fixed libsqlite3-sys yet, a dated ignore in deny.toml with this issue's id is the stopgap, decided by whoever picks this up.
