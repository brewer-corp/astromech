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
//! Phrases may contain one slot, `{system}` or `{commodity}`, resolved against a caller-supplied
//! list of names (systems: in the full app, Spansh autocomplete; commodities: the market
//! catalogue). A slot can match a name of several words ("low temperature diamonds").

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
    /// The slot's value: the name from the slot list that matched (its canonical spelling).
    pub slot: Option<String>,
    /// The slot was heard exactly (one of the name's spoken forms), not a near match.
    pub slot_exact: bool,
    pub score: f32,
    pub decision: Decision,
}

#[derive(Debug, Clone)]
pub struct Phrase {
    pub command: &'static str,
    pub text: String,
}

/// One name a slot can take: its spoken form, normalised and joined ("lowtemperaturediamonds"),
/// and the canonical value it stands for ("Low Temperature Diamonds").
#[derive(Debug, Clone)]
struct SlotName {
    joined: String,
    value: String,
}

pub struct Grammar {
    phrases: Vec<(Phrase, Vec<String>)>,
    /// Slot kind ("system", "commodity") -> the names it can take.
    slots: Vec<(&'static str, Vec<SlotName>)>,
}

/// The longest name a slot matches, in words.
const SLOT_MAX_WORDS: usize = 4;

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
    ("flight_assist.on", &["flight assist on", "enable flight assist", "turn on flight assist"]),
    ("flight_assist.off", &["flight assist off", "disable flight assist", "turn off flight assist"]),
    ("silent_running.on", &["silent running on", "enable silent running", "go silent"]),
    ("silent_running.off", &["silent running off", "disable silent running"]),
    ("hud.analysis", &["analysis mode", "hud analysis", "switch to analysis"]),
    ("hud.combat", &["combat mode", "hud combat", "switch to combat"]),
    ("galaxy_map.open", &["open galaxy map", "galaxy map", "show galaxy map"]),
    ("system_map.open", &["open system map", "system map", "show system map"]),
    ("panel.close", &["close panel", "close map", "go back", "back"]),
    ("panel.left", &["left panel", "open left panel", "navigation panel"]),
    ("panel.right", &["right panel", "open right panel", "ship panel"]),
    ("panel.comms", &["comms panel", "open comms", "communications panel"]),
    ("panel.role", &["role panel", "open role panel"]),
    ("throttle.zero", &["throttle to zero", "zero throttle", "all stop", "full stop"]),
    ("srv_handbrake.on", &["handbrake on", "hand brake on"]),
    ("srv_handbrake.off", &["handbrake off", "hand brake off"]),
    ("srv_drive_assist.on", &["drive assist on"]),
    ("srv_drive_assist.off", &["drive assist off"]),
    ("status.fuel", &["fuel", "how much fuel", "fuel status", "fuel remaining"]),
    ("status.cargo", &["cargo", "how much cargo", "cargo status"]),
    ("status.jump_range", &["jump range", "what's my jump range", "max jump"]),
    ("status.location", &["where am i", "where are we", "current system", "what system"]),
    ("status.jumps", &["jumps left", "jumps remaining", "next system", "how many jumps"]),
    ("status.hull", &["hull", "hull health", "how much hull"]),
    ("status.shields", &["shields", "shield status"]),
    ("status.credits", &["credits", "how many credits", "rebuy"]),
    ("status.ship", &["what's my ship", "what ship", "current ship", "ship name"]),
    ("status.legal", &["legal state", "wanted", "notoriety", "legal status"]),
    // Recognised but not executable in the gear-down slice; kept so they don't get mis-matched
    // to a shorter phrase (and to cover the S9 misses).
    ("request_docking", &["request docking", "request docking permission", "dock"]),
    (
        "supercruise",
        &["supercruise", "engage supercruise", "enter supercruise", "drop out of supercruise", "exit supercruise"],
    ),
    (
        "hyperspace_jump",
        &["hyperspace jump", "engage hyperspace", "frameshift jump", "engage frameshift drive", "jump to next system"],
    ),
    ("plot_route", &["plot route to {system}", "set course for {system}"]),
    // Trade lookups (Ardent Insight): answered by the core, not pressed.
    (
        "trade.sell",
        &[
            "where to sell {commodity}",
            "where can i sell {commodity}",
            "best place to sell {commodity}",
            "best price for {commodity}",
            "sell {commodity}",
        ],
    ),
    (
        "trade.buy",
        &["where to buy {commodity}", "where can i buy {commodity}", "cheapest {commodity}", "buy {commodity}"],
    ),
    ("confirm", &["yes", "confirm", "do it", "affirmative"]),
    ("cancel", &["no", "cancel", "never mind", "negative", "abort"]),
];

