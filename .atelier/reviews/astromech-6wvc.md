---
schema: atelier.review
schema_version: 1
id: astromech-6wvc
title: 'Contract/proof gap: no test proves the done-when''s ''last delivered to'' rule. In both fixtures (tools colonisation.rs tests JOURNAL, core tests/colonisation_voice.rs JOURNAL), the project last delivered to (Far Port) is also the one whose depot the game updated last, so pick()''s fallback (max depot_updated) gives the same answer. Mutation, run and reverted: replacing the last-delivery branch in astromech_tools::colonisation::pick (line ~682) with None leaves cargo test -p astromech-tools colonisation (5) and -p astromech-core --test colonisation_voice (6) passing. Failure scenario: the commander delivers to A, then docks at or passes depot B without delivering; ''how is the build going'' must still mean A. Fix: add a ColonisationConstructionDepot for the other project after the last contribution (or a third project updated later with no deliveries), and assert pick() and the spoken answer still name the project last delivered to.'
target: astromech-3ez9
identity: review-agent (opus, independent)
commit: d850f53222d41a4a69ed5c90aa8306d7f60c2ed4
created_at: 2026-10-17T08:18:09.639889432Z
event:
  event: finding
  recommendation: fix_now
---

Contract/proof gap: no test proves the done-when's 'last delivered to' rule. In both fixtures (tools colonisation.rs tests JOURNAL, core tests/colonisation_voice.rs JOURNAL), the project last delivered to (Far Port) is also the one whose depot the game updated last, so pick()'s fallback (max depot_updated) gives the same answer. Mutation, run and reverted: replacing the last-delivery branch in astromech_tools::colonisation::pick (line ~682) with None leaves cargo test -p astromech-tools colonisation (5) and -p astromech-core --test colonisation_voice (6) passing. Failure scenario: the commander delivers to A, then docks at or passes depot B without delivering; 'how is the build going' must still mean A. Fix: add a ColonisationConstructionDepot for the other project after the last contribution (or a third project updated later with no deliveries), and assert pick() and the spoken answer still name the project last delivered to.
