# Routes

Elite has no way for another program to set your destination, and you cannot type in the galaxy
map in VR. astromech fills the gap with a **route follower**: it keeps one route, puts its next
waypoint on your clipboard, and pastes it into the galaxy map's search box when you ask. You still
plot each leg in the galaxy map yourself.

## The route rail

Galaxy keeps the route you are following on the **right** while you search. The same view is
the **Route** page in the command palette:

- its name, where it came from, and how many waypoints are left;
- the **next waypoint**, with what is there (an engineer, "fuel", "neutron boost"), and whether
  it is **on your clipboard**. If the clipboard could not be written, it says why;
- every stop, with the ones behind you struck out and **you are here** next to the first stop
  when you are in it;
- **Copy** (something else took the clipboard), **Skip**, **Back** and **Clear route**.

[Plot with Spansh](#plot-a-route-with-spansh) and [Import](#import-a-route) sit under the live
route, folded closed when a route is already following.

A route comes from:

- **Plot and follow** under Plot with Spansh on this page;
- **Follow** on the engineer trip ([Engineering](engineering.md));
- **Go** on a search result (Services, Outfitting, Shipyard, Trade, Exploration, Colonisation,
  Powerplay targets and the material sites). That is a route with one stop: the result's
  system. If you are already in that system, the route is finished straight away and nothing is
  copied;
- **Import** on this page.

There is one route at a time: a new one replaces the old. astromech keeps it while Elite or the
window is closed, and after a restart. You can set a route with Elite closed.

## Following a route

When you set a route, its first stop goes on the clipboard. If you are already in that system,
the route starts at the stop after it. Two stops in the same system in a row (two engineers in
one system, a road-to-riches list with a row per body) are one stop.

When you arrive in the next waypoint (a jump, a carrier jump, or logging in there), or in a
later one (skipping the ones in between), the route moves on, the next waypoint goes on the clipboard and astromech says *"Next waypoint. Deciat."*
If the clipboard could not be written it adds *"Not copied."* At the last stop it says
*"Final waypoint reached. Route complete."* The line also shows as a toast. Jumping somewhere
that is not on the route changes nothing.

The clipboard is written only when the route changes like this, or when you ask, never on a
timer, so it does not fight you for the clipboard.

This is the `route_follow` [callout](callouts.md#route-follower). Like other callouts it is not
spoken in a fight or while docking (the clipboard and the toast still happen), and
`[callouts.route_follow] enabled = false` turns following off: jumps then do not move the route
or write the clipboard.

**At startup**, astromech reads the journal to catch up on where you are. That catch-up only
updates where you are: it never moves the route, writes the clipboard or speaks. The same goes
for an old journal that a backup or sync tool rewrites.

## Paste into the galaxy map

Open the galaxy map, select its search box, then say **"paste"** or press your **paste button**.
astromech sends Ctrl+V, waits half a second, then Enter to run the search. Then plot the route as
usual.

- It works only while the galaxy map is open. Anywhere else it says *"Open the galaxy map and
  select its search box first."* and presses nothing, because V or Enter may be bound to
  something on your ship.
- It checks again before Enter. If the map closed after Ctrl+V, Enter is not sent: *"Galaxy map
  closed. Search not sent."*
- It goes through the same safety checks as other key presses (the kill switch, Elite being the
  active window, and a repeat within 3 seconds being refused). See
  [Voice](voice.md#what-it-checks-before-it-acts).
- When it worked: *"Pasted."*

The paste button is any button on any device, set in `astromech.toml`:

```toml
[controls]
paste = { device = "231D3201", key = "Joy_12" }
```

Pick a button Elite does not use: Elite still sees the press, so its own binding would fire too.
The [Key tester](controls.md#key-tester) shows a button's device and key names. There is no
row for it in Settings yet; restart astromech after editing the file.

## Voice commands

| You say | You hear |
|---|---|
| "next waypoint", "copy next waypoint", "copy waypoint", "what's the next waypoint", "where's the next waypoint" | Copies it again. *"Next waypoint. Deciat, for Felicity Farseer. 3 waypoints left."* |
| "skip waypoint", "skip this waypoint", "skip to the next waypoint" | Skips one and copies the one after. *"Next waypoint. Kuk."* |
| "previous waypoint", "back one waypoint", "go back a waypoint" | Goes back one and copies it. |
| "clear route", "clear the route", "end route", "stop following the route" | *"Route cleared."* |
| "paste", "paste it", "paste system", "paste waypoint", "paste system name" | See [Paste into the galaxy map](#paste-into-the-galaxy-map). |

With no route set: *"No route to follow. Set one on the Route page."* (*"No route to clear."*
for "clear route"). Past the last stop: *"Route complete."*

"Plot route to ..." by voice is not built: astromech says *"Plotting is not available. Use the
Route page, then paste."*

## Plot a route with Spansh

On the Galaxy route rail, **Plot with Spansh** plots a route for the ship you are flying, and
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

## Import a route

**Import** takes a pasted list. Paste a Spansh CSV export (neutron, road to riches, fleet
carrier or any other: astromech reads its "System Name" column) or one system name per line,
give it a name if you like, and press **Follow this route**.

## Other routes in astromech

- The game's own route (`NavRoute.json`) is read for [callouts](callouts.md#fuel-and-route) when
  you plot one in the galaxy map: the jump count and distance, white dwarfs and neutron stars on
  it, fuel gaps and the next scoopable star. "Jumps left" and "next system" are
  [voice commands](voice-commands.md#questions-about-your-ship).
- [Trade, Route](trade.md#route) finds a profitable trade route with Spansh. It does not plot a
  flight route.
- Every list of "nearest" places counts jumps along the known star map at your ship's range:
  [Galaxy](galaxy.md#jumps-and-distance).

## Limits

- You still select the search box and plot each leg in the galaxy map. astromech does not drive
  the galaxy map for you.
- Paste is refused on foot: the game does not report the galaxy map there.
- If the search box is not selected, V and Enter go to the galaxy map itself and may do what
  they are bound to there.
- The check before Enter sees the map close only once the game reports it, which can take about
  a second, longer than the half-second wait. A map closed in that moment can still get Enter.
- A synthetic Ctrl+V, and Enter leaving the search box, are not yet confirmed in the real game
  (flat screen or VR).
