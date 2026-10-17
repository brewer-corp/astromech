---
created_at: "2026-10-17T22:43:25.081889725+00:00"
id: "astromech-32i6"
issue_type: "task"
labels:
- "alpha"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Engineering and Materials show the sample fleet and inventory when the journal cannot be read"
updated_at: "2026-10-17T22:43:25.081889725+00:00"
---

## Goal

When the journal folder cannot be read, Ships and Materials say so instead of showing invented data.

## Done when

loadFleet and loadMaterials (app/src/lib/engineering.ts, materials.ts) no longer fall back to the sample on a failed engineering_fleet or materials_inventory call in the real app; the pages show their error state (added in astromech-x81p) instead. A test covers it.

## Context

Found walking astromech-x81p. In Tauri, a rejected call returns the fixture with source 'Sample journal (no live journal folder)', so a commander without a journal folder sees the fixture ships and materials as their own. The label is small. Decision needed: keep as a deliberate demo mode, or show the error.
