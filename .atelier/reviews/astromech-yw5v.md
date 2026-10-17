---
schema: atelier.review
schema_version: 1
id: astromech-yw5v
title: 'R1 [high] crates/astromech-eddn/src/edsm/api.rs:47,104: the EDSM API key leaks into the UI. flight_log sends apiKey as a GET query parameter, and HttpApi::answer turns a ureq transport error into t.to_string(); ureq 2.12.1''s Transport Display prefixes the full request URL, query included. Reproduced with ureq 2.12.1: a refused connection yields ''http://127.0.0.1:9/get-logs?commanderName=J&apiKey=SECRETKEY: Connection Failed: ...''. On any DNS/connect/TLS/timeout failure, edsm_flight_log returns that string, and Edsm.svelte shows it in the Flight log row (logError), which breaks ''never shown again'' and leaks the key in screenshots and stream captures. Fix: build transport errors from t.kind() (and message) without the URL, or strip the query; add a test that a transport failure of flight_log never contains the key.'
target: astromech-ah0i
identity: review-agent (opus, independent)
commit: aa58b47beb073e878c106e16878529bca17d43f1
created_at: 2026-10-17T05:04:00.267234256Z
event:
  event: finding
  recommendation: fix_now
---

R1 [high] crates/astromech-eddn/src/edsm/api.rs:47,104: the EDSM API key leaks into the UI. flight_log sends apiKey as a GET query parameter, and HttpApi::answer turns a ureq transport error into t.to_string(); ureq 2.12.1's Transport Display prefixes the full request URL, query included. Reproduced with ureq 2.12.1: a refused connection yields 'http://127.0.0.1:9/get-logs?commanderName=J&apiKey=SECRETKEY: Connection Failed: ...'. On any DNS/connect/TLS/timeout failure, edsm_flight_log returns that string, and Edsm.svelte shows it in the Flight log row (logError), which breaks 'never shown again' and leaks the key in screenshots and stream captures. Fix: build transport errors from t.kind() (and message) without the URL, or strip the query; add a test that a transport failure of flight_log never contains the key.
