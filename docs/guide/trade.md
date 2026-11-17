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
3. Filters: pad, data age, arrival distance, fleet carriers. They show as one line; **Change**
   opens them. An edit takes effect on the next **Find route**, open or closed.
4. **Find route.** Spansh works on it as a job and can take up to a minute.

The result leads with the **estimated** profit per hour, in orange. Spansh does not give profit per
hour: hover the figure for the time model (50 seconds a jump, 2 minutes to dock and trade,
supercruise by distance from the star). Under it are the profit of one hop or one round, the
minutes it takes, and where the market data came from and how old it is. A single hop's hour
counts the flight back empty, so it compares with a loop.

Below that the route reads top to bottom as the trip: each stop (station, system with a copy
icon, largest pad and distance from the star), and between two stops the leg: what to buy at the
stop above and its buy and sell prices, the leg's profit and profit per ton, and the jumps and
light years. A loop's way back with nothing worth carrying says **Fly back empty**, and a loop ends
at the station it started from. A pad too small for your ship is struck through in red, and a
warning line above the stops names the station.

"No profitable route within these limits" suggests a longer hop, more credits or older market data.

This is a trade search, not a route plotter: see [Routes](routes.md).

## Colonisation

The **Colonisation** page is also under Trade: [Colonisation](colonisation.md).

## Limits

- Market data is what players last reported, so it can be old.
- The estimate of profit per hour is rough.
- Hold, pad and credits come from the journal's last write.
