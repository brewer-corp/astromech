# Places

**Places** is one place to ask *where is X?* Open the **Galaxy** tab: its pages are **Places**
and **Route**. The Places page shows a tile for each finder, grouped under headings:

- **Trade:** [Sell and Buy](#sell-and-buy) and [Sell my cargo](#sell-my-cargo).
- **Ship:** [Services](#services), [Outfitting](#outfitting) and [Shipyard](#shipyard).
- **Engineering:** [Material traders](#material-traders), [Raw sites](#raw-sites) and
  [Faction states](#faction-states).
- **Mining:** [Mining hotspots](#mining-hotspots).

Click a tile to open that finder. Each finder is its own page, with a **Places** link back, its
title and its main input at the top: the commodity, service, module, ship, kind of trader or
site, faction state or mineral. Below that is
the same search every "nearest" list in astromech uses, described [below](#how-searches-work).
Back and reload keep what you picked, and **Ctrl+K** opens any finder by name.

Once a finder has answered, its tile shows the first result until you close the window ("Black
market · 3 jumps"), and opens that search again. Opening the Places page asks nothing: a tile
you haven't used this session shows only its name and what it finds.

**What they need:** a network connection (Ardent Insight for prices, Spansh for everything else,
EDSM to fill in the star map) and a position. The position is your current system from the journal, or a system you type. By
voice, the same searches are [lookups](lookups.md#stations-services-modules-ships).

Other pages link here instead of listing places themselves: Trade's Commodities page links to
Sell, Buy and Sell my cargo; Materials links each kind of raw site and a material's faction state;
Exploration links the nearest Universal Cartographics and Vista Genomics.

## Sell and Buy

**Sell** lists the stations buying a commodity, highest price first. **Buy** lists the stations
selling it, cheapest first. Prices come from **Ardent Insight** (api.ardent-insight.com), which
is fed by what players share with EDDN.

1. Type a **commodity** in the **Commodity** box (part of its name, or a short name such as
   "LTDs") and pick it. It opens on Painite.
2. Pick the **Range**:
   - **Near** searches around you or a system you type, within a distance (100 light years by
     default) and data age (30 days by default).
   - **Anywhere** takes Ardent's best 100 prices in the whole galaxy that pass the filters, with
     distances from you. Use it when nothing close pays well.

Each row starts with the price, then the station (type mark) and its system, the **demand** (Sell)
or **stock** (Buy) in tonnes, distance, jumps, pad, arrival distance and how old the report is.
Filters: minimum demand or stock, pad, data age, arrival distance, fleet carriers, Odyssey
settlements, and (Near) the search distance. Sell and Buy, Near and Anywhere each remember their
own filters, and voice questions use the same ones ([Voice lookups](lookups.md#trade)). When a
Near search has more markets than Ardent returns, the list says to narrow the radius.

## Sell my cargo

Where to sell **everything in your hold**. It asks nothing: the hold comes from the journal. The
line above the results lists what is in the hold, and what was left out (mission cargo, stolen
goods, limpets). It then lists the stations that pay the most for the whole load, each commodity
capped by what the station will take: each row starts with the total and says how many tonnes the
station **takes**. Under it, **Best for each on its own** shows the best station for each
commodity separately. When no market takes all of one commodity, it shows the market that takes
the most. Voice: "where should I sell my cargo" ([Voice lookups](lookups.md#trade)), with the
same filters.

## Services

Pick a **service** in the dropdown. The nearest stations with it are listed:

Interstellar Factors, Raw / Manufactured / Encoded material trader, Material trader (any type),
Human / Guardian technology broker, Technology broker (any type), Universal Cartographics,
Vista Genomics, Pioneer Supplies, Bartender, Frontline Solutions, Apex Interstellar, Refuel,
Repair, Rearm, Black market, Search and rescue.

Filters: pad size, carriers, Odyssey settlements, data age, arrival distance. The list is ranked
by jumps. Each row shows the station (with its type mark) and its system, the allegiance,
controlling faction and state, then distance, pad, arrival distance and how old the report is.
It opens on Interstellar Factors.

## Outfitting

Type a **module** in the **Module** box and pick it from the suggestions, then pick its
**class**, **rating** and, for weapons, **mount** (fixed, gimballed or turreted). Only the sizes
and ratings the module comes in are offered. The nearest stations selling it are listed with
the modules that matched and the price.

- It opens on a Frame Shift Drive class 5, rating A.
- **Ctrl+K** finds a module by name and opens it here. So do the modules fitted on your ships.
- The Station page's [Where did I see ...](logbook.md#station) searches the outfitting you have
  opened yourself, offline.

## Shipyard

Type a **ship** in the **Ship** box and pick it. The nearest shipyards selling it are listed with
the price. Stations whose landing pads are too small for that ship are left out, whatever pad you
pick. It opens on a Krait Mk II.

## Material traders

Pick **Raw**, **Manufactured**, **Encoded** or **Any**. The nearest material traders of that
kind are listed, as on [Services](#services), with the same remembered filters. It opens on Raw.

## Raw sites

Pick a kind of **site** and the nearest bodies with it are listed:

| Site | What it gives |
|---|---|
| Crystalline shards | Grade 4 raw (polonium, ruthenium, tellurium, technetium, yttrium, antimony and more). Shoot the shards from the SRV and collect the fragments. |
| Brain trees | Grade 4 raw. Shoot them (turrets or flak), then collect with the SRV. |
| Crashed ships | Encoded data points and salvage. Scan the data points from the SRV. |
| Geological sites | Grade 1 to 3 raw from the outcrops around geysers, fumaroles and vents. |
| Known raw farms | The well-known grade 4 farms: one body for each element. |

Each row is a body, nearest in jumps first, with what is there, the distance, how far it is from
the arrival star and how old the data is. Filters: data age and arrival distance, remembered for
each kind. It opens on crystalline shards. By voice: "nearest crystalline shards" and the like
([lookups](lookups.md#raw-material-sites)).

Sources: Spansh's bodies search; the known farms are a hand-checked list from player reports
(forums, Steam, the wiki). The named bodies are starting points, not guarantees.

## Faction states

Pick a **state** and the nearest systems where a faction is in it are listed. Most are the
[High Grade Emissions](materials.md#high-grade-emissions) pools: **Federation faction**, **Empire
faction**, **Boom**, **War or Civil War**, **Civil Unrest** and **Outbreak** (Boom, Civil Unrest
and Outbreak for Independent or Alliance factions). **Famine** finds distribution centres for wake
scanning. A line under the filters says what the pick is for.

The High Grade Emissions searches are ranked by a **score** of population then distance (the first
column), with the population; Famine is ranked by jumps. Each row shows the faction and its states
and how old Spansh's report is. A system you visited recently uses the states from your own visit
("seen on your visit"). The only filter is data age, remembered for each pick. It opens on Boom. By
voice: "nearest boom system" and the like ([lookups](lookups.md#faction-states-famine-boom-war-and-more)).

## Mining hotspots

Type a **mineral** in the **Mineral** box and pick it: Void Opal, Low Temperature Diamonds,
Painite, Platinum, Musgravite, Grandidierite, Alexandrite, Benitoite, Monazite, Rhodplumsite,
Serendibite, Osmium, Palladium, Bromellite or Tritium. The rings with a hotspot of it are listed,
**most hotspots first**, then nearest.

- **Around:** where to search from. Empty means here. Click the box to pick the route's **next
  stop** (it then says "Next stop: Deciat"), or **Here** to go back; or type a system.
- **Ring:** Metallic, Metal Rich, Rocky, Icy or **Any**.
- **Reserve:** **Pristine**, **Major** (major or pristine) or **Any**.
- **Within:** the search radius, 100 light years unless you pick another.

Each row shows the ring's hotspots of the mineral, the ring and its system, **Also** (its
hotspots of other minerals, "Platinum 1"), the ring type, the reserve level, then distance, jumps
and the body's distance from the star. Hotspots never change, so there is no report age. **Add
stop** and **Set destination** put the ring's system on the route. It opens on Painite, any ring,
any reserve; **Ctrl+K** opens a mineral's hotspots by name ("Painite hotspots").

The list is the ten rings with the most hotspots in the whole radius, and the line above it says
how many bodies they were ranked from ("Within 100 ly of Sol · top 10 of 1,101 bodies with
Painite"). Spansh lists bodies nearest first, so the search asks for the highest hotspot counts
first, in up to four requests. Spansh can't pick a ring type, so when the one you chose is rare
there the requests can run out first. The line then says what was ranked: every ring with at
least so many hotspots ("5 or more hotspots only"), or the rings within a smaller distance
("Within 32 ly of Sol"). Pick **Any** ring type to see the whole radius.

Limits: Spansh files the reserve level on the body, so every ring of a body has the same one. Hotspots nobody has scanned and
reported are not known. By voice: "nearest painite hotspot" ([Mining](mining.md#finding-hotspots)).

## How searches work

Every list that says "Nearest" works the same way. It appears on these finders and on
Colonisation, Powerplay and a ship's build plan (Engineering).

- **From.** Empty means your current system. Type to search from another system: names are
  suggested as you type (Spansh), and Enter or a click picks one. The **x** goes back to your
  current system. Mining hotspots calls it **Around** and also offers the route's next stop.
- **Filters.** Only those that make sense for the list are shown, and they are **remembered**
  for each kind of search. Voice lookups use the same remembered filters, so set them here once.
  - **Pad:** **S**, **M** or **L** (S is any pad). Your ship's size is marked with a ship glyph
    and is selected until you pick another.
  - **Include:** **Carriers** and **Odyssey** are filled orange when those stations are
    included. Click one to hide them, and again to bring them back.
  - **Data age:** hide reports older than this (up to 1, 3, 7, 30, 90 or 365 days).
  - **Arrival:** hide stations or bodies further than 1,000, 5,000 or 25,000 light seconds from the star.
  - **Within** and **volume:** the search radius (20 to 500 ly) and minimum demand or stock, on trade lists.
- **Results.** Up to eight (sites, trade, faction states) or ten (stations, colonisation, Powerplay).
  A line above the list sums up the search ("Within 100 ly of Sol · large pad · reports up to 30
  days old"). Each result is one row under column heads. It starts with what the list is ranked
  by, in the largest figures: the price (trade), the High Grade Emissions score, or the jumps
  (everything else, nearest first; "~" marks a straight-line estimate). Then the station or body
  and its system with a copy mark, the facts that list has (demand, faction, bodies, ...), and the
  distance, jumps, pad, arrival distance and how old the report is ("10 d"). Two buttons end the
  row: **Add stop** makes that system the next stop of the route
  (the [route strip](routes.md#the-route-strip) under the header), and **Set destination** makes it the whole route, pausing the old one
  ([Routes](routes.md#from-a-search-result)). The line under the list
  says which jump range was used, who the source is and how old the answer is.
- **Narrow panels.** Where a list sits in a narrow side panel (a build plan's material sources,
  Powerplay's "Where to work"), each row keeps only what it is ranked by, the place and
  the two buttons.
- **Old reports.** With a data age set, older reports are left out. With no data age set
  ("Any"), reports older than 30 days are shown dimmer, with their age brighter than the rest of
  the row.
- **Nothing found.** The list says what is empty and offers one way to widen it, such as
  **Widen to 200 ly** or **Allow 90 days**. It never relaxes the pad, since a ship can't land on
  a smaller one.
- **Waiting and errors.** While a search runs, a line names who is asked ("Asking Spansh…") and
  the old results stay, dimmed. If a search, a filter change or a route button fails, a red "!"
  says why, with **Try again**.
- Searches are cached for a while, so asking twice is quick. Change a filter or the "From" to search again.

### Jumps and distance

"Jumps" uses your **ship's range** from its last loadout (20 light years if it does not know). astromech
keeps its own **star map**: a copy of the known stars within 500 light years of Sol (bundled with tagged releases),
filled in from EDSM as you travel. Inside the covered space a jump count is worked out by walking
real stars at your range, so it is exact for that range. Outside it, or if the search gives up, it is a
straight-line estimate, rounded up, shown with a "~" ("~3 jumps").

It does not count neutron boosts, fuel or the galaxy map's plotter. It is for deciding where to look, not a
route to follow.

### Where the data comes from

- **Stations, bodies, factions, Powerplay:** Spansh, which collects what players report to
  EDDN. A report can be days or weeks old.
- **Commodity prices and markets:** Ardent Insight ([Sell and Buy](#sell-and-buy)).
- **The star map:** a Spansh dump for the bundled part, and EDSM for the rest.
- **Fleet carriers** and **Odyssey settlements** are included unless you filter them.

## Limits

- Only what Spansh knows. A system or station nobody has reported is not found.
- Jump counts are for your ship at its last loadout, not for a different build.
- The finders do not plot a route. **Add stop** and **Set destination** on a result put its system on the route followed: see [Routes](routes.md#from-a-search-result).
