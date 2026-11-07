//! Running a world's steps (`world_step`), each settled before the next ([`crate::world`],
//! "Settling").

use super::script::{Game, Script, Step};
use super::{Settle, WAIT_GUARD};
use crate::app::Core;
use crate::gamewatch;
use astromech_api::Recover;
use astromech_proto::{Clock, ManualClock};
use serde_json::{json, Value};
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Duration;

/// The steps and what they act on.
pub struct Steps {
    steps: Vec<(usize, Step)>,
    /// The next step to run.
    next: usize,
    /// Why a step failed: the core may still be catching up with it, so no step runs after it.
    broken: Option<String>,
    /// The journal the last `journal` step wrote.
    journal: Option<String>,
    game_dir: PathBuf,
    clock: Arc<ManualClock>,
    processes: Arc<Mutex<Vec<String>>>,
    settle: Arc<Settle>,
}

impl Steps {
    pub fn new(
        script: Option<Script>,
        game_dir: PathBuf,
        clock: Arc<ManualClock>,
        processes: Arc<Mutex<Vec<String>>>,
        settle: Arc<Settle>,
    ) -> Self {
        let steps = script.map(|s| s.steps).unwrap_or_default();
        Self { steps, next: 0, broken: None, journal: None, game_dir, clock, processes, settle }
    }

    /// Run the steps up to and including the mark `to` (None: to the end), each once the core
    /// has consumed the one before. `{ran, at, next}`: how many ran, the mark reached, and the
    /// index of the next step.
    pub fn run(&mut self, to: Option<&str>, core: &Core) -> Result<Value, String> {
        if let Some(why) = &self.broken {
            return Err(format!("world_step: an earlier step failed, so no more run: {why}"));
        }
        let end = match to {
            None => self.steps.len(),
            Some(m) => {
                let at = self.steps[self.next..].iter().position(|(_, s)| matches!(s, Step::Mark(n) if n == m));
                at.map(|i| self.next + i + 1).ok_or_else(|| format!("world_step: no mark {m:?} ahead"))?
            }
        };
        let from = self.next;
        while self.next < end {
            let (line, _) = self.steps[self.next];
            if let Err(e) = self.one(self.next, core) {
                let e = format!("world.jsonl line {line}: {e}");
                self.broken = Some(e.clone());
                return Err(e);
            }
            self.next += 1;
        }
        Ok(json!({ "ran": end - from, "at": to, "next": self.next }))
    }

    fn one(&mut self, i: usize, core: &Core) -> Result<(), String> {
        match &self.steps[i].1 {
            Step::Mark(_) => Ok(()),
            Step::Advance(d) => self.advance(*d),
            Step::Journal { file, line, stamp } => {
                let name = match file.clone().or_else(|| self.journal.clone()) {
                    Some(n) => n,
                    None => astromech_journal::latest_journal(&self.game_dir)
                        .and_then(|p| p.file_name().map(|n| n.to_string_lossy().into_owned()))
                        .ok_or("there is no journal yet: give the first `journal` step a `file`")?,
                };
                // The pipeline takes a line stamped more than a few seconds after now for a
                // clock running ahead, so the clock gets to the line's time first.
                if let Some(ahead) = stamp.and_then(|t| t.duration_since(self.clock.utc()).ok()) {
                    self.advance(ahead)?;
                }
                let text = format!("{line}\n");
                let path = self.game_dir.join(&name);
                let gen = self.settle.files.write(&name, || {
                    std::fs::OpenOptions::new().create(true).append(true).open(&path)?.write_all(text.as_bytes())
                });
                let gen = gen.map_err(|e| format!("appending to {name}: {e}"))?;
                self.journal = Some(name.clone());
                self.settle.files.wait(&name, gen, WAIT_GUARD)
            }
            Step::File { name, body, partial } => {
                let path = self.game_dir.join(name);
                let write = || {
                    if *partial {
                        // Cut off part way, as a reader can catch a file the game is writing.
                        std::fs::write(&path, &body[..body.len() / 2])
                    } else {
                        // Whole or not at all: written beside it under a name the pipeline
                        // ignores, then renamed over it.
                        let tmp = self.game_dir.join(format!(".{name}.world"));
                        std::fs::write(&tmp, body)?;
                        std::fs::rename(&tmp, &path)
                    }
                };
                let gen = self.settle.files.write(name, write).map_err(|e| format!("writing {name}: {e}"))?;
                if crate::pipeline::watches(name) {
                    self.settle.files.wait(name, gen, WAIT_GUARD)
                } else {
                    Ok(())
                }
            }
            Step::Game(g) => self.game(*g, core),
        }
    }

    /// Advance the clock and wait until the core is quiescent again (guarded: a core that never
    /// settles fails the step instead of hanging the caller).
    fn advance(&self, d: Duration) -> Result<(), String> {
        let (tx, rx) = std::sync::mpsc::channel();
        let clock = self.clock.clone();
        std::thread::Builder::new()
            .name("world-advance".into())
            .spawn(move || {
                clock.advance(d);
                let _ = tx.send(());
            })
            .map_err(|e| e.to_string())?;
        rx.recv_timeout(WAIT_GUARD).map_err(|_| {
            format!("the core didn't settle within {} s of advancing the clock by {d:?}", WAIT_GUARD.as_secs())
        })
    }

    /// Set the process list, then let the game watch poll (a poll per [`gamewatch::POLL`]) until
    /// it reports the game so; then wait for the sessions it started to start or fail. Its
    /// `deactivate` (when the game goes) runs on the watch's own thread, so the advance that let
    /// it poll returned only once that was done.
    fn game(&self, g: Game, core: &Core) -> Result<(), String> {
        let (names, want): (&[&str], _) = match g {
            Game::Running => (&[gamewatch::GAME_EXE], gamewatch::Game::Running),
            Game::Launcher => (&[gamewatch::LAUNCHER_EXE], gamewatch::Game::Launcher),
            Game::Exited => (&[], gamewatch::Game::NotRunning),
        };
        *self.processes.lock().recover() = names.iter().map(|n| n.to_string()).collect();
        // Going from running takes MISSES_TO_IDLE polls without the game.
        for _ in 0..gamewatch::MISSES_TO_IDLE {
            self.advance(gamewatch::POLL)?;
            if *core.game.lock().recover() == want {
                break;
            }
        }
        let now = *core.game.lock().recover();
        if now != want {
            return Err(format!("the game watch reports {now:?}, not {want:?}"));
        }
        self.settle.sessions.wait(WAIT_GUARD)?;
        if g == Game::Exited && core.active() {
            return Err("the game exited, but the session is still running".into());
        }
        Ok(())
    }
}
