---
schema: atelier.review
schema_version: 2
id: astromech-34i3
title: finding on astromech-bp23 @e0fac0ea
target: astromech-bp23
identity: claude-reviewer-i8vr
commit: e0fac0ea40744ef51d4ea921bc10933300547cd8
created_at: 2026-11-07T09:47:39.425808900Z
event:
  event: finding
  recommendation: defer
---

low: app/src/lib/ui/SearchKit.svelte:97, app/src/modules/trade/Route.svelte:31, Route.browser.test.ts:18, SearchKit.browser.test.ts:86, docs/ARCHITECTURE.md:576 ('The shell resolves the pad...'), app/src/preview/shell.ts:2,311 ('The shell's commands') These still say the shell resolves or serves what the core's command layer now does. i8vr fixed three such slips but missed these. Readers may look for logic in window.rs that isn't there. Defer to astromech-96dv, which retypes the preview and these modules; reword them to the core or command layer.
