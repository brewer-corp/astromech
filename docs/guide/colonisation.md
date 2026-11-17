# Colonisation

**Trade, Colonisation** tracks your construction projects: what each still needs, in loads for your
ship, and where to buy it. Below that it lists systems open to colonise near you.

**What it needs:** the journal, which records what a construction depot needs and what you deliver
(and `Cargo.json` for what is aboard). The buy search needs a network connection (Ardent Insight).
Candidates use Spansh. astromech reads the journals again as they change, so deliveries show within
seconds.

## Projects

Docking at a construction site makes it show here: its name, its system, and how far it is built
("42 % built") with a bar under it, or "Complete" or "Failed". With several projects, pick one in
the list at the top; each option shows its progress.

Each commodity the site requires is one row, those with the most still to buy first and the
complete ones last, dimmed. A row has the commodity's name over what has been delivered against
what is required ("3,288 of 8,200 t"), a bar of the same, and on the right the tonnes still to buy,
in orange while there are any. What you carry for it is taken off what is still to buy: it shows
as an orange part of the bar after the delivered part, and in words ("90 t aboard").

Under the rows, the tonnes still to buy and the loads they make at your ship's cargo capacity
("13 loads at 784 t"), or that the capacity is not known. If everything still needed is aboard,
it says to deliver it. A project with nothing left says so.

### Where to buy

Under the rows, a search finds stations to buy it from: the ones stocking the most of the list first,
nearest first among equals (the **Where to buy** heading's Tip says so). Each row starts with how much of the list the station sells ("3 of
4"), then the station and what it has in stock; select a row to see all its facts in the side panel,
with **Set destination** and **Add stop**. It searches from the construction site's system by default (under **Change**, clear
**From** to search near you), with filters for distance, stock, pad, data age, arrival distance, fleet
carriers and Odyssey settlements. Default is 100 light years and 30 days. The filters are remembered
as the colonisation lookup, and voice uses them. Source: Ardent Insight.

## Colonisation candidates

Unclaimed systems within 15 light years of a populated one, where a claim can be made (at that
system's Colonisation Contact). Choose what you care about: **Any**, **Rings**, **Earth-like
world** or **Landable** bodies. Each row has the jumps, the system, its bodies and the populated
system in range, the distance and how old the report is. The search radius is 30 light years by default. Source: Spansh, so a
claim made since may not show yet.

## By voice and as a callout

- "How is the build going", "what is left to deliver" and "where can I buy steel for the depot"
  are [voice commands](voice-commands.md). With several projects it picks the one you are docked at,
  else the last you delivered to, else the one the game updated last. "No construction project in
  progress." when there is none. Where to buy searches from where you are, or from a system you
  name at the end ("... for the depot near Sol"; see
  [Voice lookups](lookups.md#from-another-system)).
- When you open a market, the `project_market` [callout](callouts.md#colonisation-shopping) says how many of the
  commodities your projects need are in stock: "Sells 5 project commodities."

## Limits

- Only what is in your journals. A project you have never docked at is not known.
- The depot events are documented by the community rather than by Frontier, and may change.
- Stock and prices are as Ardent Insight last heard them.
- The page tracks construction, not claims or beacons.
