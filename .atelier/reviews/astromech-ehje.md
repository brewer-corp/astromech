---
schema: atelier.review
schema_version: 2
id: astromech-ehje
title: finding on astromech-y6d6 @ab14505e
target: astromech-y6d6
identity: claude-reviewer-wgtr
commit: ab14505e796bee36cf0670bb126f90c2319b831c
created_at: 2026-11-07T14:07:54.871433564Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: claude-orchestrator-kjfs
    commit: 9c32191da44fdd56e1a80c9aa6dd883a577ecfda
    recorded_at: 2026-11-07T14:49:33.216226171Z
    resolution_commit: 25eaa5cc9589872e0dc5e7336c8a06774ff291e0
    reason: Exe uploaded; fmub names it and the command
---

low: .atelier/issues/astromech-fmub.md (Done when 1) says 'the one-process workflow uploads it' (the spike exe). It doesn't: one-process.yml uploads only one-process-report/ and one-process-restart/, and run 42376155384's artifacts are reports only (no exe). Sumeet would look for an artifact that doesn't exist, or need Rust and MSVC on his PC. Fix: in .github/workflows/one-process.yml's probe job, copy spikes/o1-one-process/target/release/astromech-spike.exe into the uploaded folder (or upload it as its own artifact, e.g. one-process-spike-exe). Push so the workflow reruns, then name that run and artifact in fmub. The exe is self-contained (custom-protocol embeds dist/), and restart mode needs only it, Python 3 and psutil/pillow. Also spell out the full command in fmub: --app <exe> --endpoint %LOCALAPPDATA%\astromech-spike\core.json --out <dir> --restart --config 'Sumeet PC'.
