---
created_at: "2026-10-16T19:18:42.528058511+00:00"
id: "astromech-fw73"
evidence_type: "validation"
commit: "1e28693cf90399521cb61803562b30d2c5ce6555"
result: "pass"
captured_at: "2026-10-16T19:18:40.904043568+00:00"
command: "scripts/ui --url 'http://127.0.0.1:5281/#/materials' --wait Manufactured --click-text Manufactured --wait Pharmaceutical --click-text 'Pharmaceutical Isolators' --wait 'Data age' --eval '(()=>{const s=document.querySelector('\"'\"'.filters select'\"'\"'); s.value='\"'\"'7'\"'\"'; s.dispatchEvent(new Event('\"'\"'change'\"'\"',{bubbles:true})); return 1})()' --wait 'Nearest to Sol' --eval 'location.reload(); 1' --wait Manufactured --click-text Manufactured --wait Pharmaceutical --click-text 'Pharmaceutical Isolators' --wait 'Up to 7 days' --eval '(()=>{const s=document.querySelector('\"'\"'.filters select'\"'\"'); if (s.options[s.selectedIndex].text !== '\"'\"'Up to 7 days'\"'\"') throw new Error('\"'\"'not restored'\"'\"'); return 1})()' --click-text 'Military Grade Alloys' --wait 'Faction in War' --eval '(()=>{const s=document.querySelector('\"'\"'.filters select'\"'\"'); if (s.value !== '\"'\"''\"'\"') throw new Error('\"'\"'leaked to another lookup'\"'\"'); return 1})()'"
exit_status: "0"
agent_identity: "claude-opus-5.5"
target:
  kind: "issue"
  id: "astromech-1imw"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-1imw"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "UI check (browser fallback, localStorage stands in for astromech-store): set Data age to 7 days on the Pharmaceutical Isolators lookup, reload the page, the select shows 'Up to 7 days'; Military Grade Alloys (another lookup) still shows Any. Screenshot /tmp/qz9u-shots/filters-restored.png. A temporary all-filters render (reverted, not committed) showed Pad 'Ship's (M)', and setting L survived a reload with pad_set=true (/tmp/qz9u-shots/filters-all-temp.png). Limit: the real app restart through the shell commands (search_filters/set_search_filters on astromech.sqlite) is not compiled or run on Linux."
updated_at: "2026-10-16T19:18:42.528568943+00:00"
---

## Summary

UI check (browser fallback, localStorage stands in for astromech-store): set Data age to 7 days on the Pharmaceutical Isolators lookup, reload the page, the select shows 'Up to 7 days'; Military Grade Alloys (another lookup) still shows Any. Screenshot /tmp/qz9u-shots/filters-restored.png. A temporary all-filters render (reverted, not committed) showed Pad 'Ship's (M)', and setting L survived a reload with pad_set=true (/tmp/qz9u-shots/filters-all-temp.png). Limit: the real app restart through the shell commands (search_filters/set_search_filters on astromech.sqlite) is not compiled or run on Linux.

## Command

```console
scripts/ui --url 'http://127.0.0.1:5281/#/materials' --wait Manufactured --click-text Manufactured --wait Pharmaceutical --click-text 'Pharmaceutical Isolators' --wait 'Data age' --eval '(()=>{const s=document.querySelector('"'"'.filters select'"'"'); s.value='"'"'7'"'"'; s.dispatchEvent(new Event('"'"'change'"'"',{bubbles:true})); return 1})()' --wait 'Nearest to Sol' --eval 'location.reload(); 1' --wait Manufactured --click-text Manufactured --wait Pharmaceutical --click-text 'Pharmaceutical Isolators' --wait 'Up to 7 days' --eval '(()=>{const s=document.querySelector('"'"'.filters select'"'"'); if (s.options[s.selectedIndex].text !== '"'"'Up to 7 days'"'"') throw new Error('"'"'not restored'"'"'); return 1})()' --click-text 'Military Grade Alloys' --wait 'Faction in War' --eval '(()=>{const s=document.querySelector('"'"'.filters select'"'"'); if (s.value !== '"'"''"'"') throw new Error('"'"'leaked to another lookup'"'"'); return 1})()'
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

Bytes: 366
Truncated: no

```text
open http://127.0.0.1:5281/#/materials
wait "Manufactured"
click-text "Manufactured"
wait "Pharmaceutical"
click-text "Pharmaceutical Isolators"
wait "Data age"
wait "Nearest to Sol"
wait "Manufactured"
click-text "Manufactured"
wait "Pharmaceutical"
click-text "Pharmaceutical Isolators"
wait "Up to 7 days"
click-text "Military Grade Alloys"
wait "Faction in War"
```

