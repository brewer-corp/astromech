# Callouts

Callouts are short spoken lines astromech says on its own as you play, for things the game's HUD
does not show, or shows too small to catch in a fight. They are written like a ship computer
reporting: a fact, then silence. This page lists every one, with an example line and the setting
that controls it.

**What they need:** Elite running, the speech models installed and set up (see
[Getting started](getting-started.md)). They come from the journal and the game's status files,
so no network is needed except for the biology other players found. Most also appear as a
**toast**, a small note in the corner of the astromech window for a few seconds, if the window is
open. A callout is never spoken for something that happened before astromech started: starting
mid-session is silent until the next live event.

## Rules for all callouts

- **Only what the HUD does not show.** The star class, security level, allegiance and system
  name on arrival are on screen, so they are never read out. COVAS's own lines (low fuel, scoop
  progress) are not repeated.
- **Short.** A callout is about eight words. If there is more, it is several lines.
- **Quiet in a fight or on approach.** Speech is held back while you are being attacked (for 30
  seconds after the last hit), in danger, being interdicted, or between a docking grant and
  landing. Toasts still show. Only the most urgent callout, a cargo scan with stolen goods
  aboard, is spoken anyway.
- **Importance.** If several are waiting, the most important is spoken first. A line that waits
  more than ten seconds is dropped, because late is wrong. Up to eight wait at once.
- **They wait for you.** While you hold push-to-talk, and until your reply is spoken, callouts
  wait so they are not recorded or spoken over the reply.
- **A cooldown** stops one burst of the same event from talking repeatedly.

## Turning callouts off

Each callout has an id (in the headings below). In `astromech.toml` (Settings, Files, **Open**):

```toml
[callouts.scan]
enabled = false      # never ask this callout
cooldown_s = 60      # or: stay silent this long after it speaks
```