/// Commands the grammar recognises but the app can't run (yet). They exist so a request is
/// answered ("I can't do that yet") instead of "didn't catch that" or a wrong near match.
pub const UNAVAILABLE: &[&str] = &["request_docking", "plot_route", "supercruise", "hyperspace_jump"];

/// Words a whole transcript can consist of when the mic caught only a noise, breath or
/// acknowledgement (Parakeet's usual output for a near-empty clip). Ignored silently.
pub const NOISE_WORDS: &[&str] =
    &["mm", "mmm", "hmm", "hm", "mhm", "mmhmm", "uh", "um", "uhm", "ah", "oh", "huh", "yeah", "okay", "ok", "uhhuh"];

/// True when a transcript has nothing but noise words ("Mm-hmm.", "Yeah.", "Okay.", "Mm.").
pub fn is_noise_only(t: &str) -> bool {
    let s: String = t.to_lowercase().chars().map(|c| if c.is_alphanumeric() { c } else { ' ' }).collect();
    let w: Vec<&str> = s.split_whitespace().collect();
    !w.is_empty() && w.iter().all(|w| NOISE_WORDS.contains(w) || *w == "hmm" || *w == "mm")
}

/// Mishearings of game words seen in real transcripts that are too ambiguous to execute on:
/// the replacement is scored, but capped below EXECUTE so it becomes "Did you mean …?".
const MISHEARD: &[(&str, &str)] = &[("you", "gear"), ("youre", "gear"), ("your", "gear"), ("yours", "gear")];
/// Score cap for a match that needed a [`MISHEARD`] replacement.
pub const MISHEARD_CAP: f32 = 0.80;

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
            // Game vocabulary (S9/slice transcripts: "Super Cruise", "Engage Super Crews").
            (
                "super",
                Some("cruise" | "crews" | "cruz" | "cruise's" | "cruises" | "cruiser" | "cruse" | "crew" | "screws"),
            ) => Some("supercruise"),
            ("frame", Some("shift" | "ship")) => Some("frameshift"),
            ("hyper", Some("space")) => Some("hyperspace"),
            ("cargo", Some("scoops")) => Some("cargo scoop"),
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
            "superus" | "supercruz" | "supercrews" | "supercruse" | "supercruiser" | "supercruises" | "supercrus" => {
                "supercruise".to_string()
            }
            "you're" => "youre".to_string(),
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

/// A slot's best name for some words: (similarity, value, exact).
type SlotFn<'a> = dyn Fn(&str, &[String]) -> (f32, String, bool) + 'a;

/// What a slot was filled with on an alignment path.
type Filled = Option<(String, bool)>;

