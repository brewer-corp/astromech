---
created_at: "2026-11-07T07:43:20.110247675+00:00"
id: "astromech-ravn"
issue_type: "task"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-ndzx"
    type: "blocked_by"
  - kind: "review"
    id: "astromech-ed64"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Command layer leftovers: the biology lookup's own client, the dead config.path note, ARCHITECTURE §5.8"
updated_at: "2026-11-07T07:44:01.976354269+00:00"
---

## Goal

The one-owner claims of the command layer are exactly true, and nothing is left that only the old two-owner design needed.

## Done when

- [ ] The known-biology lookup (crates/astromech-core/src/session.rs:163) uses the command layer's one `astromech_data::Client`, or ARCHITECTURE names it as the one exception with its reason (a session-only in-memory cache); either way ARCHITECTURE §5.8's "one limiter spaces every lookup" is true as written.
- [ ] `Paths::publish_config` / `config_in_use` and the `config.path` note (crates/astromech-store/src/paths.rs:155-180, crates/astromech-core/src/app.rs:41-50) are deleted, with any test that only covered them.
- [ ] ARCHITECTURE §5.8 no longer says the core and the window both open `remote_cache.sqlite`.
- [ ] `cargo test --workspace`, clippy and check-deps pass.

## Context

Findings astromech-ed64 and astromech-vedx from the astromech-ndzx review.
