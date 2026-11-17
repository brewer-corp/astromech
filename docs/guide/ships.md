# Ships

**Ships, Ships** is your fleet at a glance, with a build planner for any ship. It reads
your journal, so it knows every ship you own and the modules last saved for each.

**What it needs:** the journal, with the ship flown at least once since your oldest journal
file (a ship that has never been flown has no modules to show). No network for the page itself.
The nearby searches inside the planner use Spansh and EDSM (see
[Using the app](using-the-app.md#what-needs-the-network)).

**Data sources:** your journal (ships, modules, materials, engineer progress); ship, blueprint and
module data from EDCD's coriolis-data and FDevIDs (credited in NOTICE.md); Spansh and EDSM for the
nearby searches.

If the journal folder cannot be read (it is not where astromech looks, or it cannot be opened),
the page says "Could not read the fleet" and gives the reason. It never shows made-up ships.

## The fleet

The page opens on the ship you are flying (or the first ship in the journal). Its head gives the
ship's name, then its type, ID, how many of its engineerable modules are engineered, where it is
(**flying now**, stored at a system with a copy button and its Spansh and Inara links, with the
station's link when it is known, or in transit to a system), its **value** and its **rebuy**.
Four figures follow: **Jump range** (one jump's fuel, no cargo), **Shields**, **Top speed** and
**Power used** of what the power plant gives. Hover a figure for what it counts, or for why it
shows "–". "Incomplete", with a tooltip, lists items that are not in astromech's ship data and
were left out.

Choose what to show under the head:

- **Loadout**: the ship's modules, as in the game's outfitting.
- **Plan**: what the ship's build plan costs (see [Build planner](#build-planner)).

A strip at the bottom lists every ship you own, the one you are flying first, with a cartoon of the
hull, its type and where it is ("Here" for the ship you are flying). Click one to switch. Ship
drawings are CMDR Qohen Leth's cartoon vectors, credited in NOTICE.md.

## Stored modules

**Ships, Stored modules** lists every module in storage, grouped by the system it is stored in,
each heading with how many are there; modules on their way to a station are under **In transit**.
Each row is the module's name, its blueprint with its grade as engineer hexagons if engineered,
and its value. Click a row to see it in the side panel: its full value, where it is (with a copy
button for the system's name) and its blueprint and grade.

## Suits

**Ships, Suits** shows a suit loadout the way a ship's loadout reads: what is in each slot, how far
each item is upgraded and modified, what you plan for it and what you are still short of. The
inventory is the [On foot](materials.md#on-foot) kind on Materials and the suit engineers are on
[Engineers](engineering.md#engineers).

**What it needs:** the journal and the game's `ShipLocker` file, which the game writes when you
are on foot. Until the journal names a loadout the page says so, and the shopping list and
**Plan other gear** still work. No network.

The page opens on the loadout you wear. Its title is the loadout's name, and the line under it
says which loadout it is, its suit, and how many of its items are engineered or planned. Point at
that line for where the grades and mods come from.

- **Loadout**: one row per slot (Suit, Primary, Primary, Secondary): the item, its grade as
  engineer hexagons (white as fitted, then orange up to a planned grade), its fitted modifications, and your plan for it in
  orange ("Grade 3 · Night Vision"). A plan belongs to one item: of two grade-1 Maverick suits,
  only the one you planned for shows it. An item worn in several loadouts shows its plan in each.
- **Shopping list**: everything your plans need, with what you have. A short line's have is
  marked "!". Its credits and how many lines are short are beside the heading. Plans saved before
  plans belonged to one item are listed under it as **Not tied to an item**: choose the item from
  the dropdown to tie one, or remove it. Under it, **Bartender trade** suggests swaps from your
  surplus assets for the short ones; point at the heading for what it assumes.
- **Loadouts**: your other loadouts. Click one to show it; the link keeps your choice. Suits and
  weapons in no loadout are listed under them ("Also owned"); click one to plan for it.

Click a slot to open the item in the side panel: its plan, the cost of its planned upgrade, and,
for each thing planned, what it needs against what you have, where it is done (upgrades at
Pioneer Supplies, modifications by the engineers listed, with copy and set destination for their
system) and **Remove**. **Add to plan** adds the next grade (or a higher one, which raises the
item's upgrade) or a modification the item can still take. Adding a plan you saved before plans
belonged to one item ties that one to the item instead of adding a second.

**Plan other gear**, beside the title, opens the blueprint picker in the side panel: plan for an
owned item in no loadout, or for any suit or weapon (a grade upgrade or a modification, with the
weapon maker for weapon mods), which is tied to no item. Gear and mods you already have are not
offered again.

## A ship's page

Modules are as of the last time the game saved a loadout for it (hover "modules engineered" for
the date). A ship never flown since your oldest journal file has no modules to show, and the page
says its modules are unknown. The head can open the loadout in **Coriolis** or **EDSY** in your
browser. If you engineered a module since the game last saved the loadout, the link would show the
old build, so astromech says to re-dock or relog to refresh it.

### Loadout

The **Loadout** lists every fitted module under **Hardpoints**, **Utility**, **Core** and
**Optional**, as the game's outfitting does (military slots are with Optional). Fixed and cosmetic
slots, such as the planetary approach suite, are left out. Each row gives the module's class and
rating ("5A"), its name, its grade as engineer hexagons, and its blueprint and experimental. Fitted
grades are white; the grades your plan adds follow in orange.

Click a module to see it in the side panel:

- what it has now, and the plan's target for it if there is one: the planned grade, the rolls
  still to do at each grade, each material it needs against what you hold (a "!" marks one the
  plan is short of), and the engineers who apply its top grade;
- the plan's **blueprint**, **grade** and **experimental** for it. "No change" takes the module out
  of the plan. A different blueprint starts again at grade 1, and a grade must be finished before
  the next one opens;
- **Add to plan**: plans one grade up on its blueprint (or on the blueprint already planned). For
  a module not engineered yet, it opens the blueprint choice;
- **Engineers**: opens [Engineers](engineering.md#engineers) searched for that kind of module.

### Build planner

The **Plan** view shows what the plan made on the Loadout costs:

- **Now and with the plan**: the [ship stats](#ship-stats) now, with the plan done, and the change.
- **Rolls per grade.** Each roll costs one full grade recipe, and how many rolls a grade takes
  depends on engineer rank and luck. The planner uses an estimate of 1, 2, 3, 4 and 5 rolls for
  grades 1 to 5. Change them if your experience differs; **Reset to 1-5** returns to the default.

Your plan for each ship is saved on this PC by the app window and comes back next time.

From the plan it works out:

- **Shopping list.** Each material, with what you **need**, **have**, are **short**, and a
  trade that would cover it (**Trade**). A warning mark means the plan needs more than you can
  store at once: do it in batches. Click a material for where it comes from and a
  **nearby search** for it in the side panel; click it again to close the panel.
- **Trade at a material trader.** Swaps from your surplus. Trader rates: 6 for 1 up a grade, 1 for
  3 down a grade, and across categories 6 times worse. Trades only spend what the plan does not need.
- **Still to collect.** What to farm, with the amount, where from, and what to trade it into.
- **Engineer trip.** Which engineers to visit and in what order, with the work to do at each.
  It finds the fewest jumps (with a small cost for each stop) over every engineer who offers a job, exact for up to 16 candidate sets and a quicker greedy route for larger plans, from your current system, at your
  ship's range. Jobs that need an engineer you have not unlocked are listed apart, with what
  unlocks them (see [Engineers](engineering.md)). A "~" means the jump count is a
  straight-line estimate. Experimentals are always in person. Grades can also be rolled remotely
  with a blueprint pinned at a rank 5 engineer, but the game does not log pins, so these only get
  a note.

What the plan still has to collect also feeds the [arrival briefing](callouts.md#arrival-briefing):
if a system's faction state gives a material you are short of, astromech says so on arrival.

## Ship stats

The Plan view compares the ship **now** with the ship **when the plan is done**:

| Group | Rows |
|---|---|
| Jump range (ly) | Max (one jump's fuel, no cargo), Full tank, Laden (full tank and cargo), Total (jumping until empty) |
| Shields | Strength (MJ), explosive, kinetic, thermal |
| Distributor | WEP capacity and recharge, weapon draw, time to drain, SYS and ENG recharge |
| Weapons and hull | DPS (burst, while firing; sustained DPS is not shown), armour, explosive, kinetic and thermal resistance |
| Ship | Speed, boost, mass unladen and laden, hull mass, cargo and fuel capacity, power use with hardpoints retracted and deployed, power plant output |
| Handling (°/s) | Pitch, roll and yaw: a stand-in by hull at four pips to engines until the calculations include handling, the same now and with the plan |

The numbers are a port of **Coriolis**'s calculations with the same base stats. They are
tested against Coriolis's own code on reference builds, and the figures shown match to the last digit Coriolis shows (Coriolis
as of 24 April 2026), except where listed below. Read this when you compare the two:

- **Your fitted modules match.** For a module you have engineered, astromech reads the real values
  the game saved in your loadout, roll by roll, as Coriolis does when you import a loadout.
- **A planned module is a best case.** Coriolis lets you set the roll. astromech assumes each
  planned grade is fully rolled, and a module already above the planned grade on the same
  blueprint keeps its own grade and roll. Real rolls vary, so the Plan column is the most you
  can expect.
- **Incomplete builds.** A module, blueprint or experimental that is not in astromech's data is left
  out of the numbers and flagged "Incomplete" with the list. A hull it does not know has no stats
  at all. Coriolis may know it; astromech's data is a snapshot.
- **Sustained DPS.** astromech keeps an internal sustained figure that counts rail-gun charge time, which departs from Coriolis (astromech-bnnh); it is not shown.
- **Time to drain with rail guns.** For a build with rail guns, time to drain counts their charge time over the same firing cycle as sustained DPS, so it is shorter than Coriolis shows (for example 0:15 where Coriolis shows 0:21). Builds without charged weapons match Coriolis.
- **Fewer figures.** The page shows the groups above, not all of Coriolis's tables, and none of its
  build costs.
- **Missing drive.** With no frame shift drive, or one that is unpowered, jump range is blank
  where Coriolis shows 0.
- **Guardian FSD boosters** are included in jump range, as in Coriolis.

## Limits

- Suit and weapon game data (items, values, who fits what) is from the Elite Dangerous wiki and
  EDCD/FDevIDs, as of the date at the bottom of Suits. Point at that line for how many values are
  unsure or blank on the wiki. Your picks are remembered by the app window on this PC.

- Needs a loadout in the journal for the ship to plan it.
- Rolls per grade are an estimate; your luck and engineer rank change them.
- Material sources are community reports and starting points, not guarantees.
- The stats are for the ship as fitted, with no cargo or fuel changes except where noted.
