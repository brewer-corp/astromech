---
schema: atelier.review
schema_version: 1
id: astromech-w6gh
title: finding on astromech-sllo @1fd71219
target: astromech-sllo
identity: reviewer-fable-sllo
commit: 1fd712195d1b0ebd90f65ccdd3d7f695e3f16e83
created_at: 2026-11-05T18:50:34.587364698Z
event:
  event: finding
  recommendation: defer
---

low: app/src/tabrows.test.ts:21 the counter skips a row when any item holds an img/svg/canvas or when the items' classes differ, so the Ships rail (nav.rail: aria-current=page links styled as page tabs) and the Presets rows are excluded by an accident of their markup rather than by what they are; a future tab row carrying glyphs, or with a 'now'-like class on one tab, would pass. Sound today (every class:on, aria-selected and aria-current row in app/src was checked; the rail is a ship picker, not a navigation level) but exclude the rail by nav[aria-label="Your ships"] and drop the picture test when the test is next touched.
