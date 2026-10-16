---
created_at: "2026-10-16T19:28:26.573352557+00:00"
id: "astromech-l49d"
evidence_type: "validation"
commit: "75aaae2fbb5e6f8d9b8a2a4185445b58c1c9e398"
result: "pass"
captured_at: "2026-10-16T19:28:25.256895627+00:00"
command: "scripts/ui --url 'http://127.0.0.1:5281/#/materials' --wait Manufactured --click-text Manufactured --wait Pharmaceutical --click-text 'Pharmaceutical Isolators' --wait 'Data age' --eval '(()=>{Storage.prototype.setItem=()=>{throw new Error('\"'\"'disk full'\"'\"')}; const s=document.querySelector('\"'\"'.filters select'\"'\"'); s.value='\"'\"'7'\"'\"'; s.dispatchEvent(new Event('\"'\"'change'\"'\"',{bubbles:true})); return 1})()' --wait 'Couldn'\"'\"'t save the filter: disk full' --wait 'WISE 0855' --eval '(()=>{const s=document.querySelector('\"'\"'.filters select'\"'\"'); if (s.value!=='\"'\"''\"'\"') throw new Error('\"'\"'kept unsaved value'\"'\"'); return 1})()'"
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
title: "R2 in the kit (browser preview): with localStorage.setItem throwing, choosing 'Up to 7 days' shows 'Couldn't save the filter: disk full', the select goes back to Any, and results still show. Screenshot /tmp/qz9u-shots/filters-save-failed.png. The R1 load-failure path can't be forced in the preview (its loadFilters already catches storage errors), so the kit wiring for it was checked only by reading the code, plus the unit test."
updated_at: "2026-10-16T19:28:26.573726897+00:00"
---

## Summary

R2 in the kit (browser preview): with localStorage.setItem throwing, choosing 'Up to 7 days' shows 'Couldn't save the filter: disk full', the select goes back to Any, and results still show. Screenshot /tmp/qz9u-shots/filters-save-failed.png. The R1 load-failure path can't be forced in the preview (its loadFilters already catches storage errors), so the kit wiring for it was checked only by reading the code, plus the unit test.

## Command

```console
scripts/ui --url 'http://127.0.0.1:5281/#/materials' --wait Manufactured --click-text Manufactured --wait Pharmaceutical --click-text 'Pharmaceutical Isolators' --wait 'Data age' --eval '(()=>{Storage.prototype.setItem=()=>{throw new Error('"'"'disk full'"'"')}; const s=document.querySelector('"'"'.filters select'"'"'); s.value='"'"'7'"'"'; s.dispatchEvent(new Event('"'"'change'"'"',{bubbles:true})); return 1})()' --wait 'Couldn'"'"'t save the filter: disk full' --wait 'WISE 0855' --eval '(()=>{const s=document.querySelector('"'"'.filters select'"'"'); if (s.value!=='"'"''"'"') throw new Error('"'"'kept unsaved value'"'"'); return 1})()'
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

Bytes: 221
Truncated: no

```text
open http://127.0.0.1:5281/#/materials
wait "Manufactured"
click-text "Manufactured"
wait "Pharmaceutical"
click-text "Pharmaceutical Isolators"
wait "Data age"
wait "Couldn't save the filter: disk full"
wait "WISE 0855"
```

