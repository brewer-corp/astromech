---
created_at: "2026-10-16T19:03:05.144182187+00:00"
id: "astromech-zs2q"
issue_type: "task"
labels:
- "size-s"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
  - kind: "issue"
    id: "astromech-qz9u"
    type: "related"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Share one persistent data client across core and shell"
updated_at: "2026-10-16T19:03:05.144182187+00:00"
---

## Goal

One astromech-data `Client` per process, with a persistent cache, so rate limits and caching hold process-wide and a cache problem never fails a lookup that the network could answer.

## Done when

- The composition root (core, and the shell) owns one `Arc<Client>` with a file-backed `Cache`, and passes it to galaxy via `Galaxy::with_remote` and to every later lookup; no crate builds its own `Client`. A test shows two consumers share the 1 request/s limit and the cache.
- Cache read and write errors are treated as a miss (logged), not a failed lookup, and the cache connection sets a `busy_timeout`. A test with a locked or broken cache still returns the network answer.
- Expired `remote_cache` rows are pruned on put; a test shows the table does not grow past live entries.
- `Error::Schema`'s `Display` is a sentence that reads well spoken; the serde detail stays in the field and the log. Test asserts the spoken text.

## Context

From the independent review of astromech-h74l (R3, R4, R5, R8). Today `Galaxy::with_store` builds its own in-memory `Client`, so the limiter and cache are per `Client`; `Client::fetch_with` propagates `Error::Cache`; `Cache::put` never deletes; `Error::Schema` prints `({detail})`.
