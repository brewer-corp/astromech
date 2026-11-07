---
schema: atelier.review
schema_version: 2
id: astromech-nvi0
title: finding on astromech-7rcn @387084e6
target: astromech-7rcn
identity: claude-reviewer-xil2
commit: 387084e6a623284f73e619d64e1f13c38016305c
created_at: 2026-11-07T05:27:46.182567650Z
event:
  event: finding
  recommendation: no_action
---

info: app/src/lib/mining.ts:16 With no ?core= the preview's Mining page now says 'Could not read the mining run: the core did not answer' instead of the sample run (after-nocore/mining.png is the one screenshot that differs). That departs from xil2 and 7rcn Done when ('every page renders as before', 'opens with its sample data') but is recorded as a decision note on xil2, makes Mining consistent with the header's 'Not running', and scripts/preview-publish defaults to core=ready, where Mining is unchanged. The accountable agent should accept it explicitly rather than let the checkbox read as met.
