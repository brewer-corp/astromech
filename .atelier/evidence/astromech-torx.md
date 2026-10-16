---
created_at: "2026-10-16T19:19:14.193855176+00:00"
id: "astromech-torx"
evidence_type: "validation"
commit: "1e28693cf90399521cb61803562b30d2c5ce6555"
result: "pass"
captured_at: "2026-10-16T19:19:12.763575098+00:00"
command: "scripts/ui --url 'http://127.0.0.1:5281/#/materials' --wait Manufactured --click-text Manufactured --wait Pharmaceutical --click-text 'Pharmaceutical Isolators' --wait 'Nearest to Sol' --eval '(()=>{const i=document.querySelector('\"'\"'.origin input'\"'\"'); i.focus(); i.value='\"'\"'shin'\"'\"'; i.dispatchEvent(new Event('\"'\"'input'\"'\"',{bubbles:true})); return 1})()' --wait-sel .hits --click '.hits li:first-child button' --wait 'Nearest to Shinrarta' --screenshot /tmp/qz9u-shots/faction-typed.png --click .clear --wait 'Nearest to Sol' --screenshot /tmp/qz9u-shots/faction-current.png"
exit_status: "0"
agent_identity: "claude-opus-5.5"
target:
  kind: "issue"
  id: "astromech-pxy2"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-pxy2"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Re-run after astromech-1imw changed the kit: faction lookup from the current system (Nearest to Sol), typed origin via autocomplete (shin -> Shinrarta Dezhra), x back to current. Browser fallback; same limits as astromech-597s."
updated_at: "2026-10-16T19:19:14.194299775+00:00"
---

## Summary

Re-run after astromech-1imw changed the kit: faction lookup from the current system (Nearest to Sol), typed origin via autocomplete (shin -> Shinrarta Dezhra), x back to current. Browser fallback; same limits as astromech-597s.

## Command

```console
scripts/ui --url 'http://127.0.0.1:5281/#/materials' --wait Manufactured --click-text Manufactured --wait Pharmaceutical --click-text 'Pharmaceutical Isolators' --wait 'Nearest to Sol' --eval '(()=>{const i=document.querySelector('"'"'.origin input'"'"'); i.focus(); i.value='"'"'shin'"'"'; i.dispatchEvent(new Event('"'"'input'"'"',{bubbles:true})); return 1})()' --wait-sel .hits --click '.hits li:first-child button' --wait 'Nearest to Shinrarta' --screenshot /tmp/qz9u-shots/faction-typed.png --click .clear --wait 'Nearest to Sol' --screenshot /tmp/qz9u-shots/faction-current.png
```

Exit status: 0

## Stdout

Bytes: 303
Truncated: no

```text
{
  "ok": true,
  "chrome": "/usr/bin/google-chrome-stable",
  "url": "http://127.0.0.1:5281/#/materials",
  "finalUrl": "http://127.0.0.1:5281/#/materials",
  "title": "astromech",
  "screenshots": [
    "/tmp/qz9u-shots/faction-typed.png",
    "/tmp/qz9u-shots/faction-current.png"
  ],
  "eval": 1
}
```

## Stderr

Bytes: 371
Truncated: no

```text
open http://127.0.0.1:5281/#/materials
wait "Manufactured"
click-text "Manufactured"
wait "Pharmaceutical"
click-text "Pharmaceutical Isolators"
wait "Nearest to Sol"
wait-sel .hits
click .hits li:first-child button
wait "Nearest to Shinrarta"
screenshot /tmp/qz9u-shots/faction-typed.png
click .clear
wait "Nearest to Sol"
screenshot /tmp/qz9u-shots/faction-current.png
```

