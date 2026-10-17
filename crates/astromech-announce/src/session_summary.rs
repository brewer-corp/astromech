//! Session summary (astromech-oo8y): what the session earned and covered, said when it ends
//! (`Shutdown`, or `Music` back to the `MainMenu`) and on request ("how much did I make this
//! session?"). The game shows the balance, not what the session added to it, so this is the
//! rule that tells the commander.
//!
//! - **Earned**: "Earned 4.2 million credits, 1.9 million an hour." The rate waits for the
//!   session to last a minute.
//! - **Travelled**: "38 jumps, 1204 light years." FSD jumps only.
//! - **Bounties**: "6 bounties awarded."
//!
//! Each is its own line and one that is zero is left out; a session with nothing to report
//! says nothing at logout.
//!
//! The totals come from the history ledgers, which the history follower fills a moment after
//! the journal line is written. So the end of a session only notes that it ended; the next
//! clock tick asks the history, and asks again until it holds the line that ended the session
//! (its `end` is not before it), giving up after [`WAIT_SECS`]. It speaks once per session: a
//! `Shutdown` after a return to the menu says nothing new, and the next `LoadGame` starts over.
//! It arms on a `LoadGame` seen live, or when the startup catch-up ends inside a logged-in
//! session, so starting astromech mid-session still summarises that session at its logout; a
//! fresh launch's menu, before any login, says nothing. A replayed line starts the state over
//! but keeps a summary that is waiting for the history.

use crate::{Callout, Channels, Input, Priority, Rule};
use astromech_gamestate::GameState;
use astromech_journal::JournalEvent;
use std::sync::{Arc, Mutex, RwLock};
use std::time::Duration;

/// How long after the session's last line the history is asked before giving up.
pub const WAIT_SECS: i64 = 30;

/// One session's totals, as the summary needs them.
#[derive(Debug, Clone, PartialEq)]
pub struct Totals {
    /// The journal timestamp of the session's last event in the history.
    pub end: String,
    pub total: i64,
    pub per_hour: Option<i64>,
    pub jumps: i64,
    pub distance_ly: f64,
    pub bounties: i64,
}

/// The history could not be read (lock lost, database busy, read error): not the same as
/// holding no session, so the voice query says so instead of "no history yet".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Unreadable;

/// The commander's session history, as the summary needs it. Called from the pipeline thread,
/// which must never block: answer from an indexed read with a short timeout, `Err` when it
/// can't (database busy or unreadable), `Ok(None)` when it holds no such session.
pub trait SessionHistory: Send + Sync {
    /// The session of `commander` (the latest commander if `None`) that was going on at `at`
    /// (journal timestamp; the latest session if `None`).
    fn session(&self, commander: Option<&str>, at: Option<&str>) -> Result<Option<Totals>, Unreadable>;
}

/// The history the core plugs in once it has one.
#[derive(Clone, Default)]
pub struct SharedSessions(Arc<RwLock<Option<Arc<dyn SessionHistory>>>>);

impl SharedSessions {
    pub fn set(&self, history: Arc<dyn SessionHistory>) {
        *self.0.write().unwrap() = Some(history);
    }

    pub fn get(&self) -> Option<Arc<dyn SessionHistory>> {
        self.0.read().unwrap().clone()
    }
}

impl std::fmt::Debug for SharedSessions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SharedSessions({})", if self.get().is_some() { "set" } else { "none" })
    }
}

/// "5.1 million credits", "578 thousand credits", "900 credits". Rounds up into millions from
/// 999,500, so it never says "1000 thousand".
pub fn credits(c: i64) -> String {
    if c >= 999_500 {
        format!("{:.1} million credits", c as f64 / 1e6)
    } else if c >= 10_000 {
        format!("{:.0} thousand credits", c as f64 / 1e3)
    } else {
        format!("{c} credits")
    }
}

/// "1.9 million an hour" (the rate without the word credits).
fn rate(c: i64) -> String {
    credits(c).trim_end_matches(" credits").to_string() + " an hour"
}

