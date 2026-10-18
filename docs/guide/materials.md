# Materials

**Engineering, Materials** shows your engineering materials the way the in-game material trader
does, without flying to one. The game's own inventory is a flat list with no grades; here each
row is a category and each column a grade.

**What it needs:** the journal, for what you hold. The farming searches need a network
(Spansh and EDSM). It is updated as the game writes your materials.

If the journal folder cannot be read (it is not where astromech looks, or it cannot be opened),
the page says "Could not read the materials" and gives the reason. It never shows made-up counts.

## The grids

Tabs: **Raw**, **Manufactured**, **Encoded** and **Other**.

- Each cell is a material, with how many you hold out of its cap. The fill shows how close it is
  to the cap, and an empty material is dim. Caps are 300, 250, 200, 150 and 100 for grades 1 to 5.
- The line under the tabs says which trader trades this kind, and the total held out of the
  total capacity.
- **Other** lists materials not on a trader: Guardian, Thargoid and anything else the journal
  named.
- Click a material to open its panel on the right: its grade, rarity and group, how many you
  hold, **where it comes from** (with a hint), and whether a material trader takes it. When
  it has a farming source, a **Nearby** search finds the nearest systems for it.
- **Ctrl+K** finds a material by name from any page (see [Using the app](using-the-app.md#command-palette)).

## High Grade Emissions

On the **Manufactured** tab, a line under the tabs says what the system you are in offers: its
name, allegiance and population, whether a High Grade Emissions signal is on your scanner now, and which materials
they could drop. Below the grid, a list of up to eight grade 4 and 5 manufactured materials you are not full on,
least full first, each with the High Grade Emissions source that drops it.

High Grade Emissions come from a **faction** in the system, by its state and allegiance:

| Faction | Drops |
|---|---|
| Federation (most reliably in a system of over a billion people) | Federation composites: Proprietary Composites, Core Dynamics Composites |
| Empire (most reliably over a billion people) | Imperial Shielding |
| Independent or Alliance, in Boom | Proto Heat Radiators, Proto Radiolic Alloys, Proto Light Alloys |
| Any, in War or Civil War | Military Grade Alloys, Military Supercapacitors |
| Independent or Alliance, in Civil Unrest | Improvised Components |
| Independent or Alliance, in Outbreak (1 million or more people) | Pharmaceutical Isolators |

War outranks the Federation and Empire pools. The system view is a likely pool, not a promise:
the signal is spawned by one faction, and astromech's [High Grade Emissions
callout](callouts.md#high-grade-emissions) names the pool from the faction that spawned the signal when the game says
which. Biotech Conductors and Exquisite Focus Crystals are not at High Grade Emissions at all
(missions or a manufactured trader).

The nearby search for these finds the systems where the state is, ranked by population then
distance, with how old Spansh's report is. By voice, see
[Voice lookups](lookups.md#faction-states-famine-boom-war-and-more).

## Raw material sites

On the **Raw** tab, below the grid, pick a kind of site and the nearest bodies are listed:

| Site | What it gives |
|---|---|
| Crystalline shards | Grade 4 raw (polonium, ruthenium, tellurium, technetium, yttrium, antimony and more). Shoot the shards from the SRV and collect the fragments. |
| Brain trees | Grade 4 raw. Shoot them (turrets or flak), then collect with the SRV. |
| Crashed ships | Encoded data points and salvage. Scan the data points from the SRV. |
| Geological sites | Grade 1 to 3 raw from the outcrops around geysers, fumaroles and vents. |
| Known raw farms | The well-known grade 4 farms: one body for each element. |

Each row is a body with jumps, distance, how far it is from the arrival star and how old the data
is. The search starts from your current system, or one you type. You can limit the age of the data and the
arrival distance. See [Galaxy](galaxy.md#how-searches-work). By voice:
"nearest crystalline shards" and the like ([lookups](lookups.md#raw-material-sites)).

Sources: Spansh's bodies search at run time; the known farms are a hand-checked list from player
reports (forums, Steam, the wiki). The named bodies are starting points, not guarantees.

## Farming notes

Encoded materials: the Jameson Crash Site data points (scan, relog, scan) give Modified Consumer
Firmware, Cracked Industrial Firmware and Adaptive Encryptors Capture, which trades into any other
encoded category. Wake scanning gives the Wake Scans column; famine systems help.
Raw: shards and trees drop grade 4 only. Lower grades are cheapest as a trade down from grade 4.
These are community reports, not Frontier's.

## Limits

- Counts are as of the journal. Materials gained since the game last wrote are not shown.
- Where-to-find notes are community knowledge and can go stale when the game changes.
- Faction states come from Spansh, which can be days old; a system you visited recently
  uses your own visit instead.
