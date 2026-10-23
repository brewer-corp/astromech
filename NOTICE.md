# NOTICE

astromech's source code is MIT-licensed (see `LICENSE`), Copyright (c) 2026 Sumeet Padavala.

## Binary caveat: GPL-3.0 via sherpa-onnx / espeak-ng

The local voice worker (`astromech-voice-worker`, cargo feature `sherpa` of `astromech-voice`,
**off by default**) links the official prebuilt **sherpa-onnx 1.x** libraries. Those libraries
include **espeak-ng and ucd-tools, which are GPL-3.0** (confirmed in spike S8,
docs/ARCHITECTURE.md §17). Consequences:

- The **source code** in this repository stays MIT.
- A **binary built with the `sherpa` feature** is distributed under the **GPL-3.0** as a whole,
  together with the corresponding source (this repository plus the matching sherpa-onnx
  release source). Release builds must ship the GPL-3.0 text and a source offer.
- Builds without the feature (the default, and everything CI builds) contain no GPL code.
- This caveat is expected to go away with sherpa-onnx 2.0 or a lexicon-only TTS build.

## Not in this repository

- No Frontier assets (art, audio, fonts), copies of game files such as Frontier's default `.binds`,
  or datamined data. Player-visible game facts (material names and grades, blueprint recipes,
  module stats; the kind of thing on the Elite Dangerous wiki) are included in `data/` and
  credited to their sources below. Elite Dangerous is a trademark of Frontier Developments plc; astromech is an unofficial fan project
  and is not affiliated with or endorsed by Frontier.
- No speech models. They are downloaded at first run, each under its own licence:
  Parakeet TDT 0.6B (NVIDIA, CC-BY-4.0), Kokoro-82M (Apache-2.0; its `espeak-ng-data` is GPL-3.0).

Third-party Rust and npm dependency licences: see `THIRD_PARTY.md`.

