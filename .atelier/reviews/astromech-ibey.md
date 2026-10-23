---
schema: atelier.review
schema_version: 1
id: astromech-ibey
title: '[low] .github/workflows/atelier.yml:58-60: the empty commit moves claims made in the tip commit from CurrentClaims to Historical evaluation (claim_nodes retained at HEAD^), which skips report-part required fields and uses non-strict evidence freshness and validator checks for those claims. Every later doctor on main sees them as Historical anyway, and the alternative fails falsely on a tip that adds an approval or is a merge (8866d1f1, e5fe2f51 fail raw), so the trade-off is sound; the only extra leniency is for the last commit of each run, and the current close transition requires only close_reason, which Historical still checks.'
target: astromech-mk30
identity: review-agent-mk30-1
commit: 1337bcf6cbfb52a7cf1ddb7e9715267f422bad5d
created_at: 2026-10-23T00:00:02.976104929Z
event:
  event: finding
  recommendation: no_action
---

[low] .github/workflows/atelier.yml:58-60: the empty commit moves claims made in the tip commit from CurrentClaims to Historical evaluation (claim_nodes retained at HEAD^), which skips report-part required fields and uses non-strict evidence freshness and validator checks for those claims. Every later doctor on main sees them as Historical anyway, and the alternative fails falsely on a tip that adds an approval or is a merge (8866d1f1, e5fe2f51 fail raw), so the trade-off is sound; the only extra leniency is for the last commit of each run, and the current close transition requires only close_reason, which Historical still checks.
