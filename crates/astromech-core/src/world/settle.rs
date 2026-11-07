//! How a world knows the core has consumed a step ([`crate::world`], "Settling"): what the core
//! publishes for it. The installed app has none of this (`Boundaries::settle` is None), so each
//! hook there is one `Option` check.

use astromech_api::Recover;
use std::collections::HashMap;
use std::sync::{Arc, Condvar, Mutex};
use std::time::{Duration, Instant};

/// What the core tells a world, and the world's hold on the game watch.
#[derive(Default)]
pub struct Settle {
    /// The game folder's writes, and which of them the pipeline has handled.
    pub files: Arc<FileGens>,
    /// The game watch's `activate` threads: each is an event when it ends.
    pub sessions: Sessions,
    /// A world-only hold on the game watch's poll (`world_gate`).
    pub gate: Gate,
}

/// Wait on `cv` until `done` holds, or fail once `guard` (real time: it only stops a stuck call,
/// and decides nothing) has passed.
#[allow(clippy::disallowed_methods, reason = "a real-time hang guard: it only fails a stuck call and decides nothing")]
fn wait_until<T>(m: &Mutex<T>, cv: &Condvar, guard: Duration, mut done: impl FnMut(&T) -> bool) -> Result<(), ()> {
    let deadline = Instant::now() + guard;
    let mut g = m.lock().recover();
    while !done(&g) {
        let left = deadline.saturating_duration_since(Instant::now());
        if left.is_zero() {
            return Err(());
        }
        g = cv.wait_timeout(g, left).recover().0;
    }
    Ok(())
}

/// A generation per game-folder file name: the world bumps a file's after each write, and the
/// pipeline publishes the one it read before handling the file, once it has handled it.
#[derive(Default)]
pub struct FileGens {
    state: Mutex<Gens>,
    handled_cv: Condvar,
}

#[derive(Default)]
struct Gens {
    written: HashMap<String, u64>,
    handled: HashMap<String, u64>,
    /// Pipelines watching the folder now.
    watchers: usize,
}

/// A pipeline watching the game folder; dropping it says it no longer is.
pub struct Watching(Arc<FileGens>);

impl Drop for Watching {
    fn drop(&mut self) {
        self.0.state.lock().recover().watchers -= 1;
        self.0.handled_cv.notify_all();
    }
}

impl FileGens {
    /// The world: write `name` with `write`, and bump its generation, under one lock. A
    /// pipeline reading generations waits out the write, so the generation it reads is never
    /// behind what is on disk. Returns the write's generation.
    pub fn write(&self, name: &str, write: impl FnOnce() -> std::io::Result<()>) -> std::io::Result<u64> {
        let mut s = self.state.lock().recover();
        write()?;
        let g = s.written.entry(name.to_string()).or_default();
        *g += 1;
        Ok(*g)
    }

    /// The pipeline, before it reads files: the generation of each of `names` now.
    pub fn seen(&self, names: &[String]) -> Vec<(String, u64)> {
        let s = self.state.lock().recover();
        names.iter().filter_map(|n| s.written.get(n).map(|g| (n.clone(), *g))).collect()
    }

    /// [`FileGens::seen`] for every file written so far (a pipeline's startup read).
    pub fn seen_all(&self) -> Vec<(String, u64)> {
        self.state.lock().recover().written.iter().map(|(n, g)| (n.clone(), *g)).collect()
    }

    /// The pipeline has handled these files as they were at these generations, whether they
    /// parsed or not.
    pub fn handled(&self, seen: Vec<(String, u64)>) {
        if seen.is_empty() {
            return;
        }
        let mut s = self.state.lock().recover();
        for (n, g) in seen {
            let h = s.handled.entry(n).or_default();
            *h = (*h).max(g);
        }
        self.handled_cv.notify_all();
    }

    /// A pipeline starts watching the folder.
    pub fn watch(self: &Arc<Self>) -> Watching {
        self.state.lock().recover().watchers += 1;
        Watching(self.clone())
    }

