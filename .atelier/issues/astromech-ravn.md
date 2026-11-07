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
closed_at: "2026-11-07T08:01:10.790842446+00:00"
status: "done"
title: "Command layer leftovers: the biology lookup's own client, the dead config.path note, ARCHITECTURE §5.8"
updated_at: "2026-11-07T08:01:10.790842446+00:00"
---

## Goal

The one-owner claims of the command layer are exactly true, and nothing is left that only the old two-owner design needed.

## Done when

- [x] The known-biology lookup (crates/astromech-core/src/session.rs:163) uses the command layer's one `astromech_data::Client`, or ARCHITECTURE names it as the one exception with its reason (a session-only in-memory cache); either way ARCHITECTURE §5.8's "one limiter spaces every lookup" is true as written.
- [x] `Paths::publish_config` / `config_in_use` and the `config.path` note (crates/astromech-store/src/paths.rs:155-180, crates/astromech-core/src/app.rs:41-50) are deleted, with any test that only covered them.
- [x] ARCHITECTURE §5.8 no longer says the core and the window both open `remote_cache.sqlite`.
- [x] `cargo test --workspace`, clippy and check-deps pass.

## Context

Findings astromech-ed64 and astromech-vedx from the astromech-ndzx review.

## Close Reason

Implementer report (commit d82d387d on feature/astromech-bp23). Outcome: (1) The known-biology lookup shares the command layer's one client: SpanshBiology::new takes Arc<Client>, and Session::start hands it api.remote() (session.rs), as the bodies-known callout already did. Its Spansh bodies searches now share the process's limiter and the remote_cache.sqlite cache (ttl::SEARCH, an hour) instead of a session-only in-memory cache with its own limiter. The per-system Background answer cache is unchanged. biology.rs doc updated. Chose sharing over naming an exception: the Done when allowed either, and sharing makes astromech-data's one-client rule true without an exception. (2) Deleted Paths::publish_config, Paths::config_in_use, Published, Note, exe_name (both platforms), the CONFIG_IN_USE const and app.rs choose_config (app::run now takes --config or paths.config_file() inline), with the four store tests that only covered them (the_window_reads_the_config_the_core_published, a_published_config_that_does_not_exist_yet_is_still_the_one_in_use, a_note_left_by_a_core_that_is_gone_is_ignored, dropping_a_publication_leaves_a_newer_one). astromech-store drops its serde, toml and cfg(windows) windows-sys dependencies (Cargo.lock: 3 lines). (3) ARCHITECTURE: §5.8 says two processes opening an old cache file both keep it, and only the core opens it (the window's lookups are the core's commands); it also names the callouts (known biology, EDSM bodies) among the client's users. §18.5 Paths no longer describes config.path. Proof: grep for publish_config, config_in_use and config.path finds nothing outside .atelier records. cargo test -p astromech-store -p astromech-core pass. cargo test --workspace: 1289 passed, 0 failed, 2 ignored. cargo clippy --workspace --all-targets -D warnings clean, cargo fmt --all --check clean, python3 scripts/check-deps.py 'dependency rules OK (32 workspace crates)', cargo deny check all ok. pnpm check, lint and test (73 files, 528 tests) pass. No failing-first test: the changes are a deletion, a docs edit and a constructor argument, and no test starts a Session, so the known-biology wiring is checked only by reading session.rs. Open items: (1) a config.path left in a user's app dir by an older core is never read or removed (harmless). (2) Windows CI compiles astromech-store without windows-sys; nothing else in it used windows-sys (grep). Review outstanding.
