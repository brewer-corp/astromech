# Using the app

The window is for setup and for looking things up between jumps. While you fly, voice does the work
and the window can stay closed.

## The window

The header has plain text tabs for the areas, with the area's pages as sub-tabs below.

| Area | Pages |
|---|---|
| **Controls** | My controller, All keybinds, Presets, Key tester: [Controls](controls.md) |
| **Engineering** | Ships, Stored modules, Materials, On foot, Engineers: [Ships](ships.md), [Materials](materials.md), [Engineers and on foot](engineering.md) |
| **Galaxy** | Services, Outfitting, Shipyard: [Galaxy](galaxy.md) |
| **Trade** | Colonisation, Commodities: [Colonisation](colonisation.md), [Trade](trade.md) |
| **Exploration** | System, Biology: [Exploration](exploration.md) |
| **Powerplay** | [Powerplay](powerplay.md) |
| **Logbook** | Logbook, Session, Mining, Station: [Logbook](logbook.md), [Mining](mining.md) |
| **Settings** | General, Data sharing, Diagnostics, Voice tester: [Settings](settings.md), [Data sharing](data-sharing.md), [Voice tester](voice-tester.md) |

It opens on **My controller**. At the right of the header, a short line says what astromech is
doing: "Waiting for Elite", "Elite running · Voice ready", "Voice paused", "Kill switch on".
**Finish setup** appears next to it while voice is missing something; it opens the health check.

Each page has a small **Help** link that opens its page in this guide in your browser.

Pages that follow the game (ships, materials, on foot, station, session, exploration, biology,
mining, colonisation, Powerplay, the logbook, the route, data sharing and diagnostics) read your journal again
every few seconds. If a later read fails, the page keeps what it last showed and adds a line,
"Could not refresh:" and the reason. The line goes away on the next read that works. If the first
read fails, the page says "Could not read" and the reason instead.

## Command palette

Press **Ctrl+K** anywhere in the window to jump to something by name. Type part of a name:

- a **page** ("trade", "powerplay"),
- a **material** ("heat radiators"), which opens it on the Materials page,
- a **ship** of yours, a **blueprint** or an **experimental**, which opens that ship's build planner,
- a **module** you have fitted or any module, which opens the nearest stations selling it,
- a **ship** from the catalogue, which opens the nearest shipyards selling it.

Letters of each word work ("fsd" finds Frame Shift Drive). Arrow keys move, Enter opens, Esc
closes. It does not open while a dialog such as a bind capture has the keyboard.

## System names

Anywhere a system is named it has a **Copy** button that puts the name on the clipboard, for
pasting into the galaxy map's search in Elite. Many have **Spansh** and **Inara** buttons that
open that system or station in your browser. astromech does not type into the game for you.

## Searches near you

The "nearest" pages share one kit: where to search from, filters, and a list with jumps and
distance. It is described once in [Galaxy](galaxy.md#how-searches-work).

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
