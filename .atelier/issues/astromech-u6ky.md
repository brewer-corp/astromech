---
created_at: "2026-10-20T14:23:38.697541477+00:00"
id: "astromech-u6ky"
issue_type: "task"
labels:
- "alpha"
- "size-s"
workflow_branch:
  branch_base: main
  work_branch: task/astromech-u6ky
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Point hardcoded GitHub URLs at brewer-corp/astromech"
updated_at: "2026-10-20T14:26:54.421934089+00:00"
---

## Goal

The repo moved from spadaval/astromech to brewer-corp/astromech on 2026-10-20. GitHub redirects the old URLs for now, but the redirect breaks if anyone recreates spadaval/astromech, and the app rejects links that use the new URL.

## Done when

- `grep -rI spadaval/astromech` finds matches only in `.atelier/evidence`, `.atelier/issues` and `.atelier/reviews` (this work's records), past release notes, and the release-notes check, which accepts both owners so those notes still pass.
- The bundle identifier `io.github.spadaval.astromech` stays as it is. Changing it would give the app a new identity and data folder, which would break existing installs.
- An alpha ships with the new feed URL.

## Scope

- `crates/astromech-core/src/update.rs`: the `DEFAULT_FEED` updater URL and the release download URL. Installed alphas update only through the redirect until a build with the new URLs ships.
- `app/src-tauri/src/main.rs` `open_external`: allows only links that start with the old prefix. Allow the new prefix.
- The User-Agent in `crates/astromech-data/src/lib.rs`, the `Cargo.toml` repository fields, README, `docs/`, `app/src/lib/help.ts` and its test, settings View.svelte, and `scripts/*`.
- Leave `.atelier/evidence` alone. It is historical record.
- Never recreate `spadaval/astromech`. alpha.1 and alpha.2 have the old feed compiled in, and alpha.2's `latest.json` has a spadaval download URL, so they update only through GitHub's redirect. A recreated repo would stop their updates. It could not install code, because the manifest is ed25519-signed.
