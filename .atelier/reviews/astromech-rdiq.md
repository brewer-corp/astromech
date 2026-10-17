---
schema: atelier.review
schema_version: 1
id: astromech-rdiq
title: 'app/src/modules/trade/View.test.ts:67-68: the Sell assertion is vacuous, so the done-when (''a truncated sell answer shows the narrow-the-radius hint'') is not proven. After the Buy click the Buy hint is still on screen, and findByText(/narrow the radius to see them all/) matches it at once. Mutation: changing the warning condition in lib/trade.ts toResults to ''p?.truncated && s.side === "buy"'' leaves all 8 Trade page tests passing. Fix: assert the Sell hint itself, e.g. wait for the Sell results (a Sell-only heading or row, or near called with ''sell'') and then check the hint, or render with cut=true and Sell selected from the start.'
target: astromech-egyl
identity: review-agent (opus, independent)
commit: e2636e2642496f819726a94a264a8cf8187bafe3
created_at: 2026-10-17T07:05:15.183929289Z
event:
  event: finding
  recommendation: fix_now
---

app/src/modules/trade/View.test.ts:67-68: the Sell assertion is vacuous, so the done-when ('a truncated sell answer shows the narrow-the-radius hint') is not proven. After the Buy click the Buy hint is still on screen, and findByText(/narrow the radius to see them all/) matches it at once. Mutation: changing the warning condition in lib/trade.ts toResults to 'p?.truncated && s.side === "buy"' leaves all 8 Trade page tests passing. Fix: assert the Sell hint itself, e.g. wait for the Sell results (a Sell-only heading or row, or near called with 'sell') and then check the hint, or render with cut=true and Sell selected from the start.
