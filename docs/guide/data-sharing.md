# Data sharing

astromech can share what you see in game in two places, both **off until you turn them on**, in
**Settings, Data sharing**:

- **EDDN**, the Elite Dangerous Data Network. Other players' tools (Inara, Spansh, EDSM, and
  astromech's own lookups) get their market, station and system data from it. You contribute
  what you see.
- **EDSM**, the Elite Dangerous Star Map, to keep **your own** flight log, map and profile up to
  date on your EDSM account, as EDMC does.

Nothing else uploads anything. The pages that look things up send a search (a system, a
commodity) and get an answer; they do not send your journal. See [Using the
app](using-the-app.md#what-needs-the-network).

## What both have in common

- **Live play only.** Nothing is sent from the game's beta. Nothing is sent from a journal being
  re-read when astromech starts, and astromech starting mid-session does not send the session so
  far. For EDDN, an event more than **5 minutes** old by its timestamp is
  treated as a replay and not sent (EDSM has no timestamp check: it sends what arrives live). A journal
  file Elite starts while astromech runs is live from its first line.
- **Elite must be running** and astromech watching its journal.
- **You can turn them off at any time** with the same button (**Turn off**). Off, nothing new is queued
  and nothing is sent, and anything still waiting in the queue is discarded at once.
- **A queue with retries.** Messages go to a queue on your PC and are sent in the background; a failed
  one is retried, backing off from a minute up to an hour. A message is dropped after 8 tries or after
  30 minutes in the queue (at most 2,000 wait at once). The 30 minutes is for
  both services and is separate from the 5-minute check above, which happens before a message is queued. The page shows how many are **waiting**, the
  **last sent** and the **last error**. For EDDN, **Diagnostics** also shows a log of what the gateway
  answered and of what was **not sent**, each with the reason (for example "beta game version", "commander not known yet"). EDSM has no such log; its own
  page shows Waiting, Last sent and Last error.

## EDDN: exactly what is sent

astromech writes each message from the journal and the game's files to the schemas EDDN publishes
(the sender is written from EDDN's documentation). It goes to `eddn.edcd.io` (HTTPS, port 4430).

| What | EDDN schema | When it is sent |
|---|---|---|
| A station's **commodity market**: prices, demand and stock | commodity/3 | When you open the commodity market (`Market.json`) |
| A station's **shipyard**: the ships for sale | shipyard/2 | When you open the shipyard (`Shipyard.json`) |
| A **fleet carrier's materials** market | fcmaterials_journal/1 | When you open it (`FCMaterials.json`) |
| Where you **docked**, jumped to or are, and the bodies you **scan** | journal/1 | On `Docked`, `FSDJump`, `Location`, `CarrierJump`, `Scan` and `SAASignalsFound` |
| The result of an FSS **honk** and the bodies' signals | fssdiscoveryscan/1, fssallbodiesfound/1, fssbodysignals/1 | On those events |
| The **signals** found in a system, one message per run | fsssignaldiscovered/1 | After a run of `FSSSignalDiscovered` events. Mission-target USS signals are never sent. |
| A **nav beacon** scan and a **barycentre** scan | navbeaconscan/1, scanbarycentre/1 | On those events |
| A **codex** entry | codexentry/1 | On `CodexEntry`, with the body from Status.json if it matches |
| Approaching a **settlement** | approachsettlement/1 | On `ApproachSettlement` |
| **Docking** granted or denied | dockinggranted/1, dockingdenied/1 | On those events |
| A **route** you plot | navroute/1 | When the game writes a non-empty `NavRoute.json` |

Not sent, although the schema exists: **outfitting**. It is switched off in astromech until the modules you
can buy only through Powerplay or a tech broker can be filtered out.

### What is taken out first

Before anything is queued, astromech removes:

- everything the game labels `*_Localised`,
- your **commander's data**: `Commander`, `FID`, wanted state, active fines, cockpit breaches, fuel
  level and fuel used, jump distance, whether it is a new codex entry or new traits discovered,
  voucher amounts and landing coordinates where the schema does not allow them, and the faction
  keys that are about you (`MyReputation`, squadron faction, home and happiest systems),
- the fields each schema's README says to drop.

It also **checks** each message against the last location: a message that names a system you were
not in is dropped, and one missing a field the schema requires is skipped and logged.

Your **credits, cargo, missions, ships and rank** are not part of any of these schemas.

### Who sent it

The message header carries your **commander name**, which EDDN asks for and obfuscates before passing
messages on, the software name (`astromech`) and its version, and the game version and build.
The gateway also sees your internet address, as any website would. Nothing else identifies you.

### Test mode

`[eddn] test = true` in `astromech.toml` sends to EDDN's `/test` schemas, which nobody consumes.
Debug builds always do. The Data sharing page says "On (test schemas)".

## EDSM: exactly what is sent

With **Upload to EDSM** on, astromech sends your **journal events** to EDSM's Journal API, to **your own
account**.

You need:

- your **commander name**, as registered on EDSM,
- your **API key** (on EDSM, Settings, My API Key). It is saved in **Windows Credential Manager**, never in
  astromech's settings file, and is never shown again; you can only replace or remove it.

What is sent:

- **Each event as the game wrote it**, without changes, plus the context EDSM asks for: where you
  are (system, coordinates), your station and market and your ship ID. This is the same as EDMC. It
  can include things like your credits, ships and missions, because that is what EDSM keeps for your
  profile.
- **Not** the events on EDSM's own **discard list** (chat, music, the game's internal state and
  the like). astromech fetches the list before it first posts and again daily, and never sends a listed event.
- Events go in batches of up to 50, oldest first.
- Nothing is sent while you are a **crew member** in another commander's ship (EDSM keeps no log there).
- Live play only, with the same rules as above.

The page has **Waiting to send**, **Last sent**, **Last error** and a **Flight log** row with a **Show** button (disabled until the commander name and API key are saved) that shows the last week
of systems from EDSM, with "First to report" where it applies.

## What else uses these services

EDSM is also used, without an account, to fill in the star map around you (systems near you), see
[Galaxy](galaxy.md#jumps-and-distance). That sends a search for a sphere of space, not your journal.

## Turning it off

**Settings, Data sharing:** **Turn off** next to each. Or set `enabled = false` under `[eddn]` and `[edsm]`
in `astromech.toml`.
