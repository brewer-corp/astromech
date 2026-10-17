---
created_at: "2026-10-15T20:36:13.647047088+00:00"
id: "astromech-znhx"
issue_type: "feature"
labels: []
workflow_branch:
  branch_base: main
  work_branch: feature/astromech-znhx
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-e8ux"
  children:
  - kind: "issue"
    id: "astromech-0jqu"
  - kind: "issue"
    id: "astromech-7vpa"
  - kind: "issue"
    id: "astromech-8e8c"
  - kind: "issue"
    id: "astromech-a63r"
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-17T05:15:02.545192518+00:00"
status: "done"
title: "Exobiology"
updated_at: "2026-10-17T05:15:02.545192518+00:00"
---

## Goal

The user knows which bodies have biology worth sampling, what it's likely to be worth, and when they've moved far enough between samples. This replaces OD Explorer's and BioScan's exobiology features.

## Done when

Every child issue is done. Delivers S8.

## Unknowns

Licences of the species prediction tables; how complete Spansh's body signal and genus data are.

## Close Reason

Exobiology delivered: surface guidance, species prediction with values, the sampling helper and the system biology summary (voice and Galaxy > Biology page). Review outstanding; overlap with astromech-p4ls to reconcile at merge.
