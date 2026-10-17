---
created_at: "2026-10-16T19:13:12.726110806+00:00"
id: "astromech-ww05"
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
    id: "astromech-7q0z"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Click Coriolis and EDSY links in the Windows app"
updated_at: "2026-10-17T05:00:27.647524127+00:00"
---

## Goal

Confirm the Coriolis and EDSY links built from the current loadout survive the Windows shell launch (open_external -> explorer.exe).

## Done when

In the Windows app, one Coriolis and one EDSY link from the Builds page open the right build in the default browser; the long URL, including the EDSY # fragment, arrives intact (evidence).

## Notes

- 2026-10-17 [note] Use a large engineered ship (~6k-character Coriolis/EDSY URLs) to check long links open through explorer.exe (review finding astromech-f2dv on astromech-qiey); then tighten the URL-length assertion in the qiey test to the limit that held.
