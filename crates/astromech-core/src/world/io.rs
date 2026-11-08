//! The world's session devices ([`crate::session::Io`]): the platform, the buttons, the
//! microphone, speech-to-text and the speaker. Each records what reaches it in the world's
//! observations; none reaches a real device, window, worker or player.
//!
//! - The platform: the foreground window is the header's `foreground`, then each `focus` step's.
//!   Every key sent is observed (`key`); a key going down that a header reaction names changes
//!   Status.json (a flag toggled, GuiFocus set, or the ship and SRV swapped, as the mock game
//!   does), written whole and settled like a `file` step before the key send returns.
//! - The buttons: `press` and `release` steps' samples go through a [`Mapper`] on the world's
//!   clock, on a registered thread that ticks it as the Raw Input listener's thread does, to the
//!   session attached. Like the bus, it starts with the first session's buttons and stays.
//! - The microphone: one device, [`MIC_NAME`] at [`MIC_RATE`] Hz, whose stream is the `audio`
//!   steps' clips one after another. A recording (calibration, the level check) is the audio
//!   since the last one.
//! - Speech-to-text answers each clip with the next `say` step's transcript; the speaker's
//!   lines are observed as said (`said`), and it speaks aloud.

use super::script::{Effect, Input, Reaction};
use super::{Observed, Settle, WAIT_GUARD};
use crate::calibration::Clip;
use crate::config::Config;
use crate::controls::{Control, ControlRx, Mapper};
use crate::logging::log;
use crate::mic::{Mic, Mics, StreamErrors};
use crate::speech::{Speaker, Speech, Stt};
use anyhow::{anyhow, bail, Result};
use astromech_api::Recover;
use astromech_input::exec::keys::ScanKey;
use astromech_input::exec::platform::{GameIntegrity, IntegrityReport, Platform};
use astromech_input::listen::model::{DeviceMeta, Sample};
use astromech_proto::{Clock, ClockSender, ManualClock};
use serde_json::{json, Value};
use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// The world's one recording device.
pub const MIC_NAME: &str = "World microphone";
/// Its sample rate: an `audio` step's WAV must be recorded or converted at it.
pub const MIC_RATE: u32 = 16_000;

/// The world's devices, shared by its fakes and its steps.
pub struct Io {
    observed: Arc<Observed>,
    clock: Arc<ManualClock>,
    /// The clock's now at the world's start: samples and the Mapper's ticks are ms since it.
    t0: Instant,
    game_dir: PathBuf,
    settle: Arc<Settle>,
    reactions: Vec<Reaction>,
    devices: Vec<DeviceMeta>,
    foreground: Mutex<Option<String>>,
    /// The buttons' thread, once the first session has attached.
    samples: Mutex<Option<ClockSender<Sample>>>,
    /// The attached session's presses.
    current: Arc<Mutex<Option<ClockSender<Control>>>>,
    /// The microphone's stream and how far a recording has read it.
    mic: Arc<Mutex<(Vec<f32>, usize)>>,
    transcripts: Mutex<VecDeque<String>>,
    /// Held while a reaction reads and rewrites Status.json.
    reacting: Mutex<()>,
}

