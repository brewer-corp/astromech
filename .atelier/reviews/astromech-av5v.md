---
schema: atelier.review
schema_version: 1
id: astromech-av5v
title: 'R1 [low] crates/astromech-engineering/src/links.rs:62 is_openable_url: dot segments escape the path prefix. ''https://github.com/brewer-corp/astromech/../../evil/x'', the %2e%2e form and the backslash form all pass the predicate and a WHATWG browser normalises them to https://github.com/evil/x (checked with node''s URL); same for inara.cz/elite/../ and the others. The host cannot change and no explorer.exe switch can be injected (spaces, quotes, commas refused), so the impact is a compromised webview opening any page on github.com/inara.cz/spansh.co.uk. Pre-existing pattern for the other prefixes; 6yt1''s Done when is met. Fix later: also refuse ''..'' (and %2e, case-insensitive) segments and ''\\'', or compare a normalised path.'
target: astromech-0k88
identity: review-agent-0k88-1
commit: adf99099b228d9792fe2e97462a5f16a3beda99c
created_at: 2026-10-22T21:50:07.232639436Z
event:
  event: finding
  recommendation: defer
---

R1 [low] crates/astromech-engineering/src/links.rs:62 is_openable_url: dot segments escape the path prefix. 'https://github.com/brewer-corp/astromech/../../evil/x', the %2e%2e form and the backslash form all pass the predicate and a WHATWG browser normalises them to https://github.com/evil/x (checked with node's URL); same for inara.cz/elite/../ and the others. The host cannot change and no explorer.exe switch can be injected (spaces, quotes, commas refused), so the impact is a compromised webview opening any page on github.com/inara.cz/spansh.co.uk. Pre-existing pattern for the other prefixes; 6yt1's Done when is met. Fix later: also refuse '..' (and %2e, case-insensitive) segments and '\\', or compare a normalised path.
