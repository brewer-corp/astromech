---
schema: atelier.review
schema_version: 1
id: astromech-608r
title: approval on astromech-0k63 @31f39f28
target: astromech-0k63
identity: reviewer-fable-dfvp
commit: 31f39f28deb76f14cf055e1383890d34b492cf04
created_at: 2026-11-05T17:45:38.196266495Z
event:
  event: approval
---

Approval of astromech-0k63 at 31f39f28deb76f14cf055e1383890d34b492cf04 on task/astromech-0k63. git diff main...HEAD is the close record only (workflow_branch, status done, Close Reason). The work itself (c29350a2) was reviewed with task/astromech-dfvp (review account astromech-hifb, approvals astromech-4srf and astromech-jn11) and is on main (de071398; c29350a2 is an ancestor of main). Close Reason checked against the code on main: route/View.svelte renders <Failure text='Not on the clipboard: <why>' onaction={run(copyNext)}> inside the .next card on --surface, no copy glyph; route/View.test.ts 'shows a clipboard write that failed as a hard error, and TRY AGAIN copies again' asserts role=alert, the '!' text, no .glyph, the .next ancestor and the recovery, and cannot pass on the old span.clip.warn view (no alert role); docs/guide/routes.md names Try again; docs/design/cells-route-clipboard-{before,after}.png exist. Recomputed contrast: Alarm #f0412e on the old --cell-opt #2b2d2e 3.62:1, on hull #1a1310 4.81:1, as stated. Finding astromech-rtf9 (strip shows the same failure as an orange warning) disposed defer (astromech-13nh) to follow-up astromech-rpdc, whose Done when covers it. Limits: no Windows window; the 'fails on the old view' claim is by reading the old markup, not by running the test against it.
