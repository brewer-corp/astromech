# Plotting and routes

Elite has no API for setting a destination. Every tool that "auto-plots" either puts the system
name on the clipboard or drives the galaxy map with simulated keys. The galaxy-map search is
the one text field a player has to fill in mid-flight, and **you can't type in VR**. That is the
problem this solves. astromech does it in two layers:

1. **Clipboard (built).** astromech follows a route. The next waypoint is always on the
   clipboard, and "paste" (spoken, or a HOTAS button) puts it into the search box. You still
   select the search box and plot the route yourself.
2. **Plotting macro (designed, not built; off by default when it is).** astromech opens the map,
   searches, plots, and checks `NavRoute.json` to see whether it worked.

Research sources (read 15 Oct 2026): COVAS:NEXT `Plotter.py`, Directive 47 (d47)
`NavigationCapability.cs` + `RoutePlotWatch.cs` + `docs/capabilities/navigation.md`, EliteIntel
`RoutePlotter.java`, EDAPGui `EDGalaxyMap.py`, EDMC SpanshRouter, Auto_Neutron, CATS.
[REPORT.md §6](REPORT.md) has the 8 Oct survey.

## Layer 1: follow a route, paste the next waypoint

| Piece | Where |
|---|---|
| Route model, Spansh CSV / plain-list import, the one active route saved to `<data>/route.json` | `crates/astromech-core/src/route.rs` |
| Clipboard write (CF_UNICODETEXT) | `crates/astromech-core/src/clipboard.rs` |
| Follower: a hub subscriber that moves the route on whenever the journal's system changes (`FSDJump`, `CarrierJump`, `Location`) | `session::follow_route` |
| Paste: Ctrl+V, 500 ms, Enter. **Only while `GuiFocus == 6`** (galaxy map), through the policy layer | `Runner::paste` |
| Paste button: `controls.paste = { device, key }`, read with the same Raw Input listener as push-to-talk | `controls.rs`, `config.rs` |
| Voice: "next waypoint", "skip waypoint", "previous waypoint", "clear route", "paste" | `astromech-intent` MVP grammar, `Assistant::route_command` |
| Control API: `route`, `route_set`, `route_skip`, `route_back`, `route_clear`, `route_copy` | `Core::route_cmd`; shell `route_call` |
| UI: Navigation → Route page; **Follow** on the engineer trip; **Go** on Nearby lists | `app/src/modules/route`, `lib/route.ts` |

Behaviour:

- The core owns the route, not a session. It can be set from the window while Elite is closed
  and survives restarts.
- Setting a route copies its first stop straight away. If you are already in that system, the
  route starts at the stop after it. Being in a *later* stop at that moment doesn't skip
  anything.
- Arriving in the next waypoint, or a later one (stops skipped), moves the route on, copies the
  next system and says *"Next waypoint: Deciat, for Felicity Farseer. It's on your clipboard. 3
  waypoints left."* The last one says the route is finished. Catching up on the journal
  replayed at start is silent.
- Consecutive stops in the same system are merged, which covers two engineers in one system and
  road-to-riches rows (one per body).
- The clipboard is written only on these events, never on a timer, so it doesn't fight you for
  the clipboard.

Ctrl+V in the search box works: it's how players enter long procedural names on a flat screen
(confirmed by the maintainer, 15 Oct 2026). VR uses the same search box, so we assume it works
there too.

### Still to verify in the real game ⚠

- A synthetic Ctrl+V (SendInput scancodes) behaves like a real one, and it works in VR.
- Whether Enter after the paste runs the search and leaves the box. `PASTE_SETTLE` is 500 ms.
  d47 waits 1.5 s and then steps into the results.
- What V or Enter do if the search box is *not* selected while the galaxy map is open (any
  galaxy-map binding on those keys fires). The health check could warn about galaxy-map binds
  on `Key_V` or `Key_Enter`.
- On foot, Status.json has no `GuiFocus` (EliteIntel), so paste is refused on foot. The journal's
  `Music: GalaxyMap` could be used instead.

## Layer 2: the plotting macro (next)

Off by default, behind its own setting, as d47 and EliteIntel do it. Every source has converged
on this sequence; each step is there because of a failure someone saw:

| # | Step | Why |
|---|---|---|
| 0 | Clipboard first, always (layer 1) | A failed macro still leaves something to paste |
| 1 | Say the system name before the map opens | Catches a misheard name 5 s earlier; tells a VR player why the map is moving |
| 2 | If the map is open, close it with **the map key** and wait for `GuiFocus != 6` | An open map's cursor focus is unknown. `UI_Back` doesn't close the galaxy map. The key is a toggle, so pressing it on a stale reading closes the map you wanted (EliteIntel) |
| 3 | `GalaxyMapOpen`, **wait for `GuiFocus == 6`** (≤ 15 s), settle ≈ 1 s | Maps take 2 s or more on slow PCs. If it never opens, send nothing: the remaining keys would fly the ship |
| 4 | `UI_Up`, `UI_Select` | Reaches the search box |
| 5 | Ctrl+V, wait ≈ 1.5 s, **raw Return** (not a UI binding) | The box keeps keyboard focus, so a UI key typed into it is a letter (d47 typed an "S") |
| 6 | `UI_Down` + `UI_Select` (first result), or `UI_Right` to `>|` + `UI_Select` | Selects the system |
| 7 | Move focus back to the map: camera nudge right then left, 30 ms each (d47), or `CamZoomIn` (EDAP, COVAS) | Otherwise Select does nothing on the selector |
| 8 | **Hold** `UI_Select` 0.75–1.2 s | A tap opens the system; a hold plots |
| 9 | Verify: `NavRoute.json` written *after* step 2 and its last `StarSystem` equals the target. Timeout 6 s, or 3 s + 1 s per 1,000 ly | Three outcomes: plotted, assume it failed, can't tell. One retry (d47). EDAP steps to the next search result when the route ends at the wrong system |
| 10 | Close with the map key, and say so if it is still open | |

Preconditions to check before offering it (the binds health check already has most of this):
all keys bound on the keyboard (Elite's default keyboard preset leaves the galaxy map
**unbound**); no `UI_Up`/`UI_Right` clash with `CamTranslateForward`/`CamTranslateRight` (the WASD
plotter-breaker); no HOTAS axis bound to UI navigation (phantom `UI_Left`, EliteIntel 2026-09).
Limits: systems only (the search can't find stations or bodies), and routes of 20,000 ly at most.

Building blocks already here: `GuiFocus` waits in `StateHub`, `NavRoute.json` parsing, the
policy ticket (kill switch cancels mid-macro), the paste action. Missing: a multi-step sequence
runner with waits on state (EliteIntel's `WAIT_UNTIL`), a NavRoute "written after" watch, and an
input trace saved per attempt (d47's input trace) to tune timings on real machines.

**Not doing:** auto-plotting the next hop on arrival with no request from the player. That's a
grey area under the EULA (REPORT §5). A spoken "next" or a button press per hop keeps a human in
the loop.
