---
created_at: "2026-10-15T20:36:13.648747508+00:00"
id: "astromech-nf25"
issue_type: "epic"
labels: []
priority: "P2"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "astromech-d4p9"
  - kind: "issue"
    id: "astromech-um0g"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "draft"
title: "VR overlay"
updated_at: "2026-10-15T20:36:13.885777811+00:00"
---

## Scope

Deferred: no UI in VR *yet*. This is a prioritisation call (Sumeet, 15 Oct 2026), not a permanent one; it comes after the daily lookups. Voice-only in VR until then. Keep designs overlay-friendly where it's cheap.

## Outcome

In VR, the user sees the PTT and confirmation toast and a kneeboard with the lookups that matter mid-flight, in a SteamVR overlay next to Elite.

## Unknowns

Whether a Rust SteamVR overlay is stable and cheap enough next to Elite (the spike answers this).

## Design

[ARCHITECTURE §14](docs/ARCHITECTURE.md) Phase 3. V1-DESIGN §1.4's "probably for good" is superseded by this.
