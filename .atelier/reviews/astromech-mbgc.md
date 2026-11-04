---
schema: atelier.review
schema_version: 1
id: astromech-mbgc
title: finding on astromech-54k4 @1e88a1bb
target: astromech-54k4
identity: review-agent-54k4-1
commit: 1e88a1bb5948024f1f778d1d98128a42be2be4de
created_at: 2026-11-04T21:48:03.255466854Z
event:
  event: finding
  recommendation: defer
---

Low: crates/astromech-intent/src/lib.rs:972 counts EVERYDAY against every command's margin (runner.max(everyday)). Its only effect is on speech that scores higher against a command than against any everyday question yet within 0.08 of one, so it turns command-closer speech from execute into ask, and no test exercises it (every everyday test case is caught by the 'everyday >= best' rule). Measured with a scratch probe comparing Grammar::new(MVP) to Grammar::mvp over 24,887 one-word deletions, insertions and substitutions of every unslotted MVP phrase: 30 decisions change, none to a wrong execute; 8 are Execute -> Ask, e.g. 'how much is this here worth', 'what is is in this system'. Hand probes: 'how much is my dater worth' (0.920), 'how much is this cistern worth' (0.905), 'how much is my unsold cargo worth' (0.867) were Execute and now Ask. All are read-only answers, so the cost is an extra yes, not a wrong action. Suggest dropping the margin term (keep only the NoMatch rule), or pinning the intended behaviour with a test if the margin effect is wanted.
