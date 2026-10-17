---
created_at: "2026-10-16T23:30:16.342056416+00:00"
id: "astromech-kh1b"
evidence_type: "validation"
commit: "438e48d6746a93fc9f46262c5149ae2a38eec2b4"
result: "pass"
captured_at: "2026-10-16T23:30:09.606359181+00:00"
command: "bash -c 'scripts/ui --url '\"'\"'http://127.0.0.1:5173/#/powerplay'\"'\"' --wait '\"'\"'My standing'\"'\"' --press Control+K --wait '\"'\"'Page ·'\"'\"' --type '\"'\"'frame shift'\"'\"' --wait '\"'\"'Outfitting · Core internals'\"'\"' --press Enter --wait '\"'\"'Abraham Lincoln'\"'\"' --screenshot /home/supadava/.cache/astromech-2u1x/7-fsd-landed.png --eval \"[location.hash, [...document.querySelectorAll('\"'\"'select'\"'\"')].slice(0,3).map(s=>s.value)]\" && scripts/ui --url '\"'\"'http://127.0.0.1:5173/#/materials'\"'\"' --wait Raw --press Control+K --wait '\"'\"'Page ·'\"'\"' --type '\"'\"'6a frame shift'\"'\"' --wait '\"'\"'fitted on Heavy Lifter'\"'\"' --press Enter --wait '\"'\"'Abraham Lincoln'\"'\"' --screenshot /home/supadava/.cache/astromech-2u1x/8-fitted-landed.png --eval \"[location.hash, [...document.querySelectorAll('\"'\"'select'\"'\"')].slice(0,3).map(s=>s.value)]\"'"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-2u1x"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-2u1x"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "UI check (scripts/ui, browser preview, 438e48d6746a93fc9f46262c5149ae2a38eec2b4): on #/powerplay press Ctrl+K, type 'frame shift', Enter; lands on #/outfitting/Frame%20Shift%20Drive with Module=Frame Shift Drive, Class/Rating=Any and the Nearest outfitting results. From #/materials, Ctrl+K '6a frame shift' (fitted on Heavy Lifter) lands on #/outfitting/Frame%20Shift%20Drive/6A with Class 6, Rating A. Screenshots read. Preview rows are a fixed sample (5A FSD) whatever the pick; App.test checks the search request."
updated_at: "2026-10-16T23:30:16.342563492+00:00"
---

## Summary

UI check (scripts/ui, browser preview, 438e48d6746a93fc9f46262c5149ae2a38eec2b4): on #/powerplay press Ctrl+K, type 'frame shift', Enter; lands on #/outfitting/Frame%20Shift%20Drive with Module=Frame Shift Drive, Class/Rating=Any and the Nearest outfitting results. From #/materials, Ctrl+K '6a frame shift' (fitted on Heavy Lifter) lands on #/outfitting/Frame%20Shift%20Drive/6A with Class 6, Rating A. Screenshots read. Preview rows are a fixed sample (5A FSD) whatever the pick; App.test checks the search request.

## Command

```console
bash -c 'scripts/ui --url '"'"'http://127.0.0.1:5173/#/powerplay'"'"' --wait '"'"'My standing'"'"' --press Control+K --wait '"'"'Page ·'"'"' --type '"'"'frame shift'"'"' --wait '"'"'Outfitting · Core internals'"'"' --press Enter --wait '"'"'Abraham Lincoln'"'"' --screenshot /home/supadava/.cache/astromech-2u1x/7-fsd-landed.png --eval "[location.hash, [...document.querySelectorAll('"'"'select'"'"')].slice(0,3).map(s=>s.value)]" && scripts/ui --url '"'"'http://127.0.0.1:5173/#/materials'"'"' --wait Raw --press Control+K --wait '"'"'Page ·'"'"' --type '"'"'6a frame shift'"'"' --wait '"'"'fitted on Heavy Lifter'"'"' --press Enter --wait '"'"'Abraham Lincoln'"'"' --screenshot /home/supadava/.cache/astromech-2u1x/8-fitted-landed.png --eval "[location.hash, [...document.querySelectorAll('"'"'select'"'"')].slice(0,3).map(s=>s.value)]"'
```

Exit status: 0

## Stdout

Bytes: 825
Truncated: no

```text
{
  "ok": true,
  "chrome": "/usr/bin/google-chrome-stable",
  "url": "http://127.0.0.1:5173/#/powerplay",
  "finalUrl": "http://127.0.0.1:5173/#/outfitting/Frame%20Shift%20Drive",
  "title": "astromech",
  "screenshots": [
    "/home/supadava/.cache/astromech-2u1x/7-fsd-landed.png"
  ],
  "eval": [
    "#/outfitting/Frame%20Shift%20Drive",
    [
      "Frame Shift Drive",
      "",
      ""
    ]
  ]
}
{
  "ok": true,
  "chrome": "/usr/bin/google-chrome-stable",
  "url": "http://127.0.0.1:5173/#/materials",
  "finalUrl": "http://127.0.0.1:5173/#/outfitting/Frame%20Shift%20Drive/6A",
  "title": "astromech",
  "screenshots": [
    "/home/supadava/.cache/astromech-2u1x/8-fitted-landed.png"
  ],
  "eval": [
    "#/outfitting/Frame%20Shift%20Drive/6A",
    [
      "Frame Shift Drive",
      "6",
      "A"
    ]
  ]
}
```

## Stderr

Bytes: 480
Truncated: no

```text
open http://127.0.0.1:5173/#/powerplay
wait "My standing"
press Control+K
wait "Page ·"
type "frame shift"
wait "Outfitting · Core internals"
press Enter
wait "Abraham Lincoln"
screenshot /home/supadava/.cache/astromech-2u1x/7-fsd-landed.png
open http://127.0.0.1:5173/#/materials
wait "Raw"
press Control+K
wait "Page ·"
type "6a frame shift"
wait "fitted on Heavy Lifter"
press Enter
wait "Abraham Lincoln"
screenshot /home/supadava/.cache/astromech-2u1x/8-fitted-landed.png
```

