---
created_at: "2026-10-17T23:32:41.837436940+00:00"
id: "astromech-zkz8"
issue_type: "task"
labels:
- "alpha"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-22T21:44:45.098188006+00:00"
status: "done"
title: "Stale-line tests: assert the EDSM line alone and first-read failures on the production path"
updated_at: "2026-10-22T21:44:45.098188006+00:00"
---

## Goal

The data-sharing tests prove the EDSM stale line and the first-read failures through the app's real loaders.

## Done when

- staleTauri.test.ts asserts the EDSM section's own Could not refresh line (the one under the EDSM heading), with the EDDN section's line empty when only EDSM's call fails.
- First-read failures (rejected, and core_call ok false or core_status null) have production-path tests for Ships, Materials, Data sharing, EDSM and Mining. Ships and Materials are direct shell commands, so only a rejection applies to them.

## Context

Review of astromech-spxx.

On Data sharing a failed first EDDN read shows its error for the whole page (the EDSM section renders only once the EDDN log has been read), so EDSM's own first-read failure is tested with only its call failing.

## Close Reason

Done and independently reviewed on feature/astromech-jo3l (approval astromech-15yv).
