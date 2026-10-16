---
created_at: "2026-10-15T20:36:13.660178701+00:00"
id: "astromech-lt2b"
issue_type: "task"
labels:
- "size-s"
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-uxyq"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-16T19:23:24.852979108+00:00"
status: "done"
title: "EDDN commodity/3"
updated_at: "2026-10-16T19:23:24.852979108+00:00"
---

## Goal

Send `commodity/3` from `Market.json` on the `Market` event, with the prohibited list and economies.

## Done when

A fixture `Market.json` produces a message that validates against the live `commodity/3` schema.

## Context

This is the data Inara, Spansh and Ardent all run on.

## Notes

- 2026-10-16 [note] Spec read: EDCD/EDDN live 4ad669bb (16 Oct 2026), commodity-README.md + commodity-v3.0.json. The README says journal Market.json has no economies or prohibited data and they MUST be left out (never sent empty), so the goal's 'prohibited list and economies' can't come from Market.json; they'd need CAPI /market (not available to us). Choices: StationType omitted (README elision list says MUST remove it, though the schema now allows stationType); statusFlags not sent (README says include 'if available', but the CAPI flag strings aren't documented, and no data beats bad data); the CAPI 'legality' skip rule has no Market.json counterpart; an empty commodities list is sent (schema allows it). A malformed item drops the whole message. Builder: astromech_eddn::companion::message(event, file_json).

## Close Reason

astromech_eddn::companion builds commodity/3 from Market.json per the live README; fixture validates against the live commodity-v3.0.json (evidence recorded). economies/prohibited are not in Market.json and the README forbids sending them (see note). Independent review pending (orchestrator).
