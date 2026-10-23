---
schema: atelier.review
schema_version: 1
id: astromech-qdwv
title: 'Independent review (review-agent-1842-1) of astromech-nk0u and astromech-d4of at 059d7838: script, workflow, reader, download client, Tauri shell commands (read only, not compiled on Linux), Route page component, docs, NOTICE, lockfiles, evidence. Re-ran: test_gen_neutron.py (2 ok), cargo test neutron + neutron_download (7 ok), vitest NeutronFile.test.ts (4 ok), cargo metadata --locked on app/src-tauri (lock consistent). Ran gen-neutron.py on the real dump: 5,045,154 stars, 1,645,100 cells, 184.0 MB, 71.1 MB gzip -9, 77 s, 4.4 GB peak RSS. Probed the Rust reader on that file: opens in 130 ms; names at 0/255/256/257/1,000,000/last match the dump''s coordinates; 500 ly neighbour scans take 34 us (Sol), 74 us (Sgr A*, 625 hits), 349 us (Colonia, 27,012 hits), so the 128 ly cell layout serves the plotter''s query pattern. Workflow is dispatch-only, pre-release, --latest=false on create, 100 MB size check. Download: checksum verified before rename, .part removed on every error path, file handle closed before rename. Findings recorded separately; not approved (two fix_now).'
target: astromech-1842
identity: review-agent-1842-1
commit: 059d78381235b475652ae78edab486764504ddb4
created_at: 2026-10-22T21:10:50.897711965Z
event:
  event: review
---

Independent review (review-agent-1842-1) of astromech-nk0u and astromech-d4of at 059d7838: script, workflow, reader, download client, Tauri shell commands (read only, not compiled on Linux), Route page component, docs, NOTICE, lockfiles, evidence. Re-ran: test_gen_neutron.py (2 ok), cargo test neutron + neutron_download (7 ok), vitest NeutronFile.test.ts (4 ok), cargo metadata --locked on app/src-tauri (lock consistent). Ran gen-neutron.py on the real dump: 5,045,154 stars, 1,645,100 cells, 184.0 MB, 71.1 MB gzip -9, 77 s, 4.4 GB peak RSS. Probed the Rust reader on that file: opens in 130 ms; names at 0/255/256/257/1,000,000/last match the dump's coordinates; 500 ly neighbour scans take 34 us (Sol), 74 us (Sgr A*, 625 hits), 349 us (Colonia, 27,012 hits), so the 128 ly cell layout serves the plotter's query pattern. Workflow is dispatch-only, pre-release, --latest=false on create, 100 MB size check. Download: checksum verified before rename, .part removed on every error path, file handle closed before rename. Findings recorded separately; not approved (two fix_now).
