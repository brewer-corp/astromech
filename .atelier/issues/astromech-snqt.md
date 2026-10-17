---
created_at: "2026-10-16T18:12:14.997047472+00:00"
id: "astromech-snqt"
issue_type: "validation"
labels:
- "needs-human"
- "size-s"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-610m"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Walk every screen for v1.0"
updated_at: "2026-10-17T17:41:24.831155178+00:00"
---

## Goal

Sumeet walks every screen of the built app and either signs it off or files what still looks rough as new children of the v1.0 UI refinement feature.

## Done when

Every page is listed in a note with "ok" or the IDs of the issues filed for it.

## Notes

- 2026-10-17 [note] Windows: the Help link opens an https URL with a #fragment through open_external, which runs explorer.exe <url>. Check on Windows that the browser opens the page at the right heading (PR #113 review R8).
