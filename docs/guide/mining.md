# Mining

**Logbook, Mining** shows the current mining run: how much has been refined, by ore, and the rate.
It follows your journal live while you mine.

**What it needs:** Elite running, or the journal from your last run. No network.

## What it shows

- **Refined**: total tons refined in this run.
- **Per hour**: tons per hour, shown once a minute has passed. This is tons, not credits.
- **By ore**: tons refined for each ore, most first.
- A note: when the run started (UTC), "measured to the latest journal line".

## When a run starts and ends

A run starts at your first prospector or collector limpet, prospected rock or refined ton after
docking or loading the game. Other limpets, such as repair or fuel, don't start one. **Docking ends
the run.** Until then the page says "No mining yet".

It counts what the refinery has made, not what is in the hold or what you sold. For earnings from
selling, see [Session](logbook.md#session). To sell, [Trade](trade.md) finds the best price.

## Finding hotspots

Where to mine is a finder in Places: **Galaxy, Places, Mining hotspots**
([Places](places.md#mining-hotspots)). Pick a **mineral** and it lists the rings with a hotspot
of it, most hotspots first, within 100 light years of where you are, the route's next stop or a
system you type. You can narrow it to one **ring type** and to **pristine** or **major** reserves.

By voice, ask "nearest painite hotspot" (any of the [hotspot minerals](places.md#mining-hotspots)).
astromech says "Looking that up." and then the nearest ring, how far, how many jumps, and its
hotspots and reserves:

> Nearest Painite hotspot. Barnard's Star 5 A Ring. 6.0 light years, 1 jump. 2 hotspots, depleted reserves.

The voice answer is always the nearest ring of any type and reserve, however far, so it can name
a ring the finder's choices leave out. Add "near" and a system to ask from somewhere else
([Voice lookups](lookups.md#mining-hotspots)). Both need a network connection (Spansh).

## Spoken help while mining

Mining has its own [callouts](callouts.md#prospector): a prospector limpet's result is read out
when it has an ore worth mining, and a motherlode is always called out. The thresholds are per ore
in [Settings](settings.md#prospector-callouts).

## Limits

- Only what the refinery has made is counted.
- If the core is not running the page says "Could not read the mining run" and why. Before any mining, it says "No mining run yet."
