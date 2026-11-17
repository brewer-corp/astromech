# Glance test (astromech-crxa)

The mission's glance-test criterion asks whether a reader can point to each page's main value,
the thing they can change and the thing that needs them, without reading. This file holds the
expected answers, written down before any reader saw the captures. It then holds the reader's
answers and the marks against them.

## The captures

There are 33 page states, each at 2560 x 1440 at 145 percent Text size. Thirty are astromech-gvy0's
captures in this folder (`<name>-2560x1440.png`, taken at 8c41439c). Three states those lacked
were taken the same way: Now in space, Now with Elite not running, and Engineers searched for
"laser". They are in `glance/`, made by `node scripts/browser/audit.mjs --plan
docs/design/final/glance-plan.json`. `app/src` has not changed since 8c41439c.

The reader got copies named `01.png` to `33.png`, in the order of the table below, so a file
name gave away nothing about its state.

## How answers are marked

- Each page is asked three questions. The page passes only if all three answers pass.
- An answer passes when it points to the element the sheet names, by its figure, words or
  position. Different wording for the same element is fine. Where the sheet lists more than one
  element ("or"), any one of them passes.
- An answer that names a different element fails, even if that element is on the sheet for
  another question.
- Where the sheet says "none", "none" passes. Any element named in its place fails unless the
  sheet lists it as an alternative.
- The route sidebar on the left is frame. It has one entry of its own (row 0), asked once. Any
  page answer that points into the sidebar fails for that page. The header, the top and page
  tabs, Find or plot and the status bar at the bottom are frame too. An answer pointing there
  fails as well.
- Moving between pages (a tab or a link to another page) is not a "thing you can change". Picking
  a row in a list with a side panel is one, since the panel follows the pick.

## Where the expected answers come from

Each entry comes from the page's screen item's Done when lines (its "brightest things" line
where it has one) and its guide page, read for the state in the capture. The item IDs are given.
Where an item names no answer for a question, the guide page's account of the page's job is
used, and the entry says so.

## Answer sheet

