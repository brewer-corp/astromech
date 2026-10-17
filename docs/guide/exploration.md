# Exploration

Two pages under **Exploration**: **System**, the bodies of the system you are in as you scan them,
and **Biology**, the system's life ranked by value, with a sampling helper.

**What they need:** Elite running and the journal. The page follows the game and refreshes every
few seconds. The unsold-data totals need the journal history (see
[Logbook](logbook.md)); the sell search needs the network. Searches use Spansh, and the "other players reported" biology comes from
Spansh too.

## System

The system you are in, with:

- **Bodies scanned** out of the body count, and "all found" when the FSS says so.
- **Value now** (everything scanned, as it would sell) and **Mapping the best** (if you also mapped the
  worthwhile ones with the Detailed Surface Scanner).
- A table of bodies: name, class, distance from the star, gravity, facts (terraformable, landable,
  rings, signals), what it is worth now and what it is worth **mapped**. **Worth mapping** marks a
  planet you have not mapped yet that is worth at least 500,000 credits mapped efficiently. **Mapped** (and "efficiently") marks ones you did.
  **First footfall likely** marks a landable body nobody had discovered in an unpopulated system;
  the journal cannot say who walked there first. **Landed** counts your landings.

If nothing is scanned: "Honk and use the FSS: bodies show here as you scan them."

Values are estimates from the community's exploration value formulae, the same ones EDSM and OD
Explorer use, tested against EDSM's published estimates. "Mapped" assumes the Detailed Surface Scanner
within the efficiency target. They include the first-discovery bonus when the scan says nobody had
discovered the body. They do not estimate selling through a fleet carrier, which pays 25% less.

### Unsold data

What you would be paid if you sold now, since your last death:

- **Cartographic**: every star and planet scanned, at its value now (mapped if you mapped it),
  with the count of bodies and systems.
- **Organic**: every completed biology sample (all three taken), with the count of samples,
  and how many are of unknown value.
- **Total.** At 50 million credits or more, a warning says dying loses all of it and to sell it
  at the nearest station.

Selling data removes it from the totals. The first-logged bonus on organic samples is not estimated.

### Where to sell

Pick **Cartographic data: Universal Cartographics** or **Organic data: Vista Genomics** and the
nearest stations are listed on the usual search ([Galaxy](galaxy.md#how-searches-work)).

## Biology

The system's biology, ranked by value, live while Elite runs.

- **Bodies with biology**: for each body with biological signals, the **genera** present with the
  species they could be and what they are worth (a range, or exact once known). From the DSS the genera
  are known; before the DSS they are **predicted from the scan** (marked as guesses), so the best
  bodies show up before you land. "Scan the body to predict its genera." until you have.
- **Sampling.** After the first sample: the species, **Samples** (of 3), and how far you are from the
  nearest sample against the genus's **colony range**. It turns to "Far enough" when you can take the
  next one, or says how many metres to go. After three: "Analysed."
- **Reported by other players**: bodies Spansh knows have biology, with genera, species and
  values, for when you arrive before scanning. It is from Spansh, sent to EDDN by other players.

Values are Vista Genomics base values, before the first-logged bonus.

Spoken help: on a sample, **"Far enough for sample 2."** (the `sampling` callout), a summary after the
FSS and the DSS (the `biology` callout), and an Earth-like or terraformable find from the `discovery`
callout. See [Callouts](callouts.md).

## Limits

- The estimate is only as good as the scan. Bodies you have not scanned are unknown (unless
  another player reported them).
- A prediction can be wrong until the DSS gives the real genera.
- Unsold data is worked out from the journal, so it is only right if your journal files are whole.
