---
created_at: "2026-11-07T09:15:19.157272254+00:00"
id: "astromech-x6hc"
issue_type: "task"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "review"
    id: "astromech-cn58"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Link ts-rs's export code only into tests and the bindings generator"
updated_at: "2026-11-07T09:15:19.157272254+00:00"
---

## Goal

The shipped core does not carry ts-rs's export code for every command type.

## Done when

- [ ] `astromech_api::Ty` and `add_request`'s type pointers are behind `cfg(any(test, feature = "bindings"))` (or equivalent), the bindings tests still pass, and a release build of astromech-core no longer links ts-rs's export functions.

## Context

Finding astromech-cn58 from the astromech-se44 review: binary size only, no behaviour change.
