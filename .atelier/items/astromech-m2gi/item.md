---
schema: atelier.item
schema_version: 3
id: astromech-m2gi
title: Run scripts/check-release-notes.py in CI and on the publish job, after fixing its mtime default
status: draft
mission: null
labels: []
priority: low
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-18T01:52:06.557468457Z
---

## Goal

The release-notes coverage check (scripts/check-release-notes.py, astromech-yuuy) picks its default file by modification time, which is arbitrary on a fresh checkout. Pass docs/releases/$GITHUB_REF_NAME.md explicitly (or pick by version), then run it in CI and before a tag publishes.

## Done when

- [ ] The script runs on PRs that touch docs/releases or .atelier/issues, and the publish job runs it on the tag's notes file; with an ID removed from the notes it fails.
