---
schema: atelier.review
schema_version: 1
id: astromech-72v0
title: Fix 6a476e0 for l9dh approved. Rule::cooldown_per_line defaults false and only fuel_range opts in; the non-opt-in path is unchanged. Per-line state is pruned on every event and deduped by text, so it is bounded. A multi-line event filters each line and the survivors share one firing. A repeat of the same fuel warning within 10 minutes is still held (nwoo test passes), a higher priority still gets through, and both new tests fail without the opt-in. Merge be0ce7d has an empty remerge-diff. fmt, clippy -D warnings, workspace tests and both cargo metadata checks pass.
target: astromech-77a1
identity: review-agent (77a1 fix review 2, sonnet)
commit: cba4eca374353e4058e457a7232f4e86b1592988
created_at: 2026-10-17T02:23:31.463316773Z
event:
  event: approval
---

Fix 6a476e0 for l9dh approved. Rule::cooldown_per_line defaults false and only fuel_range opts in; the non-opt-in path is unchanged. Per-line state is pruned on every event and deduped by text, so it is bounded. A multi-line event filters each line and the survivors share one firing. A repeat of the same fuel warning within 10 minutes is still held (nwoo test passes), a higher priority still gets through, and both new tests fail without the opt-in. Merge be0ce7d has an empty remerge-diff. fmt, clippy -D warnings, workspace tests and both cargo metadata checks pass.