impl Io {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        observed: Arc<Observed>,
        clock: Arc<ManualClock>,
        game_dir: PathBuf,
        settle: Arc<Settle>,
        reactions: Vec<Reaction>,
        devices: Vec<DeviceMeta>,
        foreground: Option<String>,
    ) -> Arc<Self> {
        let t0 = clock.now();
        Arc::new(Self {
            observed,
            clock,
            t0,
            game_dir,
            settle,
            reactions,
            devices,
            foreground: Mutex::new(foreground),
            samples: Mutex::new(None),
            current: Arc::default(),
            mic: Arc::default(),
            transcripts: Mutex::default(),
            reacting: Mutex::new(()),
        })
    }

    /// The session's devices in this world.
    pub fn session(self: &Arc<Self>) -> crate::session::Io {
        let devices = inventory(&self.devices);
        crate::session::Io {
            platform: Arc::new(WorldPlatform(self.clone())),
            controls: Some(Arc::new(WorldControls(self.clone()))),
            mic: Arc::new(WorldMics(self.mic.clone())),
            speech: Arc::new(WorldSpeech(self.clone())),
            devices: Arc::new(move || devices.clone()),
        }
    }

    /// A `focus` step: the window in front from now on.
    pub fn focus(&self, title: Option<String>) {
        *self.foreground.lock().recover() = title;
    }

    /// A `press` or `release` step: the sample, stamped now, to the buttons' thread.
    pub fn input(&self, i: &Input) -> Result<(), String> {
        let tx = self.samples.lock().recover().clone().ok_or(
            "nothing listens to the buttons yet: a session starts the listener (a `game` step before the first \
             `press`)",
        )?;
        let fg = i.fg.clone().or_else(|| self.foreground.lock().recover().clone()).unwrap_or_default();
        let s = Sample {
            t_ms: ms(self.t0, self.clock.now()),
            dev: i.dev.clone(),
            fg,
            injected: i.injected,
            data: i.data.clone(),
        };
        tx.send(s).map_err(|_| "the buttons' thread has stopped".to_string())
    }

    /// An `audio` step: the clip, on the microphone's stream.
    pub fn audio(&self, samples: &[f32]) {
        self.mic.lock().recover().0.extend_from_slice(samples);
    }

    /// A `say` step: what speech-to-text hears next.
    pub fn say(&self, text: &str) {
        self.transcripts.lock().recover().push_back(text.to_string());
    }

    /// A key going down that a reaction names: Status.json changed as the game would, written
    /// whole and handled by the pipeline before this returns.
    fn react(&self, key: ScanKey) -> Result<(), String> {
        let rules: Vec<&Reaction> =
            self.reactions.iter().filter(|r| r.scan == key.scan && r.ext == key.extended).collect();
        if rules.is_empty() {
            return Ok(());
        }
        let _one = self.reacting.lock().recover();
        let path = self.game_dir.join("Status.json");
        let mut status = match std::fs::read(&path) {
            Ok(b) => serde_json::from_slice::<Value>(&b).map_err(|e| format!("Status.json isn't JSON: {e}"))?,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => json!({"event": "Status", "Flags": 0}),
            Err(e) => return Err(format!("reading Status.json: {e}")),
        };
        let o = status.as_object_mut().ok_or("Status.json isn't a JSON object")?;
        let mut flags = o.get("Flags").and_then(Value::as_u64).unwrap_or(0);
        let mut gui = o.get("GuiFocus").and_then(Value::as_u64).unwrap_or(0);
        for r in rules {
            match r.effect {
                Effect::Flag(bit) => flags ^= bit,
                Effect::Gui(g) => gui = g,
                Effect::Vehicle => {
                    flags ^= astromech_status::FLAG_IN_MAIN_SHIP | astromech_status::FLAG_IN_SRV;
                    gui = 0;
                }
            }
        }
        let utc: chrono::DateTime<chrono::Utc> = self.clock.utc().into();
        o.insert("timestamp".into(), json!(utc.format("%Y-%m-%dT%H:%M:%SZ").to_string()));
        o.insert("Flags".into(), json!(flags));
        o.insert("GuiFocus".into(), json!(gui));
        let body = serde_json::to_vec(&status).map_err(|e| e.to_string())?;
        let gen = self
            .settle
            .files
            .write("Status.json", || super::steps::write_whole(&self.game_dir, "Status.json", &body))
            .map_err(|e| format!("writing Status.json: {e}"))?;
        self.settle.files.wait("Status.json", gen, WAIT_GUARD)
    }
}

