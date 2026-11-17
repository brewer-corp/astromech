---
schema: atelier.review
schema_version: 3
round: 6
commit: 5d4419f80dee8afc9f9c7b02d919a98466dd10cb
reviewers:
- thread:delegated-task:command%3Amcp%3A7af80d85-cde0-4a8e-9929-43c0997f41ce%3Adelegate-task%3Ac8d603e2-7d9a-40af-9983-a1225d2c5ce0
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-17T10:44:53.451619630Z
---

## Summary

- Complexity: high. The remaining increments rebuild the pages, introduce Now and suit loadouts, and extend several core replies.
- Value: high. These are current-use pages, and the plan follows the owner's recorded feedback and approved mockups.
- Risk of merging it: medium. Visual failures show in the preview and revert per page; loadout identity, shopping costs and the new summary data can be wrong silently, but their items require focused behavior tests and recorded fixtures.

The outcome is worth the cost. The foundation already shipped; replacing only its colours or text sizes would leave the remaining pages' hierarchy and layout problems intact. No further fix-now finding.

I reviewed the mission, design and notes, the listed item briefs, DESIGN.md and CONTRIBUTING.md. I checked the screen contracts against the existing page components and generated reply types, including the suit picker and shopping identity, ship loadout and planner, engineer tree, Biology reports and sampling, cargo lookup inputs and session earnings. I inspected the Suits and Engineers reference captures and the narrow Now mockup, accounting for each item's stated changes from the mockup. I also checked the audit tool's flags and measurements and the final capture/glance-test ownership. A read-only dependency check found every dependency present and no cycles across the 48 mission items.

This is a plan review at 5d4419f80dee8afc9f9c7b02d919a98466dd10cb. I did not build the proposed screens, run the app or its test suites, regenerate fixtures, or check Windows WebView2. Their runtime and visual acceptance remain the implementation items' checks, the final sizing audit and the independent glance test.

## Finding 1: ADD TO PLAN must not add a second pick of a recipe that is already saved without an item
Recommendation: fix now
Was: 5-qhcy.8
Now: fixed

`.atelier/items/astromech-nnn0/item.md:36` now explicitly makes ADD TO PLAN tie a matching untied pick to the selected item's journal id without appending another pick. It applies the same rule to PLAN OTHER GEAR. Its required test starts with a generic Maverick 1-to-3 pick in localStorage and requires one pick, costed once, a plan on the chosen Maverick and no plan on the other Maverick. Lines 30-31 retain the separation between item identity and recipe identity. This fixes the carried finding in the plan; implementation of the rule is still required by the unchecked Done when line.
