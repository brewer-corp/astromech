---
schema: atelier.review
schema_version: 3
round: 7
commit: 933ce561f65beb96d7f2ade833db226637e5fe74
reviewers:
- thread:delegated-task:command%3Amcp%3Aee65c5e2-2014-49d7-aa45-41a730a2d1b3%3Adelegate-task%3Aacdbb636-c16b-43c0-8425-de6fd575d6a8
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-17T08:54:43.211820276Z
---

## Summary

Complexity: high. Moving the route into the frame introduces shared state, responsive docking and overlay behavior, and changes navigation and page layout ownership.

Value: high. The change delivers the requested persistent route beside the commander's current page.

Risk of merging: medium. Regressions affect the frame across pages, but are visible and reversible.

The value warrants the change. A smaller change would not deliver the requested sidebar and responsive frame behavior.

Reviewed the implementation against each Done when line and the Report, including the shared route poll, dock/collapse/overlay transitions, no-route import paths, legacy navigation, Places rename, timeline, clipboard failures, lint enrollment and guide updates. Checked the ten page container-query consumers against the available docked column width and inspected the committed 1920 px docked and 1100 px overlay captures. Read the updated browser tests and route journeys, including dock-width measurement at every Text size step, delayed/failed saved-size restoration, route appearance/clearing and the expanded dry-leg warning.

Checked the complete code change against main and the changes since round 6. The scrim fix gives the route overlay its own token and preserves the dialog token. Independently verified the actual tokens and backdrop rules in headless Chromium: a modal dialog computes rgba(0, 0, 0, 0.7), and the route scrim computes rgba(12, 13, 15, 0.72).

Independently passed pnpm lint, pnpm check, pnpm test (85 files, 711 tests), pnpm build, the journey typecheck, scripts/check-dist.py, scripts/check-guide.py and git diff --check. All seven journeys passed with ASTROMECH_CORE=/home/supadava/projects/astromech/target/debug/astromech-core, the existing debug binary; the item changes no Rust or world inputs, and no Rust build was performed. Did not run the native Windows app or interact with Elite. No new findings.

## Finding 1: Keep the route overlay's scrim from overriding every dialog backdrop
Recommendation: fix now
Was: 6-uisd.1
Response: fixed in c36b0074
Now: fixed

Confirmed in app/src/design/tokens.css, app/src/App.svelte and DESIGN.md: the route overlay uses --scrim-route, while Dialog.svelte retains --scrim. The computed-color check above verifies both intended backdrops.
