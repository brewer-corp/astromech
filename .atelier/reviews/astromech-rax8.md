---
schema: atelier.review
schema_version: 1
id: astromech-rax8
title: 'R1 [medium] app/src/modules/colonisation/View.svelte:15-22: projects load once on mount (an $effect with no reactive deps) and never refresh, unlike Materials (1 s), Powerplay (2 s) and Engineering (5 s), which poll. Scenario: with the page open, the commander delivers to the depot or buys at a market; ''Still to buy'', loads, ''aboard'' and the Where-to-buy needs stay at the old values until they navigate away and back. That falls short of k3wp''s goal of replacing Raven Colonial''s and EDCoPilot''s live trackers, and the xf8j game check (a delivery run matches progress and load counts) will run into it. Fix: re-load on an interval (colonisation_projects is cached per journal version), keep ''picked'' instead of resetting it to p[0], and keep needs stable when unchanged so SearchKit doesn''t re-run its search on every poll.'
target: astromech-k3wp
identity: review-agent (k3wp review)
commit: f5e62c69c573b451172bf1cd2e16cce3b6519e75
created_at: 2026-10-16T23:10:37.348209189Z
event:
  event: finding
  recommendation: fix_now
---

R1 [medium] app/src/modules/colonisation/View.svelte:15-22: projects load once on mount (an $effect with no reactive deps) and never refresh, unlike Materials (1 s), Powerplay (2 s) and Engineering (5 s), which poll. Scenario: with the page open, the commander delivers to the depot or buys at a market; 'Still to buy', loads, 'aboard' and the Where-to-buy needs stay at the old values until they navigate away and back. That falls short of k3wp's goal of replacing Raven Colonial's and EDCoPilot's live trackers, and the xf8j game check (a delivery run matches progress and load counts) will run into it. Fix: re-load on an interval (colonisation_projects is cached per journal version), keep 'picked' instead of resetting it to p[0], and keep needs stable when unchanged so SearchKit doesn't re-run its search on every poll.
