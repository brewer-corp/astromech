---
schema: atelier.review
schema_version: 1
id: astromech-et0g
title: finding on astromech-51ue @a43292fc
target: astromech-51ue
identity: reviewer-sonnet-f7i0
commit: a43292fced08a3c6b17cf468f59d79064e6de976
created_at: 2026-11-05T09:32:59.301198762Z
event:
  event: finding
  recommendation: defer
---

low: astromech-f7i0 app/src/lib/ui/SearchKit.svelte (res.reach empty line) decision on the open question: 'No rings within 32 ly.' with no action is not what DESIGN.md Empty state asks for (one prose line plus one widening action), and an action exists. That state needs a ring type set (a reserve is asked of Spansh, so only the post-filter on ring type can empty a reach-limited answer), and clearing it always widens the search. Offer one action, 'Any ring type', which the finder passes to the kit (an optional widen override on Results or a snippet), rather than leaving a dead end. Not blocking: the case needs a rare ring type with too many bodies to rank, the line says what happened, and the ring Choice is on screen.