/// Alignment score of two token sequences in 0..=1 (substitution cost 1 − similarity,
/// insertion/deletion cost 1). A slot token (`{system}`) takes 1..=[`SLOT_MAX_WORDS`] words of
/// the utterance at the cost of one substitution, scored by `slot`.
fn seq_score(u: &[String], p: &[String], slot: &SlotFn, filled: &mut Filled) -> f32 {
    let (n, m) = (u.len(), p.len());
    if n == 0 || m == 0 {
        return 0.0;
    }
    // dp over (i, j): min cost; keep the slot value chosen on the best path.
    let mut dp = vec![vec![(f32::MAX, None::<(String, bool)>); m + 1]; n + 1];
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
            if i >= n || j >= m {
                continue;
            }
            if let Some(kind) = p[j].strip_prefix('{').and_then(|k| k.strip_suffix('}')) {
                for k in 1..=SLOT_MAX_WORDS.min(n - i) {
                    let (sim, val, exact) = slot(kind, &u[i..i + k]);
                    let nc = c + (1.0 - sim);
                    if dp[i + k][j + 1].0 > nc {
                        dp[i + k][j + 1] = (nc, Some((val, exact)));
                    }
                }
            } else {
                let nc = c + (1.0 - word_sim(&u[i], &p[j]));
                if dp[i + 1][j + 1].0 > nc {
                    dp[i + 1][j + 1] = (nc, s.clone());
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
                phrases.push((Phrase { command: cmd, text: (*p).to_string() }, normalise(p)));
            }
        }
        let g = Self { phrases, slots: vec![] };
        g.with_slot("system", &systems.iter().map(|s| (*s, *s)).collect::<Vec<_>>())
    }

    /// The names `{kind}` can take, as (spoken form, canonical value) pairs. Several spoken
    /// forms may share a value ("LTDs" and "Low Temperature Diamonds").
    pub fn with_slot(mut self, kind: &'static str, names: &[(&str, &str)]) -> Self {
        let list = names
            .iter()
            .map(|(spoken, value)| SlotName { joined: normalise(spoken).concat(), value: value.to_string() })
            .filter(|n| !n.joined.is_empty())
            .collect();
        self.slots.retain(|(k, _)| *k != kind);
        self.slots.push((kind, list));
        self
    }

    pub fn mvp(systems: &[&str]) -> Self {
        Self::new(MVP, systems)
    }

    /// Canonical phrase for a command (for "Did you mean …?").
    pub fn canonical(&self, command: &str) -> Option<&str> {
        self.phrases.iter().find(|(p, _)| p.command == command).map(|(p, _)| p.text.as_str())
    }

    /// Intern a command id from the closed grammar (for stored aliases).
    pub fn intern(command: &str) -> Option<&'static str> {
        MVP.iter().find(|(c, _)| *c == command).map(|(c, _)| *c)
    }

    /// Personal alias: this transcript should map to `command` next time (still policy-capped).
    pub fn add_alias(&mut self, heard: &str, command: &'static str) {
        let toks = normalise(heard);
        if toks.is_empty() {
            return;
        }
        self.phrases.push((Phrase { command, text: heard.trim().to_string() }, toks));
    }

    /// The `kind` name closest to `words` (compared joined, so "low temperature diamonds"
    /// matches "lowtemperaturediamonds").
    fn slot_best(&self, kind: &str, words: &[String]) -> (f32, String, bool) {
        let heard = words.concat();
        self.slots
            .iter()
            .find(|(k, _)| *k == kind)
            .and_then(|(_, names)| {
                names
                    .iter()
                    .map(|n| (word_sim(&heard, &n.joined), n.value.clone(), heard == n.joined))
                    .max_by(|a, b| a.0.total_cmp(&b.0).then(a.2.cmp(&b.2)))
            })
            .unwrap_or((0.0, words.join(" "), false))
    }

    /// All phrases scored, best first.
    pub fn rank(&self, transcript: &str) -> Vec<Match> {
        let u = normalise(transcript);
        let misheard: Option<Vec<String>> = u.iter().any(|w| MISHEARD.iter().any(|(h, _)| h == w)).then(|| {
            u.iter()
                .map(|w| MISHEARD.iter().find(|(h, _)| h == w).map(|(_, r)| r.to_string()).unwrap_or_else(|| w.clone()))
                .collect()
        });
        let slot = |kind: &str, words: &[String]| self.slot_best(kind, words);
        let mut v: Vec<Match> = self
            .phrases
            .iter()
            .map(|(p, toks)| {
                let mut filled = None;
                let mut score = seq_score(&u, toks, &slot, &mut filled);
                if let Some(alt) = &misheard {
                    let mut f2 = None;
                    let s2 = seq_score(alt, toks, &slot, &mut f2).min(MISHEARD_CAP);
                    if s2 > score {
                        score = s2;
                        filled = f2;
                    }
                }
                let (slot, slot_exact) = match filled {
                    Some((v, exact)) => (Some(v), exact),
                    None => (None, false),
                };
                let phrase = match &slot {
                    Some(s) => fill_slot(&p.text, s),
                    None => p.text.to_string(),
                };
                Match { command: p.command, phrase, slot, slot_exact, score, decision: Decision::NoMatch }
            })
            .collect();
        v.sort_by(|a, b| b.score.total_cmp(&a.score));
        v
    }

    /// Best match with its decision.
    pub fn best(&self, transcript: &str) -> Match {
        let ranked = self.rank(transcript);
        let Some(mut best) = ranked.first().cloned() else {
            return Match {
                command: "",
                phrase: String::new(),
                slot: None,
                slot_exact: false,
                score: 0.0,
                decision: Decision::NoMatch,
            };
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
        if best.decision == Decision::Execute && best.slot.is_some() && !best.slot_exact {
            best.decision = Decision::Ask;
        }
        best
    }
}

