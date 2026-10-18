---
created_at: "2026-10-15T20:36:13.656533845+00:00"
id: "astromech-cgrw"
issue_type: "validation"
labels:
- "needs-human"
- "size-s"
priority: "P0"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "M3 exit test: eval on the VR clip set"
updated_at: "2026-10-18T00:06:47.295569535+00:00"
---

## Goal

Run `--eval` on Sumeet's VR clip set and check at least 95% execute-or-ask.

## Done when

The eval report shows ≥ 95% execute-or-ask, attached as an evidence receipt.

## Context

The harness exists (`save_clips`, `--eval`). Needs the clip set recorded in VR.

## Notes

- 2026-10-17 [note] 17 Oct: since astromech-e0zz the audio kept after push-to-talk release is the configured post_roll_ms (200 ms), down from about 260 ms. Listen for clipped command endings in the clip set; raise post_roll_ms if they clip.
