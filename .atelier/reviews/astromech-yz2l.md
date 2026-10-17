---
schema: atelier.review
schema_version: 1
id: astromech-yz2l
title: 'A route search can''t be cancelled. spansh_trade_route_wait (crates/astromech-data/src/spansh.rs:638-650) keeps polling for up to 180 s on a spawn_blocking thread after the Route view unmounts (app/src/modules/trade/View.svelte mode switch). Leaving and coming back re-enables ''Find route'' (Route.svelte:29 busy is per mount), so you can start a second job while the first is still polling about once every 3 s. The UI isn''t blocked and stale results are dropped (Route.svelte:61-74), so the cost is wasted Spansh calls and a held thread. Defer: add a cancel flag or token when the follower or voice reuse this.'
target: astromech-mljg
identity: review-agent (mljg review)
commit: 517349a4337bddd467c02933a13265ea5c64340f
created_at: 2026-10-16T23:39:53.486206586Z
event:
  event: finding
  recommendation: defer
---

A route search can't be cancelled. spansh_trade_route_wait (crates/astromech-data/src/spansh.rs:638-650) keeps polling for up to 180 s on a spawn_blocking thread after the Route view unmounts (app/src/modules/trade/View.svelte mode switch). Leaving and coming back re-enables 'Find route' (Route.svelte:29 busy is per mount), so you can start a second job while the first is still polling about once every 3 s. The UI isn't blocked and stale results are dropped (Route.svelte:61-74), so the cost is wasted Spansh calls and a held thread. Defer: add a cancel flag or token when the follower or voice reuse this.