## Cartoon ship icons and engineer access mark
The ship silhouettes on **Engineering, Ships** are CMDR Qohen Leth's cartoon ship vectors, from
[EDAssets](https://edassets.org/#/shipschematics/cartoonshipvectors)
(`app/public/ships/`). Creative Commons Attribution-NonCommercial-ShareAlike 4.0 International.
Rasterised to 200 px PNGs for the window.
The engineer access badge (`app/public/engineers/access.svg`) is CMDR Robert Phillips' engineer
icon from [EDAssets](https://edassets.org/#/engineers), same CC BY-NC-SA 4.0 licence. Recoloured
to follow the window's text colour. Credit EDAssets and the designers.
These files keep that licence; they are not MIT.

## Fonts (app UI)
Bundled from the `@fontsource` packages, all under the SIL Open Font License 1.1:
Michroma (Vernon Adams), Saira Semi Condensed (Omnibus-Type), Share Tech Mono (Carrois Apostrophe).
No fonts or other assets from Elite Dangerous or Frontier Developments are used.

## VKB reference sheets and device data
`data/devices/vkb/VKB-Gladiator-SCG-RH-v1.0.pdf` and `VKB-Gladiator-SCG-OTA-LH-v1.0.pdf` (and
their PNG renders in `app/public/devices/vkb/`) are VKB Gaming's reference sheets, design by
UntoldForce, from https://www.vkbgaming.com/fillable-templates/. Included unchanged, with
credit. Stock button numbers are from EDRefCard (MIT, Copyright (c) 2017 Richard Buckle;
github.com/brammmers/edrefcard2).
The action catalogue (`data/catalog/controls.json`: binding names, groups, labels) is generated
from EDRefCard's `controlsData.py` (same MIT licence) by `scripts/gen-catalog.py`.
The PNGs are recoloured by `scripts/recolour-sheets.py` to match the app; the PDFs are unchanged.

## Star map and live lookups
The star map preload (`starmap.db`, built by `scripts/gen-starmap.py`, not in the repository) and
`data/catalog/engineers.json` use Spansh's systems dump (downloads.spansh.co.uk), which is
community data from EDDN. The neutron star file (`neutron.bin`, built by `scripts/gen-neutron.py`,
published on the `neutron` pre-release, not in the repository) uses Spansh's neutron dump from the
same place; `crates/astromech-galaxy/tests/fixtures/neutron-sample.json` is 39 rows of it. At run time the app asks Spansh (faction states and stations near you) and EDSM
(systems near you, edsm.net) directly. Test fixtures under `crates/*/tests/fixtures` are small,
trimmed Spansh, Ardent Insight and EDSM answers. Star positions, stations and faction states are
Elite Dangerous game data, © Frontier Developments plc.
Raw-material sites (crystalline shards, brain trees, crashed ships, geological sites) come from
Spansh's bodies search at run time. `data/catalog/raw-farms.json` lists the well-known grade 4 raw
farms as reported by players on the Frontier forums, Steam and the Elite Dangerous wiki, each
checked against Spansh's body data (16 Oct 2026); the system positions are from Spansh.

## Market data
`data/catalog/commodities.json` (market commodity names, categories and journal symbols) was
written by hand from the in-game commodity list as shown on the Elite Dangerous wiki; the symbols
were checked against Ardent Insight's commodity list. At run time the app asks Ardent Insight
(api.ardent-insight.com, open source, fed by EDDN) for prices near you. Commodity names and
prices are Elite Dangerous game data, © Frontier Developments plc.

## EDDN schemas
`crates/astromech-eddn/tests/fixtures/schemas/` holds the 16 EDDN schema files the sender uses,
copied unchanged from EDCD/EDDN (github.com/EDCD/EDDN, `schemas/` on the `live` branch, commit
4ad669bb), BSD-3-Clause, Copyright (c) 2018, EDDN; the licence text is beside them in `LICENSE`.
Tests check the sender's messages against them; they are not built into astromech. The sender
itself is written from EDDN's documentation, not from EDMC's code.

## Starport pad layout
Where each of the 45 pads sits in a Coriolis, Orbis or Ocellus bay
(`crates/astromech-announce/src/docking.rs`) is player-visible game layout, © Frontier
Developments plc. The table was written by hand from how the LandingPad EDMC plugin
(github.com/bgol/LandingPad, Bernd Gollesch) places the pads; none of its code (GPL-2.0) is used.

## Engineering data
`data/catalog/engineering.json` (blueprint and experimental recipes, which modules take them,
engineers per grade, module and ship names) is generated by `scripts/gen-engineering.py` from
EDCD/coriolis-data (github.com/EDCD/coriolis-data) and EDCD/FDevIDs (github.com/EDCD/FDevIDs).
The facts are Elite Dangerous game data, © Frontier Developments plc, as shown to every player
in game; coriolis-data's notice says the same. It also carries what each blueprint grade and
experimental does to a module's stats, from the same coriolis-data files. Which engineers offer a
blueprint grade is kept per module type; two coriolis-data entries are corrected from the Elite
Dangerous wiki (`scripts/gen-engineering.py`, checked with `scripts/check-engineer-grades.py`).
`data/catalog/outfitting.json` (every module stations sell, with its classes, ratings and
mounts, and every ship) is generated by `scripts/gen-outfitting.py` from EDCD/FDevIDs, keeping
the module names Spansh lists; the same game-data notice applies.
`data/catalog/eddn-outfitting.json` (which module symbols EDDN's outfitting/2 may carry: those
without a commander-specific sku) is generated by `scripts/gen-eddn-outfitting.py` from EDCD/FDevIDs
`outfitting.csv`; the same notice applies.

## Engineer unlocks
`data/catalog/engineer-unlocks.json` (how to find, get invited by and unlock every ship and suit
engineer, and who introduces whom) is adapted by `scripts/gen-engineer-unlocks.py` from the
"Engineer Summary" tables of the Elite Dangerous wiki's Engineers page
(https://elite-dangerous.fandom.com/wiki/Engineers), whose text is under the Creative Commons
Attribution-ShareAlike 3.0 licence (https://creativecommons.org/licenses/by-sa/3.0/). The file
copies that text and stays under CC BY-SA 3.0, not the MIT licence; credit: the Elite Dangerous
wiki contributors. The requirements themselves are game facts, © Frontier Developments plc.

## Odyssey data
`data/catalog/odyssey.json` (on-foot micro-resources with bartender barter values and sale prices,
suit and weapon grade upgrade costs, suit and weapon modification costs, and the on-foot
engineers: where they are, what they fit, their invite and referral tasks) is generated by
`scripts/gen-odyssey.py` from the Elite Dangerous Wiki (elite-dangerous.fandom.com, fetched
2026-10-16 through its MediaWiki API) and EDCD/FDevIDs (`microresources.csv`, `engineers.csv`).
The engineers' invite and referral tasks are short sentences copied from the wiki's Engineers
page (elite-dangerous.fandom.com/wiki/Engineers, by its contributors, CC BY-SA per
fandom.com/licensing); everything else is facts (names, numbers) without wiki text. The facts
are Elite Dangerous game data, © Frontier Developments plc, as shown to every player in game.
Values the wiki leaves blank or contradicts itself on are listed under `uncertain` in the file.
The journal names of the suits, weapons and modifications in it (`journal` fields, used to read the
owned gear from the journal) are from EDCD/EDDI's data definitions and the community journal
schemas jixxed/ed-journal-schemas; only the identifiers are taken (no code or text), which the game writes to the player's
own journal.
ED Odyssey Materials Helper's game data is in a closed library and is not used.

## Exobiology data
`data/catalog/exobiology.json` (every exobiology genus with its colony range and journal symbol,
every species with its Vista Genomics base value, journal symbol and the conditions it was seen
growing in) is generated by `scripts/gen-exobiology.py`:
- values and colony ranges are the numbers from the Elite Dangerous wiki's "Exobiology Sample
  Values and Details" page (elite-dangerous.fandom.com, read 17 Oct 2026 through its MediaWiki
  API); no wiki text is copied. A value the wiki has wrong is fixed in the script with the
  reason (Fonticulua Fluctus: 20,000,000, not the wiki's 16,777,215); every fix and every
  disagreement with another source is listed under `uncertain`;
- the growing conditions (atmosphere type, body class, nearest star, gravity, temperature,
  pressure, volcanism) are measured by the script from Spansh's bodies search
  (https://spansh.co.uk, EDDN data: bodies other players scanned, with the species they sampled
  there): for each of the 90 Odyssey species, the 100 bodies with it nearest each of four points
  across the galaxy, fetched 17 Oct 2026 (24,756 bodies), and the stars of their systems, from
  which the nearest star is read by the body's name. No other tool's condition tables are used.
  A quarter of the bodies is held out of the measurement, and a condition needs at least two
  sightings; `crates/astromech-exobiology/tests/fixtures/spansh-held-out.json` holds 400 of the
  held-out bodies, trimmed, for the recall test. Credit: Spansh and the EDDN contributors;
- the journal genus symbols (`$Codex_Ent_Stratum_Genus_Name;`) are as Elite Observatory lists
  them (github.com/Xjph/ObservatoryCore, MIT);
- the journal species symbols are from EDR, facts only (see "Exobiology values" below); EDR
  has none for the Horizons-era species (anemones, brain trees, sinuous tubers, bark mounds,
  amphora plants, crystalline shards), whose symbols (`$Codex_Ent_SphereABCD_01_Name;`) are
  from Canonn's codex reference (https://canonn.science, its public API `query/codex/ref`, read
  17 Oct 2026), facts only: each species' symbol and English name. Credit: the Canonn
  Interstellar Research Group.
EDMC-BioScan (GPL-2.0) and SrvSurvey (GPL-3.0) do the same job; none of their code or data is
used. The facts are Elite Dangerous game data, © Frontier Developments plc.

## Exobiology values (EDR, ED Assistant)
`data/catalog/exobiology.json` is one file for every exobiology use (the species values in the
unsold organic total, the prediction, the colony ranges); its main sources are under
"Exobiology data" above. Two more sources went into it:

- Each species' journal symbol (`$codex_ent_stratum_07_name;`) was read from EDR's
  `edr/data/biology.json` (https://github.com/lekeno/edr, Apache-2.0, at commit 198ebf5). Only
  facts were taken: symbol, name and credit value, written by `scripts/gen-exobiology.py` into
  astromech's own file layout. No EDR code or file is copied, so the file is not a copy of EDR's
  and stays under astromech's MIT licence. Where EDR's values differ from the wiki's, the file
  keeps the wiki's and lists EDR's under `uncertain`; the one species EDR has and the wiki's page
  doesn't (Radicoida Unicus) has EDR's value and colony range, marked uncertain.
- The single value per Horizons-era genus (anemones, brain trees, sinuous tubers, bark mounds,
  amphora plants, crystalline shards) that ED Assistant's seed data gives
  (https://github.com/IgorBabayan/ed-assistant) was used before this file had per-species values;
  it is now only compared with the wiki's (the differences are under `uncertain`). ED Assistant
  is under the MIT licence:

```
Copyright (c) 2026 Igor Babayan

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```
