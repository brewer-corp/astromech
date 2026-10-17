# Ships

**Engineering, Ships** is your fleet at a glance, with a build planner for any ship. It reads
your journal, so it knows every ship you own and the modules last saved for each.

**What it needs:** the journal, with the ship flown at least once since your oldest journal
file (a ship that has never been flown has no modules to show). No network for the page itself.
The nearby searches inside the planner use Spansh and EDSM (see
[Using the app](using-the-app.md#what-needs-the-network)).

**Data sources:** your journal (ships, modules, materials, engineer progress); ship, blueprint and
module data from EDCD's coriolis-data and FDevIDs (credited in NOTICE.md); Spansh and EDSM for the
nearby searches.

## The fleet

Opens on a card for each ship:

- **Name and ID**, the ship type, and where it is: **Flying now**, the system it is stored in
  (with a copy button), "In transit to" a system, or "Stored".
- **Value** and **Rebuy.**
- **Jump** (the maximum range: one jump's fuel, no cargo), **Shield** (MJ) and **DPS** (burst, while firing): worked out from the modules, see
  [Ship stats](#ship-stats). "Incomplete" with a tooltip lists items that are not in astromech's ship
  data and were left out. If the stats cannot be worked out, Jump falls back to the journal's own maximum range.
- **Engineered**: how many of the ship's engineerable modules have an engineering blueprint, as
  a bar.

Below the cards, **Stored modules** lists every module in storage with its blueprint and grade
if engineered, where it is and its value.

Click a card to open that ship.

## A ship's page

The ship's page shows modules as of the last time the game saved a loadout for it ("modules as of ...").
It can open the loadout in **Coriolis** or **EDSY** in your browser. If you engineered a module
since the game last saved the loadout, the link would show the old build, so astromech says to
re-dock or relog to refresh it.

### Build planner

For each module that can be engineered:

- Choose a **blueprint**, a **grade** and an **experimental**. "No change" leaves the module alone.
- A different blueprint starts again at grade 1. A grade must be finished before the next one
  opens.
- Each roll costs one full grade recipe, and how many rolls a grade takes depends on engineer
  rank and luck. The planner uses an estimate of 1, 2, 3, 4 and 5 rolls for grades 1 to 5. Change
  them in the side panel if your experience differs; **Reset to 1-5** returns to the default.

Your plan for each ship is saved on this PC by the app window and comes back next time.

From the plan it works out:

- **Shopping list.** Each material, with what you **need**, **have**, are **short**, and a
  trade that would cover it (**Trade**). A warning mark means the plan needs more than you can
  store at once: do it in batches. Click a material for where it comes from and a
  **nearby search** for it.
- **Trade at a material trader.** Swaps from your surplus. Trader rates: 6 for 1 up a grade, 1 for
  3 down a grade, and across categories 6 times worse. Trades only spend what the plan does not need.
- **Still to collect.** What to farm, with the amount, where from, and what to trade it into.
- **Engineer trip.** Which engineers to visit and in what order, with the work to do at each.
  It finds the fewest jumps (with a small cost for each stop) over every engineer who offers a job, exact for up to 16 candidate sets and a quicker greedy route for larger plans, from your current system, at your
  ship's range. Jobs that need an engineer you have not unlocked are listed apart, with what
  unlocks them (see [Engineers and on foot](engineering.md)). A "~" means the jump count is a
  straight-line estimate. Experimentals are always in person. Grades can also be rolled remotely
  with a blueprint pinned at a rank 5 engineer, but the game does not log pins, so these only get
  a note.
- **Ship stats.** Now, with the plan, and the change.

What the plan still has to collect also feeds the [arrival briefing](callouts.md#arrival-briefing):
if a system's faction state gives a material you are short of, astromech says so on arrival.

## Ship stats

The side panel compares the ship **now** with the ship **when the plan is done**:

| Group | Rows |
|---|---|
| Jump range (ly) | Max (one jump's fuel, no cargo), Full tank, Laden (full tank and cargo), Total (jumping until empty) |
| Shields | Strength (MJ), explosive, kinetic, thermal |
| Distributor | WEP capacity and recharge, weapon draw, time to drain, SYS and ENG recharge |
| Weapons and hull | DPS (burst, while firing; sustained DPS is not shown), armour, explosive, kinetic and thermal resistance |
| Ship | Speed, boost, mass, power use with hardpoints deployed, power plant output |

The numbers are a port of **Coriolis**'s calculations with the same base stats. They are
tested against Coriolis's own code on reference builds, and the figures shown match to the last digit Coriolis shows (Coriolis
as of 24 April 2026). Read this when you compare the two:

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
- **Fewer figures.** The page shows the groups above, not all of Coriolis's tables, and none of its
  build costs.
- **Missing drive.** With no frame shift drive, or one that is unpowered, jump range is blank
  where Coriolis shows 0.
- **Guardian FSD boosters** are included in jump range, as in Coriolis.

## Limits

- Needs a loadout in the journal for the ship to plan it.
- Rolls per grade are an estimate; your luck and engineer rank change them.
- Material sources are community reports and starting points, not guarantees.
- The stats are for the ship as fitted, with no cargo or fuel changes except where noted.
