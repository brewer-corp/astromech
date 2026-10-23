---
schema: atelier.review
schema_version: 1
id: astromech-tbrk
title: 'R3 [low] crates/astromech-eddn/src/companion.rs:254 (policy a): an unknown symbol refuses the whole visit (Unusable::UnknownModule, logged to Diagnostics). This is the right call under ''no data is better than bad data'': the alternative, sending without it, also publishes a wrong full list, and the module could be commander-specific. The cost is that after any game update adding a module, outfitting/2 stops for every station selling it until FDevIDs adds the module, the data is regenerated and a release ships. Only the per-visit Diagnostics line shows this, and FDevIDs'' README says it is incomplete, notably for PowerPlay items. Recommendation: defer. File a follow-up for an early warning, for example the real-journal harness or a scheduled CI job that reports unknown symbols, or diffs FDevIDs HEAD against the source key.'
target: astromech-dlrb
identity: review-agent-dlrb-1
commit: de5a0a0e0d5870a35e9349a5a46fae2cce0517f7
created_at: 2026-10-23T00:05:52.851367898Z
event:
  event: finding
  recommendation: defer
---

R3 [low] crates/astromech-eddn/src/companion.rs:254 (policy a): an unknown symbol refuses the whole visit (Unusable::UnknownModule, logged to Diagnostics). This is the right call under 'no data is better than bad data': the alternative, sending without it, also publishes a wrong full list, and the module could be commander-specific. The cost is that after any game update adding a module, outfitting/2 stops for every station selling it until FDevIDs adds the module, the data is regenerated and a release ships. Only the per-visit Diagnostics line shows this, and FDevIDs' README says it is incomplete, notably for PowerPlay items. Recommendation: defer. File a follow-up for an early warning, for example the real-journal harness or a scheduled CI job that reports unknown symbols, or diffs FDevIDs HEAD against the source key.
