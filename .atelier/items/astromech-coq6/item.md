---
schema: atelier.item
schema_version: 3
id: astromech-coq6
title: 'Now page: the window opens on what you are doing'
status: building
mission: astromech-xajc
labels:
- size-l
- feature
priority: high
blocked_by:
- astromech-8e22
implementers:
- thread:delegated-task:command%3Amcp%3A7af80d85-cde0-4a8e-9929-43c0997f41ce%3Adelegate-task%3A26ec5306-653e-4de1-b581-5bc5ed787930
ready:
  by: Sumeet
  at: 2026-11-17T10:51:58.242665710Z
trivial: null
caused_by: null
created_at: 2026-11-10T05:55:38.805008138Z
---

## Goal

The window opens on what the commander is doing now. Today it opens on My controller and files the live views (Station, Session, Mining) under Logbook, so it has no home page (audit, 10 Nov 2026). The Now page follows the game: docked, it is the station and its services board; in space, the system; with Elite not running, the last session. Under that, the few answers that change while playing: what the hold would fetch, this session's earnings, the Powerplay cycle and a mining run. It does not show the next route stop, which the route sidebar shows on every page.

## Done when

- [x] NOW is the first top tab (`groups` in `app/src/lib/modules.ts`) and the window opens on it: `DEFAULT` in `app/src/App.svelte` becomes the new `now` module. NOW holds four page tabs, in this order: Now, Station, Session, Mining. Station, Session and Mining move there from LOGBOOK by their manifests' `group`; LOGBOOK keeps Logbook alone, with no page tab row. Their addresses (`#/station`, `#/session`, `#/mining`) do not change, so every old link still opens them, now under NOW. `#/route` opened cold lands on Now with the sidebar shown; the three `App.browser.test.ts` cases that expect `#/controller` (lines 340, 352, 419) expect `#/now`.
- [x] The header's second-row width is measured again with ten tabs at every Text size step (100, 115, 130, 145, 160, 180), as astromech-5a01 measured nine, and DESIGN.md's Frame table gives the new figures in place of the nine-tab ones. DESIGN.md's Top tabs paragraph says what NOW holds.
- [x] A new module `app/src/modules/now/` on the Board layout and the type roles. Docked (`docked_here` with `docked: true`): the station leads the page as its title: the station type icon (`StationKind.svelte`, the Glyph station set) at 24 px, the name in Title White, then the system with its copy icon and the distance from the star in Label (`dist_from_star_ls`). The station head is one component, `app/src/modules/station/Head.svelte`, which astromech-7vxi reuses on the Station page.
- [x] Docked, the station's services board follows the head, as `docs/design/glow-up-now.png`: one Seam-outlined tile per service in the game's station menu, in this fixed order: Commodities, Refuel, Repair, Restock, Outfitting, Shipyard, Universal Cartographics, Vista Genomics, Material trader, Technology broker, Interstellar Factors, Black market, Search and rescue, Vouchers, Pioneer supplies, Frontline Solutions, Bartender, Crew lounge, Apex Interstellar, Missions (the names `SERVICE_NAMES` in `app/src/lib/station.ts` gives). A service in `DockedHere.services` is lit, its name in Name, Options Grey; an absent one is shown, not omitted, in Grey Dim. The tiles are facts, not controls: no links, no cells. A service the station reports that is not on the list (`dock`, `contacts`, `shop`, carrier services) is not drawn. The board is `app/src/modules/station/Board.svelte` and is drawn on Now only; the Station page (astromech-7vxi) keeps the station's other facts, its market and its outfitting.
- [x] Under the head, Board blocks each with one main value in Value XL, shown only when they have something to say:
  - **Sell your cargo**, while the hold has cargo the commander can sell (`ship_status.sellable` not empty): the best station for the hold from `trade_cargo` (`plan.stations[0]`): what it pays in cr (`total`), and always how much of the hold that covers, "takes all 600 t" or "takes 20 of 200 t" (`sold` against the tonnes in `plan.load`), in Label beside the total; the station lifted White with its system, jumps and pad; the Set destination and Add stop icon buttons beside it; and a link to Places' Sell my cargo finder, which lists the other stations and the best for each commodity. A station that takes part of the hold never reads as what the hold would fetch: the words "takes N of M t" are on the same line as its total. The "takes" words come from one function shared with the finder's row (today inline in `cargoResults`, `app/src/lib/trade.ts:149`), with a table test: the whole hold, part of one commodity, and one commodity of a mixed hold. With no station in reach (`stations` empty) the block says no station within the search's range buys the hold, with the link to the finder and no Set destination or Add stop. The mockup's "cr profit" is not shown: `CargoPlan` has no buy cost, only what a station pays (`app/src/lib/api/types/StationSale.ts`).
  - **This session**: credits per hour (`session_summary.per_hour`), with what was earned and over how long, and the jumps, in Label; a link to the Session page.
  - **Powerplay**, while pledged: merits this cycle (`standing.week_merits`), the time left in the cycle worked out from `standing.week_ends`, and the rank with the merits to the next (`standing.next.to_go`) in Label; a link to Powerplay.
  - **Mining**, while a run is going (`status.mining.started` set): tons refined and tons per hour; a link to Mining.
