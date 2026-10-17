---
created_at: "2026-10-16T23:34:57.634576638+00:00"
id: "astromech-4f76"
evidence_type: "validation"
commit: "1ada4a07f0a8708520ac85ea853f398b211adb45"
result: "pass"
captured_at: "2026-10-16T23:34:54.865453572+00:00"
command: "bash -c 'scripts/ui --url '\"'\"'http://127.0.0.1:5173/#/powerplay'\"'\"' --wait '\"'\"'My standing'\"'\"' --press Control+K --wait '\"'\"'Page ·'\"'\"' --type '\"'\"'frame shift'\"'\"' --wait '\"'\"'Outfitting · Core internals'\"'\"' --screenshot /home/supadava/.cache/astromech-2u1x/9-typed.png --press Enter --wait '\"'\"'Abraham Lincoln'\"'\"' --screenshot /home/supadava/.cache/astromech-2u1x/10-landed.png --eval \"[location.hash, [...document.querySelectorAll('\"'\"'select'\"'\"')].slice(0,3).map(s=>s.value)]\" && scripts/ui --url '\"'\"'http://127.0.0.1:5173/#/materials'\"'\"' --wait Raw --press Control+K --wait '\"'\"'Page ·'\"'\"' --type '\"'\"'6a frame shift'\"'\"' --wait '\"'\"'fitted on Heavy Lifter'\"'\"' --press Enter --wait '\"'\"'Abraham Lincoln'\"'\"' --eval \"[location.hash, [...document.querySelectorAll('\"'\"'select'\"'\"')].slice(0,3).map(s=>s.value)]\" --eval \"(()=>{const s=[...document.querySelectorAll('\"'\"'label'\"'\"')].find(l=>l.textContent.trim().startsWith('\"'\"'Rating'\"'\"')).querySelector('\"'\"'select'\"'\"'); s.value='\"'\"''\"'\"'; s.dispatchEvent(new Event('\"'\"'change'\"'\"',{bubbles:true})); return new Promise(r=>setTimeout(r,800)).then(()=>[location.hash, [...document.querySelectorAll('\"'\"'select'\"'\"')].slice(0,3).map(s=>s.value)])})()\" --screenshot /home/supadava/.cache/astromech-2u1x/11-rating-any.png'"
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
title: "UI check (scripts/ui, browser preview, 1ada4a07f0a8708520ac85ea853f398b211adb45): (1) #/powerplay, Ctrl+K, 'frame shift', Enter -> #/outfitting/Frame%20Shift%20Drive, Module Frame Shift Drive, Class/Rating Any, Nearest results shown. (2) #/materials, Ctrl+K, '6a frame shift' (fitted on Heavy Lifter), Enter -> #/outfitting/Frame%20Shift%20Drive?class=6&rating=A with Class 6, Rating A; then Rating set to Any -> #/outfitting/Frame%20Shift%20Drive?class=6, Class stays 6. Screenshots 9-11 read. Preview rows are a fixed sample whatever the pick."
updated_at: "2026-10-16T23:34:57.634869835+00:00"
---

## Summary

UI check (scripts/ui, browser preview, 1ada4a07f0a8708520ac85ea853f398b211adb45): (1) #/powerplay, Ctrl+K, 'frame shift', Enter -> #/outfitting/Frame%20Shift%20Drive, Module Frame Shift Drive, Class/Rating Any, Nearest results shown. (2) #/materials, Ctrl+K, '6a frame shift' (fitted on Heavy Lifter), Enter -> #/outfitting/Frame%20Shift%20Drive?class=6&rating=A with Class 6, Rating A; then Rating set to Any -> #/outfitting/Frame%20Shift%20Drive?class=6, Class stays 6. Screenshots 9-11 read. Preview rows are a fixed sample whatever the pick.

## Command

```console
bash -c 'scripts/ui --url '"'"'http://127.0.0.1:5173/#/powerplay'"'"' --wait '"'"'My standing'"'"' --press Control+K --wait '"'"'Page ·'"'"' --type '"'"'frame shift'"'"' --wait '"'"'Outfitting · Core internals'"'"' --screenshot /home/supadava/.cache/astromech-2u1x/9-typed.png --press Enter --wait '"'"'Abraham Lincoln'"'"' --screenshot /home/supadava/.cache/astromech-2u1x/10-landed.png --eval "[location.hash, [...document.querySelectorAll('"'"'select'"'"')].slice(0,3).map(s=>s.value)]" && scripts/ui --url '"'"'http://127.0.0.1:5173/#/materials'"'"' --wait Raw --press Control+K --wait '"'"'Page ·'"'"' --type '"'"'6a frame shift'"'"' --wait '"'"'fitted on Heavy Lifter'"'"' --press Enter --wait '"'"'Abraham Lincoln'"'"' --eval "[location.hash, [...document.querySelectorAll('"'"'select'"'"')].slice(0,3).map(s=>s.value)]" --eval "(()=>{const s=[...document.querySelectorAll('"'"'label'"'"')].find(l=>l.textContent.trim().startsWith('"'"'Rating'"'"')).querySelector('"'"'select'"'"'); s.value='"'"''"'"'; s.dispatchEvent(new Event('"'"'change'"'"',{bubbles:true})); return new Promise(r=>setTimeout(r,800)).then(()=>[location.hash, [...document.querySelectorAll('"'"'select'"'"')].slice(0,3).map(s=>s.value)])})()" --screenshot /home/supadava/.cache/astromech-2u1x/11-rating-any.png'
```

Exit status: 0

## Stdout

Bytes: 885
Truncated: no

```text
{
  "ok": true,
  "chrome": "/usr/bin/google-chrome-stable",
  "url": "http://127.0.0.1:5173/#/powerplay",
  "finalUrl": "http://127.0.0.1:5173/#/outfitting/Frame%20Shift%20Drive",
  "title": "astromech",
  "screenshots": [
    "/home/supadava/.cache/astromech-2u1x/9-typed.png",
    "/home/supadava/.cache/astromech-2u1x/10-landed.png"
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
  "finalUrl": "http://127.0.0.1:5173/#/outfitting/Frame%20Shift%20Drive?class=6",
  "title": "astromech",
  "screenshots": [
    "/home/supadava/.cache/astromech-2u1x/11-rating-any.png"
  ],
  "eval": [
    "#/outfitting/Frame%20Shift%20Drive?class=6",
    [
      "Frame Shift Drive",
      "6",
      ""
    ]
  ]
}
```

## Stderr

Bytes: 535
Truncated: no

```text
open http://127.0.0.1:5173/#/powerplay
wait "My standing"
press Control+K
wait "Page ·"
type "frame shift"
wait "Outfitting · Core internals"
screenshot /home/supadava/.cache/astromech-2u1x/9-typed.png
press Enter
wait "Abraham Lincoln"
screenshot /home/supadava/.cache/astromech-2u1x/10-landed.png
open http://127.0.0.1:5173/#/materials
wait "Raw"
press Control+K
wait "Page ·"
type "6a frame shift"
wait "fitted on Heavy Lifter"
press Enter
wait "Abraham Lincoln"
screenshot /home/supadava/.cache/astromech-2u1x/11-rating-any.png
```

