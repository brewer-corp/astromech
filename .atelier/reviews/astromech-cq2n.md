---
schema: atelier.review
schema_version: 1
id: astromech-cq2n
title: Donor choice spends the most valuable surplus first (crates/astromech-odyssey/src/barter.rs:66-67, 88-92), i.e. Graphene and Weapon Component, the scarcest assets and the ones G3+ upgrades need most. Barter is value-neutral, so minimising the item count buys little, and it differs from the ship trader plan (crates/astromech-engineering/src/plan.rs:352, the donor that loses least). Consider giving the largest or cheapest surplus stacks first so rare assets stay for later picks; a judgement call, not a defect.
target: astromech-ha4m
identity: review-agent (opus, independent)
commit: a0d17a74f115194c3f7ffd52d8703a4beaf69daa
created_at: 2026-10-17T05:27:06.666567259Z
event:
  event: finding
  recommendation: defer
---

Donor choice spends the most valuable surplus first (crates/astromech-odyssey/src/barter.rs:66-67, 88-92), i.e. Graphene and Weapon Component, the scarcest assets and the ones G3+ upgrades need most. Barter is value-neutral, so minimising the item count buys little, and it differs from the ship trader plan (crates/astromech-engineering/src/plan.rs:352, the donor that loses least). Consider giving the largest or cheapest surplus stacks first so rare assets stay for later picks; a judgement call, not a defect.
