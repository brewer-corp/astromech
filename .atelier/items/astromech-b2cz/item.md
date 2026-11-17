---
schema: atelier.item
schema_version: 3
id: astromech-b2cz
title: Text size you choose replaces the window zoom
status: building
mission: astromech-xajc
labels:
- size-l
- feature
priority: high
blocked_by:
- astromech-n41x
implementers:
- thread:delegated-task:command%3Amcp%3A40e7bd6a-75d7-481f-bc07-e18b62a688e1%3Adelegate-task%3A8587c77f-b791-4383-a444-92fd4885a6f8
ready:
  by: Sumeet
  at: 2026-11-17T05:23:28.056709303Z
trivial: null
caused_by: null
created_at: 2026-11-17T05:20:19.201058589Z
---

## Goal

A bigger window shows more, and resizing never changes the size of the text. The window's zoom from its height (astromech-k6pl) goes; the commander's Text size sets one root size, as on any web page. Sumeet, 16 Nov 2026: the auto-zoom "feels very wrong".

## Done when

- [x] `tokens.css` and the type role classes are in rem, and no component sets a px font size or spacing that should scale with text (borders and 1 px rules stay px). `html` font size comes from Text size alone; `scale.svelte.ts`'s window formula, `--win-h`, `--win-w` and the zoom corrections in `tip.ts`, Dialog, Menu, Picker and My controller's sheet are gone.
- [x] Text size has six steps, 100, 115, 130, 145, 160 and 180 percent of a 16 px root, in Settings > General, live and persisted as today. 145 percent at 2560 x 1440 matches what the approved captures show (the old 1.44 zoom).
- [x] The config stores Text size as a percentage, and an absent field means "not chosen yet", which `window_settings` reports as such instead of as a default. With no choice saved, the window picks a step from the display's height (the screen's, never the window's) and saves it through `set_window`, once: below 1200 px, 100; below 1300, 115; below 1400, 130; below 1600, 145; below 1800, 160; otherwise 180. After that only the commander changes it. A test asserts 100 for a 1080 display, 145 for 1440 and 180 for 2160, and that a resize changes nothing.
- [x] An existing install keeps the size it has. The old values `smaller`, `normal` and `larger` still parse; on the first start the window turns one into the step nearest the old result for this display (the old height zoom, capped as it was, times 0.85, 1 or 1.15) and saves that. A 1440p install on `normal` therefore lands on 145, not 100. Config tests cover the three old strings and the missing field, and a restart that keeps the migrated or the new choice.
- [x] The `@container win` rules are in rem and still switch where the layout needs them at 100 and 180 percent.
- [x] Every page checked in the preview at 1100 x 700 and 1920 x 1080 at 100 percent, 2560 x 1440 at 145 percent and at 100 percent, and 3840 x 2160 at 180 percent: nothing clipped, no sideways scroll. Past its natural width a layout leaves the rest of the window empty; it does not stretch.
- [x] Captures of the Sell finder at 2560 x 1440 at each of the six steps are in `docs/design/text-size/`, for Sumeet to see the choice.
- [x] The user guide's Settings page says what Text size does. `pnpm check`, `pnpm lint`, `pnpm test` pass.

## Scope

Sizes and the setting. Colours are astromech-3kf5. The Windows check of the result is astromech-3340.

## Context

DESIGN.md revision 5 (astromech-n41x) states the rule. The zoom was a shortcut to leave px tokens alone; media queries, `vh` and tooltip placement all needed workarounds for it.

## Report

