---
created_at: "2026-10-17T22:43:25.081889725+00:00"
id: "astromech-32i6"
issue_type: "task"
labels:
- "alpha"
workflow_branch:
  branch_base: main
  work_branch: task/astromech-32i6
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-17T23:34:19.247297333+00:00"
status: "done"
title: "Engineering and Materials show the sample fleet and inventory when the journal cannot be read"
updated_at: "2026-10-17T23:34:19.247297333+00:00"
---

## Goal

When the journal folder cannot be read, Ships and Materials say so instead of showing invented data.

## Done when

loadFleet and loadMaterials (app/src/lib/engineering.ts, materials.ts) no longer fall back to the sample on a failed engineering_fleet or materials_inventory call in the real app; the pages show their error state (added in astromech-x81p) instead. A test covers it.

## Context

Found walking astromech-x81p. In Tauri, a rejected call returns the fixture with source 'Sample journal (no live journal folder)', so a commander without a journal folder sees the fixture ships and materials as their own. The label is small. Decision needed: keep as a deliberate demo mode, or show the error.

## Notes

- 2026-10-17 [note] Cause: lib/engineering.ts loadFleet and lib/materials.ts loadMaterials caught every rejected call in the app and returned the fixture labelled 'Sample journal (no live journal folder)'. The shell already rejects with 'journal folder not found: <path>'. Fix: let the rejection through so the pages show their error state. Audit of other pages: no other in-app fallback to sample data (all sample/fixture paths are gated on !inTauri).

## Close Reason

loadFleet and loadMaterials no longer fall back to the sample in the real app; Ships and Materials say the journal could not be read and why. Test fails before the fix. Reviewed; no other page has the fallback (audited twice).
