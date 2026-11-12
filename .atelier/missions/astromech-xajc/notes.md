# Notes of astromech-xajc

What the old mission record held beyond its mission sections.

## Outcome

A commander opens the window on a page about what they are doing now. Every page outside Controls uses the window well at any size from 1100 x 700 to 4K, shows one obvious main value per element, and uses a screen built for its job where a table was. A sparse page stays sparse: a layout places content, it never adds any. Text is readable from a seat next to the game. The plan, the design-system amendments and the mockups are in .atelier/missions/astromech-xajc/.

## Success criteria

Both cover every page outside Controls (My controller, All keybinds, Presets, Key tester), which get only the frame, the scale and Tip.

- fills-the-window: every page is built on one of the four layout components and passes the page-sizing check (no page file sets its own width or font size); shown by the check being on for every page file except the Controls ones and the frame's two parts in the route module, and by captures of every page at 1100 x 700, 1920 x 1080, 2560 x 1440 and 3840 x 2160 with nothing clipped and no action further than one column from what it acts on.
- glance-test: on every page a reader can point to the main value, the thing they can change and the thing that needs them, without reading; shown by astromech-crxa: expected answers written per page first ('none' where a page has nothing to change or nothing needing attention), a fresh agent answering from the 2560 x 1440 screenshots alone, and every page passing. A page left failing leaves the criterion unmet.

## Notes

- 2026-11-12 [note] Increment 1 checkpoint (astromech-irtr): the browser preview, published with scripts/preview-publish from main at 0033cc0c. Route page (exact-plotter route): https://claude.ai/artifact/Syy9cQByFc1AAqD9UJaVbQ. Materials: https://claude.ai/artifact/1N4Dkdu8crQgLJyxw8KNPH. Both are private to Sumeet until shared. Captures: docs/design/increment1/{route,materials,places-sell,places-sell-selected,trade-commodities,settings-general,powerplay,exploration}-{1100x700,1920x1080,2560x1440,3840x2160}.png, with the per-capture measurements in docs/design/increment1/report.json.
- 2026-11-12 [evidence] Increment 1 checkpoint (astromech-irtr), fills-the-window for the Route page and the converted pages. 32 captures, 8 page states at 4 sizes, taken by an agent that wrote none of increment 1's items. Measured per capture in headless Chrome: no horizontal scroll of the document or main, no text cut by an ancestor, no overlapping text or controls in view, no text below 12 px before scaling, and no row action more than 360 layout px from its row's last text. Looked at by eye at 1100 x 700 and 3840 x 2160: nothing clipped or overlapping, and every action sits at the end of its row, its field or the inputs it submits. No bug filed. Page-sizing, from ESLint's resolved config for every file under app/src/modules: width is on for materials/View, places/FinderPage, the nine finder pages, trade/View and trade/Route, and settings/View (astromech-mwh5); font-size for materials/View, places/Home, settings/View, settings/Callouts, settings/ProspectorThresholds, session/View and mining/View (astromech-z5oh); both for route/View, Timeline, Title, PlotSetup, ImportDialog and NeutronFile (the Route page). No other file is on either list; route/Strip, route/PlotBar and route/Plot (drawn only inside PlotBar) are off. Seen but not counted as problems, since neither moves an action from its row: at 3840 x 2160 Exploration's bodies table spreads across the full width with about 640 px between a body's tags and its value (astromech-dpaz replaces that table), and the Sell finder's From input runs the list's full width.
