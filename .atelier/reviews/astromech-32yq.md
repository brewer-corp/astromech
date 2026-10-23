---
schema: atelier.review
schema_version: 1
id: astromech-32yq
title: 'R2 [low] crates/astromech-eddn/src/companion.rs:431 and docs/guide/data-sharing.md:56: FDevIDs entitlement ''?'' means the sku is unknown (commits 4ee5105 ''SKU still unknown'', ee298a6), not commander-only. Hpt_CausticSinkLauncher_Turret_Tiny and Hpt_AntiUnknownShutdown_Tiny_V2 (Thargoid Pulse Neutraliser, listed at Jameson Memorial) are therefore always dropped. That is the right conservative call, but the test files the Caustic Sink Launcher under TECH_BROKER, and the guide says the list is ''only what the station sells to anyone'', which reads as complete when it is a subset (also ''removed'' scanners, the ARX armour in R1). Recommendation: defer. Relabel the test entry ''unknown sku'' and word the guide as ''leaves out modules that may depend on your status''.'
target: astromech-dlrb
identity: review-agent-dlrb-1
commit: de5a0a0e0d5870a35e9349a5a46fae2cce0517f7
created_at: 2026-10-23T00:05:52.698817401Z
event:
  event: finding
  recommendation: defer
---

R2 [low] crates/astromech-eddn/src/companion.rs:431 and docs/guide/data-sharing.md:56: FDevIDs entitlement '?' means the sku is unknown (commits 4ee5105 'SKU still unknown', ee298a6), not commander-only. Hpt_CausticSinkLauncher_Turret_Tiny and Hpt_AntiUnknownShutdown_Tiny_V2 (Thargoid Pulse Neutraliser, listed at Jameson Memorial) are therefore always dropped. That is the right conservative call, but the test files the Caustic Sink Launcher under TECH_BROKER, and the guide says the list is 'only what the station sells to anyone', which reads as complete when it is a subset (also 'removed' scanners, the ARX armour in R1). Recommendation: defer. Relabel the test entry 'unknown sku' and word the guide as 'leaves out modules that may depend on your status'.
