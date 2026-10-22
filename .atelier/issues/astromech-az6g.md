---
created_at: "2026-10-22T21:08:14.143466459+00:00"
id: "astromech-az6g"
issue_type: "task"
labels:
- "size-s"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "EDSM: drain refused events in batches after an outage and honour EDSM's rate-limit headers"
updated_at: "2026-10-22T21:08:14.143466459+00:00"
---

## Goal

After an EDSM outage (Cloudflare 502/503 count as refusals), the queue drains without one request per event, and the courier respects EDSM's X-Rate-Limit-Remaining and X-Rate-Limit-Reset headers as EDMC's EDSM plugin does.

## Done when

A test simulating a 60-minute 503 outage then recovery empties the queue in far fewer requests than events (refused events batched again after a pass that stored something, or 5xx refusals in a pass that stored nothing counted as outage tries), and a test shows the courier waiting when X-Rate-Limit-Remaining is 0 until X-Rate-Limit-Reset.

## Context

Review finding R2 (astromech-amp0) on feature/astromech-ux0h; existed before that branch. The reviewer's simulation: 171 requests to drain 201 events after recovery. crates/astromech-eddn/src/edsm/mod.rs, deliver().