/// `t` as ms since `t0`: from whole nanoseconds, so the world's round advances give exact ms and
/// a debounce of exactly `debounce_ms` ends on the advance that reaches it.
fn ms(t0: Instant, t: Instant) -> f64 {
    t.saturating_duration_since(t0).as_nanos() as f64 / 1e6
}

/// The header's devices as the binds check sees connected controllers.
fn inventory(devices: &[DeviceMeta]) -> astromech_keybinds::devices::Inventory {
    use astromech_keybinds::devices::{kind_name, Controller, Inventory};
    let controllers = devices
        .iter()
        .filter_map(|d| {
            Some(Controller {
                vid: format!("{:04X}", d.vid),
                pid: format!("{:04X}", d.pid),
                elite_id: d.elite_id()?,
                product: d.product.clone(),
                manufacturer: String::new(),
                usage: d.usage,
                kind: kind_name(d.usage).into(),
                buttons: d.buttons_declared,
                axes: d.axes_declared.clone(),
                hats: d.hats_declared,
                other_values: vec![],
            })
        })
        .collect();
    Inventory { source: "world".into(), controllers, ..Inventory::default() }
}

/// The world's platform: its foreground window, and keys observed and reacted to.
struct WorldPlatform(Arc<Io>);

impl Platform for WorldPlatform {
    fn foreground_title(&self) -> Option<String> {
        self.0.foreground.lock().recover().clone()
    }

    fn send_key(&self, k: ScanKey, up: bool) -> Result<(), String> {
        let name = astromech_input::exec::keys::table().find(|(_, s)| *s == k).map(|(n, _)| n);
        self.0.observed.record("key", json!({ "key": name, "scan": k.scan, "extended": k.extended, "up": up }));
        if !up {
            // The key reached the game; a reaction that can't be applied is the world's fault.
            if let Err(e) = self.0.react(k) {
                log(&format!("world mode: the reaction to {} failed: {e}", name.unwrap_or("a key")));
            }
        }
        Ok(())
    }

    fn integrity(&self) -> IntegrityReport {
        IntegrityReport { own: Some(0x2000), game: GameIntegrity::Level(0x2000) }
    }
}

/// The world's buttons ([`crate::controls::Source`]).
struct WorldControls(Arc<Io>);

impl crate::controls::Source for WorldControls {
    fn attach(&self, cfg: &Config, debug: bool) -> (Box<dyn ControlRx>, String, Vec<DeviceMeta>) {
        let io = &self.0;
        let clock: Arc<dyn Clock> = io.clock.clone();
        let mut samples = io.samples.lock().recover();
        if samples.is_none() {
            let c = io.clock.clone();
            let mut m = Mapper::with_now(
                cfg.controls.ptt.clone(),
                cfg.controls.kill_switch.clone(),
                debug,
                crate::logging::log,
                move || c.now(),
            );
            m.paste = cfg.controls.paste.clone();
            m.debounce_ms = cfg.controls.debounce_ms as f64;
            for d in &io.devices {
                m.add_device(d.clone());
            }
            let (tx, rx) = clock.channel::<Sample>();
            let (c, t0, current) = (clock.clone(), io.t0, io.current.clone());
            // Registered from the session's starter, like the push-to-talk loop: it ticks on the
            // world's clock.
            let r = clock.spawn(std::thread::Builder::new().name("world-controls".into()), move || loop {
                let wait = Duration::from_secs_f64(m.wait_ms(ms(t0, c.now())) / 1000.0);
                let cs = match rx.recv_timeout(wait) {
                    Ok(s) => m.sample(&s),
                    Err(std::sync::mpsc::RecvTimeoutError::Timeout) => m.tick(ms(t0, c.now())),
                    Err(_) => break,
                };
                if let Some(tx) = current.lock().recover().as_ref() {
                    for control in cs {
                        let _ = tx.send(control);
                    }
                }
            });
            match r {
                Ok(_) => *samples = Some(tx),
                Err(e) => log(&format!("world mode: couldn't start the buttons' thread: {e}")),
            }
        }
        let (tx, rx) = clock.channel::<Control>();
        *io.current.lock().recover() = Some(tx);
        (Box::new(rx), "world buttons (press and release steps)".into(), io.devices.clone())
    }