| # | Page and state | Capture | Main value | Can change | Needs the commander |
|---|---|---|---|---|---|
| 0 | Route sidebar (frame), asked once on 01 | every capture | The next stop, Arietis Sector LD-H a11-1 (astromech-uc7i, crxa) | The destination, Maia (the "Sol to Maia" line), or the plotter choice (Exact, By Spansh, Replot from here) | The refuel warnings: "fuel" at Hyades Sector DB-W b2-6, "neutron boost and fuel" at 98 k Tauri |
| 1 | Now, docked | `now-2560x1440.png` | The hold's best total, 183,674,700 cr (Sell your cargo) (coq6: mining rate, else the hold's total, else credits per hour; no mining run while docked) | Set destination or Add stop beside Good Landing (coq6) | none (coq6: only exploration data at risk, which this state has not got) |
| 2 | Now, in space | `glance/now-space-2560x1440.png` | The mining rate, 18 t per hour (coq6: first is the mining run's tons per hour) | Set destination or Add stop beside Good Landing | none |
| 3 | Now, Elite not running | `glance/now-not-running-2560x1440.png` | The last session's credits per hour, 14.6 M cr | none (coq6: only Last session and Powerplay show; both are facts with a link to their page) | none |
| 4 | Station, docked | `station-2560x1440.png` | The gain on the hold's first line, +3,921 cr/t for Gold (astromech-7vxi) | CHECK AGAIN (7vxi names it and the search box; the search box is below the fold here) | none (7vxi) |
| 5 | Session | `session-2560x1440.png` | Credits per hour, 14,643,677 (astromech-fzhp) | none (fzhp) | none (fzhp) |
| 6 | Mining | `mining-2560x1440.png` | The run's tons: Refined 9 t or Per hour 18 t/h. No item names one; the guide (mining.md) gives the page's job as how much was refined and the rate | none (guide: the page follows the journal; nothing to set) | none |
| 7 | Ships, a module selected | `ships-module-2560x1440.png` | The selected module's planned grade, G5 (Increased range, Mass Manager), with its needs (astromech-8wr4) | The three dropdowns (Increased range, G5, Mass Manager) or ADD TO PLAN (8wr4) | The short materials' "!" marks in the Have column (8wr4) |
| 8 | Stored modules, a module selected | `stored-selected-2560x1440.png` | The selected module's value, 5,000,000 cr (astromech-gvy0: the panel's one value) | Picking a module row (the panel follows it) | none |
| 9 | Suits, a loadout item selected | `suits-selected-2560x1440.png` | The cost to the planned grade, 2,850,000 cr to grade 3 (astromech-nnn0) | The Grade dropdown or ADD TO PLAN (nnn0); REMOVE or PLAN OTHER GEAR also pass | The short items' "!" marks in the Have columns (nnn0) |
| 10 | Materials, Raw | `materials-2560x1440.png` | The held counts in the grade grid (each tile's count, such as Carbon 89). DESIGN.md's Board gives each tile its own main value; the page has no single one | The kind tabs (Raw, Manufactured, Encoded, On foot), or picking a tile | none (DESIGN.md: a material at its cap is the one thing that needs the commander; none is at its cap here) |
| 11 | Engineers, a node selected | `engineers-selected-2560x1440.png` | The selected engineer's distance or step: The Dweller, 75.5 ly, or its unlock step (astromech-m6gw) | SET DESTINATION or ADD STOP (m6gw; the search box is scrolled out of view), or picking a node | The engineers the commander can work on now, the orange names (The Sarge, Marco Qwent, Zacariah Nemo, Mel Brandon, Hera Tani, Etienne Dorn), or the selected engineer's orange unlock step (m6gw) |
| 12 | Engineers, "laser" searched | `glance/engineers-laser-2560x1440.png` | The nearest engineer to unlock, The Dweller, with 75.5 ly (astromech-7did: with none unlocked, the main answer is the nearest to unlock and its next step) | The search box ("laser"), or SET DESTINATION or Add stop | The Dweller's next step (Invitation: deal in stolen or illicit goods at 5 Black Markets), or none: 7did makes the step part of the main answer and names nothing else |
| 13 | Places home | `places-home-2560x1440.png` | none (gvy0: the tiles keep DESIGN.md's Places home spec; each is a finder to open, not a value) | Picking a finder tile (Sell, Buy, Sell my cargo and the rest) | none |
| 14 | Sell (Painite), a row selected | `sell-selected-2560x1440.png` | The best price, 211,587 cr at Good Landing, in the list or the panel (astromech-3ca6, places.md: the ranked value) | The commodity (Painite), CHANGE for the filters, or SET DESTINATION, ADD STOP and the row actions | none; the struck-through M pads (Cousteau Vista and others: a pad too small for the ship, 3ca6) also pass |
| 15 | Buy (Painite), a row selected | `buy-selected-2560x1440.png` | The best price, 44,738 cr at Bogacki Extraction Territory | The commodity, CHANGE, or SET DESTINATION, ADD STOP and the row actions | The low stock against the free hold (31 t and 385 t, marked against 136 t free; n41x: demand against the hold), or the struck-through M pads |
| 16 | Sell my cargo, a row selected | `sell-cargo-selected-2560x1440.png` | The best total for the hold, 183,674,700 cr at Good Landing | CHANGE, or SET DESTINATION, ADD STOP and the row actions | none; the struck-through M pads also pass |
| 17 | Services (Raw material trader), a row selected | `services-selected-2560x1440.png` | The nearest, ~0 jumps to Rescue Ship - Li Qing Jao (places.md: nearest first) | The service dropdown, CHANGE, or SET DESTINATION, ADD STOP and the row actions | The stale report ages in orange (2,042 d old and the others; 3ca6: a report older than 30 days); the struck-through M pads also pass |
| 18 | Outfitting (Pulse Laser 1F Fixed), a row selected | `outfitting-selected-2560x1440.png` | The nearest, ~0 jumps to Titan City | The module, the Class, Rating and Mount choices, CHANGE, or SET DESTINATION, ADD STOP and the row actions | The stale report age (755 d old on G6G-8KJ), or none |
| 19 | Shipyard (Anaconda), a row selected | `shipyard-selected-2560x1440.png` | The nearest, ~0 jumps to Ehrlich City | The ship, CHANGE, or SET DESTINATION, ADD STOP and the row actions | none |
| 20 | Material traders (Encoded), a row selected | `material-traders-selected-2560x1440.png` | The nearest, ~0 jumps to Rescue Ship - Li Qing Jao | The trader choice (Raw, Manufactured, Encoded, Any), CHANGE, or SET DESTINATION, ADD STOP and the row actions | The stale report ages in orange; the struck-through M pads also pass |
| 21 | Raw sites (Brain trees), a row selected | `raw-sites-selected-2560x1440.png` | The nearest, ~3 jumps to B 2 in Ch'i Lingo | The site dropdown, CHANGE, or SET DESTINATION, ADD STOP and the row actions | The stale report age, 44 d old, or none |
| 22 | Faction states (War or Civil War), a row selected | `faction-states-selected-2560x1440.png` | The best score, 150 for Sol (places.md: ranked by score) | The state dropdown, CHANGE, or SET DESTINATION, ADD STOP and the row actions | none |
| 23 | Mining hotspots (Platinum), a row selected | `hotspots-selected-2560x1440.png` | The most hotspots, 10 at 3 A Ring, LHS 3602 | The mineral, CHANGE, or SET DESTINATION, ADD STOP and the row actions | none |
| 24 | Commodities, a single-hop trade route found | `trade-route-2560x1440.png` | The profit per hour, 1,065,123 cr an hour (astromech-el3i) | The inputs (From, Single hop or Loop, Station, Cargo, Credits, Max hop, CHANGE) or FIND ROUTE (el3i) | none (el3i: a pad too small, else none; both pads fit) |
| 25 | Colonisation, a station selected | `colonisation-selected-2560x1440.png` | The site's progress, 42 % built (astromech-m07p) | The site dropdown, CHANGE, or SET DESTINATION, ADD STOP and the row actions (m07p) | The commodities still to buy, the orange figures (Steel 4,912 t, CMM Composite 3,820 t, Ceramic Composites 512 t, and Insulating Membrane 310 t) (m07p) |
| 26 | System, a body selected | `system-selected-2560x1440.png` | The value if mapped, 10,510,589 cr (astromech-dpaz); the selected body's 6,344 cr mapped also passes, as the panel's own main value | Picking a body row, or OPEN IN BIOLOGY (dpaz) | none (dpaz: the unsold-data "!" line, which this state does not show) |
| 27 | Biology, sampling | `biology-2560x1440.png` | The distance against the colony range, 209 m of 500 m (astromech-evj3) | none (evj3: nothing on the page can be changed) | The distance still to go, 291 m to go (evj3) |
| 28 | Powerplay, pledged | `powerplay-2560x1440.png` | The merits to the next rank, 7,871 to rank 10 (astromech-rj6i) | The Where to work choice (Acquire, Reinforce, Undermine), CHANGE, or its row actions (rj6i) | none (rj6i) |
| 29 | Logbook | `logbook-2560x1440.png` | The totals: 4 systems, 3 stations, 6 arrivals, 4 dockings (fzhp: no main value beyond the totals), or none | The search box (fzhp) | none (fzhp) |
| 30 | Settings > General | `settings-2560x1440.png` | none (DESIGN.md: a Form is values to set). Naming the settings' current values as a whole also passes | Any setting: the Text size steps, On or Off, CHANGE, RUN AGAIN, PAUSE, the Speaker, Colour and Speed choices | none |
| 31 | Settings > Data sharing | `datasharing-2560x1440.png` | Whether sharing is on: Share with EDDN, Off (or Upload to EDSM, Off), or none, as a Form | TURN ON (EDDN or EDSM), or the Commander name and API key fields | The last EDDN error, the orange "! Oct 16 … HTTP 503" line (guide: data-sharing.md, the last error) |
| 32 | Settings > Diagnostics | `diagnostics-2560x1440.png` | none (a list of facts), or the status facts as a whole | SAVE DIAGNOSTICS or OPEN LOG FOLDER | none |
| 33 | Settings > Voice tester, a phrase tested | `voicetester-2560x1440.png` | The result for "gear down": the intent (gear down, landing_gear.down), its 100% confidence, or what it would do (Press the keys, then say "Landing gear deployed.") (guide: voice-tester.md) | The phrase box and TEST, or REMOVE on a learned phrase | none |

## The reader's prompt

Each reader is a fresh agent that wrote none of the pages and has not seen this sheet. It gets
this prompt, with `<dir>` the folder of copies, and nothing else:

> You are testing whether an app's pages can be understood at a glance. In `<dir>` there are 33
> screenshots, `01.png` to `33.png`, of a desktop companion app for the game Elite Dangerous,
> each 2560 x 1440. Open and look at each one. Read no other file, run no commands and do not
> look at any code or documentation: the screenshots are all you have.
>
> Every screenshot has the same frame: a header across the top with the app's tabs and a search
> field, a status bar along the bottom, and a ROUTE panel down the left side. Leave the frame out
> of your page answers: answer only about the page in the main area to the right of the ROUTE
> panel and under the tabs.
>
> For each screenshot, answer three questions as quickly as a commander glancing at the window
> would, without reading the page's text closely:
>
> 1. Main value: what is the one value this page is about?
> 2. Can change: what on this page can you change or act on?
> 3. Needs you: what on this page needs your attention now?
>
> Point at each by its visible figure or words ("1,234 cr", "the Grade dropdown"). If a page has
> nothing for a question, answer "none"; that is a fine answer when it is true.
>
> Then answer the same three questions once for the ROUTE panel on the left, looking at `01.png`.
>
> Reply with one line per screenshot, in the form
> `NN | main value | can change | needs you`, then one line `route | main value | can change | needs you`.

## Round 1, 17 Nov 2026

The reader was a fresh general-purpose agent (Claude Opus 5.5, agent a037e1b028b2b33d1). It
got the prompt above with the copies in a scratch folder, and nothing else. Its answers are
below verbatim. M, C and N are the marks for main value, can change and needs you.

### Marking notes, added after the answers came in

The rules above did not say how to mark an answer that names more than one thing. These two
readings are used for every row:

- When the main value answer is "unclear" or offers two candidates, the reader could not
  point to one value, so it fails. This holds even when the sheet's element is among the
  candidates (rows 2, 4, 7, 10).
- A "can change" or "needs you" answer that lists several things passes when the sheet's
  element is among them. With this reading rows 1, 2, 7, 9, 11, 14 to 25, 28, 30, 31 and 33
  pass on can change, and rows 15 and 25 pass on needs you. Read strictly, the extra items
  (links to other pages, copy icons) would have failed them.

The prompt asked what the reader "can change or act on" but did not say that links to other
pages don't count, which the sheet's rules say. That is a fault in the prompt, not the reader.
On row 3 it is the reason can change fails, since the reader named only the two page links.

### Answers and marks

| # | Reader's answer (main value \| can change \| needs you) | M | C | N |
|---|---|---|---|---|
| 1 | 183,674,700 cr (orange, under SELL YOUR CARGO) \| Sell my cargo, Session and Powerplay links; the route-add and flag icons beside Good Landing \| none | pass | pass | pass |
| 2 | unclear: orange 18 t per hour (MINING) pulls the eye over the larger white 183,674,700 cr \| Sell my cargo, Session, Powerplay and Mining links; route-add and flag icons beside Good Landing \| none | **fail** | pass | pass |
| 3 | "Elite isn't running" headline, then 14.6 M cr per hour (orange) \| Session and Powerplay links \| "Elite isn't running" | pass | **fail** | **fail** |
| 4 | unclear: "Test Orbital", or the orange +52,506 cr/t on Tritium \| CHECK AGAIN; SPANSH and INARA links \| market is "4 d ago" (nothing is flagged as needing action) | **fail** | pass | **fail** |
| 5 | 14,643,677 cr per hour \| none \| none | pass | pass | pass |
| 6 | 9 t refined (orange) \| none \| none | pass | pass | pass |
| 7 | unclear: orange G5 (planned) competes with 32.9 ly jump range \| Loadout/Plan toggle; Increased range, G5 and Mass Manager dropdowns; ADD TO PLAN; ENGINEERS; CORIOLIS, EDSY, SPANSH and INARA links; module rows \| the "!" short materials in the Have column (Arsenic !3, Datamined Wake Exceptions !0, and others) | **fail** | pass | pass |
| 8 | 5,000,000 cr \| the stored-module rows (select one) \| none | pass | pass | pass |
| 9 | 2,850,000 cr to grade 3 \| Grade 4 dropdown, ADD TO PLAN, REMOVE (twice), PLAN OTHER GEAR, loadout cards (Explore, Raid, Fight, Spare) \| "6 of 14 short" and the "!" shortfalls (Carbon Fibre Plating !0, NOC Data !0, and others) | pass | pass | pass |
| 10 | unclear: 89 Carbon is the biggest figure; "132 / 6,300" is small \| Raw, Manufactured, Encoded and On foot tabs; the raw-site links (Crystalline shards and others) \| none | **fail** | pass | pass |
| 11 | The Dweller (Wyrd, 75.5 ly) \| SET DESTINATION, ADD STOP, the engineer cards \| the orange unlock steps ("Invitation: Deal in stolen or illicit goods at 5 Black Markets") and the orange engineer names | pass | pass | pass |
| 12 | 75.5 ly to The Dweller \| "laser" search field and its clear button, SET DESTINATION, route-add and flag icons \| "None unlocked yet" | pass | pass | **fail** |
| 13 | none \| the 10 cards (Sell, Buy, Sell my cargo, Services, and the rest) \| none | pass | pass | pass |
| 14 | 211,587 cr \| Painite field, CHANGE, result rows, SET DESTINATION, ADD STOP, route-add and flag icons \| none | pass | pass | pass |
| 15 | 44,738 cr \| Painite field, CHANGE, rows, SET DESTINATION, ADD STOP, icons \| the orange 31 t stock (also 385 t) and "29 d ago" | pass | pass | pass |
| 16 | 183,674,700 cr \| CHANGE, rows, SET DESTINATION, ADD STOP, icons \| none | pass | pass | pass |
| 17 | ~0 jumps \| Raw material trader dropdown, CHANGE, rows, SET DESTINATION, ADD STOP, icons \| the orange ages: 2,042 d old, 1,093 d old, and others | pass | pass | pass |
| 18 | ~0 jumps (5,103,953 cr) \| Pulse Laser field; Class, Rating and Mount toggles; CHANGE; rows; SET DESTINATION; ADD STOP; icons \| 755 d old. Also, the filter says Pulse Laser but the results and detail panel say 5A Frame Shift Drive. | pass | pass | pass |
| 19 | ~0 jumps (45,814,205 cr) \| Anaconda field, CHANGE, rows, SET DESTINATION, ADD STOP, icons \| none | pass | pass | pass |
| 20 | ~0 jumps \| Raw/Manufactured/Encoded/Any toggles, CHANGE, rows, SET DESTINATION, ADD STOP, icons \| the orange ages (2,042 d old and others). Same list as 17. | pass | pass | pass |
| 21 | ~3 jumps \| Brain trees dropdown, CHANGE, rows, SET DESTINATION, ADD STOP, icons \| 44 d old | pass | pass | pass |
| 22 | 150 (score) \| War or Civil War dropdown, CHANGE, rows, SET DESTINATION, ADD STOP, icons \| none | pass | pass | pass |
| 23 | 10 (hotspots) \| Platinum field, CHANGE, rows, SET DESTINATION, ADD STOP, icons \| none | pass | pass | pass |
| 24 | 1,065,123 cr an hour \| From, Station, Cargo, Credits and Max hop fields; Single hop/Loop; CHANGE; FIND ROUTE; Sell, Buy and Sell my cargo links \| none | pass | pass | pass |
| 25 | 42 % built (3 of 4 competes on the right) \| construction-site dropdown, CHANGE, rows, SET DESTINATION, ADD STOP, icons \| the orange "to buy" figures (4,912 t Steel and others) and the orange Insulating Membrane bar (90 t aboard) | pass | pass | pass |
| 26 | 10,510,589 cr if mapped (orange; competes with 5,346,554 cr now) \| body rows, OPEN IN BIOLOGY, Nearest Universal Cartographics and Nearest Vista Genomics links \| 38.7 M cr unsold data; the "Worth mapping" bodies | pass | pass | **fail** |
| 27 | 209 m of 500 m \| none \| 291 m to go (sample 1 of 3) | pass | pass | pass |
| 28 | 7,871 merits to rank 10 \| Acquire/Reinforce/Undermine, CHANGE, SPANSH and INARA links, route-add and flag icons \| 1 d 19 h left; "Undermining is ahead by 9" | pass | pass | **fail** |
| 29 | none (4 systems, 3 stations, 6 arrivals and 4 dockings all weigh the same) \| the "System or station" search field \| none | pass | pass | pass |
| 30 | none \| everything: Text size, Start with Windows, RUN AGAIN, CHANGE (several), PAUSE, Speaker dropdown, PREVIEW, Colour, Speed, Callouts \| none | pass | pass | pass |
| 31 | Share with EDDN: Off \| TURN ON (twice), Commander name with SAVE, API key, SHOW \| "! Oct 16 ... HTTP 503: Service Unavailable" (Last error) | pass | pass | pass |
| 32 | none \| SAVE DIAGNOSTICS, OPEN LOG FOLDER \| none highlighted ("Rejected" and "Not sent" rows sit in plain grey) | pass | pass | pass |
| 33 | "gear down" result (100 %) \| gear down field and TEST, REMOVE (twice), RUN LIVE (dimmed) \| "Live runs need Elite running" | pass | pass | **fail** |
| route | next stop: Arietis Sector LD-H a11-1 (1 jump away) \| Exact dropdown, By Spansh, ?, ..., REPLOT FROM HERE, the Sol to Maia endpoints, copy icons \| the orange "fuel" at Hyades Sector DB-W b2-6 and "neutron boost and fuel" at 98 k Tauri | pass | pass | pass |

Round 1: 24 of 33 pages and the sidebar pass. Nine pages fail: 2, 3, 4, 7, 10, 12, 26, 28 and 33.

## Round 2, 17 Nov 2026: the nine failed pages, unchanged, with the prompt fixed

No page was changed between rounds. Each failure was checked against its screen item and
DESIGN.md, and each page is drawn as its item says (see "What failed and why" below). Round 2
asks the nine pages again, with a new reader, for two reasons. The prompt's fault may have
caused row 3's can change mark. A single reader's reading is also one sample, so it is worth
knowing whether a failure repeats.

The prompt gets one sentence, after the three questions:

> A link that only opens another page does not count as something you can change.

It also says 9 screenshots in place of 33. The nine pages were copied as `01.png` to `09.png` in
this order: 28 Powerplay, 2 Now in space, 33 Voice tester, 4 Station, 12 Engineers "laser",
10 Materials, 3 Now not running, 26 System, 7 Ships. The route panel question is not asked
again, since it passed.

### Round 2 answers and marks

The reader was a new general-purpose agent (Claude Opus 5.5, agent ab4ff4c3005b2ab08) with the
amended prompt. Answers verbatim, under the round 1 marking notes.

| # | Reader's answer (main value \| can change \| needs you) | M | C | N |
|---|---|---|---|---|
| 28 Powerplay | 7,871 merits to rank 10 \| the Acquire / Reinforce / Undermine toggle, CHANGE (the 40 ly range), the plot and flag icons on each system row \| none clearly flagged (the nearest candidate is "Undermining is ahead by 9 this cycle", but nothing marks it) | pass | pass | pass |
| 2 Now, in space | 183,674,700 cr for selling your cargo (unclear: four columns of the same weight, and the orange "18 t per hour" pulls the eye just as much) \| the plot and flag icons beside Good Landing \| none (the orange "18 t per hour" looks like it wants attention but doesn't say why) | **fail** | pass | pass |
| 33 Voice tester | the "gear down" test result, 100% confidence \| the "gear down" text field and TEST, RUN LIVE (looks unavailable), REMOVE on each learned phrase \| none ("Live runs need Elite running" reads as a note, not a call to act) | pass | pass | pass |
| 4 Station | Test Orbital's market, led by the orange "+52,506 cr/t" for Tritium \| CHECK AGAIN \| none (the market being 4 days old is in small grey text, not flagged) | **fail** | pass | pass |
| 12 Engineers, "laser" | The Dweller, 75.5 ly away \| the "laser" search field, SET DESTINATION, the plot and flag icons on each engineer row \| "None unlocked yet: nearest to unlock", The Dweller | pass | pass | **fail** |
| 10 Materials | unclear: a grid of raw material counts with no single lead figure (89 Carbon is the biggest number) \| the Raw / Manufactured / Encoded / On foot toggle \| none | **fail** | pass | pass |
| 3 Now, Elite not running | 14.6 M cr per hour from the last session \| none \| "Elite isn't running" | pass | pass | **fail** |
| 26 System | 10,510,589 cr if mapped (against 5,346,554 cr now) \| clicking a body row to select it (A 4 a is selected) \| the "Worth mapping" tags on A 3, B 1 and AB 1, and possibly the 38.7 M cr of unsold data | pass | pass | **fail** |
| 7 Ships, a module selected | Long Way Round's 32.9 ly jump range \| the Loadout / Plan toggle, the module rows, the Increased range / G5 / Mass Manager dropdowns, ADD TO PLAN \| the "!" material shortfalls (Arsenic !3, Chemical Distillery !2, Chemical Manipulators !1, Datamined Wake Exceptions !0, Galvanising Alloys !0) | **fail** | pass | pass |

Powerplay and Voice tester pass in round 2 with no change to either page. Their round 1 fails
were one reader's picks. The other seven pages fail again, on the same question each time.

## What failed and why

Each of the seven pages that failed twice is drawn as its screen item and DESIGN.md say. The
failures are in the approved design, not in a page that strayed from it. So none is restyled
here. Each is filed as a bug in astromech-xajc that blocks this item, for the lead and Sumeet to
decide: change the design, or take the page out of the criterion's scope.

| # | Page | Failed on | What the readers did | Why, from the design |
|---|---|---|---|---|
| 2 | Now, in space | main value | Both readers saw two candidates: the orange 18 t per hour and the larger white 183,674,700 cr. | astromech-coq6 draws every block's value in Value XL and makes the first that shows (here the mining rate) Orange Lit. In space with cargo, the cargo total is the same size and has more digits. Docked, where the cargo total is the orange one, the page passed. |
| 3 | Now, Elite not running | needs you | Both readers named the "Elite isn't running" title. | coq6 makes the title "Elite isn't running" in Title White and names only exploration data at risk as what needs the commander. Both readers read a title saying the game is off as the thing to deal with. |
| 4 | Station | main value | Both readers went to For sale here's +52,506 cr/t on Tritium, not the hold's +3,921 cr/t on Gold. | astromech-7vxi makes the first cargo line's gain the main value (Orange Lit) and For sale here's profit the ranked value (also orange), both Value L. The larger ranked figure further down outshouts the main value. |
| 7 | Ships, a module selected | main value | Round 1 saw two candidates; round 2 picked the head's 32.9 ly jump range. | astromech-8wr4 keeps the head's four hero values in Value XL (Options Grey once a module is selected) and moves the main value to the panel's planned grade, G5. The head's Value XL figures sit higher and as large. |
| 10 | Materials | main value | Both readers found no single lead figure among the grid's counts. | DESIGN.md's Board gives every tile its own main value, and the grid has 28. The page has no one main value, which the criterion asks for. |
| 12 | Engineers, "laser" | needs you | Both readers named the "None unlocked yet: nearest to unlock" line. | astromech-7did makes the nearest engineer to unlock and its next step the main answer, and names nothing separate as needing the commander. The readers took the "none unlocked" state as that. |
| 26 | System, a body selected | needs you | Both readers named the "Worth mapping" tags or the 38.7 M cr unsold data. | astromech-dpaz names only the unsold-data "!" line, shown when dying would lose data, as what needs the commander. Without it, the readers took the worth-mapping badges and the unsold total as the call to act. |

## Round 3, astromech-ng71: Ships with the head's stats in Value L

The lead of astromech-xajc decided row 7 as a design change: on Ships the head's four stats
(jump range, shields, top speed, power used) are a row of equal facts in Value L, Options Grey,
with or without a module selected. With a module selected the panel's planned grade is the
page's one Value XL, Orange Lit. With none selected the page has no Value XL and its main value
is the ship's name in Title. DESIGN.md's Hierarchy now says a row of equal stats is Value L.

The sheet had no row for Ships with nothing selected. It is added here as row 7a, written down
before any reader saw the new capture. Its main value is the lead's; the other two come from
astromech-8wr4's item for that state (selecting a module fills the panel; nothing is marked
"!" until a module with a plan is selected). Row 7 is unchanged.

| # | Page and state | Capture | Main value | Can change | Needs the commander |
|---|---|---|---|---|---|
| 7a | Ships, nothing selected | `ng71/after/ships-2560x1440.png` | The ship's name, Long Way Round (ng71) | Picking a module row (the panel follows it) (8wr4) | none |

The captures are taken by `node scripts/browser/audit.mjs --plan docs/design/final/ng71-after.json`
(2560 x 1440 at 145 percent, the way `glance-plan.json` takes its own), beside the unchanged
`ng71/before/` set. `ng71/before/ships-module-2560x1440.png` is byte for byte the
`ships-module-2560x1440.png` that rounds 1 and 2 used.

Each reader is a fresh general-purpose agent given one capture, copied as `01.png` into an
otherwise empty folder, and the round 2 prompt with "33 screenshots, `01.png` to `33.png`"
read as "1 screenshot, `01.png`" and the route panel question left out, as in round 2.

### Round 3 answers and marks, 17 Nov 2026

Two new general-purpose agents (Claude Opus 5.5), one per capture. Answers verbatim, marked
under the round 1 marking notes. A main value answer that picks one value and notes that others
compete passes, as rows 25 and 26 did in round 1; "unclear" fails.

| # | Reader's answer (main value \| can change \| needs you) | M | C | N |
|---|---|---|---|---|
| 7 Ships, a module selected (`ng71/after/ships-module-2560x1440.png`) | "G5" (the planned grade for the selected 5A Frame Shift Drive), though "Long Way Round" and "32.9 ly" also compete for first glance \| the Increased range, G5 and Mass Manager dropdowns, the ADD TO PLAN button (it looks greyed, so it may be disabled), the Loadout/Plan toggle, and picking a module row such as "5A Frame Shift Drive" \| the materials marked "!" where you have fewer than you need: Arsenic !3 of 5, Chemical Distillery !2 of 4, Chemical Manipulators !1 of 5, Datamined Wake Exceptions !0 of 5, Galvanising Alloys !0 of 3 | pass | pass | pass |
| 7a Ships, nothing selected (`ng71/after/ships-2560x1440.png`) | the ship "Long Way Round" (Krait MkII). No single figure stands out; its stats sit as equal peers: 32.9 ly, 420 MJ, – m/s, – MW \| the Loadout / Plan toggle; the four ship cards at the bottom pick which ship is shown (Heavy Lifter, Long Way Round, Unnamed, Synthetic_Krait) \| none (the "–" top speed and power used, with the note beside them, are missing data, not an alert) | pass | **fail** | pass |

Row 7 passes: the reader picked the planned grade, G5, where rounds 1 and 2 saw the jump range.

Row 7a passes on main value, the answer ng71's decision expects. It fails on can change as the
sheet was written: the reader named the Loadout / Plan choice and the fleet strip, not picking
a module row. With no panel open, nothing on the loadout rows says they can be picked. This is
a separate question from ng71's (the head's stats against the main value) and is left for the
lead of astromech-xajc: either the sheet's row 7a accepts the fleet strip (picking a ship, like
picking a row, changes what the page shows) or the unselected loadout needs a cue.
