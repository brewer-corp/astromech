---
schema: atelier.review
schema_version: 2
id: astromech-eu8h
title: finding on astromech-xajc @148c7445
target: astromech-xajc
identity: Grok 4.6 (round 2)
commit: 148c74450f482a93c1e4d2813fbfd83d530f2572
created_at: 2026-11-10T06:24:02.468060099Z
event:
  event: finding
  recommendation: fix_now
---

The page-sizing check still cannot pass for the Route files increment 1 enrolls. astromech-ok29 Done when 2 requires "the page-sizing lint rule is enabled for app/src/modules/route and passes". astromech-irtr Done when 2 (commit 148c7445) requires the width and font-size parts on for "app/src/modules/route". Strategy fills-the-window still shows the check "on for every module except the Controls ones"; astromech-gvy0 Done when 2 enrolls every file under app/src/modules/ except Controls.

astromech-mwh5 specifies the check fails on a max-width or font-size in an enrolled file, allowing only media-query conditions and max-width: var(--measure) on prose. app/src/modules/route/ holds frame chrome this increment does not convert: Strip.svelte:114 (.sys max-width 33% and font-size), PlotBar.svelte (font-size and min-width 12rem; used from App.svelte's header), NeutronFile.svelte:116 (progress max-width 24rem). Strip is the 36 px route strip in App.svelte, not the Route page.

A worker who enables the check for app/src/modules/route, as ok29 and irtr require, cannot pass pnpm lint without converting that chrome or weakening the check. Enroll only the page files each item converts (route/View.svelte for ok29/qwvs), name those files in ok29 and irtr, and change fills-the-window and gvy0 to converted page files outside Controls, not every file in the module tree.
