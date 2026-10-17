# Voice commands

Everything astromech understands by voice. Say a phrase the way it is written, or close to it
(see [Voice](voice.md) for how matching works). Filler words like "please" and "the" are
ignored. Replies are in quotes.

Commands that press a key need a **keyboard key** bound in Elite for that action. Each is checked
against your ship's state first and refused with a short reason when it cannot work, such as
"Landing gear not available: docked." For the rules shared by all of them, see
[Voice](voice.md#what-it-checks-before-it-acts).

The [Voice tester](voice-tester.md) shows what any phrase would do without pressing a key.

## Ship controls

These press a key. They need you in your ship (or the SRV where noted). The reply is spoken as
soon as the key is sent.

| You say | What it does | Where it works |
|---|---|---|
| "gear down", "landing gear down", "deploy landing gear", "deploy gear", "lower landing gear", "lower gear", "drop gear" | Deploys the landing gear. "Landing gear deployed." | In your ship, not docked, not landed, not in supercruise |
| "gear up", "landing gear up", "retract landing gear", "retract gear", "raise landing gear", "raise gear" | Retracts it. "Landing gear retracted." | Same |
| "toggle landing gear", "toggle gear", "landing gear" | Switches it the other way | Same |
| "lights on", "turn on lights", "turn lights on", "headlights on", "ship lights on" | "Lights on." | Ship or SRV |
| "lights off", "turn off lights", "turn lights off", "headlights off", "ship lights off" | "Lights off." | Ship or SRV |
| "deploy hardpoints", "hardpoints out", "hardpoints away", "weapons out" | "Hardpoints deployed." | Ship, not docked, not landed, not in supercruise |
| "retract hardpoints", "hardpoints in", "stow hardpoints", "weapons in" | "Hardpoints retracted." | Same |
| "open cargo scoop", "deploy cargo scoop", "cargo scoop open", "cargo scoop out" | "Cargo scoop open." | Ship, not docked, not landed (supercruise is fine) |
| "close cargo scoop", "retract cargo scoop", "cargo scoop close", "cargo scoop in" | "Cargo scoop closed." | Same |
| "night vision on", "turn on night vision", "enable night vision" | "Night vision on." | Ship or SRV |
| "night vision off", "turn off night vision", "disable night vision" | "Night vision off." | Ship or SRV |
| "flight assist on", "enable flight assist", "turn on flight assist" | "Flight assist on." | Ship |
| "flight assist off", "disable flight assist", "turn off flight assist" | "Flight assist off." | Ship |
| "silent running on", "enable silent running", "go silent" | Asks "Silent running produces heat. Confirm?" and, on yes, "Silent running on." | Ship |
| "silent running off", "disable silent running" | "Silent running off." | Ship |
| "analysis mode", "hud analysis", "switch to analysis" | "HUD analysis mode." | Ship or SRV |
| "combat mode", "hud combat", "switch to combat" | "HUD combat mode." | Ship or SRV |
| "throttle to zero", "zero throttle", "all stop", "full stop" | Sets the throttle to zero. "Throttle zero." | Ship; refused in supercruise near a body |
| "handbrake on", "hand brake on" | "Handbrake on." | SRV |
| "handbrake off", "hand brake off" | "Handbrake off." | SRV |
| "drive assist on" | "Drive assist on." | SRV |
| "drive assist off" | "Drive assist off." | SRV |

If the thing is already in the state you asked for, nothing is pressed: "Landing gear is already
deployed." If the game has not started writing its state yet: "No game state yet. Is Elite
running?"

## Panels and maps

These open or close a screen. They need you in your ship or the SRV. After the key press, astromech
reports the screen name; "Galaxy map is already open." if it is.

| You say | What it does |
|---|---|
| "open galaxy map", "galaxy map", "show galaxy map" | "Galaxy map open." |
| "open system map", "system map", "show system map" | "System map open." |
| "close panel", "close map", "go back", "back" | "Panel closed." or "No panel is open." |
| "left panel", "open left panel", "navigation panel" | "Left panel open." |
| "right panel", "open right panel", "ship panel" | "Right panel open." |
| "comms panel", "open comms", "communications panel" | "Comms panel open." |
| "role panel", "open role panel" | "Role panel open." |

## Questions about your ship

These only read. No key is pressed, and they work during the kill switch. The answer comes from
the journal and Elite's status files, so it is as current as the game's last write. If the game
has not said yet, you hear "No fuel data yet." and the like.

| You say | You hear (examples) |
|---|---|
| "fuel", "how much fuel", "fuel status", "fuel remaining" | "Fuel: 24.5 tonnes main, 0.63 tonnes reservoir. Tank 76 percent." |
| "cargo", "how much cargo", "cargo status" | "Cargo: 12 of 64 tonnes. 8 Painite, 4 Gold." (the top three) |
| "jump range", "what's my jump range", "max jump" | "Jump range: 42.1 light years." This is the game's own figure for your last loadout. |
| "where am I", "where are we", "current system", "what system" | "System Sol. Body Earth. Docked at Abraham Lincoln." |
| "jumps left", "jumps remaining", "next system", "how many jumps" | "Next system Alpha Centauri. 5 jumps remaining." or "No route plotted." |
| "hull", "hull health", "how much hull" | "Hull: 100 percent." |
| "shields", "shield status" | "Shields are up." or "Shields are down." |
| "credits", "how many credits", "rebuy" | "Credits: 1234567. Rebuy: 500000." |
| "what ship", "what's my ship", "current ship", "ship name" | "Name (ID), Type." |
| "legal state", "wanted", "notoriety", "legal status" | "Legal state: Clean." |
| "session summary", "how much did I make this session", "how much did I earn this session", "how much have I earned", "what did I earn this session", "session earnings", "how's the session going" | "Earned 4.2 million credits, 1.9 million an hour. 38 jumps, 1204 light years. 6 bounties awarded." Zero items are left out. See [Callouts](callouts.md#session-summary). |

Powerplay questions, from the journal (examples again):

| You say | You hear (examples) |
|---|---|
| "what's my powerplay rank", "powerplay rank", "powerplay status", "how many merits", "how many merits do I have", "what are my merits", "merits" | "Pledged to Zachary Hudson, rank 3, 5000 merits. 2000 merits to rank 4: reward. 300 merits this session." or "Not pledged to a power." |
| "who controls this system", "who controls the system", "system power", "system powerplay", "powerplay here" | "Zachary Hudson fortified, 40 percent. Reinforcement 10, undermining 20." |
| "what earns merits here", "how do I earn merits here", "merits here", "powerplay activities" | "Merits here: trade, bounties, scanning, mining, and 2 more on the Powerplay page." (up to four names, then the rest) |

## Lookups

These search the galaxy and answer with the nearest or best result. They need a network
connection. [Voice lookups](lookups.md) has the details of each.

| You say | What it finds |
|---|---|
| "nearest" or "find" plus a station service ("nearest interstellar factors", "find material trader") | The nearest station with that service |
| "nearest" or "where can I buy" plus a module ("where can I buy a 5A frame shift drive") | The nearest station selling it |
| "where can I buy" or "nearest" plus a ship ("where can I buy a Krait Mk II") | The nearest shipyard selling it |
| "where to sell gold", "best price for painite", "where to buy tritium", "cheapest tritium" | The best price near you |
| "where to sell my cargo", "where should I sell my cargo" | The best station for your whole hold |
| "nearest crystalline shards", "nearest brain trees", "nearest crashed ship", "nearest geological sites", "nearest raw material farm" | The nearest raw material site |
| "nearest famine system", "nearest boom system", "nearest war system", "nearest civil unrest system", "nearest outbreak system", "nearest federation system", "nearest empire system" | The nearest or best system in that state |
| "where can I acquire near me", "where can I reinforce near me", "where can I undermine near me" | A Powerplay target for your power |
| "how is the build going", "what is left to deliver", "where can I buy steel for the depot" | Your colonisation build |

## Answering

| You say | Meaning |
|---|---|
| "yes", "confirm", "do it", "affirmative" | Yes, to "Did you mean" or a confirmation |
| "no", "cancel", "never mind", "negative", "abort" | No |

These work only right after a question. On their own, astromech says "Say again."

## Recognised, but not built yet

These are understood so you get a clear answer instead of a wrong guess. They do nothing.

| You say | You hear |
|---|---|
| "supercruise", "engage supercruise", "enter supercruise", "drop out of supercruise", "exit supercruise" | "Supercruise is not available yet. Flight commands come later." |
| "hyperspace jump", "engage hyperspace", "frameshift jump", "engage frameshift drive", "jump to next system" | "Hyperspace jumps are not available yet. Flight commands come later." |
| "request docking", "request docking permission", "dock" | "Docking requests are not available yet." |
| "plot route to {system}", "set course for {system}" | "Route plotting is not available yet." |

## What it does not do

No fire control, no boost, heat sink or chaff, no jumping, no docking, no destination setting,
and no free-form questions. The command list is fixed on purpose.
