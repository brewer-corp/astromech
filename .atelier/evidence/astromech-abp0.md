---
created_at: "2026-10-16T19:50:48.506085258+00:00"
id: "astromech-abp0"
evidence_type: "validation"
commit: "d5215195f8d11868b394cc294138b0c3dbaab4c0"
result: "pass"
captured_at: "2026-10-16T19:50:47.524647555+00:00"
command: "scripts/ui --url 'http://127.0.0.1:5281/#/materials' --wait Manufactured --click-text Manufactured --wait Pharmaceutical --click-text 'Pharmaceutical Isolators' --wait 'WISE 0855' --eval '(()=>{if (document.querySelectorAll('\"'\"'.kit .chip'\"'\"').length!==4 || !document.querySelector('\"'\"'.kit h4 .chip'\"'\"')) throw new Error('\"'\"'chips missing'\"'\"'); const i=document.querySelector('\"'\"'.origin input'\"'\"'); i.focus(); i.value='\"'\"'shin'\"'\"'; i.dispatchEvent(new Event('\"'\"'input'\"'\"',{bubbles:true})); return 1})()' --wait-sel .hits --click '.hits li:first-child button' --wait-sel '.kit h4 .chip' --eval '(()=>{const h=document.querySelector('\"'\"'.kit h4 .chip .name'\"'\"').textContent; if (h!=='\"'\"'Shinrarta Dezhra'\"'\"') throw new Error(h); return h})()'"
exit_status: "0"
agent_identity: "claude-opus-5.5"
target:
  kind: "issue"
  id: "astromech-qz9u"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-qz9u"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "After the merge, SystemChip renders on the kit's origin heading and every result row, from the current system (Nearest to Sol) and a typed origin (Shinrarta Dezhra via autocomplete). 4 chips in the kit each time. Browser preview; screenshots /tmp/qz9u-shots/chips-current.png and chips-typed.png."
updated_at: "2026-10-16T19:50:48.506537823+00:00"
---

## Summary

After the merge, SystemChip renders on the kit's origin heading and every result row, from the current system (Nearest to Sol) and a typed origin (Shinrarta Dezhra via autocomplete). 4 chips in the kit each time. Browser preview; screenshots /tmp/qz9u-shots/chips-current.png and chips-typed.png.

## Command

```console
scripts/ui --url 'http://127.0.0.1:5281/#/materials' --wait Manufactured --click-text Manufactured --wait Pharmaceutical --click-text 'Pharmaceutical Isolators' --wait 'WISE 0855' --eval '(()=>{if (document.querySelectorAll('"'"'.kit .chip'"'"').length!==4 || !document.querySelector('"'"'.kit h4 .chip'"'"')) throw new Error('"'"'chips missing'"'"'); const i=document.querySelector('"'"'.origin input'"'"'); i.focus(); i.value='"'"'shin'"'"'; i.dispatchEvent(new Event('"'"'input'"'"',{bubbles:true})); return 1})()' --wait-sel .hits --click '.hits li:first-child button' --wait-sel '.kit h4 .chip' --eval '(()=>{const h=document.querySelector('"'"'.kit h4 .chip .name'"'"').textContent; if (h!=='"'"'Shinrarta Dezhra'"'"') throw new Error(h); return h})()'
```

Exit status: 0

## Stdout

Bytes: 234
Truncated: no

```text
{
  "ok": true,
  "chrome": "/usr/bin/google-chrome-stable",
  "url": "http://127.0.0.1:5281/#/materials",
  "finalUrl": "http://127.0.0.1:5281/#/materials",
  "title": "astromech",
  "screenshots": [],
  "eval": "Shinrarta Dezhra"
}
```

## Stderr

Bytes: 234
Truncated: no

```text
open http://127.0.0.1:5281/#/materials
wait "Manufactured"
click-text "Manufactured"
wait "Pharmaceutical"
click-text "Pharmaceutical Isolators"
wait "WISE 0855"
wait-sel .hits
click .hits li:first-child button
wait-sel .kit h4 .chip
```

