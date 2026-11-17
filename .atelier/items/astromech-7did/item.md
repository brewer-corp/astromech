---
schema: atelier.item
schema_version: 3
id: astromech-7did
title: 'Engineers: search by module type answers where to engineer it'
status: in_review
mission: astromech-xajc
labels:
- size-m
- feature
priority: high
blocked_by:
- astromech-3kf5
- astromech-5a01
implementers:
- thread:delegated-task:command%3Amcp%3A7af80d85-cde0-4a8e-9929-43c0997f41ce%3Adelegate-task%3Ae656cbbe-5a7a-4232-82ca-3aa67f871c76
ready:
  by: Sumeet
  at: 2026-11-17T05:23:28.056709303Z
trivial: null
caused_by: null
created_at: 2026-11-17T05:20:19.387957133Z
---

## Goal

"Where do I go to engineer my lasers?" is answered by typing "laser". Sumeet, 16 Nov 2026: it is the most common reason to open the Engineers page.

## Done when

- [x] A search box at the top of Engineering > Engineers takes a module type in the commander's words: "laser", "fsd", "shield", "beam", "thrusters". A small committed table maps words to the catalog's module types; an unknown word says so and lists near matches.
- [x] A new window command in the command layer returns, per engineer who offers a matched module: the best grade per module type, the commander's rank and unlock state with that engineer, the engineer's system, and its distance from the commander's current system (not Sol). It reads `data/catalog/engineering.json` and reuses the core's `engineers::offers`. It has a test over the recorded preview world and an `every_window_command` entry.
- [x] The main answer is the best grade the commander can get today: among unlocked engineers, the highest grade their current rank allows, nearest first, with Set destination and Add stop beside it. Engineers who reach higher show the rank or the unlock step still needed. With none unlocked, the main answer is the nearest to unlock and its next step.
- [x] The rest read as answers in three groups, as `tone5-engineers-laser-2560.png`: what they reach (orange hexagons and a line such as "Pulse 4 · burst 3 · beam 3"), the engineer and the commander's rank (white hexagons, or the invite step), and where (system with copy, distance).
- [x] Rank and grade use the engineer hexagon (`app/public/engineers/access.svg`) through one shared component, also used wherever the page shows a rank.
- [x] Suit engineers match by modification ("range", "magazine", "night vision"); the page says so when a weapon word finds none.
- [x] Clearing the box returns the page as it was. Browser tests cover a match, no match, and rank below the best grade, on recorded replies. The Engineers guide page says how to search.

## Scope

The search, its command and the hexagon component. The unlock tree is astromech-m6gw, which follows this item.

## Context

Spike: branch `spike/nav-rework` at `dea5f0bb` (never merges): mockups in `app/src/mockups/nav/`, captures in `docs/design/nav-spike/`. Sumeet approved Variant B and the `tone5` versions on 17 Nov 2026 (mission notes). The mockup named the nearest engineer who reaches grade 5 while the commander was rank 3 with him; that is wrong and the third line above replaces it. `lib/engineerSpecialties.ts` lists only two module types per engineer and cannot drive this.

## Report

Typing "laser" on Engineering > Engineers now answers where to engineer it. Commit `6cc9b4bf`.

**Done when, line by line**

1. The search box sits at the top of the page (`app/src/modules/unlocks/View.svelte`). The word
   table is `data/catalog/engineer-search.json`: 42 entries of words to `engineering.json` group
   ids, each with the short label the grade line uses. A trailing "s" also matches, and a full
   catalog group name ("fragment cannon") matches without an entry. An unknown word lists up to
   five near matches as buttons ("lazer" offers "laser").
