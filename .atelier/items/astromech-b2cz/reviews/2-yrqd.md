---
schema: atelier.review
schema_version: 3
round: 2
commit: f075d93528b026d79f8a0dba674e773f8dedb711
reviewers:
- thread:delegated-task:command%3Amcp%3Aabd79cdf-d396-4efc-9442-ac0c983bc4ca%3Adelegate-task%3Aef9c3ce5-f398-453d-a3a7-0fa99f580f00
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-17T06:54:50.387511586Z
---

## Summary

Complexity: medium. A persisted preference crosses the Rust config and window API, and replacing zoom touches styles and measured layouts across the app, without adding a subsystem.

Value: high. Resizing now shows more content while the commander keeps a chosen, persisted reading size, which is the requested outcome.

Risk of merging it: medium. Every screen and an existing preference are affected, but failures are visible and the change can be reverted.

The value warrants the conversion. I found no materially smaller change that satisfies the root-font-size, persistence and migration requirements. Both carried findings are fixed; I found no new merge blocker.

I reviewed the whole diff from main through f075d93528b026d79f8a0dba674e773f8dedb711, including config parsing/writing, generated API types, initialization, settings, measured layouts, styles and the guide, and checked the changes since round 1. Normalizing the old rem values at 15 px and the new ones at 16 px preserves the stylesheet dimensions, apart from the intended viewport/zoom removal and controller track cap. The migration follows the specified nearest-step rule: the reported 1080p normal -> 115 result is correct under that rule, since 108 is closer to 115 than 100.

`pnpm check`, `pnpm lint` and `pnpm test` pass (84 files, 640 tests). `cargo test -p astromech-core text_size` passes both config/persistence cases, and `cargo test -p astromech-app --test every_window_command` passes. `python3 scripts/check-guide.py`, `pnpm exec tsc -p journeys` and `git diff --check main...f075d935` pass.

I independently checked all 24 modules, ten finder routes and engineering/7 in headless Chromium at 1100 x 700 / 100%, 1920 x 1080 / 100%, 2560 x 1440 / 145%, 2560 x 1440 / 100%, and 3840 x 2160 / 180%. No checked page had horizontal document/main overflow, elements outside the horizontal viewport, or page errors. Onboarding also passed that sweep. These geometry checks do not prove every interaction or text state fits. I confirmed the rem container switch at 1920 px: subnav gap 24 px at 100%, 28.8 px at 180%. All six submitted Sell captures exist at 2560 x 1440; I visually inspected the 145% and 180% captures.

For persistence, I replaced only the transport boundary and exercised an absent choice and all three legacy strings on a simulated 2560 x 1440 display. Each failed write returned false without changing the saved value; after writes recovered, the next call saved the expected step, and another call made no further read or write. Resizing to 1100 x 700 preserved computed root and body font sizes.

I did not rerun the known failing journeys documented in round 1, the full Rust/clippy suite, or the installed Windows/WebView2 app. The Windows check remains astromech-3340. The header limitation outside the required viewport/step combinations remains explicitly reported and assigned to astromech-5a01.

## Finding 1: A failed first-run or migration save is permanently treated as successful
Recommendation: fix now
Was: 1-3y69.1
Response: fixed in fc2deb62
Now: fixed

`app/src/lib/scale.svelte.ts:76` now checks the `set_window` reply. Failure clears the initialization promise and returns false, allowing a subsequent call to finish persistence; successful initialization remains cached. The added browser test passes. Independent transport-boundary checks confirmed failure, recovery and no further writes after success for both a first-run selection and every legacy word.

## Finding 2: My controller stretches its column after the sheet reaches its natural width
Recommendation: fix now
Was: 1-3y69.2
Response: fixed in 3ff2108e
Now: fixed

`app/src/modules/controller/View.svelte:244` caps the first grid track at 73.75rem. Chromium measured the sheet ending at x=1220 and the panel beginning at x=1248 at 2560 x 1440 / 100%, and x=2196 and x=2246.39 at 3840 x 2160 / 180%. These match the declared 28 px and 50.4 px gaps, leaving spare width after the layout. At 1100 x 700 / 100%, the sheet and content track both measure 636 px with no horizontal overflow. I captured and visually inspected all three cases in `/tmp/astromech-b2cz-r2-controller-{1100-100,2560-100,3840-180}.png`.
