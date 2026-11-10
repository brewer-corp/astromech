---
schema: atelier.review
schema_version: 2
id: astromech-83wb
title: finding on astromech-xajc @7b1eb202
target: astromech-xajc
identity: Grok 4.6
commit: 7b1eb20293d9255df8c2a77a59f656e5c44b8421
created_at: 2026-11-10T06:01:23.255008924Z
event:
  event: finding
  recommendation: defer
---

A 32px status bar (astromech-q79q) plus the existing 64px header, 36px route strip and 44px page tabs is 176px of chrome before content, more when the header stacks, at the 1100x700 minimum. That fights hierarchy over density on the size the strategy designs for. Add a min-size chrome budget to q79q and DESIGN.md Frame (what drops or stacks) so the bar is not specified only at 2560px.
