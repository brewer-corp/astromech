---
created_at: "2026-10-17T05:14:57.280037456+00:00"
id: "astromech-1bc7"
evidence_type: "test"
commit: "b20834cbc571f5945c2e3acbce37fca031a9c0f1"
result: "pass"
captured_at: "2026-10-17T05:14:57.279981782+00:00"
agent_identity: "implementer (Claude Opus)"
target:
  kind: "issue"
  id: "astromech-a63r"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-a63r"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "At b20834c: cargo test -p astromech-announce --test biology (5 passed): arrival into Luhman 16 with the recorded Spansh answer (astromech-data fixture spansh-bodies-biology.json) says 'Known biology on 2 bodies.' 'B 3: Fungoida Bullarum, 3.7 M.' before any scan; a late answer is said on the tick within a minute and once, not after a minute or after the FSS reported biology; leaving the FSS gives 'Biology on 1 body.' + best body 'up to' line, nothing when unchanged, 'Biology on 2 bodies.' after another FSS result; after the DSS the body's line from its genus list. Core unit tests (astromech-core biology, 3 passed): recorded Spansh answer -> known bodies (value 8,859,500 = Spansh landmark_value), background lookup, page view. astromech-data recorded test checks the request shape (system_id64 + signals Biological, verified live 17 Oct 2026). replay.rs catch-up covers biology and sampling (silent in catch-up, both speak live); copy.rs checks every biology line (all genera/species, long body names) against the 8-word budget. UI: vitest exobiology page (4 passed); scripts/ui headless Chrome on pnpm dev ?core=ready#/exobiology rendered the page (screenshot checked). Full gates /tmp/landq/gates.sh exit 0 (fmt, clippy -D warnings, cargo test --workspace 99 suites, check-deps, shell cargo metadata --locked, pnpm lint/check/build/test 170 tests)."
updated_at: "2026-10-17T05:14:57.280352835+00:00"
---

At b20834c: cargo test -p astromech-announce --test biology (5 passed): arrival into Luhman 16 with the recorded Spansh answer (astromech-data fixture spansh-bodies-biology.json) says 'Known biology on 2 bodies.' 'B 3: Fungoida Bullarum, 3.7 M.' before any scan; a late answer is said on the tick within a minute and once, not after a minute or after the FSS reported biology; leaving the FSS gives 'Biology on 1 body.' + best body 'up to' line, nothing when unchanged, 'Biology on 2 bodies.' after another FSS result; after the DSS the body's line from its genus list. Core unit tests (astromech-core biology, 3 passed): recorded Spansh answer -> known bodies (value 8,859,500 = Spansh landmark_value), background lookup, page view. astromech-data recorded test checks the request shape (system_id64 + signals Biological, verified live 17 Oct 2026). replay.rs catch-up covers biology and sampling (silent in catch-up, both speak live); copy.rs checks every biology line (all genera/species, long body names) against the 8-word budget. UI: vitest exobiology page (4 passed); scripts/ui headless Chrome on pnpm dev ?core=ready#/exobiology rendered the page (screenshot checked). Full gates /tmp/landq/gates.sh exit 0 (fmt, clippy -D warnings, cargo test --workspace 99 suites, check-deps, shell cargo metadata --locked, pnpm lint/check/build/test 170 tests).
