---
created_at: "2026-11-10T05:55:46.920933807+00:00"
id: "astromech-3340"
issue_type: "loose_end"
labels:
- "needs-human"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-k6pl"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Check the height scale and the new orange in the installed app on Windows"
updated_at: "2026-11-10T05:55:46.920933807+00:00"
---

## Goal

The scale rule (one root zoom from window height) and the stronger orange were built and checked in Chrome on Linux. WebView2 on Sumeet's monitor is the real target.

## Done when

- [ ] On Sumeet's PC, in a release that has astromech-k6pl: text is comfortably readable at his usual window size, the Text size setting changes it, nothing is clipped at the smallest window, and the orange no longer looks washed out.
