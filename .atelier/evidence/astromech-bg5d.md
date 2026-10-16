---
created_at: "2026-10-16T22:29:13.250521455+00:00"
id: "astromech-bg5d"
evidence_type: "validation"
commit: "1f3c1e73960eb8f5b7fb6ee6975ad821200db102"
result: "pass"
captured_at: "2026-10-16T22:29:10.531639010+00:00"
command: "scripts/ui --url 'http://127.0.0.1:5173/#/powerplay' --viewport 1280x1500 --wait 'My standing' --press 'Control+K' --wait 'Page ·' --screenshot /home/supadava/.cache/astromech-2u1x/1-open.png --type 'proto heat' --wait 'Material · Manufactured' --screenshot /home/supadava/.cache/astromech-2u1x/2-typed.png --press Enter --wait 'Ross 154' --eval 'new Promise(r=>setTimeout(r,1500))' --screenshot /home/supadava/.cache/astromech-2u1x/3-landed.png --eval '[location.hash, document.querySelector('\"'\"'.about h3'\"'\"').textContent, document.querySelector('\"'\"'.about h4'\"'\"').textContent]'"
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
title: "UI check (scripts/ui, browser preview, 1f3c1e7): on #/powerplay press Ctrl+K, type 'proto heat', Enter; lands on #/materials/protoheatradiators with Proto Heat Radiators selected and its Nearest search (Sol, Ross 154 rows) shown. Screenshots read: palette open, filtered, landed (1280x1500)."
updated_at: "2026-10-16T22:29:13.250962398+00:00"
---

## Summary

UI check (scripts/ui, browser preview, 1f3c1e7): on #/powerplay press Ctrl+K, type 'proto heat', Enter; lands on #/materials/protoheatradiators with Proto Heat Radiators selected and its Nearest search (Sol, Ross 154 rows) shown. Screenshots read: palette open, filtered, landed (1280x1500).

## Command

```console
scripts/ui --url 'http://127.0.0.1:5173/#/powerplay' --viewport 1280x1500 --wait 'My standing' --press 'Control+K' --wait 'Page ·' --screenshot /home/supadava/.cache/astromech-2u1x/1-open.png --type 'proto heat' --wait 'Material · Manufactured' --screenshot /home/supadava/.cache/astromech-2u1x/2-typed.png --press Enter --wait 'Ross 154' --eval 'new Promise(r=>setTimeout(r,1500))' --screenshot /home/supadava/.cache/astromech-2u1x/3-landed.png --eval '[location.hash, document.querySelector('"'"'.about h3'"'"').textContent, document.querySelector('"'"'.about h4'"'"').textContent]'
```

Exit status: 0

## Stdout

Bytes: 504
Truncated: no

```text
{
  "ok": true,
  "chrome": "/usr/bin/google-chrome-stable",
  "url": "http://127.0.0.1:5173/#/powerplay",
  "finalUrl": "http://127.0.0.1:5173/#/materials/protoheatradiators",
  "title": "astromech",
  "screenshots": [
    "/home/supadava/.cache/astromech-2u1x/1-open.png",
    "/home/supadava/.cache/astromech-2u1x/2-typed.png",
    "/home/supadava/.cache/astromech-2u1x/3-landed.png"
  ],
  "eval": [
    "#/materials/protoheatradiators",
    "Proto Heat Radiators",
    "Nearest to Sol Copy"
  ]
}
```

## Stderr

Bytes: 347
Truncated: no

```text
open http://127.0.0.1:5173/#/powerplay
wait "My standing"
press Control+K
wait "Page ·"
screenshot /home/supadava/.cache/astromech-2u1x/1-open.png
type "proto heat"
wait "Material · Manufactured"
screenshot /home/supadava/.cache/astromech-2u1x/2-typed.png
press Enter
wait "Ross 154"
screenshot /home/supadava/.cache/astromech-2u1x/3-landed.png
```