    /// The world: wait until the pipeline has handled `name` at generation `gen`, or no pipeline
    /// watches the folder (no session: nothing reads it).
    pub fn wait(&self, name: &str, gen: u64, guard: Duration) -> Result<(), String> {
        wait_until(&self.state, &self.handled_cv, guard, |s| {
            s.watchers == 0 || s.handled.get(name).is_some_and(|h| *h >= gen)
        })
        .map_err(|()| format!("the pipeline didn't handle write {gen} of {name} within {} s", guard.as_secs()))
    }
}

/// The game watch's `activate` threads: started, and ended (the session started or failed).
#[derive(Default)]
pub struct Sessions {
    state: Mutex<(u64, u64)>,
    cv: Condvar,
}

impl Sessions {
    /// An `activate` began.
    pub fn started(&self) {
        self.state.lock().recover().0 += 1;
    }

    /// An `activate` ended: the session started, failed, or was already there.
    pub fn ended(&self) {
        self.state.lock().recover().1 += 1;
        self.cv.notify_all();
    }

    /// Wait until every `activate` that began has ended.
    pub fn wait(&self, guard: Duration) -> Result<(), String> {
        wait_until(&self.state, &self.cv, guard, |(started, ended)| ended >= started)
            .map_err(|()| format!("the session didn't start or fail within {} s", guard.as_secs()))
    }
}

/// A hold on the game watch: while held, the watcher stops at the top of its next poll, before
/// it reads the process list, waits there until the rest of the core is quiescent, and says it
/// has reached the gate.
#[derive(Default)]
pub struct Gate {
    state: Mutex<GateState>,
    cv: Condvar,
}

#[derive(Default)]
struct GateState {
    held: bool,
    /// The polls that have come to the gate.
    polls: u64,
    /// The poll stopped at the gate, once the rest was quiescent.
    reached: Option<u64>,
}

impl Gate {
    /// The game watch, at the top of each poll: while the gate is held, wait for `quiet` (the
    /// rest of the core parked), say so, and wait to be let through. Each wait gives up after
    /// `guard`, so a world that never lets go can't keep the core from quitting.
    pub fn pass(&self, guard: Duration, quiet: impl FnOnce(Duration) -> bool) {
        let poll = {
            let mut s = self.state.lock().recover();
            if !s.held {
                return;
            }
            s.polls += 1;
            s.polls
        };
        quiet(guard);
        let mut s = self.state.lock().recover();
        s.reached = Some(poll);
        self.cv.notify_all();
        drop(s);
        let _ = wait_until(&self.state, &self.cv, guard, |s| !s.held || s.reached != Some(poll));
        let mut s = self.state.lock().recover();
        if s.reached == Some(poll) {
            s.reached = None;
        }
    }

    /// Hold the gate, call `held`, and wait until the watcher is stopped at it. If it is
    /// stopped there already, let it through that poll and hold it at the next.
    pub fn hold(&self, guard: Duration, held: impl FnOnce()) -> Result<(), String> {
        let from = {
            let mut s = self.state.lock().recover();
            s.held = true;
            if s.reached.take().is_some() {
                self.cv.notify_all();
            }
            s.polls
        };
        held();
        wait_until(&self.state, &self.cv, guard, |s| s.reached.is_some_and(|p| p > from))
            .map_err(|()| format!("the game watch didn't reach the gate within {} s", guard.as_secs()))
    }

    /// Let the watcher go on.
    pub fn release(&self) {
        let mut s = self.state.lock().recover();
        s.held = false;
        s.reached = None;
        self.cv.notify_all();
    }
}

#[cfg(test)]
#[allow(clippy::disallowed_methods)] // tests are exempt for now (astromech-c7lc)
mod tests {
    use super::*;

    const LONG: Duration = Duration::from_secs(600);

