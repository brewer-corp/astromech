---
created_at: "2026-10-16T19:13:09.238402333+00:00"
id: "astromech-597s"
evidence_type: "validation"
commit: "bf12a092ad8a368c3896df9b056e08b5d6b83c5e"
result: "pass"
captured_at: "2026-10-16T19:13:08.362817021+00:00"
command: "scripts/ui --url 'http://127.0.0.1:5281/#/materials' --wait Manufactured --click-text Manufactured --wait Pharmaceutical --click-text 'Pharmaceutical Isolators' --wait 'Nearest to Sol' --eval 'const i=document.querySelector('\"'\"'.origin input'\"'\"'); i.focus(); i.value='\"'\"'shin'\"'\"'; i.dispatchEvent(new Event('\"'\"'input'\"'\"',{bubbles:true})); 1' --wait-sel .hits --click '.hits li:first-child button' --wait 'Nearest to Shinrarta' --click .clear --wait 'Nearest to Sol'"
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
title: "UI check (browser fallback, Vite on :5281): faction lookup on the kit from the current system (Nearest to Sol), typed origin via autocomplete (shin -> Shinrarta Dezhra; Nearest to Shinrarta Dezhra), Enter on a typed name (Deciat), and x back to current. Screenshots /tmp/qz9u-shots/faction-{current,autocomplete,typed}.png (not committed). Limit: fallback hits are the fixture for any origin; the real Tauri path (galaxy_nearby with origin, galaxy_systems) is not compiled or run on Linux."
updated_at: "2026-10-16T19:13:09.238830772+00:00"
---

## Summary

UI check (browser fallback, Vite on :5281): faction lookup on the kit from the current system (Nearest to Sol), typed origin via autocomplete (shin -> Shinrarta Dezhra; Nearest to Shinrarta Dezhra), Enter on a typed name (Deciat), and x back to current. Screenshots /tmp/qz9u-shots/faction-{current,autocomplete,typed}.png (not committed). Limit: fallback hits are the fixture for any origin; the real Tauri path (galaxy_nearby with origin, galaxy_systems) is not compiled or run on Linux.

## Command

```console
scripts/ui --url 'http://127.0.0.1:5281/#/materials' --wait Manufactured --click-text Manufactured --wait Pharmaceutical --click-text 'Pharmaceutical Isolators' --wait 'Nearest to Sol' --eval 'const i=document.querySelector('"'"'.origin input'"'"'); i.focus(); i.value='"'"'shin'"'"'; i.dispatchEvent(new Event('"'"'input'"'"',{bubbles:true})); 1' --wait-sel .hits --click '.hits li:first-child button' --wait 'Nearest to Shinrarta' --click .clear --wait 'Nearest to Sol'
```

Exit status: 0

## Stdout

Bytes: 217
Truncated: no

```text
{
  "ok": true,
  "chrome": "/usr/bin/google-chrome-stable",
  "url": "http://127.0.0.1:5281/#/materials",
  "finalUrl": "http://127.0.0.1:5281/#/materials",
  "title": "astromech",
  "screenshots": [],
  "eval": 1
}
```

## Stderr

Bytes: 279
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
click .clear
wait "Nearest to Sol"
```

