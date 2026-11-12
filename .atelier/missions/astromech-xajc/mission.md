---
schema: atelier.mission
schema_version: 3
id: astromech-xajc
title: 'Glow-up: every page reads at a glance and fills the window'
status: ready
labels: []
lead: 65a8ff7c-e901-48a1-be52-323bbbd8e80c
ready:
  by: Sumeet
  at: 2026-11-12T09:32:43.470033653Z
created_at: 2026-11-10T05:54:16.662362547Z
---

## Why

The core works, and some features are ones Sumeet won't play without: neutron plotting, and
seeing his binds on his own stick. The window lets them down. An audit of all 24 pages at
1920 px on 10 Nov 2026 found:

- **The pages don't follow DESIGN.md's main rule.** "One main value, largest" is the rule; the
  28 px value size (`--fs-xxl`) is used nowhere, and a material tile shows its count at 13 px.
- **Half the window is empty, or the content is stretched.** Eight pages cap themselves at
  `max-width: 60rem` and sit pinned left; Stored modules and On foot spread three columns over
  1,800 px.
- **Nothing ranks.** Almost all text is one orange at 13 to 15 px. On a 1440p screen it is
  smaller than the game's smallest text.
- **Tables are the universal hammer.** Ten page files are plain tables, most others label and
  value lists.
- **There is no home.** The app opens on My controller, and the live views (Station, Session,
  Mining) are filed under Logbook.
- **Explanations are printed on the page**: 13 paragraphs on Powerplay, footnotes elsewhere.

The Route page redesign agreed on 6 Nov (astromech-ok29) is the model for the fix and has not
been built.

## Outcome

A commander opens the window on a page about what they are doing now. Every page outside
Controls uses the window well at any size from 1100 x 700 to 4K, shows one obvious main value
per element, and uses a screen built for its job where a table was. A sparse page stays
sparse: a layout places content, it never adds any. Text is readable from a seat next to the game.
Worth shipping on its own: the foundation (scale, orange, type scale, layouts, tooltip, status
bar) and the Route page.

## Success criteria

Both cover every page outside Controls (My controller, All keybinds, Presets, Key tester),
which already read best and get only the frame, the scale and Tip.

- fills-the-window: every page is built on one of the four layout components and passes the
  page-sizing check (no page file sets its own width or font size). Shown by the check
  being on for every page file except the Controls ones and the frame's two parts in the
  route module, and by captures of every page at
  1100 x 700, 1920 x 1080, 2560 x 1440 and 3840 x 2160 with nothing clipped and no action
  further than one column from what it acts on.
- glance-test: on every page a reader can point to the main value, the thing they can change
  and the thing that needs them, without reading (DESIGN.md's own test). Shown by
  astromech-crxa: expected answers are written per page first ("none" where a page has
  nothing to change or nothing needing attention), a fresh agent answers from the
  2560 x 1440 screenshots alone, and every page passes. A failing page is fixed and asked
  again; one left failing leaves the criterion unmet.

## Scope

In: the window's frame, the design system and its tokens, shared UI components, the Now page,
the Route page, and the nine screens above. Core changes only where a screen needs data it
lacks (route data, body parents).

Out:

| Thing | Why | Instead |
|---|---|---|
| New features (system and station pages, missions, bookmarks) | A different outcome | They stay in the backlog and adopt the layouts when built |
| Controls pages | They already read best | Only the frame, scale and Tip reach them |
| VR or in-flight UI, overlays | Deferred (PRODUCT.md) | Unchanged |
| A galaxy map of the route | Most work, least proven | Backlog item, after the timeline |
| Themes, colour customisation, a second hue | DESIGN.md fixed rule | Unchanged |
| Voice, callouts, lookups' behaviour | Not visual | Unchanged |

Environment: Windows with WebView2 next to Elite; designed and tested in Chrome on Linux
through the browser preview.

## Authority

The orchestrator may decide the layout and detail of each screen within the approved mockups
and DESIGN.md, the order of screens, and how work is split. It may cut a visual that fails the
"answers a question the commander acts on" test.

One planned stop: after astromech-irtr the orchestrator sends Sumeet the preview links and
waits. It plans the Now page and the screens only after his answer, and records that answer
as a decision.

Back to Sumeet at any time: a change to navigation or the
default page beyond this brief, a new hue or typeface, removing a feature a page has today,
and any screen whose built result departs from its mockup in kind, not detail.
