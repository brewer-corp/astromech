---
created_at: "2026-11-07T00:34:42.337772489+00:00"
id: "astromech-jdks"
issue_type: "task"
labels:
- "size-m"
workflow_branch:
  branch_base: main
  work_branch: task/astromech-jdks
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-07T01:10:56.348574473+00:00"
status: "done"
title: "Faster release builds: one workspace, a mock game crate, the version from the tag"
updated_at: "2026-11-07T01:10:56.348574473+00:00"
---

## Goal

A release takes about 20 minutes from tag to published, 17 of them compiling on one Windows runner (alpha.4 tag run 42316659846: core 5.2, mock game 2.3, voice worker 1.9, window and installer 7.5), and a release PR runs the full CI only because the version bump touches Cargo files. Cut the compile time without dropping the smoke tests, and let release notes be reviewed without CI (Sumeet, 2026-11-06).

## Done when

- The mock game is its own small crate (serde, serde_json, windows-sys), not an astromech-core example that builds every dev-dependency; sign-update builds without the dev-dependencies too. ci.yml, criteria.yml, release.yml and the scripts use the new paths.
- The window (app/src-tauri) is a member of the root workspace, with Tauri behind a `window` feature that tauri.conf.json's build.features turns on; plain `cargo test --workspace` on Linux builds a stub and needs no webkit2gtk. One Cargo.lock.
- release.yml builds the core, voice worker and mock game in one cargo invocation.
- The repo version is a dev placeholder that sorts below every release; release.yml stamps the tag's version into every version file before building. A docs-only release-notes branch then runs no CI.
- CONTRIBUTING's local checks pass here; Windows CI and the Release build pass on the PR; the PR's Release build time is recorded against the baseline above.
- CONTRIBUTING and ARCHITECTURE describe the new layout and release steps.

## Context

Discussion on 2026-11-06 after shipping alpha.4 (astromech-ja5a). Rejected: separate repo for the mock game; one runner per binary (more moving parts than it saves at about one release a week).

## Notes

- 2026-11-06 [note] Release build time, both without a cache: PR run 42320563253 at 2a5cad4b took 14.4 min (core+mock game+sign-update 4.0, voice worker 1.5, tauri build 6.8) against the alpha.4 tag run 42316659846's 19.5 min (core 5.2, mock game 2.3, voice worker 1.9, tauri build 7.5). astromech-shell.exe grew 16.6 -> 21.7 MB (its dependencies now build at the workspace's opt-level 3; no_action, astromech-sxey); core and voice worker unchanged. NSIS accepted 0.0.0-dev. Tag builds still start cold: astromech-5skm.

## Close Reason

Window in the root workspace behind a window feature, mock game crate, sign-update bin, one cargo build for core+mock+sign-update, version stamped from the tag (0.0.0-dev in the repo). All 8 PR checks pass; release build 14.4 vs 19.5 min cold; approved (astromech-x1mw).
