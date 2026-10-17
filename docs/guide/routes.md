# Routes

The **route follower** is not in this version of the guide yet. In short: **Galaxy → Route**
shows the route astromech follows, and its next waypoint is on your clipboard, ready to paste
into the galaxy map's search box.

## Plot a route with Spansh

On **Galaxy → Route**, **Plot with Spansh** plots a route for the ship you are flying, and
astromech follows it straight away. If you are in the start system, the first jump is next and
on your clipboard.

- **Neutron** boosts from neutron stars on the way. The waypoints are the neutron stars and the
  destination: plot each leg in the galaxy map. **Efficiency** (1 to 100, 60 by default) is how
  far Spansh may stray from the straight line to use neutron stars.
- **Exact** has every jump as a waypoint, worked out for your frame shift drive, fuel tank and
  mass. Waypoints marked **fuel** are where to scoop. **Refuel at every scoopable star** (on by
  default) tops up whenever it can; turn it off to stop only where the tank needs it.

**From** blank means your current system. **To** is a system name as Spansh knows it. Above the
button is the jump range used: your ship's range with a full tank and the cargo aboard now,
worked out from its last loadout in the journal, engineering and a Guardian FSD booster
included. If some modules are not known, it says the range may be off.

When it is done, the page says how many jumps, how far, and how many neutron boosts and refuel
stops. "Next waypoint" by voice adds what to do there: *"Next waypoint. Hyades Sector DB-W
b2-6, for fuel."*

It needs:

- internet: Spansh plots the route. It can take a minute. You can leave the page; the route is
  still followed when it is done. One route is plotted at a time.
- a ship flown since the journal began, so its loadout is there.
- a fuel scoop. Exact routes plan their refuel stops by scooping, and astromech does not check
  that the ship has one.

Limits:

- No FSD injections, via systems or fuel reserve. Exact routes always use neutron boosts.
- Exact routes assume the usual neutron boost, four times the range. A drive with a stronger
  boost is planned as an ordinary one, so the route may have more jumps than it needs.
- Neutron routes do not plan fuel.
- A misspelled system gives "Spansh turned the request down" with Spansh's reason, such as
  "Unable to find route" or "Could not find finishing system".

What exists today:

- The game's own route (`NavRoute.json`) is read for [callouts](callouts.md#fuel-and-route) when you plot one
  in the galaxy map: the jump count and distance, white dwarfs and neutron stars on it, fuel gaps and
  the next scoopable star. "Jumps left" and "next system" are [voice commands](voice-commands.md#questions-about-your-ship).
- [Trade, Route](trade.md#route) finds a profitable trade route with Spansh. It does not plot a flight
  route.
- Every list of "nearest" places counts jumps along the known star map at your ship's range:
  [Galaxy](galaxy.md#jumps-and-distance). That is for choosing where to go, not a route to follow.
- Plotting a route by voice ("plot route to ...") is recognised and answered "Route plotting is
  not available yet."
