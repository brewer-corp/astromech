# Trade

**Trade, Commodities** finds a profitable trade route from a station. Where to sell or buy one
commodity, and where to sell your whole hold, are finders in [Places](places.md): the line at the
top of the page links to them.

**What it needs:** a network connection, and your position (journal) or a system you type. It uses
your hold and credits from the journal.

## Sell, Buy and Sell my cargo

The links at the top open the Places finders: **Sell** and **Buy** on Painite
([Sell and Buy](places.md#sell-and-buy)) and **Sell my cargo** ([Sell my
cargo](places.md#sell-my-cargo)). Each keeps the filters it had on this page, and voice
questions use the same ones ([Voice lookups](lookups.md#trade)).

## Route

The most profitable trade from a station within one hop, or a loop that brings the best cargo back,
worked out by Spansh's trade router.

1. Choose **Single hop** or **Loop**.
2. Set **From** (or leave your current system), the **Station** to start from (empty: the station
   you are docked at, or else the one nearest the star), **Cargo** (tons), **Credits** and
   **Max hop** (light years). They start from your ship and the journal, and you can change them.
   If the journal can't be read, the page says so and the fields start empty, with Max hop at 20 ly.
3. Filters: pad, data age, arrival distance, fleet carriers.
4. **Find route.** Spansh works on it as a job and can take up to a minute.

The result lists each leg: from and to station, what to carry, buy and sell prices, profit per ton and
in all, jumps and distance. At the end: the profit per round trip and an **estimated** profit per hour.
Spansh does not give profit per hour. The line is labelled Estimated; hover it for the time model
(50 seconds a jump, 2 minutes to dock and trade, supercruise by distance from the star).

"No profitable route within these limits" suggests a longer hop, more credits or older market data.

This is a trade search, not a route plotter: see [Routes](routes.md).

## Colonisation

The **Colonisation** page is also under Trade: [Colonisation](colonisation.md).

## Limits

- Market data is what players last reported, so it can be old.
- The estimate of profit per hour is rough.
- Hold, pad and credits come from the journal's last write.
