---
schema: atelier.review
schema_version: 2
id: astromech-igbw
title: finding on astromech-vucf @7045a4d2
target: astromech-vucf
identity: claude-reviewer-jkgv
commit: 7045a4d27585ba1223a8c6459d18bfb3d7274cda
created_at: 2026-11-07T20:55:34.052664367Z
event:
  event: finding
  recommendation: fix_now
---

medium: app/dev/bridge.ts:32-80 (jkgv Done when 1) The bridge answers any request to /__astromech/call, including a cross-origin simple request, and drops the control API's token gate. Reproduced on ASTROMECH_WORLD=../worlds/preview/history with vite on :5391: curl -X POST with 'Origin: https://evil.example' and 'Content-Type: text/plain' (what a web page's fetch(..., {mode: 'no-cors'}) sends) got 200 and the logbook. set_folders {bindings_dir: <a folder outside the world>} was accepted, and binds_copy then wrote Planted.4.2.binds into that folder. Vite 7's cors middleware only sets headers, and its host check passes Host: localhost, so neither stops it. Impact: any web page open in the developer's browser while the bridge runs can point the world core at real folders and write there (the Elite Bindings folder, for example). That breaks the sandbox world mode gives and the token gate in ARCHITECTURE's Exposure paragraph. Fix: in answer(), refuse with 403 unless Sec-Fetch-Site is same-origin (or the Origin header, when present, equals the dev server's own) and the content-type is application/json, which lib/bridge.ts already sends. Add a bridge-check or unit case that a foreign-origin text/plain POST is refused.
