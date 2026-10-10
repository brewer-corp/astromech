//! # astromech-intent
//!
//! Maps a speech transcript to a command from a **closed grammar**, with fuzzy matching
//! (docs/ARCHITECTURE.md §5.12). S9 showed every microphone miss was domain vocabulary
//! ("Request stocking", "Definitely hard points"), so matching is token-level edit distance
//! plus a crude phonetic key, after normalisation ("hard points" → "hardpoints").
//!
//! Decision thresholds:
//! - score ≥ [`EXECUTE`] (0.85) with a clear margin over the next *different* command: execute;
//! - [`ASK`] (0.60) ≤ score, or close candidates: ask "Did you mean …?";
//! - below 0.60: "Didn't catch that".
//!
//! Phrases may contain one `{system}` slot, resolved against a caller-supplied list of known
//! names (in the full app: Spansh autocomplete).

/// Score at or above which a match executes without asking.
pub const EXECUTE: f32 = 0.85;
/// Score at or above which we ask "Did you mean …?".
pub const ASK: f32 = 0.60;
/// Minimum lead over the best *different* command needed to execute.
pub const MARGIN: f32 = 0.08;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Decision {
    Execute,
    Ask,
    NoMatch,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Match {
    /// Command id, e.g. "landing_gear.down".
    pub command: &'static str,
    /// The grammar phrase that matched (slot filled in).
    pub phrase: String,
    pub slot: Option<String>,
    pub score: f32,
    pub decision: Decision,
}

#[derive(Debug, Clone)]
pub struct Phrase {
    pub command: &'static str,
    pub text: &'static str,
}

pub struct Grammar {
    phrases: Vec<(Phrase, Vec<String>)>,
    systems: Vec<String>,
}

/// (command id, phrases). The first phrase is the canonical one used in "Did you mean …?".
pub const MVP: &[(&str, &[&str])] = &[
    (
        "landing_gear.down",
        &[
            "gear down",
            "landing gear down",
            "deploy landing gear",
            "deploy gear",
            "lower landing gear",
            "lower gear",
            "drop gear",
        ],
    ),
    (
        "landing_gear.up",
        &["gear up", "landing gear up", "retract landing gear", "retract gear", "raise landing gear", "raise gear"],
    ),
    ("landing_gear.toggle", &["toggle landing gear", "toggle gear", "landing gear"]),
    ("lights.on", &["lights on", "turn on lights", "turn lights on", "headlights on", "ship lights on"]),
    ("lights.off", &["lights off", "turn off lights", "turn lights off", "headlights off", "ship lights off"]),
    ("hardpoints.deploy", &["deploy hardpoints", "hardpoints out", "hardpoints away", "weapons out"]),
    ("hardpoints.retract", &["retract hardpoints", "hardpoints in", "stow hardpoints", "weapons in"]),
    ("cargo_scoop.open", &["open cargo scoop", "deploy cargo scoop", "cargo scoop open", "cargo scoop out"]),
    ("cargo_scoop.close", &["close cargo scoop", "retract cargo scoop", "cargo scoop close", "cargo scoop in"]),
    ("night_vision.on", &["night vision on", "turn on night vision", "enable night vision"]),
    ("night_vision.off", &["night vision off", "turn off night vision", "disable night vision"]),
    // Recognised but not executable in the gear-down slice; kept so they don't get mis-matched
    // to a shorter phrase (and to cover the S9 misses).
    ("request_docking", &["request docking"]),
    ("plot_route", &["plot route to {system}", "set course for {system}"]),
    ("confirm", &["yes", "confirm", "do it", "affirmative"]),
    ("cancel", &["no", "cancel", "never mind", "negative", "abort"]),
];

const FILLER: &[&str] = &["please", "the", "uh", "um", "my", "now", "a", "okay", "ok", "hey"];

/// Lowercase, strip punctuation, fold common splits ("hard points" → "hardpoints"),
/// and drop filler words.
pub fn normalise(s: &str) -> Vec<String> {
    let s: String =
        s.to_lowercase().chars().map(|c| if c.is_alphanumeric() || c == '{' || c == '}' { c } else { ' ' }).collect();
    let mut t: Vec<String> = s.split_whitespace().map(str::to_string).collect();
    let mut out: Vec<String> = vec![];
    let mut i = 0;
    while i < t.len() {
        let w = std::mem::take(&mut t[i]);
        let next = t.get(i + 1).map(|s| s.as_str());
        let joined = match (w.as_str(), next) {
            ("hard", Some("points" | "point")) => Some("hardpoints"),
            ("head", Some("lights" | "light")) => Some("headlights"),
            ("night", Some("visions")) => Some("night vision"),
            _ => None,
        };
        if let Some(j) = joined {
            out.extend(j.split(' ').map(str::to_string));
            i += 2;
            continue;
        }
        let w = match w.as_str() {
            "hardpoint" => "hardpoints".to_string(),
            "light" => "lights".to_string(),
            "gears" => "gear".to_string(),
            _ => w,
        };
        if !FILLER.contains(&w.as_str()) {
            out.push(w);
        }
        i += 1;
    }
    out
}

fn lev(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let mut prev: Vec<usize> = (0..=b.len()).collect();
    for i in 1..=a.len() {
        let mut cur = vec![i; b.len() + 1];
        for j in 1..=b.len() {
            let c = if a[i - 1] == b[j - 1] { 0 } else { 1 };
            cur[j] = (prev[j] + 1).min(cur[j - 1] + 1).min(prev[j - 1] + c);
        }
        prev = cur;
    }
    prev[b.len()]
}

/// Crude phonetic key: first letter, then consonant classes with vowels dropped and repeats
/// collapsed ("seoul" and "sol" → "sl").
pub fn phonetic(w: &str) -> String {
    let class = |c: char| -> Option<char> {
        Some(match c {
            'b' | 'p' => 'p',
            'c' | 'k' | 'q' | 'g' | 'x' => 'k',
            'd' | 't' => 't',
            'f' | 'v' => 'f',
            's' | 'z' => 's',
            'j' => 'j',
            'l' => 'l',
            'm' | 'n' => 'n',
            'r' => 'r',
            _ => return None,
        })
    };
    let mut out = String::new();
    let mut last = None;
    for (i, c) in w.chars().enumerate() {
        let k = class(c);
        if i == 0 {
            out.push(k.unwrap_or(c));
            last = k;
            continue;
        }
        if let Some(k) = k {
            if Some(k) != last {
                out.push(k);
            }
        }
        last = k;
    }
    out
}

/// Similarity of two words in 0..=1.
pub fn word_sim(a: &str, b: &str) -> f32 {
    if a == b {
        return 1.0;
    }
    let n = a.chars().count().max(b.chars().count()) as f32;
    let s = 1.0 - lev(a, b) as f32 / n;
    if a.len() >= 3 && b.len() >= 3 && phonetic(a) == phonetic(b) {
        s.max(0.85)
    } else {
        s.max(0.0)
    }
}

/// Alignment score of two token sequences in 0..=1 (substitution cost 1 − similarity,
/// insertion/deletion cost 1). `slot` is called for a `{system}` token to score a word.
fn seq_score(u: &[String], p: &[String], slot: &dyn Fn(&str) -> (f32, String), filled: &mut Option<String>) -> f32 {
    let (n, m) = (u.len(), p.len());
    if n == 0 || m == 0 {
        return 0.0;
    }
    // dp over (i, j): min cost; keep the slot value chosen on the best path.
    let mut dp = vec![vec![(f32::MAX, None::<String>); m + 1]; n + 1];
    dp[0][0] = (0.0, None);
    for i in 0..=n {
        for j in 0..=m {
            let (c, s) = dp[i][j].clone();
            if c == f32::MAX {
                continue;
            }
            if i < n && dp[i + 1][j].0 > c + 1.0 {
                dp[i + 1][j] = (c + 1.0, s.clone());
            }
            if j < m && dp[i][j + 1].0 > c + 1.0 {
                dp[i][j + 1] = (c + 1.0, s.clone());
            }
            if i < n && j < m {
                let (sim, val) = if p[j] == "{system}" { slot(&u[i]) } else { (word_sim(&u[i], &p[j]), String::new()) };
                let nc = c + (1.0 - sim);
                if dp[i + 1][j + 1].0 > nc {
                    dp[i + 1][j + 1] = (nc, if p[j] == "{system}" { Some(val) } else { s.clone() });
                }
            }
        }
    }
    let (cost, s) = dp[n][m].clone();
    *filled = s;
    (1.0 - cost / n.max(m) as f32).max(0.0)
}

impl Grammar {
    pub fn new(table: &[(&'static str, &[&'static str])], systems: &[&str]) -> Self {
        let mut phrases = vec![];
        for (cmd, ps) in table {
            for p in *ps {
                phrases.push((Phrase { command: cmd, text: p }, normalise(p)));
            }
        }
        Self { phrases, systems: systems.iter().map(|s| s.to_string()).collect() }
    }

    pub fn mvp(systems: &[&str]) -> Self {
        Self::new(MVP, systems)
    }

    /// Canonical phrase for a command (for "Did you mean …?").
    pub fn canonical(&self, command: &str) -> Option<&'static str> {
        self.phrases.iter().find(|(p, _)| p.command == command).map(|(p, _)| p.text)
    }

    fn slot_best(&self, w: &str) -> (f32, String) {
        self.systems
            .iter()
            .map(|s| (word_sim(w, &s.to_lowercase()), s.clone()))
            .max_by(|a, b| a.0.total_cmp(&b.0))
            .unwrap_or((0.0, w.to_string()))
    }

    /// All phrases scored, best first.
    pub fn rank(&self, transcript: &str) -> Vec<Match> {
        let u = normalise(transcript);
        let slot = |w: &str| self.slot_best(w);
        let mut v: Vec<Match> = self
            .phrases
            .iter()
            .map(|(p, toks)| {
                let mut filled = None;
                let score = seq_score(&u, toks, &slot, &mut filled);
                let phrase = match &filled {
                    Some(s) => p.text.replace("{system}", s),
                    None => p.text.to_string(),
                };
                Match { command: p.command, phrase, slot: filled, score, decision: Decision::NoMatch }
            })
            .collect();
        v.sort_by(|a, b| b.score.total_cmp(&a.score));
        v
    }

    /// Best match with its decision.
    pub fn best(&self, transcript: &str) -> Match {
        let ranked = self.rank(transcript);
        let Some(mut best) = ranked.first().cloned() else {
            return Match { command: "", phrase: String::new(), slot: None, score: 0.0, decision: Decision::NoMatch };
        };
        let runner = ranked.iter().find(|m| m.command != best.command).map(|m| m.score).unwrap_or(0.0);
        let clear = best.score - runner >= MARGIN;
        best.decision = if best.score >= EXECUTE && clear {
            Decision::Execute
        } else if best.score >= ASK {
            Decision::Ask
        } else {
            Decision::NoMatch
        };
        // A filled slot that wasn't an exact name always asks.
        if best.decision == Decision::Execute {
            if let Some(s) = &best.slot {
                if !normalise(transcript).contains(&s.to_lowercase()) {
                    best.decision = Decision::Ask;
                }
            }
        }
        best
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn g() -> Grammar {
        Grammar::mvp(&["Sol", "Colonia", "Shinrarta Dezhra", "Achenar"])
    }

    #[test]
    fn normalisation() {
        assert_eq!(normalise("Deploy hard points, please!"), vec!["deploy", "hardpoints"]);
        assert_eq!(normalise("Retrapped hard point."), vec!["retrapped", "hardpoints"]);
        assert_eq!(phonetic("seoul"), phonetic("sol"));
    }

    #[test]
    fn exact_phrases_execute() {
        let g = g();
        for (t, c) in [
            ("Gear down.", "landing_gear.down"),
            ("Landing gear up", "landing_gear.up"),
            ("toggle landing gear", "landing_gear.toggle"),
            ("Lights on", "lights.on"),
            ("lights off.", "lights.off"),
            ("Deploy hardpoints.", "hardpoints.deploy"),
            ("retract hard points", "hardpoints.retract"),
            ("Open cargo scoop", "cargo_scoop.open"),
            ("close the cargo scoop", "cargo_scoop.close"),
            ("Night vision on", "night_vision.on"),
            ("night vision off", "night_vision.off"),
            ("yes", "confirm"),
            ("cancel", "cancel"),
        ] {
            let m = g.best(t);
            assert_eq!((m.command, m.decision), (c, Decision::Execute), "{t:?} -> {m:?}");
        }
    }

    /// The four real microphone misses from S9 (docs/ARCHITECTURE.md §5.12 table).
    #[test]
    fn s9_mic_misses() {
        let g = g();
        let m = g.best("What route to Seoul?");
        assert_eq!(
            (m.command, m.phrase.as_str(), m.decision),
            ("plot_route", "plot route to Sol", Decision::Ask),
            "{m:?}"
        );
        let m = g.best("Definitely hard points.");
        assert_eq!((m.command, m.decision), ("hardpoints.deploy", Decision::Ask), "{m:?}");
        let m = g.best("Request stocking.");
        assert_eq!((m.command, m.decision), ("request_docking", Decision::Execute), "{m:?}");
        let m = g.best("Retrapped hard point.");
        assert_eq!(m.command, "hardpoints.retract", "{m:?}");
        assert_ne!(m.decision, Decision::NoMatch);
    }

    #[test]
    fn near_misses_ask_and_noise_rejects() {
        let g = g();
        let m = g.best("Gear dawn");
        assert_eq!(m.command, "landing_gear.down");
        assert_ne!(m.decision, Decision::NoMatch);
        for t in ["What's the weather like in Paris tomorrow", "", "banana"] {
            assert_eq!(g.best(t).decision, Decision::NoMatch, "{t:?} -> {:?}", g.best(t));
        }
        // Opposites must never execute each other.
        assert_eq!(g.best("lights of").command, "lights.off");
        assert_ne!(g.best("gear up").command, "landing_gear.down");
    }

    #[test]
    fn thresholds_are_the_documented_ones() {
        assert_eq!((EXECUTE, ASK), (0.85, 0.60));
    }
}