/// `text` with its `{slot}` replaced by `value`.
fn fill_slot(text: &str, value: &str) -> String {
    match (text.find('{'), text.find('}')) {
        (Some(a), Some(b)) if a < b => format!("{}{value}{}", &text[..a], &text[b + 1..]),
        _ => text.to_string(),
    }
}

/// One labelled clip or transcript for the M3 eval harness.
#[derive(Debug, Clone)]
pub struct EvalItem {
    pub id: String,
    pub transcript: String,
    /// Expected command id, or `"nomatch"`.
    pub label: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvalRow {
    pub id: String,
    pub transcript: String,
    pub label: String,
    pub got: String,
    pub decision: Decision,
    pub kind: EvalKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvalKind {
    /// Execute or Ask on the labelled command (the 95% target).
    CorrectExecuteOrAsk,
    /// Said no-match and the label was `nomatch`.
    CorrectReject,
    /// Executed a *different* command (must stay near 0).
    WrongExecute,
    /// Labelled command, but NoMatch.
    Miss,
    /// Asked or executed the wrong command without a wrong-execute (Ask on a different command).
    WrongAsk,
}

#[derive(Debug, Clone, Default)]
pub struct EvalReport {
    pub total: usize,
    pub correct_execute_or_ask: usize,
    pub correct_reject: usize,
    pub wrong_execute: usize,
    pub miss: usize,
    pub wrong_ask: usize,
    pub rows: Vec<EvalRow>,
}

impl EvalReport {
    pub fn execute_or_ask_rate(&self) -> f32 {
        let n = self.correct_execute_or_ask + self.wrong_execute + self.miss + self.wrong_ask;
        if n == 0 {
            0.0
        } else {
            self.correct_execute_or_ask as f32 / n as f32
        }
    }
    pub fn summary(&self) -> String {
        format!(
            "eval: {} clips; execute-or-ask {}/{} ({:.0}%), wrong execute {}, miss {}, reject {}",
            self.total,
            self.correct_execute_or_ask,
            self.total.saturating_sub(self.correct_reject),
            100.0 * self.execute_or_ask_rate(),
            self.wrong_execute,
            self.miss,
            self.correct_reject
        )
    }
}

/// Score transcripts against labels (no speech models needed).
pub fn evaluate(grammar: &Grammar, items: &[EvalItem]) -> EvalReport {
    let mut r = EvalReport { total: items.len(), ..Default::default() };
    for it in items {
        let m = grammar.best(&it.transcript);
        let nomatch_label = it.label == "nomatch" || it.label.is_empty();
        let kind = if nomatch_label {
            if m.decision == Decision::NoMatch {
                EvalKind::CorrectReject
            } else if m.decision == Decision::Execute {
                EvalKind::WrongExecute
            } else {
                EvalKind::WrongAsk
            }
        } else if m.command == it.label && (m.decision == Decision::Execute || m.decision == Decision::Ask) {
            EvalKind::CorrectExecuteOrAsk
        } else if m.decision == Decision::Execute && m.command != it.label {
            EvalKind::WrongExecute
        } else if m.decision == Decision::NoMatch {
            EvalKind::Miss
        } else {
            EvalKind::WrongAsk
        };
        match kind {
            EvalKind::CorrectExecuteOrAsk => r.correct_execute_or_ask += 1,
            EvalKind::CorrectReject => r.correct_reject += 1,
            EvalKind::WrongExecute => r.wrong_execute += 1,
            EvalKind::Miss => r.miss += 1,
            EvalKind::WrongAsk => r.wrong_ask += 1,
        }
        r.rows.push(EvalRow {
            id: it.id.clone(),
            transcript: it.transcript.clone(),
            label: it.label.clone(),
            got: m.command.to_string(),
            decision: m.decision,
            kind,
        });
    }
    r
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

    /// Sumeet's real-game transcripts (gear-down preview, 10 Oct).
    #[test]
    fn real_game_transcripts() {
        let g = g();
        let d = |t: &str| {
            let m = g.best(t);
            (m.command, m.decision)
        };
        assert_eq!(d("Bunding gear down."), ("landing_gear.down", Decision::Execute));
        assert_eq!(d("Year down."), ("landing_gear.down", Decision::Execute));
        assert_eq!(d("Gear down."), ("landing_gear.down", Decision::Execute));
        assert_eq!(d("You're down."), ("landing_gear.down", Decision::Ask), "too ambiguous to press on");
        for t in ["Super Cruise", "Supercruise.", "Engage Super Crews", "Engage Superus."] {
            assert_eq!(d(t), ("supercruise", Decision::Execute), "{t}");
        }
        assert_eq!(d("Hyperspace jump."), ("hyperspace_jump", Decision::Execute));
        assert_eq!(d("Engage frame shift drive"), ("hyperspace_jump", Decision::Execute));
    }

    #[test]
    fn noise_only_transcripts() {
        for t in ["Mm-hmm.", "Yeah.", "Okay.", "Mm.", "Hmm?", "Uh, um."] {
            assert!(is_noise_only(t), "{t}");
        }
        for t in ["Yes.", "Gear down.", "", "Okay, gear down."] {
            assert!(!is_noise_only(t), "{t}");
        }
    }

    #[test]
    fn thresholds_are_the_documented_ones() {
        assert_eq!((EXECUTE, ASK), (0.85, 0.60));
    }

    #[test]
    fn v1_t1_and_status_phrases_execute() {
        let g = g();
        for (t, c) in [
            ("flight assist off", "flight_assist.off"),
            ("silent running on", "silent_running.on"),
            ("analysis mode", "hud.analysis"),
            ("open galaxy map", "galaxy_map.open"),
            ("close panel", "panel.close"),
            ("left panel", "panel.left"),
            ("throttle to zero", "throttle.zero"),
            ("how much fuel", "status.fuel"),
            ("where am i", "status.location"),
            ("jumps left", "status.jumps"),
            ("what's my ship", "status.ship"),
        ] {
            let m = g.best(t);
            assert_eq!((m.command, m.decision), (c, Decision::Execute), "{t:?} -> {m:?}");
        }
        assert_ne!(g.best("close cargo scoop").command, "panel.close");
        assert_eq!(g.best("close cargo scoop").command, "cargo_scoop.close");
    }

    /// A few market commodities as the core passes them (spoken form, canonical name).
    fn trade() -> Grammar {
        g().with_slot(
            "commodity",
            &[
                ("Gold", "Gold"),
                ("Silver", "Silver"),
                ("Painite", "Painite"),
                ("Platinum", "Platinum"),
                ("Low Temperature Diamonds", "Low Temperature Diamonds"),
                ("LTDs", "Low Temperature Diamonds"),
                ("Void Opal", "Void Opal"),
                ("void opals", "Void Opal"),
                ("Tritium", "Tritium"),
                ("Water", "Water"),
                ("Coltan", "Coltan"),
            ],
        )
    }

    #[test]
    fn trade_phrases_fill_the_commodity_slot() {
        let g = trade();
        for (t, cmd, slot) in [
            ("Where can I sell painite?", "trade.sell", "Painite"),
            ("where to sell gold", "trade.sell", "Gold"),
            ("Best price for low temperature diamonds.", "trade.sell", "Low Temperature Diamonds"),
            ("Where can I sell LTDs?", "trade.sell", "Low Temperature Diamonds"),
            ("Sell void opals.", "trade.sell", "Void Opal"),
            ("Where can I buy tritium?", "trade.buy", "Tritium"),
            ("Where to buy water", "trade.buy", "Water"),
            ("cheapest silver", "trade.buy", "Silver"),
        ] {
            let m = g.best(t);
            assert_eq!(
                (m.command, m.slot.as_deref(), m.decision),
                (cmd, Some(slot), Decision::Execute),
                "{t:?} -> {m:?}"
            );
        }
        assert_eq!(g.best("Where can I sell LTDs?").phrase, "where can i sell Low Temperature Diamonds");
    }

    #[test]
    fn misheard_commodities_ask() {
        let g = trade();
        // Parakeet's spelling of a name it doesn't know: close, so ask, never execute.
        let m = g.best("Where can I sell pain night?");
        assert_eq!((m.command, m.slot.as_deref(), m.decision), ("trade.sell", Some("Painite"), Decision::Ask), "{m:?}");
        assert_eq!(m.phrase, "where can i sell Painite");
        let m = g.best("Where can I sell low temperature diamond?");
        assert_eq!((m.command, m.slot.as_deref()), ("trade.sell", Some("Low Temperature Diamonds")), "{m:?}");
        assert_eq!(m.decision, Decision::Ask, "a near name asks: {m:?}");
        // The status question is unchanged.
        assert_eq!((g.best("cargo").command, g.best("cargo").decision), ("status.cargo", Decision::Execute));
        // Not a commodity: at most "Did you mean …?", never a lookup on a guess.
        assert_ne!(g.best("Where can I sell the weather in Paris").decision, Decision::Execute);
        assert_eq!(g.best("What's the weather like in Paris tomorrow").decision, Decision::NoMatch);
    }

    #[test]
    fn multi_word_system_names_fill_the_slot() {
        let m = g().best("Plot route to Shinrarta Dezhra");
        assert_eq!(
            (m.command, m.slot.as_deref(), m.decision),
            ("plot_route", Some("Shinrarta Dezhra"), Decision::Execute)
        );
    }

    #[test]
    fn personal_alias_executes_next_time() {
        let mut g = g();
        let m = g.best("You're down.");
        assert_eq!(m.command, "landing_gear.down");
        assert_eq!(m.decision, Decision::Ask);
        g.add_alias("You're down.", "landing_gear.down");
        let m = g.best("You're down.");
        assert_eq!((m.command, m.decision), ("landing_gear.down", Decision::Execute));
    }

    #[test]
    fn eval_counts_execute_or_ask() {
        let g = g();
        let items = [
            EvalItem { id: "1".into(), transcript: "Gear down.".into(), label: "landing_gear.down".into() },
            EvalItem { id: "2".into(), transcript: "You're down.".into(), label: "landing_gear.down".into() },
            EvalItem { id: "3".into(), transcript: "banana".into(), label: "nomatch".into() },
            EvalItem { id: "4".into(), transcript: "Gear down.".into(), label: "lights.on".into() },
        ];
        let r = evaluate(&g, &items);
        assert_eq!(r.correct_execute_or_ask, 2);
        assert_eq!(r.wrong_execute, 1);
        assert_eq!(r.miss, 0);
        assert_eq!(r.correct_reject, 1);
    }
}
