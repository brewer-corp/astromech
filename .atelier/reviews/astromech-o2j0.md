---
schema: atelier.review
schema_version: 1
id: astromech-o2j0
title: finding on astromech-ja5a @8fd31c63
target: astromech-ja5a
identity: claude-reviewer-ja5a
commit: 8fd31c6366b5b5ca540fcacf3dd0315bcf6a0288
created_at: 2026-11-07T00:07:24.780989443Z
event:
  event: finding
  recommendation: fix_now
---

medium: docs/releases/v1.0.0-alpha.4.md:96 and :103 send the tester to the neutron star file 'on the Route page', but it is no longer there: the Route page (app/src/modules/route/View.svelte) has no NeutronFile; it sits under Neutron in the Find or plot panel (Plot.svelte:105, rendered by PlotBar.svelte:28; guide routes.md:246 'Under Neutron, the plot panel also shows the neutron star file'). alpha.3's notes told players to download it from the Route page, and the What's new section never says it moved. Fix: rk4s 'Do: in Find or plot in the header, pick a system, choose Neutron and download the neutron star file in the panel. Then plot a neutron route (it should say "on this PC") and an exact route...'; snqt '...the route strip, the plot panel's neutron star file and a narrow window'; and add to the Find or plot bullet (line 12): 'The neutron star file's download moved with it, under Neutron in the plot panel.'