Everything is on by default except [Powerplay arrival](#powerplay-arrival). An unknown id, or
a cooldown that is not a number of seconds, is reported in the log and ignored. Prospector
thresholds have their own boxes in [Settings](settings.md#prospector-callouts).

There is no master switch and no volume setting yet. **Pause voice** does not silence callouts.

## Arrival briefing

Id: `arrival`. On each jump (or carrier jump), it says only what matters for your goals, as
separate short lines:

- **A faction state the shopping list wants.** The shopping list is what your build plans in
  [Ships](ships.md) still have to collect. If the most influential faction in the system is in a state that
  gives a material you still need, it says so: "Boom here. Proto Heat Radiators." (Boom, war, civil
  war, civil unrest and outbreak give High Grade Emissions; famine helps wake scans.) In a system of
  over a billion people, a Federation or Empire pool counts too: "Good for Imperial Shielding."
- **Powerplay.** For the power you are pledged to: "Undermining target." or "Acquisition target."
  Reinforcing your own power's system is normal and stays quiet. Left out when
  [Powerplay arrival](#powerplay-arrival) is on.
- **"Undiscovered system."** When nobody had discovered the arrival star.
- **"Visited 2 times. Last here 3 days ago."** From your journal history. A system you have never
  been to says nothing, and neither does returning by your own carrier.
- **"Landed on A 2 twice before."** or "Landed on 3 bodies here before." From your history of
  landings.

A system with none of these gets nothing. Visit and landing lines need the
[journal history](logbook.md) to have finished importing.

## Known biology

Id: `biology`. Part of the arrival briefing and the FSS.

- On arrival, before you scan: **"Known biology on 3 bodies."** and the best find, **"A 2: Stratum
  Tectonicas, 19 M."** This is from what other players reported (Spansh). It is also said on login, once per system. If the answer arrives
  late it is said within the first minute.
- After you leave the FSS: **"Biology on 3 bodies."** and **"A 2: Stratum, up to 19 M."**, the
  most valuable body's best genus. The value is the top of the predicted range, "up to" when it
  is a range.
- After a DSS shows a body's genera: its best genus.

Values are Vista Genomics base values, before the first-logged bonus. Per-body signal counts
are never spoken because the FSS shows them. See [Exploration](exploration.md#biology).

## Powerplay arrival

Id: `powerplay_arrival`. **Off by default.** On a jump into a system with Powerplay activity: "Zachary
Hudson fortified, 40 percent." "Reinforcement 10, undermining 20." and, if you are pledged,
what it means for you ("Undermining target."). The game shows this only in the maps. Turn it on
with `[callouts.powerplay_arrival] enabled = true`. While on, it replaces the Powerplay line of
the arrival briefing. Speech only, no toast.

## Docking

Id: `docking`. Two lines:

- **Pad position.** At a Coriolis, Orbis, Ocellus or Bernal starport, on a docking grant: "Pad at
  seven, middle." The HUD shows the number, not where it is. Pads are read as a clock face seen from
  the slot (twelve is the top), at the front, middle or back of the bay. Outposts, carriers and
  surface ports get no line: their pads are in plain sight.
- **Denied.** "Docking denied. No free pad." with the reason (ship too large, station hostile,
  offences here, too far away, fighter still out, access restricted, docking offline, carrier
  about to jump). COVAS only says it was denied.

## Fuel and route

When you plot a route (a new route in the game):

- Id `route`: **"Route: 19 jumps, 380 ly."**, then **"White dwarf at jump 4."** or "3 white dwarfs,
  first at jump 4.", and the same for neutron stars. White dwarfs have an exclusion zone.
- Id `fuel_range` (High priority): **"Fuel too low to reach a scoopable star."**, "Fuel too low to reach the
  destination.", **"Fuel gap: 18 jumps between scoopable stars."** for the first stretch a full
  tank will not cross, or **"Fuel gap: 6 jumps to the destination."** when that stretch ends at the destination.

As you jump along it:

- Id `scoopable`: at a star you cannot scoop, after one you can (or on the first jump), **"Next
  scoopable star in 3 jumps."** or "No scoopable stars left on the route." Once per stretch, and not
  for a one-jump stretch, so a neutron route does not repeat it.
- Id `fuel_range`: on arrival, when the main tank will not reach the next scoopable star or the end
  of the route, **"Fuel too low to reach a scoopable star."**, or at a scoopable star **"Scoop here.
  Next scoopable star out of reach."** The same warning is said at most once in 10 minutes.

Fuel per jump is estimated from your last ordinary jump (boosted jumps are skipped), scaled for the
distance of each hop by the size of your frame shift drive. Mass changes along the way are ignored,
so it is an estimate. Scoopable stars are classes O, B, A, F, G, K and M, giants included. It needs
a jump to go by, so there is no fuel line before your first jump.

## Prospector

Id: `prospector`. When a prospector limpet reports a rock worth mining: **"Platinum 34%."** At most
two ores are named, most valuable first, or one after a motherlode: **"Motherlode. Void Opals.
Platinum 34%."** A motherlode is always read out. A depleted rock only gets its motherlode line. A second limpet on the same rock (the same ores
in the same proportions) is not read again for ten minutes.
The rock's content (low, medium, high) is not read: the prospector panel shows it. The per-ore
percentage is set in [Settings](settings.md#prospector-callouts).

## Scans

Id: `scan`, 30-second cooldown. When another ship scans you: **"Cargo scan."**, "Crime scan.",
"Cabin scan.", "Data scan." or "Scan." The HUD warning is small and easy to miss. A cargo scan with
stolen goods aboard, **"Cargo scan. Stolen goods aboard."**, is urgent and is spoken even in a
fight or on approach. Who scanned is not named because the game does not say. Goods prohibited in the
system are not checked.

## Signals

Id: `signals`. When an FSS honk finds one of: **"Thargoid signal."**, **"Thargoid Titan in system."**,
**"Conflict zone, medium intensity."** (also low, high, very high; Thargoid and Powerplay conflict zones
say so), **"Powerplay signal."**, **"Megaship in system."** Each line once per system. Stations and fleet
carriers stay quiet, and High Grade Emissions have their own callout. Speech only.

## High Grade Emissions

Id: `hge`, 30-second cooldown (one honk finds several). **"High Grade Emissions. Federation
composites."** names the material pool of the faction that spawned the signal. If a material on your
shopping list comes from that pool it names the material instead: "High Grade Emissions. Proto Light
Alloys." Every High Grade Emissions signal is called out; with no known pool you hear just "High Grade Emissions." See [Materials](materials.md#high-grade-emissions).

## Discoveries

Id: `discovery`. When a scan shows a planet worth flying to: **"Earth-like world. 4.4 million
mapped."** Called out are Earth-like, water and ammonia worlds, any terraformable planet
("Terraformable high metal content world. 1.2 million mapped.") and any other planet worth at
least about 500,000 credits mapped. The value is the efficient mapping value, including the first
discovery bonus when nobody had discovered it. Once per body per visit. Stars and nav beacon
scans stay quiet. See [Exploration](exploration.md).

## Exobiology sampling

Id: `sampling`. After you take a sample, once Status shows you far enough from every earlier
sample of that species: **"Far enough for sample 2."** The genus's colony range is how far.
Once per sample. If you restart astromech, a cue already due is not said again.
See [Exploration](exploration.md#biology).

## Materials at the cap

Id: `materials_cap`. The game shows no warning when a material is full. When something fills one:
**"Arsenic at 300/300."** Then, when you dock at a station with a material trader: **"Trader here.
Trade down Arsenic."** (up, for a grade 1 material, which cannot go down). At most two per
dock. The line does not name the trader type or the exchange rates. See
[Materials](materials.md).

## Colonisation shopping

Id: `project_market`. When you open the market at a station: **"Sells 5 project commodities."**
How many of what your colonisation projects still need are in stock. Once per dock, when the
game writes the market, which is when you open it. Nothing if no project, or nothing in stock.
Speech only. See [Colonisation](colonisation.md).

## Mission reminders

Id: `missions`.

- **"Mission expires within an hour."** and **"Mission expires within 10 minutes."** Once per mission per
  threshold. Several at once: "3 missions expire within an hour." The game announces neither.
- On a jump: **"Mission target in this system."** or **"Mission target faction here."**

Limit: the destination and target faction are only known for missions accepted in the journal
astromech is following, so a mission accepted in an earlier session does not trigger the
target-here lines. Expiry warnings are unaffected.

## Session summary

Id: `session_summary`. When your session ends (you log out, or return to the main menu), it says what the
session earned and covered: **"Earned 4.2 million credits, 1.9 million an hour."** (the rate waits for a
minute), **"38 jumps, 1204 light years."**, **"6 bounties awarded."** A zero is left out, and a
session with nothing to report says nothing. It speaks once per session and starts over at the next
login. Ask for it any time by voice: "session summary". The numbers come from the journal history, a
moment after the game writes the line.

## Not spoken on purpose

The HUD or another tool covers these, so astromech stays quiet: star class, security and
allegiance on arrival; low fuel and scoop progress; which body is in the system (aside from
discoveries); signal counts per body; who scanned you; trade exchange rates; and the pads of
outposts and surface ports.
