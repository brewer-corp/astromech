---
schema: atelier.review
schema_version: 1
id: astromech-4k88
title: 'Design nit (low): astromech-odyssey, a pure library crate, now logs through a process-global sink (astromech_log::log), so its messages depend on whichever process initialised logging; before init they go to stdout. astromech-data instead takes an injected log callback. Acceptable and rule-conforming, but an injected callback would keep libraries side-effect free. Other eprintln! in the core (speech.rs, secrets.rs keyring, mixer.rs) still miss the file; out of scope here.'
target: astromech-4kw5
identity: review-agent (sonnet, independent)
commit: 0d5a990db3565889bd0d823992046eff75dabc28
created_at: 2026-10-17T06:35:26.443996555Z
event:
  event: finding
  recommendation: defer
---

Design nit (low): astromech-odyssey, a pure library crate, now logs through a process-global sink (astromech_log::log), so its messages depend on whichever process initialised logging; before init they go to stdout. astromech-data instead takes an injected log callback. Acceptable and rule-conforming, but an injected callback would keep libraries side-effect free. Other eprintln! in the core (speech.rs, secrets.rs keyring, mixer.rs) still miss the file; out of scope here.
