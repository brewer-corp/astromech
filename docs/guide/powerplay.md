# Powerplay

The **Powerplay** page is for daily use: your standing, the state of the system you are in, what earns
merits here, and where your power wants work nearby.

**What it needs:** the journal (your pledge, rank and merits, and the system's Powerplay state from
your last jump), and for **Where to work** a network connection (Spansh). If you are not pledged
to a power, the page says so and only shows the system.

## My standing

Your power, **rank**, **merits** (lifetime), merits **this week** and when the cycle ends, merits
**this session**, how long you have been **pledged**, and the **next rank** with its reward and
the merits to go. Week merits count what the journal logs since Thursday 07:00 UTC. Weekly
assignment rewards are not logged, so they are not in the count. The game sometimes logs one
award twice but counts it once; the count drops back by that award at your next award or login.

## This system

For the system of your last jump or login: who controls it, its state and a **control** bar
showing progress through the band, this cycle's **reinforcement** and **undermining**, and any
other powers active there. For an unoccupied system it shows who is acquiring it and how far.

When the system is one your power could work, a line says what: reinforce, undermine or acquire.
The game shows this only in the system and galaxy maps, not on the HUD. The values are as of
your last jump.

## What earns merits here

What earns your power merits in this system, with how, in a short list. The power's ethos
activities come first, tagged **ethos** (bonus merits), and the controlling power's **weak
spots** tagged too. A small "?" marks a rule not yet checked in game, with the reason. The
rules come from community guides and are dated at the foot of the list ("as of ... rules v...").
Frontier rebalances Powerplay often, so check a surprising line in game.

It lists reinforcing (when your power controls the system), undermining (another power's system) and acquiring (an unoccupied system your power is acquiring). Only an unoccupied system your power is not acquiring has nothing, and the page says so.

## Where to work

Nearby systems where your power wants work, on the usual search ([Galaxy](galaxy.md#how-searches-work)):

- **Acquire.** Unoccupied systems your power can take, with its progress.
- **Reinforce.** Your power's systems.
- **Undermine.** Other powers' systems.

Choose how far to look: 20, 40 or 80 light years (40 by default). Each row has the system,
jumps and distance and its Powerplay state. The only filter is the age of the data. Source: Spansh.

## By voice and as callouts

- "Powerplay rank", "how many merits", "who controls this system", "what earns merits here" and
  "where can I acquire / reinforce / undermine near me" are [voice commands](voice-commands.md) and
  [lookups](lookups.md#powerplay-targets). Voice searches within 40 light years.
- The arrival briefing says "Undermining target." or "Acquisition target." for your power.
  The `powerplay_arrival` [callout](callouts.md#powerplay-arrival) says a system's whole Powerplay state
  on arrival; it is off by default.

## Limits

- Not pledged: no standing, no activities, no targets.
- The system view is from the last jump; the game updates it every cycle.
- The merit rules are guides, not Frontier's tables.
