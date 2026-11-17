---
schema: atelier.item
schema_version: 3
id: astromech-8e22
title: 'Increment 2 checkpoint: the new frame, colours and tabs seen together'
status: approved
mission: astromech-xajc
labels:
- size-s
- feature
priority: high
blocked_by:
- astromech-4wi1
- astromech-uc7i
- astromech-5a01
- astromech-7did
- astromech-3ca6
- astromech-fom5
- astromech-srab
- astromech-7obs
implementers:
- thread:delegated-task:command%3Amcp%3A7af80d85-cde0-4a8e-9929-43c0997f41ce%3Adelegate-task%3Af88202b7-1e1b-4c33-90e8-dc41206c38b6
ready:
  by: Sumeet
  at: 2026-11-17T05:23:28.056709303Z
trivial: null
caused_by: null
created_at: 2026-11-17T05:20:19.503240761Z
---

## Goal

The reworked foundation is seen whole before the screens are built on it: the new colours, the text size, the route sidebar, the new tabs and the grouped rows, on the real pages.

## Done when

- [x] An agent that built none of increment 2 captures, in the browser preview, the Sell finder (a result selected, sidebar open), Materials, Engineers with "laser" searched, Ships, Suits, Powerplay, Settings and My controller at 1100 x 700 and 1920 x 1080 (100 percent), 2560 x 1440 (145 percent) and 3840 x 2160 (180 percent), into `docs/design/increment2/`.
- [x] Each capture is measured as astromech-irtr's were: no sideways scroll, no clipped or overlapping text, no readable text under 4.5:1, no action further than one column from what it acts on. A failure is filed as a bug in the mission. Before this item closes each bug is either fixed and its page captured again, or, where a planned screen item rebuilds the page, handed to that item by the lead (astromech-lf3a to rj6i, 8og8 to nnn0, 5g9w to 8wr4), or is older than increment 2 and filed on its own (astromech-wo8s).
- [x] On each page the brightest things are the main value, what can be changed and what needs the commander; a page where something else on the page out-shines them is filed. The sidebar is judged once, as frame: its next stop is its brightest thing and it has no fill.
- [x] Preview links for the Sell finder and Engineers are published with `scripts/preview-publish` and recorded with the capture paths as a note on the mission.
- [x] The measured dock width, header second-row width and rows-visible figures in DESIGN.md match what the captures show.

## Scope

Checking and evidence. It changes no page; it files what it finds.

## Context

Mission astromech-xajc. This checkpoint does not stop the mission: the screens start when it merges, and Sumeet sees the links when he next looks.

## Report

**Increment 2 holds together. Of the five failures the checkpoint found, one is fixed and captured again, and the lead has decided the other four.** The frame, the colours and the new tabs hold at all four sizes. The five bugs, and what became of each:

- **astromech-vbz3, Engineers:** fixed on main (ca33619d). After merging main, I captured Engineers with "laser" searched again at all four sizes. The audit lists nothing clipped, and Mel Brandon's step now reads in full over two lines ("…raise your rank with them past grade 3.").
- **astromech-lf3a, Powerplay:** handed to astromech-rj6i.
- **astromech-8og8, Suits:** handed to astromech-nnn0.
- **astromech-5g9w, Ships:** handed to astromech-8wr4.
- **astromech-wo8s, My controller:** older than increment 2 and filed on its own at low priority.

I captured the three handed-over pages again on the merged branch, and they are unchanged:

- Powerplay still cuts 10 texts at 1920 x 1080 and 3 at 3840 x 2160.
- Suits' Remove is still about 1,100 layout px from its plan at 2560 x 1440.
- Ships' Open in links are still about 1,000 layout px from the ship at 2560 x 1440.

The second Done when line is the lead's rewording, which allows these hand-overs.

**Captures.** There are 35 captures in `docs/design/increment2/`, taken in headless Chrome at device scale 1 from `pnpm dev` with `?core=ready&route=exact&textsize=<step>`:

- the eight pages at 1100 x 700 and 1920 x 1080 (100 percent), 2560 x 1440 (145) and 3840 x 2160 (180). Sell has its first result selected.
- `sell-selected-route-over-1100x700.png`. 1100 is below the dock width, so "sidebar open" there means opened over the page from the strip.
- `sell-anywhere-no-route-{1100x700,2560x1440}.png`, for the rows-visible count.

Each capture loads fresh in its own browser context, and the run fails if the page drew at a step other than the one asked for. I built none of increment 2.

**Tool.** astromech-irtr's measuring script was never committed, so I wrote `scripts/browser/audit.mjs`, driven by `docs/design/increment2/plan.json`. It writes `report.json` with these, per capture:

- sideways scroll anywhere;
- text cut by an ancestor that hides overflow, or by an ellipsis;
- overlapping text or controls in view;
- text under 12 layout px;
- contrast under 4.5:1 against the composited background;
- a row action more than 360 layout px from the row's last text;
- the brightest colours in view, with their texts, and the solid fills, the sidebar's apart.

