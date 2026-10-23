---
created_at: "2026-10-20T18:48:41.225650431+00:00"
id: "astromech-rcui"
issue_type: "task"
labels:
- "alpha"
- "size-s"
workflow_branch:
  branch_base: main
  work_branch: task/astromech-rcui
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-23T01:31:31.057325024+00:00"
status: "done"
title: "Ship an alpha with the brewer-corp updater feed"
updated_at: "2026-10-23T01:31:31.057325024+00:00"
---

## Goal

alpha.1 and alpha.2 reach updates only through GitHub's redirect from spadaval/astromech (see astromech-u6ky). Ship the next alpha with the brewer-corp feed and download URLs, so installs that update to it stop depending on the redirect.

## Done when

- A tagged alpha built from a main that includes astromech-u6ky is published on brewer-corp/astromech.
- Its `latest.json` `url` and the feed it compiles in point at brewer-corp.
- Before tagging: `ASTROMECH_UPDATE_KEY` is present on brewer-corp/astromech (it moved with the transfer on 2026-10-20). Without it, release.yml fails rather than publishing unsigned.

## Notes

- 2026-10-22 [note] 2026-10-22: Sumeet approved publishing an alpha once the current batch of review-leftover PRs (#176, #177, #178 and the neutron increment #174) has merged.
- 2026-10-22 [note] 2026-10-22: release commit 9f376ee0 on task/astromech-rcui is ready for review: version 1.0.0-alpha.3 in Cargo.toml, both Cargo.locks, app/package.json, app/src-tauri/Cargo.toml and tauri.conf.json, README's notes link; docs/releases/v1.0.0-alpha.3.md (check-release-notes: 39 open needs-human IDs, 0 problems). Feed and download URLs compiled in point at brewer-corp (update.rs DEFAULT_FEED and download URL, pack-update.py latest.json url); release.yml publishes to $GITHUB_REPOSITORY; ASTROMECH_UPDATE_KEY is listed on brewer-corp/astromech. Not done: tagging and publishing; the done claim waits for the published alpha.

## Close Reason

v1.0.0-alpha.3 published on brewer-corp/astromech with the brewer-corp feed and download URLs; reviewed (astromech-051q).
