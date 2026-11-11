---
schema: atelier.item
schema_version: 3
id: astromech-xj3u
title: check-release-notes.py reads the deleted .atelier/issues, so it finds no open needs-human item
status: draft
mission: null
labels:
- bug
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: 775b8544
created_at: 2026-11-11T18:29:44.517188725Z
---

## Goal

`scripts/check-release-notes.py`, step 1 of Releasing in CONTRIBUTING.md, checks that every open `needs-human` item is a test step in the release notes once. It reads `.atelier/issues/*.md`, which the Atelier cutover (775b8544) moved to `.atelier/items/<id>/item.md` in a new front-matter form. On `docs/releases/v1.0.0-alpha.4.md` it now finds 0 open needs-human items and reports 41 problems; at 94aa144f, before the cutover, it found 45 and reported 4. Make it read the records where they are now.

## Done when

- [ ] The script reads the open `needs-human` items from `.atelier/items/*/item.md` (status neither `done` nor `dropped`, label `needs-human`), and knows every item ID there.
- [ ] On `docs/releases/v1.0.0-alpha.4.md` it counts the same open needs-human items `atelier list --label needs-human` shows.
- [ ] A test covers the record parsing, run by CI beside the script's siblings.

## Scope

In: `scripts/check-release-notes.py` and a test for it.

Out: the release notes' content, and the Release build workflow.

Found by the gardener run astromech-qjle.
