---
created_at: "2026-10-22T20:41:31.971311485+00:00"
id: "astromech-ux0h"
issue_type: "feature"
labels: []
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
status: "todo"
title: "Data sharing hardening"
updated_at: "2026-10-22T20:41:37.879976702+00:00"
---

## Goal

The EDSM and EDDN senders cope with poisoned and stale events, and the data-sharing limits test checks whole sentences.

## Done when

Every child is done, independently reviewed and merged on this branch, with CONTRIBUTING.md's checks passing locally and on the PR's CI.

## Background

Grouped on 22 Oct 2026 so related review leftovers share one branch, one review and one CI run.
