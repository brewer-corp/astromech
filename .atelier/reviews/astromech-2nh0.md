---
schema: atelier.review
schema_version: 1
id: astromech-2nh0
title: Poll guard in 2f214f5 answers x81v. inFlight is cleared in finally on success and error; loadProjects is async so a sync throw cannot happen. A pick change is local state and unaffected by polling. Unmount leaves at most one late state write to a dead component, harmless in Svelte 5. Both new tests fail against the previous View.svelte and pass now. lint, check and test pass (91 tests).
target: astromech-k3wp
identity: review-agent (k3wp fix review 3, sonnet)
commit: d7d31b18da75f0ab0d8fa198a9d07eec802094bb
created_at: 2026-10-16T23:24:29.549600711Z
event:
  event: approval
---

Poll guard in 2f214f5 answers x81v. inFlight is cleared in finally on success and error; loadProjects is async so a sync throw cannot happen. A pick change is local state and unaffected by polling. Unmount leaves at most one late state write to a dead component, harmless in Svelte 5. Both new tests fail against the previous View.svelte and pass now. lint, check and test pass (91 tests).
