---
created_at: "2026-10-15T20:21:44.887324248+00:00"
id: "astromech-817p"
issue_type: "task"
labels:
- "size-l"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Frontier cAPI client"
updated_at: "2026-10-15T20:21:44.887324248+00:00"
---

## Goal

OAuth PKCE client with 25-day re-auth, for fleet-carrier data and `fcmaterials_capi`.

## Done when

A logged-in session fetches the carrier endpoint, and re-auth is prompted before expiry.

## Context

Needs a Frontier client-id approval. The journal covers almost everything else.
