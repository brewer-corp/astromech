---
created_at: "2026-11-07T05:34:24.723280855+00:00"
id: "astromech-278z"
issue_type: "validation"
labels:
- "needs-human"
- "size-s"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-xec1"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "GPU timing for the Rust voice stack on Sumeet's PC with Elite in VR"
updated_at: "2026-11-07T05:34:24.723280855+00:00"
---

## Goal

Measure whether speech on the RX 9070 XT (burn on wgpu, DX12 or Vulkan) beats the CPU while Elite runs in the headset, and whether it causes frame drops or reprojection.

## Done when

Sumeet has run the probe the runtime spike ships, with Elite in VR, and its results JSON is attached as an evidence receipt: CPU and GPU decode medians and p95, first-run kernel compile time, and any reprojection seen.
