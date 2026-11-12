---
schema: atelier.item
schema_version: 3
id: astromech-k6pl
title: Scale the window with its height, and a stronger orange
status: building
mission: astromech-xajc
labels:
- size-m
- feature
priority: high
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3Ac95b33ef-c19f-4414-ad69-248cbccdf307%3Adelegate-task%3Ad6aa10d8-5cc9-4785-a6fd-a4e4aa2e7bfb
ready:
  by: conversion
  at: 2026-11-10T06:03:07.884354190Z
trivial: null
caused_by: null
created_at: 2026-11-07T03:00:10.391746191Z
---

## Goal

Text is readable on a big screen. The game scales its UI with the screen's height; astromech's fixed pixel sizes leave a 1440p or 4K window's text about the size of the game's smallest. Sumeet, 6 Nov 2026: 'the text is just too small across the whole app', and the orange looks washed out.

## Done when

- [x] At 2560 x 1440 the window renders at about 1.4x; at 1100 x 700 it is unchanged; at 3840 x 2160 it is capped at 1.8x. A tall, narrow window (1100 x 1400) stays at 1.0x.
- [x] A Text size setting in Settings > General (Smaller, Normal, Larger: 0.85, 1.0, 1.15 times the height scale) changes it live and persists across restarts. It is stored in the core's config and read and set through commands in the command layer, as the voice settings are (voice_settings and its setter), with a fake handler in the preview and a test.
- [x] DESIGN.md's Typography, Layout and Token map say the scale rule and the new values, and tokens.css has them.
- [x] Every page checked in the preview at 1100 x 700 and 2560 x 1440; the design-system screenshots re-rendered.

## Context

Mission astromech-xajc. This item owns DESIGN.md's Typography sizes, colour values and Token map; astromech-b2k3 owns the other revision 4 amendments. The window has no settings store of its own: settings live in the core's config.

## Design

- The whole window scales with its height, as one root zoom so DESIGN.md's px tokens stay as they are: zoom = clamp(1, min(height / 1000, width / 1100), 1.8). The width term stops a tall, narrow window from squeezing the layout below its 1100 px minimum; the floor of 1 means text never shrinks, and a short window scrolls. The Text size setting multiplies the result.
- Orange text at weight 500, Orange Mid #d97a34, the base orange #ff7100, and fewer labels in Orange Mid. The before/after is in the Route mockup's toggles (astromech-ok29).

## Report

The window now scales with its height, and Orange and Orange Mid are stronger.

**Scale.** `app/src/lib/scale.svelte.ts` sets one root `zoom` (and `--zoom`) from
`clamp(1, min(h / 1000, w / 1100), 1.8) × text size`, on start and on every resize; `App.svelte`
starts it, so onboarding scales too. `scale.test.ts` holds the table: 1100 x 700 is 1,
2560 x 1440 is 1.44, 3840 x 2160 is capped at 1.8, 1100 x 1400 stays 1.

Chromium's CSS zoom also scales `vh` and `vw` (checked in headless Chrome 148: `100vh` drew
1.44 x the window), and media queries measure the screen, not the zoomed layout. So:
- `tokens.css` adds `--win-h` (`100vh / --zoom`), `--win-w` (`1vw / --zoom`) and `--side` (the
  frame's `clamp(16px, 3vw, 40px)` margin); the five `100vh`/`vw` uses now take them.
- The frame (`App.svelte`, `Onboarding.svelte`) is a named size container, `win`, and the 13
  page `@media (max|min-width)` rules are now `@container win (...)`, which measure the layout.
- My controller's sheet width divides `innerWidth` by the zoom.

**Text size.** `[window] text_size = "smaller" | "normal" | "larger"` in the core's config
(`config::TextSize`), read by `window_settings` and set by `set_window` (through `edit_config`,
as `set_voice` is; it applies at once). Settings → General has a new first section, Window, with
a Text size choice that saves and rescales at once. Tests: `setup::tests::the_text_size_persists`
(written to the file, read back; bad values refused), `every_window_command` (default `normal`,
set and read back `larger`), the preview's fake handlers, and a Settings browser test that
clicking Larger sets the root zoom to 1.15 and the fake core holds `larger`. Guide:
docs/guide/settings.md, Window.

**Orange.** `--accent` #ff7100 (7.1:1 on Void), `--orange-mid` #d97a34 (6.3:1), placeholder
#ff7100bf; the page's base weight is 500. DESIGN.md: front matter, Colors (new ratios, and Orange
Mid now says to use labels sparingly), Typography (weight note, the Height Scale Rule with its
table, Figures at 500), Frame (sizes are pre-zoom layout px) and the Token map (new values and
`--zoom`, `--win-h`, `--win-w`, `--side`).

**Checked.** Every page (24) in the preview with `?core=ready` at 1100 x 700 (zoom 1) and
2560 x 1440 (zoom 1.44): no horizontal overflow of the document or `main`, nothing past the right
edge. Captures of Settings and Route at both sizes are in docs/design/scale-*.png. The
design-system mockups were re-rendered (`scripts/mockup-shots.sh ds-*`). `pnpm lint`,
`pnpm check`, `pnpm test` (572), `cargo test -p astromech-core`, `every_window_command`, clippy on
both crates and `check-guide.py` pass.

**Left for others.** Labels, prose and meta still inherit weight 500 until pages name their type
roles (astromech-z5oh), which set Body, Label and Meta back to 400. Which labels a page drops is
each page's work in this mission. The prose grey is unchanged (the Route mockup also lifted it to
#d4d4d4; this item didn't ask for that). WebView2 itself is only checked on Windows
(astromech-3340).

