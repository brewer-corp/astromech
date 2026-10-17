//! The session summary by voice (astromech-oo8y; read-only, tier 0): "how much did I make this
//! session?" The core reads the session from the history and these turn it into the spoken
//! lines, the same ones the logout summary says (`astromech_announce::session_summary`).

use astromech_announce::session_summary::{lines, Totals, Unreadable};

/// The `astromech-intent` command id.
pub const COMMAND: &str = "session.summary";

/// Said when the session has earned and covered nothing yet.
pub const NOTHING_YET: &str = "Nothing earned yet this session.";
/// Said when there is no session in the history (no journals imported yet).
pub const NO_HISTORY: &str = "No session history yet.";

pub fn is_summary(command: &str) -> bool {
    command == COMMAND
}

/// What the voice tester shows instead of reading the history.
pub fn describe() -> String {
    "Reads the session summary from your journal history and says it.".into()
}

/// Said when the history exists but could not be read (busy, locked, a read error).
pub const CANT_READ: &str = "Session history is unreadable right now.";

/// The lines to speak for `session` (`Ok(None)`: the history holds none; `Err`: it could not
/// be read), one sentence each.
pub fn answer(session: Result<Option<&Totals>, Unreadable>) -> Vec<String> {
    match session.map(|s| s.map(lines)) {
        Err(Unreadable) => vec![CANT_READ.into()],
        Ok(None) => vec![NO_HISTORY.into()],
        Ok(Some(v)) if v.is_empty() => vec![NOTHING_YET.into()],
        Ok(Some(v)) => v,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn totals() -> Totals {
        Totals {
            end: "t".into(),
            total: 4_200_000,
            per_hour: Some(1_900_000),
            jumps: 38,
            distance_ly: 1203.6,
            bounties: 6,
        }
    }

    #[test]
    fn the_answer_is_one_sentence_per_fact() {
        assert_eq!(
            answer(Ok(Some(&totals()))),
            ["Earned 4.2 million credits, 1.9 million an hour.", "38 jumps, 1204 light years.", "6 bounties awarded."]
        );
    }

    #[test]
    fn what_is_zero_is_left_out() {
        let t = Totals { total: 52_000, per_hour: None, jumps: 1, bounties: 1, ..totals() };
        assert_eq!(
            answer(Ok(Some(&t))),
            ["Earned 52 thousand credits.", "1 jump, 1204 light years.", "One bounty awarded."]
        );
        let t = Totals { total: 0, per_hour: None, jumps: 0, bounties: 0, ..totals() };
        assert_eq!(answer(Ok(Some(&t))), [NOTHING_YET]);
        assert_eq!(answer(Ok(None)), [NO_HISTORY]);
        assert_eq!(answer(Err(Unreadable)), [CANT_READ]);
    }
}
