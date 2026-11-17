---
schema: atelier.review
schema_version: 3
round: 1
commit: e1c39faa4fd10b39126eb8a92d670eea048b68c4
reviewers:
- thread:delegated-task:command%3Amcp%3Ad2073044-2e88-4286-924f-bf012ca17640%3Adelegate-task%3A9e3c33e5-07e8-4eff-90c0-8d5132383b43
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-17T07:05:45.715168708Z
---

## Summary

Reviewed `git diff main...e1c39faa4fd10b39126eb8a92d670eea048b68c4`, all three Done when lines, and the Report against the journeys, their harness and world inputs, `Timeline.svelte`, the Route page's polling, `App.svelte`, `StatusBar.svelte`, and `headerState`.

- **Complexity: low.** Two journey files replace stale selectors with existing roles, accessible names and text; the only product-source change corrects a comment.
- **Value: medium.** Restores regression coverage for plotting, route following and the kill switch, all existing user paths.
- **Risk of merging it: low.** No runtime behavior changes; a selector failure is visible in the journeys and the change is easy to revert.

The value justifies this small change. The next-stop helper identifies the timeline row labelled “Next stop” and its system-specific copy button, excluding the route strip's duplicate button. The journeys still check the previous stop before the page clock advances, then the new stop after the route poll. Clipboard observations and the next-waypoint spoken callout remain asserted. The kill-switch journey checks the exact core state in the footer before activation, after the status poll, and after re-arming. These selectors use the commander's accessible interface rather than presentation classes. The `headerState` comment now accurately names its consumer.

Independent validation in `app/`: `ASTROMECH_CORE=/home/supadava/projects/astromech/target/debug/astromech-core node --test 'journeys/*.journey.ts'` passed all 7 tests, including the three repaired journeys; `pnpm lint`, `pnpm check` and `pnpm exec tsc -p journeys` passed. The cached headless Chromium shell worked without installation.

The Report covers each journey separately, and the passing real-core journeys support its assessment that the product behavior still works. Source inspection confirms why the old selectors no longer find it. I used the supplied core binary rather than rebuilding it; I did not exercise historical binaries or the Windows/Tauri shell. No findings.
