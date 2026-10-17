---
created_at: "2026-10-17T17:29:23.169251395+00:00"
id: "astromech-yerd"
issue_type: "bug"
labels:
- "alpha"
- "docs-found"
workflow_branch:
  branch_base: main
  work_branch: bug/astromech-yerd
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "review"
title: "Spoken lines that break the copy guide are not covered by copy.rs"
updated_at: "2026-10-17T19:08:38.196880886+00:00"
---

## Goal

V1-DESIGN 6.5 says no contractions and little "I", but several spoken lines have both and are outside crates/astromech-core/tests/copy.rs: "I don't know which system you're in yet." (lookups.rs), "I don't know where you are yet." (trade.rs), "Spansh doesn't know X yet, so I can't search from it." (lookups.rs), "Didn't catch what to look for." (assistant.rs), "Couldn't look that up." (actions.rs), "That's never automated." (policy), "I can't press the key K." (policy), "Voice isn't loaded." (session.rs), and the health.rs missing_reason help lines (allowed to run long, but they use "I" and contractions).

## Done when

The lines are reworded to the guide, and copy.rs covers every fixed phrase and the lookup and policy replies so a new one cannot slip in. Found while writing the user guide.
