---
schema: atelier.review
schema_version: 2
id: astromech-aqe3
title: review on astromech-vucf @4744ece0
target: astromech-vucf
identity: claude-reviewer-1hh9
commit: 4744ece07c9a33b4183c5178bf79bbd8e9b5c504
created_at: 2026-11-07T18:48:26.767081131Z
event:
  event: review
---

Review of astromech-1hh9 (06c33b0c, 4744ece0) against its Done when, notes, 7 decision notes and 4 deviations, decisions e1a5, 1zhg and 6i8s, and the strategy's Shape and Authority.

Equivalence on the installed app (SystemClock, Io::real) was checked line by line against git show main: and the base 06c33b0c. Main's start_voice, bus and Session::drop are byte-identical to the base, so the diff is the only change.
- start_voice: bus::attach goes through BusSource, the same receiver, with recv_timeout(1s) through ControlRx. start_named goes through DeviceMics::open. Every Instant::now becomes clk.now, which is Instant::now. The post-roll is clk.sleep, which is thread::sleep. Builder::spawn becomes clock.spawn, which is spawn plus an empty registration. wait_after stays on real time. The STT thread, the paste thread::spawn and the kill and re-arm logic are unchanged. impl Mic for Recorder calls inherent methods, which all exist (voice mic.rs:96-251), so nothing recurses.
- Session::drop: stop, then BusSource::detach, which is bus::detach.
- start_buttons: Some(BusSource) on Windows, and the same log line and drop elsewhere.
- Session::start: worker, stt and tts are the same checks. TtsSpeaker and WorkerStt take the same arguments. real_platform is the stateless WinPlatform.
- Mapper::new is with_now(Instant::now), and saturating_duration_since equals elapsed.
- app.rs: preview, mic_list, calibrate_record and mic_check are the same calls, and with_devices is enumerate_live, the default.
- main.rs console mode passes Io::real.
- The bus module is unchanged apart from the cfg(test) forward.
- The off-Windows binds inventory change affects dev runs only.

Proof rerun fresh: cargo test -p astromech-core (all pass, lib 247), clippy -D warnings, fmt --check, Windows-target clippy with the fake tools (clean), world_voice plus world_steps 20 times in a row (20/20), and world_voice 8 at once (8/8). The kill-switch slice mutant was driven over the control protocol: 24 ms got no reply in 3 of 3 runs, and 25 ms replied in 3 of 3. The WAV was re-rendered with render-voices (bm_george, 'Gear down.') and ffmpeg to 16 kHz: the same length and a waveform correlation of 0.91, so it is synthetic Kokoro (Apache-2.0, NOTICE.md:27).

Limits:
- Not run: the Windows test push_to_talk_and_the_kill_switch_through_the_real_bus, any Windows runtime and the in-game check. Done when 2 and 6 stay unticked until Windows CI runs them, and that is required before merge.
- I read its fixture order by hand: PttUp at 1500 ms and KillDown at 2000 ms, past the 150 ms debounce. Its gear confirmation is said before the verify (actions.rs:240), so a hub with no pipeline doesn't block it.
