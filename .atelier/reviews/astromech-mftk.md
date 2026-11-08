---
schema: atelier.review
schema_version: 2
id: astromech-mftk
title: approval on astromech-vucf @6b4fed34
target: astromech-vucf
identity: claude-reviewer-jkgv
commit: 6b4fed3429bdc122275101c6e4bd89a397c6a183
created_at: 2026-11-07T21:12:46.573782154Z
event:
  event: approval
---

Re-review of jkgv fixes 71c4d4f8, 3784aa01, 2e79bea3, 6b4fed34. igbw fixed: bridge.ts refusal() returns 403 unless the Host is loopback with the server's port, the content-type is application/json, and Sec-Fetch-Site is same-origin or the Origin is the server's own. Live on worlds/preview/history: the original text/plain exploit, JSON from a foreign origin, Origin null, no Origin and no Sec-Fetch-Site, Sec-Fetch-Site same-site or none, another localhost port's origin, a rebound Host, /call/x and an uppercase path are all refused. Real Chrome on another origin (127.0.0.1:5393) got opaque or 403 for no-cors and CORS fetches to both localhost and 127.0.0.1 with set_folders and binds_copy, and no file was written. The page's own calls still work, and bridge.test.ts (6) and bridge-check's 403 case pass. nk8x fixed: exobiology is dumped from worlds/preview/exobiology except /known, and the Report now says 24 whole and 3 partial. ptge fixed: uncovered() walks every level, and its test catches added siblings, array items and missing parts. v0xh fixed: nearby-hge is dumped from a recorded Federation search. The sol-nov pages have whole rows (36-54 fields, Spansh's search, search_reference and count echoes), only trailing rows dropped, provenance in astromech.toml, and no keys, commander or personal data (the EDSM 503 recorded as {}). 1.6 MB of JSON, about 185 KB gzipped, is acceptable next to the repo's larger PNGs and catalogs. Proof: preview_fixtures 2/2, fmt and clippy -D warnings clean, pnpm check 0 errors, lint clean, pnpm test 544 passed, pnpm build plus check-dist clean, bridge-check passes, and no world core or home was left behind.