- [x] The page's one main value, in Orange Lit, is the first of these that shows: the mining run's tons per hour, the hold's best total, the session's credits per hour. The other blocks' values are Options Grey. Orange otherwise only for actions and links. What needs the commander is unsold exploration data at risk, from `exploration_unsold` (`risky: true`): an orange "!" line under the head with the amount and a link to System, as System shows it today.
- [x] The Sell your cargo answer is for the hold, place and ship the commander has now. `trade_cargo` reads all three on each call (the origin, range and filters in `crates/astromech-api/src/lookups.rs:377`, the hold from `Cargo.json`; the search and jumps from that origin and range in `crates/astromech-market/src/cargo.rs`), so the block asks again whenever any of them changes: the page keeps a change key made of `ship_status`'s `sellable` (each commodity and its tonnes), `ship_id`, `pad` and `max_jump_range`, and the current system (`docked_here.system`), read from the polls the page and the frame already make, and asks when the page opens and whenever the key differs from the one its answer was asked with. It never asks on a timer alone, and an unchanged key asks nothing. The core caches each commodity's answer (`cargo.rs:199`), so a re-ask after a jump costs only the new origin's lookups. The key is one pure function with a table test: a jump with the same cargo changes it; the same tonnes of another commodity change it; another ship of the same capacity changes it; mission cargo or limpets changing (not in `sellable`) does not; identical inputs give the same key.
- [x] While a new answer is on its way, the old one stays in place in Grey Dim with its Set destination and Add stop disabled, as DESIGN.md's Loading state says for a refresh, and "Asking Ardent…" in Label Grey says why; with no old answer the line shows alone. A failed ask shows the shared Failure line with TRY AGAIN in that block only; the other blocks stand. With no network the block says it could not reach Ardent.
- [x] In space (`docked: false` with a system): the system leads as the title (Title White, copy icon), with no board; the blocks as above.
- [x] Elite not running (`status.game` is not `running`): the title is "Elite isn't running" in Title White, with the last place from `docked_here` in Label ("Last docked at Ehrlich City, Liaedin" or "Last in Liaedin"). Only This session (as the last session) and Powerplay show; the hold and the mining run are not known.
- [x] No game in the journal yet (`?core=empty`): one prose line saying so, and nothing else. Loading: each block one Label Grey line naming what it reads. A block whose poll fails keeps its last answer with the shared Stale line, or shows the failure in words with no action, since it re-reads on its own.
- [x] Sparse stays sparse: nothing is printed to explain a block; what a figure counts goes in its Tip (the session's earnings rule, now in `session/View.svelte`'s `EARNED` Tip, moves with it).
- [x] The browser preview can show every state: its fake core answers `status` with Elite running under a new `?game=running` option (`app/src/preview/fakecore.ts`, `status`, line 530 answers `not_running` always), listed with the other options in CONTRIBUTING.md. The in-space state needs a recorded not-docked `docked_here`: the `station` world under `worlds/preview/` gains an `Undocked` after its docking and the fixture `app/src/preview/fixtures/station.json` gains that reply, dumped through `app/src-tauri/tests/preview_fixtures.rs`; the fixture drift test passes.
- [x] Browser tests, in `app/src/modules/now/View.browser.test.ts`, on the recorded replies: docked shows the board with absent services dim and the cargo sale with its "takes all 600 t" (the recorded `trade.json` `cargo` answer); a jump in the fake core with the cargo unchanged (`fakeJump`, `app/src/preview/fakecore.ts:421`, which today moves only the route's `here`: this item makes the fake's `docked_here` answer undocked in the new system after it, as the core does after an `FSDJump`) asks `trade_cargo` again and shows the old answer dimmed until the new one comes; an unchanged poll asks nothing; an empty `stations` (a reply written in the test, an empty state the fixtures lack) shows no recommendation and no destination action; a failed `trade_cargo` leaves the other blocks; in space shows no board; Elite not running shows the last session and no sale; an old `#/session` link opens Session under NOW. A reply written in a test only sets Elite running or not and the failure.
- [x] Checked in the browser preview at 1100 x 700 and 1920 x 1080 at 100 percent Text size and at 2560 x 1440 at 145 percent, docked, in space and not running: with a route followed the sidebar is docked at 1920 and 2560 and is the strip at 1100. Nothing is clipped or scrolls sideways; at 1100 x 700 the blocks wrap to two columns and the board follows them, as `docs/design/glow-up-now-1100.png`. Captures at 1100 x 700 and 2560 x 1440 in `docs/design/now/` for each state, and before captures of the window opening on My controller. Each size is measured with `node scripts/browser/audit.mjs` (astromech-8e22's tool) from a plan of this page's states saved beside the captures (`plan.json`), and lists nothing under `sideways`, `clipped`, `overlaps`, `small`, `far` or `contrast` (outside its exempt list), or the Report says why a listing is not a fault; its `bright` list for the page, the sidebar apart, leads with the main value, what can be changed and what needs the commander.
- [x] Both parts of the page-sizing check are on for `app/src/modules/now/**`, `station/Head.svelte` and `station/Board.svelte` in `PAGE_SIZING` (`app/eslint.config.js`); none of them is in `TOKENS_EXEMPT`.
- [x] A new guide page `docs/guide/now.md`, listed in `docs/guide/index.md`, says what each state shows and where each figure comes from; `using-the-app.md` says the window opens on Now and lists the ten tabs; `logbook.md`'s Station and Session sections move to `now.md` and the two manifests' `help` become `now.md#station` and `now.md#session` (Mining keeps `mining.md`), and `python3 scripts/check-guide.py` passes.
- [x] `pnpm check`, `pnpm lint` and `pnpm test` pass, then the journeys (`pnpm exec tsc -p journeys` and `node --test 'journeys/*.journey.ts'`; `harness.journey.ts` and `voice.journey.ts` open `controller` by name and need no change). One full test run at a time: each agent here has 4 GB.

## Scope

The NOW tab, the default page, the move of Station, Session and Mining under NOW, the Now page, the shared station head and services board, and the header's re-measure. The Station page's own body is astromech-7vxi; Session's is astromech-fzhp; Mining's moves onto a layout in astromech-gvy0. No new core command: every block reads a reply the window already has.

## Context

Mockups: `docs/design/glow-up-now.png` (2560 x 1440) and `glow-up-now-1100.png`, drawn before revision 5. Their layout stands; their colours, filled cells and NEXT STOP block do not: the services tiles are Seam outlines on Void, not cells (they are read-only), and the next stop is the sidebar's. DESIGN.md revision 5 is binding: Orange Means Act, the Lifted Name Rule (the station a Now answer names is White), One Fill Per Page (no fill on this page), the station type icons.

Replies: `docked_here` (`app/src/lib/api/types/DockedHere.ts`: station, system, station_type, services, dist_from_star_ls), `trade_cargo` (`CargoSale`, `app/src/lib/trade.ts:123`), `session_summary` (`Session.ts`), `powerplay_overview` (`StandingView.ts`: week_merits, week_ends, next), `status` (`CoreStatus.ts`: game, mining), `ship_status` (cargo tonnes, already polled by the frame's status bar), `exploration_unsold` (`UnsoldData.ts`: risky). The default page is `DEFAULT` in `App.svelte:28`; old links go through its `MOVED` map, which needs nothing here since the addresses stay.

The mockup's "! Refuel here before you undock" line is left out: nothing in the core decides it. The route strip's "Refuel before the next jump" comes from the plotter's per-stop fuel flag (`app/src/modules/route/Strip.svelte:62`), which is the sidebar's.

Its files start outside `TOKENS_EXEMPT`; the moved Station, Session and Mining pages keep their own entries until astromech-7vxi, astromech-fzhp and astromech-gvy0.

## Report

**The window now opens on Now, which follows the game: docked, in space, or Elite not running.** NOW is the first of ten top tabs and the default page. It holds Now, Station, Session and Mining, whose addresses are unchanged. LOGBOOK keeps the Logbook alone.

**One step goes beyond the item's wording.** The browser preview has a second new option, `?docked=no`, besides `?game=running`. The audit plan can only click and wait, so it can't reach the in-space state through `fakeJump`. `?docked=no` answers `docked_here` with the newly recorded undocked reply. Both options are listed in CONTRIBUTING.md and at the top of `fakecore.ts`.

### What is where

- **Page:** `app/src/modules/now/`. `View.svelte` is the page, `Sale.svelte` the Sell your cargo block, `Block.svelte` one answer block, and `poll.svelte.ts` the page's own polls (docked_here, session_summary and powerplay_overview every 5 s, exploration_unsold every 15 s, as the pages they summarise poll them).
- **Station head and services board:** `station/Head.svelte` and `station/Board.svelte`. The board is drawn on Now only.
- **Frame polls:** the status and ship polls the status bar used are now shared by context (`app/src/lib/frame.svelte.ts`, after `route/follow.svelte.ts`). The page adds no poll for them. A page rendered alone in a test starts its own.
- **Answer blocks:** `lib/ui/layout/Blocks.svelte` is the Board's row of answer blocks: four across, else two, else one, never three over one. It is noted in DESIGN.md's Page layouts. At 1100 x 700 the blocks are two columns and the board follows them. `Tiles` gained a `list` option so the board is a list of services.

### Sell your cargo

- **Same answer as the finder:** it asks `trade_cargo` with the Sell my cargo finder's saved `trade.cargo` filters, so its station is that finder's first row.
- **When it asks:** the change key is `saleKey()` in `lib/now.ts`, with the table test the item lists in `now.test.ts`. The block asks when it first draws and when the key differs from the last key *asked*, not the last answered. So a failed ask isn't repeated on every poll; TRY AGAIN repeats it.
- **No double ask on opening:** the block waits until `docked_here` has answered once, so opening the page asks once.
- **A missed frame read:** the page keeps the last answered status and ship, as its own polls keep theirs. So one failed `status` or `ship_status` read neither hides the blocks nor asks again. This was found in review and fixed test first.
- **While it asks:** the old total drops Orange Lit and is Grey Dim, and the jump test checks that colour.
- **Filters:** a failed read of the saved filters fails the ask, with TRY AGAIN. It doesn't fall back to defaults that would disagree with the finder.
- **"takes" words:** they come from `takes()` in `lib/trade.ts`, which the finder's row now uses too, with its table test.
- **No network:** the block shows the core's own words: "Could not ask Ardent Insight: Cannot reach Ardent Insight. Check your internet connection…".

### Main value and warning

- **Orange Lit:** whether a block "shows" its figure means the figure is on screen. With no station in reach the sale has no total, so the session takes Orange Lit.
- **Exploration data at risk:** "! 38.1 M cr of exploration data aboard, unsold. Dying loses all of it." with a link to System. No test or capture covers it: the recorded `exploration_unsold` has `risky: false`, and the test rules allow written replies only for Elite running and failures. I looked at it once at 1100 x 700, with a local change to the sample that I did not keep. It reads on one line, and the audit listed nothing.

### Station type icons (own commit, d66ee9fd)

`StationKind` drew one starport icon for every starport. It now uses the Coriolis, Orbis, Ocellus and asteroid base icons astromech-dsyu drew, and DESIGN.md says these replace the old kinds. It reads the journal's StationType ("Bernal", "CraterOutpost", "FleetCarrier") as well as Spansh's names, with a table test, and takes a size for 24 px beside a title. The finders' rows change with it.

### Preview and fixture

- **Recorded reply:** the `station` world gains an `Undocked` (mark `undocked`), and `station.json` gains `undocked`, dumped by `preview_fixtures.rs`. The drift test passes.
- **Tool fix (own commit):** `ASTROMECH_UPDATE_FIXTURES=1` panicked with "no /undocked to write" on a newly added part. `put()` now adds a missing key to its parent object.
- **`?game=running`:** the status says Elite is running. The mining run follows the dock as the core's does: `MiningRun` resets on `Docked`, so it is empty while docked.
- **`fakeJump`:** now undocks the commander in the new system, as the core does after an `FSDJump`.

### Header

Measured by bisecting the window width in the preview at each step. One row needs 99.75 rem: 1596 px at 100 percent, 1834 at 115, 2074 at 130, 2313 at 145, 2554 at 160 and 2872 at 180. DESIGN.md's Frame table and `using-the-app.md` give the new figures.

### Checks

- **Results:** `pnpm check`, `pnpm lint` and `pnpm test` pass (93 files, 788 tests). So do the journeys (7), `python3 scripts/check-guide.py`, `pnpm build` with `scripts/check-dist.py`, `pnpm bridge-check`, and the fixture drift test after the dump.
- **Order:** the blocks are in the item's order: Sell your cargo, This session, Powerplay, Mining.
- **V1-DESIGN.md:** its navigation and its decision on the default screen now say the window opens on Now. The old decision is kept, marked as changed.
- **Lint lists:** both page-sizing parts and the token check are on for `now/**` and `station/{Head,Board}.svelte`, confirmed with `eslint --print-config`. Station's `TOKENS_EXEMPT` entry is narrowed to `station/{View,Market}.svelte`.
- **Browser tests:** in `now/View.browser.test.ts`. Each written reply only sets the failure, the empty `stations`, or holds the second answer back, which is the recorded answer released later.

### Captures and audit

`docs/design/now/` holds docked, in space and Elite not running, each at 1100 x 700, 1920 x 1080 and 2560 x 1440 at 145 percent, with the exact route followed. `plan.json` and `report.json` are beside them. The before captures of the window opening on My controller are `before-opening-*` (`plan-before.json`, `before-report.json`).

- **Audit:** every capture lists nothing under sideways, clipped, overlaps, small, contrast or far.
- **Sidebar:** docked at 1920 and 2560, and the strip at 1100.
- **`bright` list:** it is sorted by luminance, so it leads with the White title and the White lifted station name. That is what DESIGN.md's rules make them, so it is not a fault. The page's only Orange Lit text is its main value in each state: the hold's 183,674,700 docked, 18 t per hour in space (the sample mining run), and 14.5 M cr per hour not running. The orange beyond that is links, icon buttons and the at-risk line.

### Seen, not changed

- **"0 h 0 min left in the cycle":** shown in the preview, on Powerplay too. The recorded world's clock is October, and the core works `week_ends` out from the current time (`astromech-powerplay` `standing.rs:217`), so live it shows the real time left.
- **Audit tool:** `scripts/browser/audit.mjs` takes the repo root as three folders above the plan, so a plan has to sit at `docs/design/<name>/plan.json`.
- **No Finish setup width:** Finish setup's width is not measured. The preview has no scenario with setup unfinished after onboarding, as astromech-5a01 found.