    fn detach(&self) {
        if let Some(tx) = self.0.current.lock().recover().take() {
            tx.wake();
        }
    }
}

/// The world's microphones: the one device.
struct WorldMics(Arc<Mutex<(Vec<f32>, usize)>>);

impl Mics for WorldMics {
    fn open(&self, _want: Option<&str>) -> Result<Box<dyn Mic>> {
        Ok(Box::new(WorldMic(self.0.clone())))
    }

    fn record(&self, _want: &str, _ms: u64) -> Result<Clip> {
        let mut m = self.0.lock().recover();
        let (stream, read) = &mut *m;
        if *read >= stream.len() {
            bail!("no audio from {MIC_NAME}: an `audio` step gives it a clip");
        }
        let samples = stream[*read..].to_vec();
        *read = stream.len();
        Ok(Clip { device: MIC_NAME.into(), rate: MIC_RATE, samples })
    }

    fn names(&self) -> (Vec<String>, Option<String>) {
        (vec![MIC_NAME.into()], Some(MIC_NAME.into()))
    }
}

/// The world's microphone, opened.
struct WorldMic(Arc<Mutex<(Vec<f32>, usize)>>);

impl Mic for WorldMic {
    fn device_name(&self) -> String {
        MIC_NAME.into()
    }
    fn sample_rate(&self) -> u32 {
        MIC_RATE
    }
    fn default_name(&self) -> Option<String> {
        Some(MIC_NAME.into())
    }
    fn position(&self) -> u64 {
        self.0.lock().recover().0.len() as u64
    }
    fn take(&self, from: u64, to: u64) -> Vec<f32> {
        let m = self.0.lock().recover();
        let to = (to as usize).min(m.0.len());
        m.0.get(from as usize..to).map(<[f32]>::to_vec).unwrap_or_default()
    }
    /// At once: an `audio` step has put the clip on the stream already.
    fn wait_after(&self, _t: Instant, _max: Duration) {}
    fn take_errors(&self) -> Option<StreamErrors> {
        None
    }
    fn recover(&mut self) -> Result<bool> {
        Ok(false)
    }
}

/// The world's speech worker: always there, hearing the script's transcripts and saying lines
/// into the observations.
struct WorldSpeech(Arc<Io>);

impl Speech for WorldSpeech {
    fn worker_found(&self, _exe: &Path) -> bool {
        true
    }
    fn stt_found(&self, _models: &Path) -> bool {
        true
    }
    fn tts_found(&self, _models: &Path) -> bool {
        true
    }
    fn stt(&self, _exe: &Path, _models: &Path, _threads: i32, _data: &Path) -> Result<Box<dyn Stt>> {
        Ok(Box::new(WorldStt(self.0.clone())))
    }
    fn speaker(
        &self,
        _exe: &Path,
        _models: &Path,
        _data: &Path,
        _v: &crate::config::Voice,
        _phrases: Vec<String>,
    ) -> Result<Arc<dyn Speaker>> {
        Ok(Arc::new(WorldSpeaker(self.0.observed.clone())))
    }
}

struct WorldStt(Arc<Io>);

impl Stt for WorldStt {
    fn transcribe(&mut self, _samples: &[f32], _rate: u32) -> Result<String> {
        self.0
            .transcripts
            .lock()
            .recover()
            .pop_front()
            .ok_or_else(|| anyhow!("the world has no transcript: a `say` step gives one"))
    }
}

/// Lines said, observed.
struct WorldSpeaker(Arc<Observed>);

impl Speaker for WorldSpeaker {
    fn say(&self, text: &str) {
        self.0.record("said", json!({ "text": text }));
    }
    fn speaks_aloud(&self) -> bool {
        true
    }
}