DESIGN.md marks Grey Dim, Orange Dim and Placeholder as state rather than text to read, so they are listed apart as exempt. That covers the past route stops and Find or plot's placeholder. Every lead was checked against the PNG. Three kinds of false hit were removed from the tool:

- a glyph box taller than its line box;
- a wrapped title's lines touching;
- content inside an inner scroller, such as Ships' `.stage`.

**Results, as first captured.**

- No capture scrolls sideways.
- No readable text is under 4.5:1.
- Nothing overlaps.
- The 360 px row check finds nothing.

By eye, five pages fail:

- **Powerplay (astromech-lf3a, caused by uc7i).** At 1920 x 1080 beside the docked sidebar, This system's name and every Where to work system and state end in "...". At 3840 x 2160 one system and two states are still cut. At 2560 x 1440, Where to work sits under My standing and nothing is cut.
- **Engineers (astromech-vbz3, caused by 7did).** Mel Brandon's step left to unlock ("Introduction from Elvira Martuuk: raise your rank with them past ...") was cut at every size, with the full text only in a Tip. It is fixed, and the four Engineers captures were taken again after merging main.
- **Suits (astromech-8og8).** Plan blocks run the full column width, so Remove is about 1,100 layout px from its plan's name at 2560 and about 1,650 at 3840. The shopping list spreads its columns the same way. The 360 px check misses this because the blocks are not rows; the 3840 capture shows it.
- **Ships (astromech-5g9w).** Open in Coriolis and EDSY sit about 1,000 layout px from the ship's name at 2560 and further at 3840. The stat bands stretch too.
- **My controller (astromech-wo8s, low).** At 1100 x 700 the sheet's bound actions draw at 8 px. The sheet scales with its width, not with Text size, so it reads at 14 px at 1920 and 18 px at 2560. Those two sizes show up in `small` only because the check divides by the step. This predates increment 2.

Judged and not filed: on Engineers, Set destination ends the nearest-to-unlock card about 620 layout px right of "Wyrd 75.5 ly", at every size. The card is the list's width and does not grow with the window, and the button acts on the card's engineer, as a side panel's actions do. Every capture was opened by eye at 1100 and 3840, and every page at 2560.

**Brightness.** No page has something out-shining its main value, what can change, or what needs the commander, so nothing was filed:

| Page | Brightest |
|---|---|
| Sell | Good Landing (white) and 211,587 (Orange Lit) in the panel, and the ranked prices; Set destination is the page's one fill |
| Materials | the counts and names of held materials; zero counts are Grey Dim |
| Engineers | The Dweller and 75.5 ly; Set destination is the fill |
| Ships | the ship's name |
| Suits | plan names; a plan with everything in the locker is lifted to white on purpose (`.pick.ready`), with Add as the fill |
| Settings | the orange cells, every one something the commander can change |
| My controller | the controller's name |
| Powerplay | the commander's name; merits and the activity names share Options Grey, so the standing does not stand out but nothing out-shines it (rj6i makes standing the main value) |

**The sidebar, judged once as frame:**

- The next stop (Arietis Sector LD-H a11-1, Orange Lit at Title size, on a raised row) is its brightest thing.
- It has no fill. The only fills in any capture are the active top tab and each page's primary button.
- Opened over the page at 1100, the strip's chevron becomes an orange square while open (`Strip.svelte` `.fold.open`). That is a toggle's on state on the strip, not a fill in the sidebar.

**DESIGN.md figures, against the captures:**

- **Header.** It is 108 px (stacked) at 1100 x 700 and 64 px at the other three sizes. One row measured from 1529, 2216 and 2750 px at 100, 145 and 180 percent, by binary search on fresh loads. DESIGN.md has 2751 at 180, which is within rounding of 95.5 rem, so I left it.
- **Dock width.** The sidebar is a strip at 1100 and docked at 1920, 2560 and 3840. It docks from 1659, 2406 and 2987 px (1659 at 100 percent), as DOCK says.
- **Rows visible.** These did not match, and DESIGN.md is corrected. On opening, 5 rows show at 1100 and 11 at 2560, as written. Scrolled with the list's head row still in view, the captures show 8 at 1100, in a 524 px column of 60 px rows, not 7. At 2560 they give 14 (897 px column) where DESIGN.md had 13. The 2560 sample's 12 rows all fit, so 14 is worked out from the column's height. astromech-3ca6 counted 7 and 13 before the route sidebar and strip changed the frame, and astromech-fom5 re-measured only the opening counts.

**Links.** Both are built with `scripts/preview-publish --query route=exact` and checked to open on their page:

- Sell finder: https://claude.ai/artifact/QNYTRUABGUvbUkhYyEfsEb
- Engineers with "laser": https://claude.ai/artifact/2As2BUSDRzaNU8F7mgDJGB

They are recorded with the capture paths as a `[note]` in `.atelier/missions/astromech-xajc/notes.md`, which reaches main when this item merges. Both are private until shared.
