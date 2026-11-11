---
schema: atelier.item
schema_version: 3
id: astromech-lp3o
title: Frontier cAPI client
status: draft
mission: null
labels:
- size-l
priority: low
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-15T20:36:13.659916322Z
---

## Goal

OAuth PKCE client with 25-day re-auth, for fleet-carrier data and `fcmaterials_capi`.

## Done when

- [ ] A logged-in session fetches the carrier endpoint, and re-auth is prompted before expiry.

## Context

Needs a Frontier client-id approval. The journal covers almost everything else.
