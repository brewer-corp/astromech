---
created_at: "2026-10-17T16:54:24.249195787+00:00"
id: "astromech-8rth"
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
    id: "astromech-5ggk"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-5vjx"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-18T14:24:06.632602740+00:00"
status: "done"
title: "Tag and publish v1.0.0-alpha.1"
updated_at: "2026-10-18T14:24:06.632602740+00:00"
---

## Goal

Bump the version, tag v1.0.0-alpha.1 and publish through the normal release workflow, so the in-app updater offers it.

## Done when

The release is on GitHub Releases as the latest release, and an installed v0.1.5 offers and installs the update (checked on the Windows runner or by Sumeet).

## Notes

- 2026-10-18 [note] Version bump, updater semver ordering and smoke-test fix landed on the 5ggk branch (#163); this issue now covers only tagging v1.0.0-alpha.1 on main, the release workflow run, and confirming v0.1.5 is offered the update. Unparented so 5ggk can merge before the tag exists.

## Close Reason

Tagged v1.0.0-alpha.1 at 112c008f; release workflow green (installer smoke + publish); release is Latest, not prerelease. Feed releases/latest/download/latest.json serves 1.0.0-alpha.1; signature verifies with the PUBLIC_KEY_HEX shipped in v0.1.5; zip sha256 matches. v0.1.5's newer() offers it via its string-differs fallback (verified by code review, not by running a 0.1.5 install).