    #[test]
    fn a_write_settles_once_its_generation_is_handled_or_nothing_watches() {
        let f = Arc::new(FileGens::default());
        // No pipeline: settles at once.
        let g = f.write("Status.json", || Ok(())).unwrap();
        assert_eq!(g, 1);
        f.wait("Status.json", g, Duration::ZERO).unwrap();
        let w = f.watch();
        assert!(f.wait("Status.json", 1, Duration::ZERO).is_err(), "watched, and not handled yet");
        let seen = f.seen(&["Status.json".into(), "Cargo.json".into()]);
        assert_eq!(seen, vec![("Status.json".to_string(), 1)], "a file never written has no generation");
        let g2 = f.write("Status.json", || Ok(())).unwrap();
        f.handled(seen);
        f.wait("Status.json", 1, Duration::ZERO).unwrap();
        assert!(f.wait("Status.json", g2, Duration::ZERO).is_err(), "read before the second write");
        f.handled(f.seen_all());
        f.wait("Status.json", g2, Duration::ZERO).unwrap();
        let g3 = f.write("Status.json", || Ok(())).unwrap();
        drop(w);
        f.wait("Status.json", g3, Duration::ZERO).unwrap();
    }

    #[test]
    fn a_failed_write_bumps_nothing() {
        let f = FileGens::default();
        assert!(f.write("Status.json", || Err(std::io::ErrorKind::Other.into())).is_err());
        assert!(f.seen_all().is_empty());
    }

    /// The pipeline's read of the generations waits out a write in progress: it never sees
    /// the file written but its generation not yet bumped.
    #[test]
    fn a_read_of_the_generations_waits_for_the_write_in_progress() {
        let f = Arc::new(FileGens::default());
        let (inside, go) = (std::sync::mpsc::channel(), std::sync::mpsc::channel::<()>());
        let f2 = f.clone();
        let writer = std::thread::spawn(move || {
            f2.write("Status.json", || {
                inside.0.send(()).unwrap();
                go.1.recv().unwrap();
                Ok(())
            })
        });
        inside.1.recv().unwrap();
        let (read_tx, read_rx) = std::sync::mpsc::channel();
        let f3 = f.clone();
        let reader = std::thread::spawn(move || read_tx.send(f3.seen(&["Status.json".into()])).unwrap());
        // The writer is inside its write: a read can't have finished (it needs the lock).
        assert!(read_rx.try_recv().is_err());
        go.0.send(()).unwrap();
        assert_eq!(read_rx.recv().unwrap(), vec![("Status.json".to_string(), 1)]);
        writer.join().unwrap().unwrap();
        reader.join().unwrap();
    }

    #[test]
    fn sessions_settle_when_every_activate_has_ended() {
        let s = Sessions::default();
        s.wait(Duration::ZERO).unwrap();
        s.started();
        assert!(s.wait(Duration::ZERO).is_err());
        s.ended();
        s.wait(LONG).unwrap();
    }

    #[test]
    fn a_held_gate_stops_the_watcher_until_released() {
        let g = Arc::new(Gate::default());
        g.pass(LONG, |_| panic!("an open gate waits for nothing"));
        g.state.lock().unwrap().held = true;
        let (tx, passed) = std::sync::mpsc::channel();
        let (quiet_tx, quiet) = std::sync::mpsc::channel();
        let g2 = g.clone();
        let w = std::thread::spawn(move || {
            for _ in 0..2 {
                g2.pass(LONG, |_| quiet_tx.send(()).is_ok());
                tx.send(()).unwrap();
            }
        });
        // Held before the watcher came: it stops at its first poll, once the rest is quiet.
        quiet.recv().unwrap();
        wait_until(&g.state, &g.cv, LONG, |s| s.reached == Some(1)).unwrap();
        assert!(passed.try_recv().is_err(), "still at the gate");
        // Held again while it is there: through that poll, stopped at the next.
        g.hold(LONG, || {}).unwrap();
        passed.recv().unwrap();
        quiet.recv().unwrap();
        assert!(passed.try_recv().is_err(), "stopped at the second poll");
        g.release();
        passed.recv().unwrap();
        w.join().unwrap();
    }
}
