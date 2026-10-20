---
created_at: "2026-10-20T14:23:38.697541477+00:00"
id: "astromech-u6ky"
issue_type: "task"
labels:
- "alpha"
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
title: "Point hardcoded GitHub URLs at brewer-corp/astromech"
updated_at: "2026-10-20T14:23:38.697541477+00:00"
---

## Goal

The repo moved from spadaval/astromech to brewer-corp/astromech on 2026-10-20. GitHub redirects the old URLs for now, but the redirect breaks if anyone recreates spadaval/astromech, and the app rejects links that use the new URL.

## Done when

- `grep -rI spadaval/astromech` finds matches only in `.atelier/evidence` and past release notes.
- An alpha ships with the new feed URL.

## Scope

- `crates/astromech-core/src/update.rs`: the `DEFAULT_FEED` updater URL and the release download URL. Installed alphas update only through the redirect until a build with the new URLs ships.
- `app/src-tauri/src/main.rs` `open_external`: allows only links that start with the old prefix. Allow the new prefix.
- The User-Agent in `crates/astromech-data/src/lib.rs`, the `Cargo.toml` repository fields, README, `docs/`, `app/src/lib/help.ts` and its test, settings View.svelte, and `scripts/*`.
- Leave `.atelier/evidence` alone. It is historical record.
