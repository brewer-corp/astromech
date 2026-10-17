# Routes

The **route follower** and the **Spansh route plotters** are being added to astromech and are not
in this version of the guide yet. This page will cover them when they land: what they do, how to
use them, what they need and their limits.

What exists today:

- The game's own route (`NavRoute.json`) is read for [callouts](callouts.md#fuel-and-route) when you plot one
  in the galaxy map: the jump count and distance, white dwarfs and neutron stars on it, fuel gaps and
  the next scoopable star. "Jumps left" and "next system" are [voice commands](voice-commands.md#questions-about-your-ship).
- [Trade, Route](trade.md#route) finds a profitable trade route with Spansh. It does not plot a flight
  route.
- Every list of "nearest" places counts jumps along the known star map at your ship's range:
  [Galaxy](galaxy.md#jumps-and-distance). That is for choosing where to go, not a route to follow.
- Plotting a route by voice ("plot route to ...") is recognised and answered "Route plotting is
  not available yet."
