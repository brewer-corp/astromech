---
schema: atelier.review
schema_version: 3
round: 6
commit: 7b57aac956790a9e9894ed67b14492257edb2a85
reviewers:
- thread:delegated-task:command%3Amcp%3Aee65c5e2-2014-49d7-aa45-41a730a2d1b3%3Adelegate-task%3Add0ff8c7-e77f-4e22-be66-1c53ba65b8a1
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-17T08:50:30.087761361Z
---

## Summary

Complexity: high. The change replaces a page with shared frame state, a persistent sidebar and an overlay, and changes navigation and responsive layout ownership.

Value: high. It delivers the requested persistent route beside the commander's current page.

Risk of merging: medium. Regressions affect the window across pages, but are visible and reversible.

The value warrants the change; a smaller change would not deliver the requested frame behavior.

Reviewed the full diff against main and the merge since the previous reviewed commit, with particular attention to the App.svelte and Timeline.svelte conflict resolutions. Checked the Done when requirements against the implementation, browser tests, journeys, guide and DESIGN.md. Read the shared route polling and action state, dock/collapse/overlay transitions, legacy route navigation, no-route import paths, timeline and menu layout. Inspected the committed 1920 px docked and 1100 px overlay captures. Checked all ten page container-query consumers: their largest switching width is below the narrowest docked page column.

Independently passed pnpm lint, pnpm check, pnpm test (85 files, 711 tests), pnpm build, the journey typecheck, scripts/check-dist.py, scripts/check-guide.py and git diff --check. All seven journeys passed using ASTROMECH_CORE=/home/supadava/projects/astromech/target/debug/astromech-core, the existing debug binary; no Rust build was performed. The browser suite includes the dock-width measurement at every Text size step, import from both empty states, route appearance/clearing, overlay dismissal, and delayed/failed saved-size restoration. Did not run the native Windows app or interact with Elite.

The two CSS conflicts preserve the intended sidebar layout. However, the merge also leaves two different definitions of the same scrim token, silently changing dialog backdrops despite all checks passing.

## Finding 1: Keep the route overlay's scrim from overriding every dialog backdrop
Recommendation: fix now

app/src/design/tokens.css:32 defines --scrim as Void at 72 percent, but line 23 already defines --scrim as black at 70 percent, brought in by the token migration on main. The later declaration wins globally. app/src/lib/ui/Dialog.svelte:33 now consumes this token for every dialog::backdrop, so merging the sidebar silently changes all dialogs to the route overlay's shade. DESIGN.md:1274 explicitly reserves the existing token and its black value for dialogs.

Confirmed in headless Chromium by loading the committed tokens and Dialog.svelte's backdrop rule and opening a dialog: the computed backdrop is rgba(12, 13, 15, 0.72), rather than the existing black scrim. Give the route overlay a distinct token and update App.svelte to consume it, preserving the dialog token and removing the duplicate declaration. This resolves the merge collision without changing either intended surface.
