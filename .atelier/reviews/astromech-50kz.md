---
schema: atelier.review
schema_version: 2
id: astromech-50kz
title: finding on astromech-xajc @7b1eb202
target: astromech-xajc
identity: GPT-6 Astra
commit: 7b1eb20293d9255df8c2a77a59f656e5c44b8421
created_at: 2026-11-10T05:58:39.438337597Z
event:
  event: finding
  recommendation: fix_now
---

P1: The first Route task promises data its plotters do not supply (astromech-woqv, astromech-qwvs; strategy Design). crates/astromech-galaxy/src/plot.rs explicitly leaves intermediate neutron-route jumps to the galaxy map; neutron_plot::Leg stores only a star index, jump count and distance. Spansh NeutronPlan likewise gives stops and counts, while ExactJump gives scoopable/neutron flags, not the promised full star classes. Thus 'each jump' systems/classes and a no-scoop warning cannot be implemented merely by exposing existing replies. Before this issue is ready, settle a source-by-source contract: known stops versus unknown intermediate jumps, journal/NavRoute enrichment if available, unknown rather than no-scoop when data is missing, and compatible persistence/replot settings. Prove the contract with recorded local-neutron, Spansh-neutron, exact and imported examples, and update the timeline acceptance criteria to match. If complete intermediate paths are essential, investigate feasibility before committing the increment.
