# Routes

Elite has no way for another program to set your destination, and you cannot type in the galaxy
map in VR. astromech fills the gap with a **route follower**: it keeps one route, puts its next
waypoint on your clipboard, and pastes it into the galaxy map's search box when you ask. You still
plot each leg in the galaxy map yourself. A route you plot in the galaxy map is followed too.

## The route rail

Galaxy keeps the route you are following on the **right** while you search. The same view is
the **Route** page in the command palette:

- its name, where it came from, and how many waypoints are left;
- the **next waypoint**, with what is there (an engineer, "fuel", "neutron boost"), and whether
  it is **on your clipboard**. If the clipboard could not be written, it says why;
- every stop, with the ones behind you struck out and **you are here** next to the first stop
  when you are in it;
- **Copy** (something else took the clipboard), **Skip**, **Back** and **Clear route**;
- **Paused** and **Resume** when a route you plotted in the galaxy map took over from another
  one ([Routes plotted in the galaxy map](#routes-plotted-in-the-galaxy-map)).

[Import](#import-a-route) sits under the live route.

A route comes from:

- **Plot to a system** in the top bar ([Plot a route](#plot-a-route));
- the galaxy map, when you plot a route there ([Routes plotted in the galaxy map](#routes-plotted-in-the-galaxy-map));
- **Follow** on the engineer trip ([Engineering](engineering.md));
- **Go** on a search result (Services, Outfitting, Shipyard, Trade, Exploration, Colonisation,
  Powerplay targets and the material sites). That is a route with one stop: the result's
  system. If you are already in that system, the route is finished straight away and nothing is
  copied;
- **Import** on this page.

There is one route at a time: a new one replaces the old, and **Clear route** clears a paused
one too. astromech keeps it while Elite or the
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
| "next waypoint", "copy next waypoint", "copy waypoint", "what's the next waypoint", "where's the next waypoint" | Copies it again. *"Next waypoint. Deciat. For Felicity Farseer. 3 waypoints left."* |
| "skip waypoint", "skip this waypoint", "skip to the next waypoint" | Skips one and copies the one after. *"Next waypoint. Kuk."* |
| "previous waypoint", "back one waypoint", "go back a waypoint" | Goes back one and copies it. |
| "clear route", "clear the route", "end route", "stop following the route" | *"Route cleared."* |
| "paste", "paste it", "paste system", "paste waypoint", "paste system name" | See [Paste into the galaxy map](#paste-into-the-galaxy-map). |

With no route set: *"No route to follow. Plot one in the app."* (*"No route to clear."*
for "clear route"). Past the last stop: *"Route complete."*

"Plot route to ..." by voice is not built: astromech says *"Plotting is not available.
Plot in the app, then paste."*

## Routes plotted in the galaxy map

When you plot a route in the galaxy map, astromech reads it from the game (`NavRoute.json`):

- **A leg of the route you are following.** If it ends at the next waypoint, or one further on,
  nothing changes. This is how you fly a neutron route: paste the next neutron star, plot to it,
  and astromech keeps the neutron route.
- **Anything else** becomes the route astromech follows, named "Route to" its destination, with
  every jump as a waypoint. The page says **Next jump** and *Plotted in the galaxy map*. It moves
  on as you jump, but astromech says nothing and copies nothing: the game already has the route
  plotted and shows the next jump. Copy, Skip and Back still copy when you press them.
- **A detour.** If you were following a route of your own (a neutron route, say) and it isn't
  finished, it is **paused**, not lost. The route rail shows it with **Resume**, which follows it
  again and puts its next waypoint on your clipboard. Plotting in the galaxy map to one of its
  waypoints ahead resumes it too, with nothing copied (you have just plotted it).
- **Clearing the route in the galaxy map** stops following a route that came from the galaxy
  map. A route of your own stays: clearing a leg, or arriving at the end of one, doesn't end it.

Only plots made while astromech is running count: the route already plotted when astromech
starts is not picked up. With `[callouts.route_follow] enabled = false`, galaxy-map routes are
not followed either. Plotting a new route in astromech replaces a paused one.

## Plot a route

Type a system into **Plot to a system** in the top bar, on any page. Suggestions come from
Spansh as you type; pick one with the arrow keys and Enter, or a click. A panel opens under the
box: choose the plotter and press **Plot and follow**, or press Enter again. astromech plots a
route from your current system for the ship you are flying and follows it straight away: the
first jump is next and on your clipboard. **Show the route** opens the Route page. The panel
also has the system's **Spansh** and **Inara** pages. Escape or a click elsewhere closes it.

- **Neutron** boosts from neutron stars on the way. The waypoints are the neutron stars and the
  destination: plot each leg in the galaxy map. With the [neutron star file](#the-neutron-star-file)
  downloaded, astromech works the route out on this PC in a second or two. Without it, Spansh
  plots it, and **Efficiency** (1 to 100, 60 by default) is how far Spansh may stray from the
  straight line to use neutron stars. Routes plotted on this PC have no such setting: they take
  the fewest jumps astromech can find.
- **Exact** has every jump as a waypoint, worked out by Spansh for your frame shift drive, fuel
  tank and mass. Waypoints marked **fuel** are where to scoop. **Refuel at every scoopable star**
  (on by default) tops up whenever it can; turn it off to stop only where the tank needs it.

Routes always start from your current system, as the journal last reported it. Above the
button is the jump range used: your ship's range with a full tank and the cargo aboard now,
worked out from its last loadout in the journal, engineering and a Guardian FSD booster
included. It updates within a few seconds when you swap ships or your cargo changes. If some
modules are not known, it says the range may be off. Both plotters use your drive's own neutron
boost: six times the range with the size 8 Mk II supercharge-optimised drive, four times with any
other.

When it is done, the panel says how many jumps, how far, how many neutron boosts and refuel
stops, the jump range the route was plotted at, and where it was plotted: **on this PC** or **by
Spansh**. A long route plotted on this PC may say **near the fewest jumps**: see the limits
below. If the neutron star file is there but can't be read, Spansh plots the route instead and
the panel says why; press Download to replace the file. "Next waypoint" by voice adds what to do
there: *"Next waypoint. Hyades Sector DB-W b2-6. For fuel."*

It needs:

- internet, for Spansh: exact routes, and neutron routes until the neutron star file is
  downloaded. Spansh can take a minute. You can close the panel; the route is still followed when
  it is done. One route is plotted at a time.
- your current system, from the game's journal: log in to Elite at least once on this PC.
- for a neutron route on this PC, internet only to find a destination that isn't on
  astromech's star map ([Galaxy](galaxy.md#jumps-and-distance)) yet: it asks Spansh where the
  system is.
- a ship flown since the journal began, so its loadout is there.
- a fuel scoop. Exact routes plan their refuel stops by scooping, and astromech does not check
  that the ship has one.

Limits:

- No FSD injections, via systems or fuel reserve. Exact routes always use neutron boosts.
- Neutron routes do not plan fuel.
- Neutron routes plotted on this PC count the normal jumps between neutron stars from the
  distance. They don't check that there are stars to jump to in sparse space, between the arms
  and at the galaxy's edge, so a leg there may take more jumps than the page says, or need a
  detour.
- A short route plotted on this PC has the fewest jumps possible with the known neutron stars.
  On a long route the search stops at a good route instead of trying every one: it is near the
  fewest jumps (a few more on a route across the galaxy), and the panel says so.
- A misspelled system gives "Spansh does not know a system called ..." for a route plotted on
  this PC, and "Spansh turned the request down" with Spansh's reason, such as "Unable to find
  route" or "Could not find finishing system", for one Spansh plots.

### The neutron star file

Under **Neutron**, the plot panel also shows the **neutron star file**: a list of every known neutron
star (about 5 million), built from Spansh's neutron star data, for plotting neutron routes on
this PC instead of on Spansh. Until it is downloaded, Spansh plots neutron routes.

- **Download** fetches it once: about 71 MB, unpacked to about 184 MB as `neutron.bin` in
  astromech's data folder (the folder that holds `galaxy.db`; `data_dir` in `astromech.toml`
  moves it). The panel shows how much has come in. Nothing is downloaded until you press it.
- The download is checked against the published checksum before it is used. If it fails, is
  damaged or is interrupted, the file you had (or none) stays as it was. Press Download again.
- Once it is there, the panel shows how many neutron stars it lists and the date it was built.
  **Check for a newer file** asks whether a different file has been published since. If one
  has, **Download the newer file** replaces yours.
- It needs internet only for the download and the check. The file is rebuilt by hand now and
  then, not with every astromech update.
- astromech reads the file when it plots its first neutron route, or when a download finishes,
  and keeps it in memory (about 80 MB) while the window is open. A newer file you download is
  used from the next route.

## Import a route

**Import** takes a pasted list. Paste a Spansh CSV export (neutron, road to riches, fleet
carrier or any other: astromech reads its "System Name" column) or one system name per line,
give it a name if you like, and press **Follow this route**.

## Other routes in astromech

- The game's own route (`NavRoute.json`) is followed as described in
  [Routes plotted in the galaxy map](#routes-plotted-in-the-galaxy-map), and read for
  [callouts](callouts.md#fuel-and-route) when you plot one: the jump count and distance, white dwarfs and neutron stars on
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
