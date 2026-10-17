# Trade

**Trade, Commodities** answers "where do I sell this?" and "where do I buy that?". It has four
modes: **Sell**, **Buy**, **My cargo** and **Route**. Prices come from **Ardent Insight**
(api.ardent-insight.com), which is fed by what players share with EDDN.

**What it needs:** a network connection, and your position (journal) or a system you type. **My
cargo** and **Route** use your hold and credits from the journal. Each mode remembers its own
filters, and voice questions use the same ones (see [Voice lookups](lookups.md#trade)).

## Sell and Buy

1. Choose **Sell** or **Buy**.
2. Type a **commodity** (part of its name, or a short name such as "LTDs") and pick it. The
   page opens on painite.
3. Read the list: best price first for Sell (stations buying), cheapest first for Buy (stations
   selling).

Each row has the station and system, jumps and distance, the price, the **demand** (Sell) or
**stock** (Buy), the pad, the distance from the arrival star and when it was reported.

- **Near** searches around you or a system you type, within a distance (100 light years by
  default) and data age (30 days by default).
- **Anywhere** takes Ardent's best 100 prices in the whole galaxy that pass the filters, with
  distances from you. Use it when nothing close pays well.
- Filters: minimum demand or stock, pad, data age, arrival distance, no fleet carriers, no Odyssey
  settlements, and (near) the search distance.

## My cargo

Where to sell **everything in your hold**. It reads your hold, and lists the stations that pay the
most for the whole load, each commodity capped by what the station will take. Under it, **Best for
each on its own** shows the best station for each commodity separately. When no market takes all
of one commodity, it shows the market that takes the most.

Mission cargo, stolen goods and limpets are left out ("Left out: ..."). Ask the same by voice:
"where should I sell my cargo".

## Route

The most profitable trade from a station within one hop, or a loop that brings the best cargo back,
worked out by Spansh's trade router.

1. Choose **Single hop** or **Loop**.
2. Set **From** (or leave your current system), the **Station** to start from (empty: the station
   you are docked at, or else the one nearest the star), **Cargo** (tons), **Credits** and
   **Max hop** (light years). They start from your ship and the journal, and you can change them.
3. Filters: pad, data age, arrival distance, fleet carriers.
4. **Find route.** Spansh works on it as a job and can take up to a minute.

The result lists each leg: from and to station, what to carry, buy and sell prices, profit per ton and
in all, jumps and distance. At the end: the profit per round trip and an **estimated** profit per hour.
Spansh does not give profit per hour, so astromech estimates it from 50 seconds a jump, 2 minutes to
dock and trade, and supercruise time by distance from the star; the estimate is labelled.

"No profitable route within these limits" suggests a longer hop, more credits or older market data.

This is a trade search, not a route plotter: see [Routes](routes.md).

## Colonisation

The **Colonisation** page is also under Trade: [Colonisation](colonisation.md).

## Limits

- Prices are what players last reported, so they can be old. Each row shows its age.
- The estimate of profit per hour is rough.
- Ardent Insight's coverage and speed are not astromech's; if it is down the page says so.
- Hold, pad and credits come from the journal's last write.
