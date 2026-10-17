---
schema: atelier.review
schema_version: 1
id: astromech-kwom
title: 'app/src/lib/trade.ts:149 / View.test.ts: no test covers the claim that the Anywhere list shows no hint. The mock never sets truncated on tradeAnywhere. Mutation: dropping ''&& !anywhere'' leaves every Trade test passing. On the real backend the galaxy-wide answer is almost always full (truncated = len >= GALAXY_LIMIT 100, crates/astromech-market/src/trade.rs:318), so losing the guard would show a wrong ''narrow the radius'' hint on nearly every Anywhere lookup, where there is no radius to narrow. Fix: in the same test, mark the anywhere answer truncated, switch to Anywhere and assert the hint is absent once its rows render.'
target: astromech-egyl
identity: review-agent (opus, independent)
commit: e2636e2642496f819726a94a264a8cf8187bafe3
created_at: 2026-10-17T07:05:15.342774312Z
event:
  event: finding
  recommendation: fix_now
---

app/src/lib/trade.ts:149 / View.test.ts: no test covers the claim that the Anywhere list shows no hint. The mock never sets truncated on tradeAnywhere. Mutation: dropping '&& !anywhere' leaves every Trade test passing. On the real backend the galaxy-wide answer is almost always full (truncated = len >= GALAXY_LIMIT 100, crates/astromech-market/src/trade.rs:318), so losing the guard would show a wrong 'narrow the radius' hint on nearly every Anywhere lookup, where there is no radius to narrow. Fix: in the same test, mark the anywhere answer truncated, switch to Anywhere and assert the hint is absent once its rows render.
