---
schema: atelier.review
schema_version: 1
id: astromech-5bji
title: finding on astromech-ja5a @8fd31c63
target: astromech-ja5a
identity: claude-reviewer-ja5a
commit: 8fd31c6366b5b5ca540fcacf3dd0315bcf6a0288
created_at: 2026-11-07T00:07:25.113700421Z
event:
  event: finding
  recommendation: fix_now
---

low: docs/releases/v1.0.0-alpha.4.md:48 (astromech-1per) says 'from the Commodities page, ask "where do I sell" a commodity', but Commodities no longer sells or searches: it is trade routes plus links to the Places finders (docs/guide/trade.md:3-14, kwoi d72c4098). The tester is pointed at UI that is gone. Fix: 'Do: in Places, Sell (or by voice, "where can I sell ..."), look up a commodity you carry, fly to the first answer and sell one load.'
