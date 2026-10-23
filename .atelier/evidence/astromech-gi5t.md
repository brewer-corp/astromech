---
created_at: "2026-10-23T00:00:58.821928917+00:00"
id: "astromech-gi5t"
evidence_type: "test"
commit: "7dcf837243bd1833275664022f894d96439f5324"
result: "pass"
captured_at: "2026-10-23T00:00:53.625063440+00:00"
command: "/home/supadava/.cache/dlrb-scratch/rj.sh"
exit_status: "0"
agent_identity: "claude-implementer-dlrb"
target:
  kind: "issue"
  id: "astromech-dlrb"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-dlrb"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Real-journal harness at 7dcf8372 on a copy of ~/edjournal (copy deleted after): the EDDN dry run lists outfitting-v2.0.json 1 (only the newest Outfitting.json is on disk, Jameson Memorial, 1084 items) validated against the live schema, no 'outfitting/2 is off' skips (22 Oct: 111), 0 problems. Script (outside the repo) prints only the counts."
updated_at: "2026-10-23T00:00:58.822241940+00:00"
---

## Summary

Real-journal harness at 7dcf8372 on a copy of ~/edjournal (copy deleted after): the EDDN dry run lists outfitting-v2.0.json 1 (only the newest Outfitting.json is on disk, Jameson Memorial, 1084 items) validated against the live schema, no 'outfitting/2 is off' skips (22 Oct: 111), 0 problems. Script (outside the repo) prints only the counts.

## Command

```console
/home/supadava/.cache/dlrb-scratch/rj.sh
```

Exit status: 0

## Stdout

Bytes: 818
Truncated: no

```text
## EDDN dry run (nothing sent) (1.22 s)
  2949 messages would be sent (4 from companion files): journal-v1.0.json 2119, fsssignaldiscovered-v1.0.json 414, dockinggranted-v1.0.json 154, scanbarycentre-v1.0.json 68, fssallbodiesfound-v1.0.json 46, fssdiscoveryscan-v1.0.json 41, approachsettlement-v1.0.json 27, dockingdenied-v1.0.json 27, navbeaconscan-v1.0.json 25, fssbodysignals-v1.0.json 24, codexentry-v1.0.json 1, commodity-v3.0.json 1, outfitting-v2.0.json 1, shipyard-v2.0.json 1
  3 skipped or dropped: approachsettlement-v1.0.json (no location to cross-check the system against) 2, navroute-v1.0.json (companion file doesn't match its event) 1
==== 0 problem(s), 0 known and filed ====
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.75s
outfitting/2-is-off skips: 0
```

## Stderr

Bytes: 0
Truncated: no

```text
```

