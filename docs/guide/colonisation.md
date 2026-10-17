# Colonisation

**Trade, Colonisation** tracks your construction projects: what each still needs, in loads for your
ship, and where to buy it. Below that it lists systems open to colonise near you.

**What it needs:** the journal, which records what a construction depot needs and what you deliver
(and `Cargo.json` for what is aboard). The buy search needs a network connection (Ardent Insight).
Candidates use Spansh. astromech reads the journals again as they change, so deliveries show within
seconds.

## Projects

Docking at a construction site makes it show here, with its name, system and how built it is, or
"complete" or "failed". With several projects, pick one from the buttons at the top.

**Still to buy** lists each commodity you still have to buy, in tonnes, with what you already carry. The total is
given in tonnes and in loads at your ship's cargo capacity ("3 loads at 256 t"). If everything still
needed is aboard, it says to deliver it. A project with nothing left says so.

### Where to buy

Under the list, a search finds stations to buy it from: the ones stocking the most of the list first,
nearest first among equals. It searches from the construction site's system by default (clear the
"From" to search near you), with filters for distance, stock, pad, data age, arrival distance, fleet
carriers and Odyssey settlements. Default is 100 light years and 30 days. The filters are remembered
as the colonisation lookup, and voice uses them. Source: Ardent Insight.

## Colonisation candidates

Unclaimed systems within 15 light years of a populated one, where a claim can be made (at that
system's Colonisation Contact). Choose what you care about: **Any**, **Rings**, **Earth-like
world** or **Landable** bodies. Each row has the system, jumps and distance, its bodies and the
populated system in range. The search radius is 30 light years by default. Source: Spansh, so a
claim made since may not show yet.

## By voice and as a callout

- "How is the build going", "what is left to deliver" and "where can I buy steel for the depot"
  are [voice commands](voice-commands.md). With several projects it picks the one you are docked at,
  else the last you delivered to, else the one the game updated last. "No construction project in
  progress." when there is none.
- When you open a market, the `project_market` [callout](callouts.md#colonisation-shopping) says how many of the
  commodities your projects need are in stock: "Sells 5 project commodities."

## Limits

- Only what is in your journals. A project you have never docked at is not known.
- The depot events are documented by the community rather than by Frontier, and may change.
- Stock and prices are as Ardent Insight last heard them.
- The page tracks construction, not claims or beacons.
