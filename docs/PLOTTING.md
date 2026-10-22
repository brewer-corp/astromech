# Plotting and routes

Elite has no API for setting a destination. Every tool that "auto-plots" either puts the system
name on the clipboard or drives the galaxy map with simulated keys. The galaxy-map search is
the one text field a player has to fill in mid-flight, and **you can't type in VR**. That is the
problem this solves. astromech does it in two layers:

1. **Clipboard (built).** astromech follows a route. The next waypoint is always on the
   clipboard, and "paste" (spoken, or a HOTAS button) puts it into the search box. You still
   select the search box and plot the route yourself. The route can come from Spansh's neutron
   or exact plotter, plotted in the app for the current ship.
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
| Follower: the callout rule `route_follow`, on live `FSDJump`, `CarrierJump` and `Location` only, so a replayed journal (the startup catch-up, an old journal a backup tool touched) never moves the route or writes the clipboard; it only updates where the commander is. Its line is spoken through the callout queue (after push-to-talk, quiet in combat and docking) and shown as a toast; `[callouts.route_follow] enabled = false` turns following off (the route then doesn't move, but where the commander is still updates, through the engine's `while_off`) | `astromech_announce::route_follow`; the core's `Desk` is its `Follower` |
| Paste: Ctrl+V, 500 ms, Enter. **Only while `GuiFocus == 6`** (galaxy map), checked before Ctrl+V and again before Enter (if the map closed, Enter is not sent), through the policy layer | `Runner::paste` |
| Paste button: `controls.paste = { device, key }`, read with the same Raw Input listener as push-to-talk | `controls.rs`, `config.rs` |
| Voice: "next waypoint", "skip waypoint", "previous waypoint", "clear route", "paste" | `astromech-intent` MVP grammar, `Assistant::route_command` |
| Control API: `route`, `route_set`, `route_skip`, `route_back`, `route_clear`, `route_copy` | `Core::route_cmd`; the shell's `core_call` allows them |
| UI: Galaxy route rail (and the Route page); **Follow** on the engineer trip; **Go** on every search-kit result | `app/src/modules/route`, `lib/route.ts`, `lib/ui/SearchKit.svelte` |

Behaviour:

- The core owns the route, not a session. It can be set from the window while Elite is closed
  and survives restarts.
- Setting a route copies its first stop straight away. If you are already in that system, the
  route starts at the stop after it. Being in a *later* stop at that moment doesn't skip
  anything.
- Arriving in the next waypoint, or a later one (stops skipped), moves the route on, copies the
  next system and says *"Next waypoint. Deciat."* (*"Not copied."* after it if the clipboard
  write failed). The last one says *"Final waypoint reached. Route complete."* Catching up on
  the journal replayed at start is silent. "Next waypoint" by voice copies it again and adds what
  is there and how many are left: *"Next waypoint. Deciat, for Felicity Farseer. 3 waypoints
  left."* The lines follow the callout copy guide (V1-DESIGN §6.5); the system name is a sentence
  of its own, which the word budget does not count, because a procedural name alone can outrun
  a callout's budget.
- Consecutive stops in the same system are merged, which covers two engineers in one system and
  road-to-riches rows (one per body).
- The clipboard is written only on these events, never on a timer, so it doesn't fight you for
  the clipboard.

### Spansh plotters

**Galaxy route rail → Plot with Spansh** plots a neutron or exact route for the current ship and
hands it to the follower (`route_set`), so it is followed the same way as any other route.

| Piece | Where |
|---|---|
| The ship: the FSD (optimal mass and max fuel per jump with engineering, base fuel multiplier and power), base mass (hull and modules plus the reservoir, no main-tank fuel), tank, reservoir, Guardian booster and neutron boost (×6 for the size 8 Mk II drive, ×4 for any other, as on Spansh's site) from the latest `Loadout`; range with a full tank and the cargo aboard | `astromech_ship::Fsd` (`ShipStats::fsd`), `astromech_galaxy::plot::Ship::current` |
| Requests and results: neutron `/api/route` (range, efficiency, `supercharge_multiplier`), exact `/api/generic/route` (the FSD numbers as a form, see below); both are jobs polled at `/api/results/{job}` | `astromech_data::spansh` (`NeutronRoute`, `ExactRoute`, `spansh_*_wait`) |
| Plotting: one call that submits, polls until done and returns the stops with labels (`neutron boost`, `fuel`) and totals | `astromech_galaxy::plot::plot` |
| The window: `route_plot_ship` (range and ship for the page), `route_plot` (plots on a blocking worker in the shell, one at a time, then `route_set` on the core) | `app/src-tauri/src/main.rs`, `app/src/modules/route/Plot.svelte` |

The core's pipeline thread is not involved: the shell plots and only the finished route reaches
the core, through the control API. A route plotted while the window is left still gets followed.

**The exact plotter's input** (read from spansh.co.uk's exact-plotter page and checked against
the live API on 17 Oct 2026): Spansh's site doesn't send the ship build for the plotter to read.
It works out the numbers in the browser from a journal `Loadout` (its "SLEF" import) or a
Coriolis build and posts them as a form: `fuel_power`, `fuel_multiplier`, `optimal_mass`,
`max_fuel_per_jump`, `base_mass`, `tank_size`, `internal_tank_size`, `range_boost`, `cargo`,
`source`, `destination` (names work as well as id64s) and option flags. `ship_build` is
optional. `base_mass` is `UnladenMass` plus the reservoir: Spansh adds the main-tank fuel jump by
jump. The recorded job (`crates/astromech-data/tests/fixtures/spansh-exact-*.json`) confirms
the fuel maths: 38.29 ly from Sol used 4.52 t, as the formula gives for that mass. astromech
leaves FSD injections, already-supercharged drives, skipping secondary stars and a fuel reserve
at the site's defaults (off).

Ctrl+V in the search box works: it's how players enter long procedural names on a flat screen
(confirmed by the maintainer, 15 Oct 2026). VR uses the same search box, so we assume it works
there too.

### Still to verify in the real game ⚠

- A synthetic Ctrl+V (SendInput scancodes) behaves like a real one, and it works in VR.
- Whether Enter after the paste runs the search and leaves the box. `PASTE_SETTLE` is 500 ms.
  d47 waits 1.5 s and then steps into the results.
- What V or Enter do if the search box is *not* selected while the galaxy map is open (any
  galaxy-map binding on those keys fires). The health check warns about galaxy-map binds on
  `Key_V` or `Key_Enter` ("Clashes with route paste"); it does not look at the Interface (`UI_*`)
  actions, and whether the map reacts to those with the box unfocused is not known.
- The second `GuiFocus` check before Enter sees the map close only once Status.json says so;
  the game writes it up to about a second after a change (S5), longer than `PASTE_SETTLE`.
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