2. The new command is `engineering_search {query}` (`crates/astromech-api/src/commands.rs`). The
   ranking lives in `crates/astromech-engineering/src/search.rs`. Per engineer it returns the best
   grade per type (the max of `engineers::offers` over the group's blueprints), status, rank and
   step from the unlock tree, the grade available today, the system, and the distance from the
   journal's current system. Tests:
   - five Rust tests in `search.rs`, one of which checks that every table type and every engineer
     it names resolve to `engineers.json` and the unlock data;
   - an `every_window_command` entry (fsd from Sol: Farseer at 131.43 ly is the nearest to unlock);
   - recorded fixtures `app/src/preview/fixtures/engineer-search.json` from `worlds/preview/unlocks`
     (fsd, laser, range, pistol, lazer), checked by `preview_fixtures.rs`. That world gained a
     `Location` line so distances show. `unlocks.json` didn't change.
3. The main answer is computed in Rust. It is the highest `min(best, rank)` among unlocked
   engineers, nearest first, with Set destination (the primary button) and Add stop. An unlocked
   engineer whose rank caps the grade shows "Rank 2 · grade 5 at rank 5". With none unlocked, the
   main answer is the engineer nearest to unlocking, with its step. "Nearest" means invited, then
   known, then locked but workable now, then distance. A barred engineer is never the main answer.
4. Rows follow `tone5-engineers-laser-2560.png` in three groups: Reaches, Engineer · your rank, and
   Where. Reaches shows orange hexagons for grades you can get today, Orange Mid for higher grades
   the engineer reaches, and a line such as "Pulse 4 · burst 3 · beam 3" or "Grade 5 on all three".
   The answer is as wide as its list, so the actions stay next to what they act on.
5. `app/src/lib/ui/Hexes.svelte` is the one hexagon component. It is used for the search's ranks and
   grades and for the tree cards' rank, which replaces the `.badge` spans. Cards now show lit and
   unlit hexagons of 5. A suit engineer, who has no ranks, shows one.
6. A word that names no ship module is matched against suit modifications from `odyssey.json`
   ("range" finds Greater Range, "night vision" finds Night Vision). A suit or weapon word in the
   table's `suit_words` ("pistol", "Maverick") that matches nothing says suit engineers are
   searched by modification, and offers range, magazine and night vision.
7. Clearing the box (× or Escape) brings the tree back and the hash back to `#/unlocks`. The search
   is kept in the link (`#/unlocks?q=laser`, via replaceState). Browser tests in
   `View.browser.test.ts` cover a match with rank below the best grade (fsd) followed by clearing,
   and no match (lazer → laser, then pistol → range). The guide's Engineers section gained "Search
   by module".

**Captures:** `docs/design/engineer-search/` holds the tree before and after (the hexagons),
laser, fsd and range at 2560, fsd at 1100 and lazer.

**Checks:**
- Passed: cargo fmt, clippy `-D warnings`, `cargo test --workspace`, check-deps, check-guide,
  `pnpm lint`, `pnpm check`, `pnpm build`, check-dist, and the journeys (7 of 7).
- `pnpm test`: 751 of 752 pass. The one failure is the Sell dock-width test in
  `App.browser.test.ts`, already filed as astromech-7obs (fails on main, held by another thread).
- Not run: `cargo deny`, since no dependency changed.

**Gaps and follow-ups**
- A suit engineer has no distance. `data/catalog/engineers.json` holds only ship engineers'
  coordinates, and `odyssey.json` has each suit engineer's `id64` but no position.
  `distance_ly` is null for them, the page leaves it blank, and the guide says so. The fix is for
  `scripts/gen-odyssey.py` (or gen-starmap) to write their coordinates from the Spansh dump. This
  is worth an item; I didn't file one from this branch.
- The page is not yet on a layout component: the Engineers rebuild is astromech-m6gw.
- Rows truncate a long unlock step with an ellipsis, and a Tip shows the full text.


### Responses

1-4sr2.1: fixed in 11d7c60a (today's ties go nearest first; regression `equal_grades_today_go_nearest_first_whatever_they_reach_later`)
1-4sr2.2: fixed in 11d7c60a (unlock answer picked across non-barred hits; `a_barred_engineer_is_never_the_main_answer` covers cannons and the all-barred case)
1-4sr2.3: fixed in 11d7c60a (`SearchHit.unsure` from `unconfirmed_mods`, shown as the tree's "(?)" with its Tip; browser test on Rosa Dayette's recorded range offer)
1-4sr2.4: fixed in 11d7c60a (only an answer for the words in the box is shown, else "Searching the engineers…"; browser test holds the laser reply at the transport)