**For the starting thread to settle (carried on meanwhile):** the migration rule, applied
literally, moves a 1080p install on `normal` to **115**, not 100. Its old zoom there was 1.08,
and 108 is nearer 115 (7) than 100 (8). A fresh 1080p install gets 100. A maximised 1080p
window's inner height is about 1010 to 1040 px, so it actually drew at about 1.0 before,
which argues for 100. I followed the
rule and pinned it in `scale.test.ts`; changing it is one line in `migratedStep` (round down,
or measure from the old zoom's floor).

**Also for the starting thread:** three journeys fail on this branch, and they fail the same
way on the base commit (e8890914, checked in a separate worktree with its own core build).
The failing journeys are `plot a route from the page and follow it`, `a jump into the next stop
copies and reads out the one after`, and `the kill switch shows on in the page and goes off
after a re-arm`. Each one times out waiting for a `.next .sys` locator that the page no longer
draws that way. They broke before this item, probably with the nav rework merge, and need an
issue. I'm on an item branch, where `atelier new` can't file one.

What changed:

- **Core** (`config.rs`, `setup.rs`, `app.rs`): `[window] text_size` is `Option<TextSize>`,
  an untagged enum of a percentage or the old word (`LegacyTextSize`). Absent means not
  chosen, and `window_settings` reports `null`. `set_window` accepts only the six steps
  (`TEXT_SIZE_STEPS`) and refuses the old words. The sample config leaves it commented out,
  so a new install takes the first-run pick, not the migration. Tests: the three old words,
  the missing field, a bad value, and a restart that keeps a saved step
  (`the_text_size_persists` starts from `"normal"` and saves 145, the migration's write).
- **Window** (`lib/scale.svelte.ts`): no zoom and no resize listener. `startScale` reads the
  setting once. With `null` it saves `firstStep(screen.height)`; with an old word it saves
  `migratedStep(word, screen.width, screen.height)`. Either way it sets `html`'s font size
  in percent. The display is `screen` in CSS px, as the old zoom measured `innerHeight`, so
  Windows display scaling counts the same way it did before. Settings > General shows
  100% to 180%.
- **CSS**: every px size in the style blocks and `tokens.css` is now rem (px / 16). The
  conversion was scripted and its output reviewed. 1 px values, borders, outlines, scrollbars,
  underline offsets and the 2 px gradient rules stay px. Rem values that were written against
  the old 15 px root (`html` used to carry `--fs`) are rescaled by 15/16. `html` now takes
  only Text size and `body` takes `--fs`. Powerplay's three-column container query is
  recomputed in rem. `--zoom`, `--win-h` and `--win-w` are gone, along with the zoom
  corrections in `tip.ts`, Dialog, Menu, Picker, PlotBar, Build and the controller sheet.
  The sheet now measures its column (`bind:clientWidth`, `minmax(0, 1fr)`) and caps at
  1180 px at 100 percent. The sheet's own labels stay px times its image scale, because
  they follow the image and not the text. Glyph, ShipIcon and SearchKit's script-side column
  widths are in rem. Split, Workbench and SearchKit's compact row compared px-at-100%
  numbers with measured widths, and now compare through `layout.ts`'s `at100()`, with a test.
- **Late core**: `startScale` asks again on each status poll until the core answers, then
  stops. Before, a core that wasn't up yet left the window at 100 percent for the session.
  There's a test for this.
- **CONTRIBUTING**: the journeys' core binary is built by `cargo build -p astromech-app --bin
  astromech-core`. The old `-p astromech-core` builds only the library.
- **Docs**: the Settings guide page, DESIGN.md's "not landed yet" notes, and six captures in
  `docs/design/text-size/`.

Checks:

- 145 percent at 2560 x 1440 matches the old 1.44 capture taken before the change, within a
  pixel or two.
- I swept these headless at 1100 x 700 and 1920 x 1080 at 100, 2560 x 1440 at 145 and at
  100, and 3840 x 2160 at 180: the 24 modules, the 10 Places finders, a ship's Engineering
  plan, onboarding (`core=new`) and the Bind dialog. None scrolls sideways, has an element
  past the right edge, or clips text in an overflow-hidden box.
- At 1100 x 700 at 100 and 3840 x 2160 at 180, I opened the Find or plot panel, the route
  Menu and the Plotter Picker. Each stays inside the window. The tip is covered by its
  browser test at 180 percent.
  To confirm the sweep can fail, I ran it at 1100 x 700 at 180 percent and it flagged every
  page.
- The `@container win` rules are in rem. One switch checked directly: `nav.sub`'s gap is the
  narrow rule's 1rem (28.8 px) at 1920 x 1080 at 180 percent and the wide 24 px at 100.
- The controller sheet at 2560 x 1440 at 145 percent: labels fit their boxes.
- Engineering's module cards truncate long blueprint names in their dropdowns, with an
  ellipsis, at 100 percent as well as at 180. The cause is the `auto-fill` card width, not
  this change.
- `pnpm check`, `pnpm lint`, `pnpm test` (639 tests), `cargo test -p astromech-core` and
  `-p astromech-app`, clippy with `-D warnings`, `cargo fmt --check` and `check-guide.py`
  all pass. Of the journeys, the lookup and voice journeys pass; the three above fail as on
  the base commit.

Known limit, outside this item's check sizes: at 1100 px wide with 145 percent or more, the
stacked header's tabs run past the right edge. The header doesn't wrap them. DESIGN.md's
Frame row already gives re-measuring the header in rem at each step to astromech-5a01.