/// The lines for a session: earned, travelled, bounties. A zero is left out.
pub fn lines(t: &Totals) -> Vec<String> {
    let mut v = vec![];
    if t.total > 0 {
        v.push(match t.per_hour.filter(|p| *p > 0) {
            Some(p) => format!("Earned {}, {}.", credits(t.total), rate(p)),
            None => format!("Earned {}.", credits(t.total)),
        });
    }
    if t.jumps > 0 {
        let s = if t.jumps == 1 { "" } else { "s" };
        v.push(format!("{} jump{s}, {:.0} light years.", t.jumps, t.distance_ly));
    }
    match t.bounties {
        0 => {}
        1 => v.push("One bounty awarded.".into()),
        n => v.push(format!("{n} bounties awarded.")),
    }
    v
}

#[derive(Default)]
struct Ended {
    /// The line that ended the session: its commander and journal timestamp, and the first
    /// clock time seen after it.
    pending: Option<(Option<String>, String, Option<i64>)>,
    /// The session already said its summary.
    spoken: bool,
    /// A session is going on that this run has seen begin: a `LoadGame` seen live, or one in
    /// the startup catch-up with no logout after it (astromech started mid-session). The game
    /// writes `Music` `MainMenu` at launch, and possibly after `Commander` and before
    /// `LoadGame`, when the state still holds the previous session: nothing ends a session
    /// that this run has not seen begin.
    armed: bool,
}

pub struct SessionSummary {
    pub sessions: SharedSessions,
    ended: Mutex<Ended>,
}

impl SessionSummary {
    pub fn new(sessions: SharedSessions) -> Self {
        Self { sessions, ended: Mutex::default() }
    }
}

fn ends_session(e: &JournalEvent) -> bool {
    match e {
        JournalEvent::Other { event, raw } => {
            event == "Shutdown"
                || (event == "Music" && raw.get("MusicTrack").and_then(|t| t.as_str()) == Some("MainMenu"))
        }
        _ => false,
    }
}

fn callout(text: String) -> Callout {
    Callout { text, priority: Priority::Normal, channels: Channels::SPEECH | Channels::TOAST }
}

impl Rule for SessionSummary {
    fn id(&self) -> &'static str {
        "session_summary"
    }
    fn cooldown(&self) -> Duration {
        Duration::ZERO
    }
    fn replayed(&self, input: &Input, state: &GameState) {
        if matches!(input, Input::Journal(_)) {
            // A summary already waiting for the history survives a replayed line: that session
            // really ended live, and the replay says nothing about it.
            let mut ended = self.ended.lock().unwrap();
            let pending = ended.pending.take();
            *ended = Ended { pending, armed: state.logged_in, ..Ended::default() };
        }
    }
    fn react(&self, input: &Input, state: &GameState) -> Option<Callout> {
        self.react_all(input, state).into_iter().next()
    }
    fn react_all(&self, input: &Input, state: &GameState) -> Vec<Callout> {
        let mut ended = self.ended.lock().unwrap();
        match input {
            Input::Journal(l) => {
                if matches!(l.event, JournalEvent::LoadGame { .. }) {
                    *ended = Ended { armed: true, ..Ended::default() };
                } else if ended.armed && !ended.spoken && ends_session(&l.event) {
                    ended.pending = Some((state.commander.clone(), l.timestamp.clone(), None));
                }
                vec![]
            }
            Input::Tick { wall_secs } => {
                let Some((commander, ts, since)) = ended.pending.as_mut() else { return vec![] };
                let since = *since.get_or_insert(*wall_secs);
                let found = self
                    .sessions
                    .get()
                    .and_then(|h| h.session(commander.as_deref(), Some(ts)).ok().flatten())
                    .filter(|t| t.end.as_str() >= ts.as_str());
                match found {
                    Some(t) => {
                        ended.pending = None;
                        ended.spoken = true;
                        lines(&t).into_iter().map(callout).collect()
                    }
                    None => {
                        if *wall_secs - since > WAIT_SECS {
                            ended.pending = None;
                        }
                        vec![]
                    }
                }
            }
            _ => vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::credits;

    #[test]
    fn credits_round_into_millions_at_the_boundary() {
        for (c, said) in [
            (900, "900 credits"),
            (9_999, "9999 credits"),
            (10_000, "10 thousand credits"),
            (578_400, "578 thousand credits"),
            (999_499, "999 thousand credits"),
            (999_500, "1.0 million credits"),
            (999_999, "1.0 million credits"),
            (5_100_000, "5.1 million credits"),
        ] {
            assert_eq!(credits(c), said);
        }
    }
}
