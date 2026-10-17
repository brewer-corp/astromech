# Galaxy

**Galaxy** has three pages that find the nearest station with what you need: **Services**,
**Outfitting** and **Shipyard**. They sit on the same search that every "nearest" list in
astromech uses, described [below](#how-searches-work).

**What they need:** a network connection (Spansh for stations, EDSM to fill in the star map) and a
position. The position is your current system from the journal, or a system you type. By
voice, the same searches are [lookups](lookups.md#stations-services-modules-ships).

## Services

Pick a service from the menu and the nearest stations with it are listed:

Interstellar Factors, Raw / Manufactured / Encoded material trader, Material trader (any type),
Human / Guardian technology broker, Technology broker (any type), Universal Cartographics,
Vista Genomics, Pioneer Supplies, Bartender, Frontline Solutions, Apex Interstellar, Refuel,
Repair, Rearm, Black market, Search and rescue.

Filters: pad size, no fleet carriers, no Odyssey settlements, data age, arrival distance.
Each row shows the station and system, jumps and light years, the pad size, how far it is from
the arrival star, and when it was reported.

## Outfitting

Pick a **module** by name, then its **class**, **rating** and, for weapons, **mount** (fixed,
gimballed or turreted). Only the sizes and ratings the module comes in are offered. The nearest
stations selling it are listed with the price.

- It opens on a Frame Shift Drive class 5, rating A.
- **Ctrl+K** finds a module by name and opens it here. So do the modules fitted on your ships.
- The Station page's [Where did I see ...](logbook.md#station) searches the outfitting you have
  opened yourself, offline.

## Shipyard

Pick a **ship**. The nearest shipyards selling it are listed with the price. Stations whose
landing pads are too small for that ship are left out, whatever pad you pick. It opens on a
Krait Mk II.

## How searches work

Every list that says "Nearest" works the same way. It appears on these pages and on Materials,
Trade, Colonisation, Powerplay and Exploration.

- **From.** Empty means your current system. Type to search from another system: names are
  suggested as you type (Spansh), and Enter or a click picks one. The **x** goes back to your
  current system.
- **Filters.** Only those that make sense for the list are shown, and they are **remembered**
  for each kind of search. Voice lookups use the same remembered filters, so set them here once.
  - **Pad:** your ship's by default, following the ship you fly until you pick one. Or **S or
    larger**, **M or larger**, **L**, or **Any**.
  - **Fleet carriers** and **Odyssey settlements** can be hidden.
  - **Data age:** hide reports older than this (up to 1, 3, 7, 30, 90 or 365 days).
  - **Arrival:** hide stations or bodies further than 1,000, 5,000 or 25,000 light seconds from the star.
  - **Within** and **volume:** the search radius (20 to 500 ly) and minimum demand or stock, on trade lists.
- **Results.** Up to about ten, nearest first. Each row has the place with a **Copy** button, the jumps and
  light years, and facts such as pad, arrival distance and when it was reported. The line under the list says which
  jump range was used, who the source is and how old the answer is.
- Searches are cached for a while, so asking twice is quick. Change a filter or the "From" to search again.

### Jumps and distance

"Jumps" uses your **ship's range** from its last loadout (20 light years if it does not know). astromech
keeps its own **star map**: a bundled copy of the known stars within 500 light years of Sol,
filled in from EDSM as you travel. Inside the covered space a jump count is worked out by walking
real stars at your range, so it is exact for that range. Outside it, or if the search gives up, it is a
straight-line estimate, rounded up, shown with a "~" ("~3 jumps").

It does not count neutron boosts, fuel or the galaxy map's plotter. It is for deciding where to look, not a
route to follow.

### Where the data comes from

- **Stations, bodies, factions, Powerplay:** Spansh, which collects what players report to
  EDDN. A report can be days or weeks old.
- **Commodity prices and markets:** Ardent Insight (see [Trade](trade.md)).
- **The star map:** a Spansh dump for the bundled part, and EDSM for the rest.
- **Fleet carriers** and **Odyssey settlements** are included unless you filter them.

## Limits

- Only what Spansh knows. A system or station nobody has reported is not found.
- Jump counts are for your ship at its last loadout, not for a different build.
- The pages do not plot or follow a route: see [Routes](routes.md).
