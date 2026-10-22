---
created_at: "2026-10-22T20:41:31.971311485+00:00"
id: "astromech-ux0h"
issue_type: "feature"
labels: []
workflow_branch:
  branch_base: main
  work_branch: feature/astromech-ux0h
priority: "P2"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "astromech-dxv1"
  - kind: "issue"
    id: "astromech-otsm"
  - kind: "issue"
    id: "astromech-st8o"
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-22T21:08:27.901114290+00:00"
status: "done"
title: "Data sharing hardening"
updated_at: "2026-10-22T21:08:27.901114290+00:00"
---

## Goal

The EDSM and EDDN senders cope with poisoned and stale events, and the data-sharing limits test checks whole sentences.

## Done when

Every child is done, independently reviewed and merged on this branch, with CONTRIBUTING.md's checks passing locally and on the PR's CI.

## Background

Grouped on 22 Oct 2026 so related review leftovers share one branch, one review and one CI run.

## Close Reason

EDSM poisoned events no longer hold up good ones, the no-age-limit choice for EDSM is documented, and the limits test checks whole sentences; independently reviewed (astromech-35u9).
