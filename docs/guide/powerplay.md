# Powerplay

The **Powerplay** page answers three questions at a glance: how far you are from the next rank,
how long the cycle has left, and what the system you are in wants. Your power's name heads the
page, with how long you have been pledged and your rank.

**What it needs:** the journal (your pledge, rank and merits, and the system's Powerplay state from
your last jump), and for **Where to work** a network connection (Spansh). If you are not pledged
to a power, the page says **Not pledged** and how to pledge, and shows only the system.

## Next rank

The merits still to earn for the next rank, the page's main figure, with a bar from what your
current rank needed to what the next one needs, filled to your lifetime merits. Under it, what the
next rank needs and its reward. Below rank 1 the page says to complete the five initial
assignments instead.

## This cycle

How long the weekly cycle has left ("2 d 14 h left") and when it ends, in UTC ("Ends Thursday
07:00 UTC").

## Your merits

Merits this cycle, with this session's and your lifetime total under them. Point at the cycle's
figure for what it counts: what the journal logs since the cycle began. Weekly assignment rewards
are not logged, so they are not in the count. The game sometimes logs one award twice but counts
it once; the count drops back by that award at your next award or login.

## This system

The system of your last jump or login (point at its name), with a copy button and Spansh and
Inara links, then its state and who controls it, and what your power can do here: reinforce,
undermine or acquire. The game shows this only in the system and galaxy maps, not on the HUD.

In a controlled system, one bar shows this cycle's reinforcement against undermining, with both
figures under it and a line saying which is ahead and by how much. A thin bar under that shows
the progress through the state's band; point at its percentage for which band. In a system no
power controls, each power acquiring it has a bar with its progress. Other powers active here are
named last.

## What earns merits here

What earns your power merits in this system, in a short list. Point at an activity for how to
do it. The power's ethos activities come first, tagged **Ethos** (bonus merits), and the
controlling power's **Weak spots** tagged too; **On foot** marks an activity done on foot. A small
"?" marks a rule not yet checked in game; pointing at the activity gives the reason. The rules come
from community guides and are dated at the foot of the list ("as of ... rules v..."). Frontier
rebalances Powerplay often, so check a surprising line in game.

It lists reinforcing (when your power controls the system), undermining (another power's system) and acquiring (an unoccupied system your power is acquiring). Only an unoccupied system your power is not acquiring has nothing, and the page says so.

## Where to work

Nearby systems where your power wants work, on the usual search ([Places](places.md#how-searches-work)):

- **Acquire.** Unoccupied systems your power can take, with its progress.
- **Reinforce.** Your power's systems.
- **Undermine.** Other powers' systems.

Choose how far to look: 20, 40 or 80 light years (40 by default). Each row has the jumps and the distance,
the system and its Powerplay state (with how many other powers are active there; point at it for which), and for a controlled system this cycle's reinforcement over its undermining. The filters, under **Change**, are this radius and the age of the data. Source: Spansh.

## By voice and as callouts

- "Powerplay rank", "how many merits", "who controls this system", "what earns merits here" and
  "where can I acquire / reinforce / undermine near me" are [voice commands](voice-commands.md) and
  [lookups](lookups.md#powerplay-targets). Voice searches within 40 light years.
- The arrival briefing says "Undermining target." or "Acquisition target." for your power.
  The `powerplay_arrival` [callout](callouts.md#powerplay-arrival) says a system's whole Powerplay state
  on arrival; it is off by default.

## Limits

- Not pledged: no standing, no activities, no targets. The game says you are pledged when you
  start it, so a login with no pledge clears one you left in a journal you have since deleted.
- The system view is from the last jump; the game updates it every cycle.
- The merit rules are guides, not Frontier's tables.
