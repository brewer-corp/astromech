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
updated_at: "2026-10-16T19:13:12.726110806+00:00"
---

## Goal

Confirm the Coriolis and EDSY links built from the current loadout survive the Windows shell launch (open_external -> explorer.exe).

## Done when

In the Windows app, one Coriolis and one EDSY link from the Builds page open the right build in the default browser; the long URL, including the EDSY # fragment, arrives intact (evidence).
