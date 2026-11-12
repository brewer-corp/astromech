# Using the app

The window is for setup and for looking things up between jumps. While you fly, voice does the work
and the window can stay closed.

## The window

The header has plain text tabs for the areas. Areas with more than one page list them as
sub-tabs below. Under the header, on every page,
the [route strip](routes.md#the-route-strip) shows the next stop of the route you are following;
click its destination for the full Route page, which [Find or plot](#find-or-plot) also opens.

| Area | Pages |
|---|---|
| **Controls** | My controller, All keybinds, Presets, Key tester: [Controls](controls.md) |
| **Engineering** | Ships, Stored modules, Materials, On foot, Engineers: [Ships](ships.md), [Materials](materials.md), [Engineers and on foot](engineering.md) |
| **Galaxy** | Places, Route: [Places](places.md), [Routes](routes.md) |
| **Trade** | Colonisation, Commodities: [Colonisation](colonisation.md), [Trade](trade.md) |
| **Exploration** | System, Biology: [Exploration](exploration.md) |
| **Powerplay** | [Powerplay](powerplay.md) |
| **Logbook** | Logbook, Session, Mining, Station: [Logbook](logbook.md), [Mining](mining.md) |
| **Settings** | General, Data sharing, Diagnostics, Voice tester: [Settings](settings.md), [Data sharing](data-sharing.md), [Voice tester](voice-tester.md) |

The header holds the **Find or plot** box too (below). In a window too narrow for everything on
one line (about 1460 pixels wide, about 1610 with Finish setup), the area tabs move to a row of
their own under the name.

It opens on **My controller**. **Finish setup** appears in the header while voice is missing
something; it opens the health check.

A status bar runs along the foot of every page. On the left is the ship you're flying: its name,
type and unladen jump range ("Heavy Lifter Anaconda Jump 18.0 ly"), then, while Elite is running,
its fuel and cargo against what the tank and hold take ("Fuel 24.6 | 32 t", "Cargo 120 | 256 t").
They follow the game every 2 seconds, and the jump range updates after you refit or swap ships.
With Elite closed, or if astromech can't read them, the fuel and cargo figures are left out and
the ship stays. On the right, a short line says what astromech is doing: "Waiting for Elite",
"Elite running · Voice ready", "Voice paused", "Kill switch on".

Each page has a small **Help** link that opens its page in this guide in your browser.

Point at an icon, a value or a button that has more to say and its explanation appears at once
beside it: what a button does, the conditions behind a figure ("Full tank and full cargo"), the
detail of a warning. It moves to the next thing you point at and goes when you move away, scroll
or press **Esc**. Moving to one with **Tab** shows it too, and screen readers read the same words.
Pages keep their explanations there rather than in notes under the content: a column heading says
what its figures mean, a tag such as **Ethos** says what it marks. Where the data comes from and
how old it is stays on the page, in one small line.

Pages that follow the game (ships, materials, on foot, station, session, exploration, biology,
mining, colonisation, Powerplay, the logbook, the route, data sharing and diagnostics) read your journal again
every few seconds, and so do Settings, setup and the engineer trip. If a later read fails, the page keeps what it last showed and adds a line,
"Could not refresh:" and the reason. The line goes away on the next read that works. If the first
read fails, the page says "Could not read" and the reason instead (setup's push-to-talk and
kill switch rows say only "Could not read").

## Find or plot

The **Find or plot** box in the header finds things by name and plots routes, from any page.
Press **Ctrl+K** anywhere in the window to put the cursor in it. Type part of a name:

- a **system**, for a route to it. Its row has **Plot**, which opens the
  [plot panel](routes.md#plot-a-route), and the two route buttons every search result has:
  **Add stop** and **Set destination** ([From a search result](routes.md#from-a-search-result)).
  What they did shows under the box. System names come from Spansh as you type;
- a **finder**, or what you want it to find, which opens that [Places](places.md) finder
  filled in: "interstellar factors", "raw trader", "outfitting fsd", "shipyard python", "sell
  painite", "crystalline shards", "painite hotspots". The finder's name can come first or last,
  and the start of each word is enough ("sel painite");
- a **page** ("trade", "powerplay");
- a **material** ("heat radiators"), which opens it on the Materials page;
- a **ship** of yours, a **blueprint** or an **experimental**, which opens that ship's build planner;
- a **module** you have fitted, which opens the nearest stations selling it at that size.

Pages, finders and systems are listed together, best match first: a system you typed in full
comes first, and one you have only started comes after the pages and finders whose names start
the same way. Letters of each word work ("fsd" finds Frame Shift Drive). Arrow keys move, Enter
opens the highlighted row (for a system, the plot panel), Esc closes the list, then the panel,
then leaves the box. With nothing listed, Enter plots to what you typed. Ctrl+K does nothing
while a dialog such as a bind capture has the keyboard.

## System names

Anywhere a system is named it has a **Copy** button that puts the name on the clipboard, for
pasting into the galaxy map's search in Elite. Many have **Spansh** and **Inara** buttons that
open that system or station in your browser. astromech does not type into the game for you.

## Searches near you

The "nearest" pages share one kit: where to search from, filters, and a list with jumps and
distance. It is described once in [Places](places.md#how-searches-work).

## What needs the network

Nothing you do in the window or by voice needs a network, except these. Each is only asked when
you use it.

| Service | Used for |
|---|---|
| **Spansh** (spansh.co.uk) | Nearest stations, modules and ships; raw material sites; faction states; Powerplay targets; colonisation candidates; trade routes; system name suggestions; biology other players found |
| **Ardent Insight** (api.ardent-insight.com) | Commodity prices and markets, the cargo and colonisation buy lists |
| **EDSM** (edsm.net) | Filling in the star map around you as you travel; the bodies other players scanned, on arrival; your flight log, if you turn on EDSM sharing |
| **EDDN** | Only if you turn on [Data sharing](data-sharing.md) |
| **GitHub** | Update checks and downloads; the speech models (sherpa-onnx releases) |

If a service is unreachable, the page says so. The pages that read the journal, the keybinds
pages and your history work offline.
