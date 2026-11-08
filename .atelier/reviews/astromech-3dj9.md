---
schema: atelier.review
schema_version: 2
id: astromech-3dj9
title: finding on astromech-vucf @86b4d3e8
target: astromech-vucf
identity: claude-reviewer-xk0n
commit: 86b4d3e81bb36e6f278617b25d59ae0d32c6e4b9
created_at: 2026-11-07T17:12:57.019908323Z
event:
  event: finding
  recommendation: fix_now
---

medium: crates/astromech-data/src/download.rs:38, crates/astromech-core/src/update.rs:267/283, crates/astromech-voice/src/download.rs:134 (with world/net.rs:107 and replay.rs:463). xk0n Done when 2 and its Report ('none reaches the internet') fail when a proxy variable is set: the three download agents call try_proxy_from_env(true), and ureq 2.12.1 reads ALL_PROXY/HTTPS_PROXY/HTTP_PROXY (either case) and ignores NO_PROXY, so the rewritten http://127.0.0.1:<port>/github.com/... URL is sent to the proxy, off the host, instead of the replay server. Reviewer run, inside bwrap --unshare-net so nothing could leave: HTTPS_PROXY=HTTP_PROXY=http://192.0.2.1:3128 cargo test -p astromech-core --test world_net -> downloads_and_the_flight_log_are_answered_from_the_recording fails after 30 s, 'nothing matching ...latest.json was observed (0 observed)'; the same test passes without the variables. Impact: on any machine or CI runner with a proxy configured, world-mode update/model/neutron requests leave the machine carrying the real host and path, and the world tests fail slowly. The in-process paths (remote client, EDDN, EDSM) are unaffected. Fix: in world mode make the download agents skip the proxy (e.g. the Http boundary hands the stacks a no-proxy flag), or have world::run refuse to start while any of the six proxy variables is set and have WorldCore env_remove them; add a world test that sets one and still sees the request answered.
