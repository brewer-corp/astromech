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
//! - below 0.60: "Say again".
//!
//! Phrases may contain one slot, `{name}`, filled by one or more words of the transcript and
//! resolved against values the caller supplies for that slot (`{system}`: known system names;
//! the lookups add `{commodity}`, `{service}`, `{module}` and `{ship}`). A slot can also allow free words that
//! are kept in [`Match::slot_text`] but not matched against its values (a module's "5A" or
//! "gimballed").

use std::cell::RefCell;
use std::collections::HashMap;

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
    /// The grammar phrase that matched, its slot filled with the phrasing it matched.
    pub phrase: String,
    /// The slot's value (as the caller supplied it), when the phrase has a slot.
    pub slot: Option<String>,
    /// The transcript words the slot took, normalised and joined with spaces.
    pub slot_text: Option<String>,
    /// The slot was heard exactly (one of its phrasings, free words aside), not a near match.
    pub slot_exact: bool,
    pub score: f32,
    pub decision: Decision,
}

#[derive(Debug, Clone)]
pub struct Phrase {
    pub command: &'static str,
    pub text: String,
}

/// Words a slot accepts without matching them against its values.
pub type FreeWord = fn(&str) -> bool;

struct Slot {
    name: String,
    /// (normalised words, as written, value)
    values: Vec<(Vec<String>, String, String)>,
    free: Option<FreeWord>,
}

pub struct Grammar {
    phrases: Vec<(Phrase, Vec<String>)>,
    slots: Vec<Slot>,
    /// The phrases added by [`Grammar::add_alias`], as (text, command), so they can be removed.
    personal: Vec<(String, &'static str)>,
    /// How many of `phrases` are the grammar's own; personal phrases follow them.
    builtin: usize,
}

/// Most transcript words one slot can take.
const MAX_SLOT_WORDS: usize = 8;

/// How well some transcript words fill a slot.
#[derive(Debug, Clone, Default)]
struct Fill {
    sim: f32,
    value: String,
    /// The phrasing it matched, as the caller wrote it (for "Did you mean …?").
    said: String,
    text: String,
    /// The words (free words aside) are exactly one of the slot's phrasings.
    exact: bool,
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
    // The session summary from the journal history (astromech-oo8y): spoken, nothing pressed.
    (
        "session.summary",
        &[
            "session summary",
            "how much did i make this session",
            "how much did i earn this session",
            "how much have i earned",
            "what did i earn this session",
            "session earnings",
            "how's the session going",
        ],
    ),
    ("status.ship", &["what's my ship", "what ship", "current ship", "ship name"]),
    ("status.legal", &["legal state", "wanted", "notoriety", "legal status"]),
    // Raw-material site lookups (astromech-galaxy `SITE_KINDS`): spoken answer, nothing pressed.
    (
        "sites.crystalline_shards",
        &[
            "nearest crystalline shards",
            "find crystalline shards",
            "where are the nearest crystalline shards",
            "nearest crystalline structures",
            "nearest shard site",
        ],
    ),
    ("sites.brain_trees", &["nearest brain trees", "find brain trees", "where are the nearest brain trees"]),
    ("sites.crashed_ships", &["nearest crashed ship", "find a crashed ship", "nearest wreck site"]),
    ("sites.geological", &["nearest geological sites", "find geological sites", "nearest geology"]),
    ("sites.raw_farms", &["nearest raw material farm", "nearest raw farm", "where can i farm raw materials"]),
    // Faction-state lookups (astromech-galaxy `QUERIES`, `faction.<id>`): spoken answer, nothing
    // pressed.
    (
        "faction.famine",
        &[
            "nearest famine system",
            "nearest famine",
            "where is there a famine near me",
            "find a famine system",
            "famine near me",
        ],
    ),
    ("faction.proto", &["nearest boom system", "nearest boom", "where is there a boom near me", "find a boom system"]),
    (
        "faction.military",
        &[
            "nearest war system",
            "nearest war",
            "where is there a war near me",
            "nearest civil war",
            "find a war system",
        ],
    ),
    (
        "faction.improvised",
        &["nearest civil unrest system", "nearest civil unrest", "where is there civil unrest near me"],
    ),
    ("faction.pharma", &["nearest outbreak system", "nearest outbreak", "where is there an outbreak near me"]),
    (
        "faction.fed_composites",
        &[
            "nearest federation system",
            "nearest federal system",
            "nearest federation faction",
            "nearest federation",
            "nearest federal",
        ],
    ),
    (
        "faction.empire_shielding",
        &[
            "nearest empire system",
            "nearest imperial system",
            "nearest empire faction",
            "nearest empire",
            "nearest imperial",
        ],
    ),
    (
        "status.system_power",
        &["who controls this system", "who controls the system", "system power", "system powerplay", "powerplay here"],
    ),
    (
        "status.powerplay",
        &[
            "what's my powerplay rank",
            "powerplay rank",
            "powerplay status",
            "how many merits",
            "how many merits do i have",
            "what are my merits",
            "merits",
        ],
    ),
    (
        "status.merits_here",
        &["what earns merits here", "how do i earn merits here", "merits here", "powerplay activities"],
    ),
    // Nearby Powerplay targets for the pledged power (astromech-galaxy `Target`,
    // `powerplay.<kind>`): spoken answer, nothing pressed.
    (
        "powerplay.acquire",
        &[
            "where can i acquire near me",
            "nearest system to acquire",
            "nearest acquisition target",
            "nearest acquisition system",
            "where can i acquire",
        ],
    ),
    (
        "powerplay.reinforce",
        &[
            "where can i reinforce near me",
            "nearest system to reinforce",
            "nearest reinforcement target",
            "nearest system to fortify",
            "where can i reinforce",
        ],
    ),
    (
        "powerplay.undermine",
        &[
            "where can i undermine near me",
            "nearest system to undermine",
            "nearest undermining target",
            "nearest undermining system",
            "where can i undermine",
        ],
    ),
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
    // Route following (docs/PLOTTING.md): the next waypoint is on the clipboard; "paste" puts it
    // into the galaxy map's search box.
    (
        "route.next",
        &[
            "next waypoint",
            "copy next waypoint",
            "copy waypoint",
            "what's the next waypoint",
            "where's the next waypoint",
        ],
    ),
    ("route.skip", &["skip waypoint", "skip this waypoint", "skip to the next waypoint"]),
    ("route.back", &["previous waypoint", "back one waypoint", "go back a waypoint"]),
    ("route.clear", &["clear route", "clear the route", "end route", "stop following the route"]),
    ("galaxy_map.paste", &["paste", "paste it", "paste system", "paste waypoint", "paste system name"]),
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
    (
        "trade.sell_cargo",
        &["where to sell my cargo", "where should i sell my cargo", "best place to sell my cargo", "sell my cargo"],
    ),
    // Colonisation (the construction tracker, Ardent for where to buy): answered by the core.
    (
        "colonisation.progress",
        &[
            "how is the build going",
            "how's the build going",
            "how is the construction going",
            "build progress",
            "construction progress",
            "colonisation progress",
            "how far along is the build",
        ],
    ),
    (
        "colonisation.remaining",
        &[
            "what is left to deliver",
            "what's left to deliver",
            "how much is left to deliver",
            "what does the build need",
            "what is left for the build",
            "construction shopping list",
        ],
    ),
    (
        "colonisation.buy",
        &[
            "where can i buy {commodity} for the depot",
            "where can i buy {commodity} for the build",
            "where can i buy {commodity} for the construction",
            "where can i buy {commodity} for the project",
            "where to buy {commodity} for the depot",
            "where to buy {commodity} for the build",
            "buy {commodity} for the depot",
        ],
    ),
    // Station lookups (the slots' values come from astromech-galaxy).
    (
        "find.service",
        &[
            "nearest {service}",
            "where's the nearest {service}",
            "where is the nearest {service}",
            "find the nearest {service}",
            "find {service}",
        ],
    ),
    (
        "find.module",
        &[
            "nearest {module}",
            "where can i buy {module}",
            "where can i get {module}",
            "where's the nearest {module}",
            "where is the nearest {module}",
            "find {module}",
            "buy {module}",
        ],
    ),
    (
        "find.ship",
        &[
            "where can i buy {ship}",
            "where can i get {ship}",
            "nearest {ship}",
            "where's the nearest {ship}",
            "find {ship}",
            "buy {ship}",
            "who sells {ship}",
        ],
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

/// Mishearings of game words, too ambiguous to execute on: the replacement is scored, but capped
/// below EXECUTE so it becomes "Did you mean …?". "you" for "gear" is from real transcripts;
/// "I'll" (normalised to "iwill") and "old" for "all" ("all stop") and "pays" for "paste" (the
/// final "st" lost) from the voice eval. They are everyday words, so a match through them must
/// never press a key.
const MISHEARD: &[(&str, &str)] = &[
    ("you", "gear"),
    ("youre", "gear"),
    ("your", "gear"),
    ("yours", "gear"),
    ("iwill", "all"),
    ("old", "all"),
    ("pays", "paste"),
];
/// True when `heard` has a word from the [`MISHEARD`] table ("I'll stop", "Pays", "You're
/// down"). Such a phrase only ever asks, so it is never learned as a personal phrase.
pub fn is_ask_only(heard: &str) -> bool {
    normalise(heard).iter().any(|w| MISHEARD.iter().any(|(h, _)| h == w))
}

/// Score cap for a match that needed a [`MISHEARD`] replacement.
pub const MISHEARD_CAP: f32 = 0.80;

/// Words dropped wherever they appear. "sir" is politeness, like "please" ("Yes, sir.").
const FILLER: &[&str] = &["please", "sir", "the", "uh", "um", "my", "now", "a", "an", "okay", "ok", "hey"];

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
            // Ship marks: "Mk II", "Mark two", "Mk 2" → "mkii" (as in "Krait MkII").
            ("mk" | "mark", Some("ii" | "two" | "2")) => Some("mkii"),
            ("mk" | "mark", Some("iii" | "three" | "3")) => Some("mkiii"),
            ("mk" | "mark", Some("iv" | "four" | "4")) => Some("mkiv"),
            ("mk" | "mark", Some("v" | "five" | "5")) => Some("mkv"),
            ("power", Some("play" | "plays")) => Some("powerplay"),
            // "I'll" loses its apostrophe above. Keep it one word so it can stand for "all", but
            // not one close to "all" ("ill"), or fuzzy matching would score it past the cap.
            ("i", Some("ll")) => Some("iwill"),
            _ => None,
        };
        if let Some(j) = joined {
            out.extend(j.split(' ').map(str::to_string));
            i += 2;
            continue;
        }
        // A module's class and rating said apart ("5 A", "five a") becomes one word, "5a", so
        // the rating isn't dropped as the filler "a".
        if let (Some(d), Some(r)) = (class_digit(&w), next.filter(|n| is_rating(n))) {
            out.push(format!("{d}{r}"));
            i += 2;
            continue;
        }
        // A rating said alone ("A-rated", "A grade", "grade A") becomes one word, "arated", for
        // the same reason.
        let rating = match (w.as_str(), next) {
            (r, Some("rated" | "grade")) if is_rating(r) => Some(r.to_string()),
            ("grade", Some(r)) if is_rating(r) => Some(r.to_string()),
            _ => None,
        };
        if let Some(r) = rating {
            out.push(format!("{r}rated"));
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

fn class_digit(w: &str) -> Option<char> {
    const WORDS: [&str; 8] = ["one", "two", "three", "four", "five", "six", "seven", "eight"];
    match w.as_bytes() {
        [d @ b'1'..=b'8'] => Some(*d as char),
        _ => WORDS.iter().position(|n| *n == w).map(|i| (b'1' + i as u8) as char),
    }
}

fn is_rating(w: &str) -> bool {
    matches!(w.as_bytes(), [b'a'..=b'i'])
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

/// Alignment score of two word sequences in 0..=1 (substitution cost 1 − similarity,
/// insertion and deletion cost 1).
fn align(u: &[String], p: &[String]) -> f32 {
    let (n, m) = (u.len(), p.len());
    if n == 0 || m == 0 {
        return 0.0;
    }
    let mut dp = vec![vec![f32::MAX; m + 1]; n + 1];
    dp[0][0] = 0.0;
    for i in 0..=n {
        for j in 0..=m {
            let c = dp[i][j];
            if c == f32::MAX {
                continue;
            }
            if i < n {
                dp[i + 1][j] = dp[i + 1][j].min(c + 1.0);
            }
            if j < m {
                dp[i][j + 1] = dp[i][j + 1].min(c + 1.0);
            }
            if i < n && j < m {
                dp[i + 1][j + 1] = dp[i + 1][j + 1].min(c + 1.0 - word_sim(&u[i], &p[j]));
            }
        }
    }
    (1.0 - dp[n][m] / n.max(m) as f32).max(0.0)
}

fn slot_name(w: &str) -> Option<&str> {
    w.strip_prefix('{')?.strip_suffix('}')
}

/// Score of transcript `u` against phrase `p` in 0..=1. A slot word in `p` takes 1 to
/// [`MAX_SLOT_WORDS`] transcript words at a cost of (words × (1 − fill similarity)); `fill`
/// scores the words `u[i..i + k]` for a slot. Returns the score and the slot's fill.
fn phrase_score(u: &[String], p: &[String], fill: &dyn Fn(&str, usize, usize) -> Fill) -> (f32, Option<Fill>) {
    let (n, m) = (u.len(), p.len());
    if n == 0 || m == 0 {
        return (0.0, None);
    }
    let mut dp = vec![vec![(f32::MAX, None::<Fill>); m + 1]; n + 1];
    dp[0][0] = (0.0, None);
    for i in 0..=n {
        for j in 0..=m {
            let (c, f) = dp[i][j].clone();
            if c == f32::MAX {
                continue;
            }
            if i < n && dp[i + 1][j].0 > c + 1.0 {
                dp[i + 1][j] = (c + 1.0, f.clone());
            }
            if j < m && dp[i][j + 1].0 > c + 1.0 {
                dp[i][j + 1] = (c + 1.0, f.clone());
            }
            if i == n || j == m {
                continue;
            }
            match slot_name(&p[j]) {
                Some(name) => {
                    for k in 1..=MAX_SLOT_WORDS.min(n - i) {
                        let got = fill(name, i, k);
                        let nc = c + k as f32 * (1.0 - got.sim);
                        if dp[i + k][j + 1].0 > nc {
                            dp[i + k][j + 1] = (nc, Some(got));
                        }
                    }
                }
                None => {
                    let nc = c + (1.0 - word_sim(&u[i], &p[j]));
                    if dp[i + 1][j + 1].0 > nc {
                        dp[i + 1][j + 1] = (nc, f.clone());
                    }
                }
            }
        }
    }
    let (cost, f) = dp[n][m].clone();
    ((1.0 - cost / n.max(m) as f32).max(0.0), f)
}

impl Grammar {
    pub fn new(table: &[(&'static str, &[&'static str])], systems: &[&str]) -> Self {
        let mut phrases = vec![];
        for (cmd, ps) in table {
            for p in *ps {
                phrases.push((Phrase { command: cmd, text: (*p).to_string() }, normalise(p)));
            }
        }
        let builtin = phrases.len();
        Self { phrases, slots: vec![], personal: vec![], builtin }.with_slot(
            "system",
            systems.iter().map(|s| (*s, *s)),
            None,
        )
    }

    /// Values for the `{name}` slot: (a way to say it, the value it stands for). Words `free`
    /// accepts may appear in the slot without counting against the match.
    pub fn with_slot<'a>(
        mut self,
        name: &str,
        values: impl IntoIterator<Item = (&'a str, &'a str)>,
        free: Option<FreeWord>,
    ) -> Self {
        let values = values.into_iter().map(|(said, value)| (normalise(said), said.to_string(), value.to_string()));
        match self.slots.iter_mut().find(|s| s.name == name) {
            Some(s) => {
                s.values.extend(values);
                s.free = free.or(s.free);
            }
            None => self.slots.push(Slot { name: name.to_string(), values: values.collect(), free }),
        }
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
    /// An [ask-only](is_ask_only) phrase is refused, so it keeps asking. True if added.
    pub fn add_alias(&mut self, heard: &str, command: &'static str) -> bool {
        let toks = normalise(heard);
        if toks.is_empty() || is_ask_only(heard) {
            return false;
        }
        self.phrases.push((Phrase { command, text: heard.trim().to_string() }, toks));
        self.personal.push((heard.trim().to_string(), command));
        true
    }

    /// Forget a personal alias added by [`Grammar::add_alias`] (every copy of it). The grammar's own
    /// phrases are untouched, even one with the same words. True if there was one.
    pub fn remove_alias(&mut self, heard: &str) -> bool {
        let heard = heard.trim();
        let mut removed = false;
        while let Some(i) = self.personal.iter().position(|(t, _)| t == heard) {
            let (text, command) = self.personal.remove(i);
            // Aliases are appended after the grammar's own phrases.
            if let Some(j) = self.phrases.iter().rposition(|(p, _)| p.text == text && p.command == command) {
                self.phrases.remove(j);
            }
            removed = true;
        }
        removed
    }

    /// How well the words `words` fill the slot `name`.
    fn fill(&self, name: &str, words: &[String]) -> Fill {
        let text = words.join(" ");
        let Some(slot) = self.slots.iter().find(|s| s.name == name) else {
            return Fill { text, ..Fill::default() };
        };
        let said: Vec<String> = words.iter().filter(|w| !slot.free.is_some_and(|f| f(w))).cloned().collect();
        if said.is_empty() {
            return Fill { text, ..Fill::default() };
        }
        let mut best = Fill { text, ..Fill::default() };
        let joined = said.concat();
        for (toks, written, value) in &slot.values {
            // Word by word, or run together: a name the recogniser split or merged
            // ("pain night" for Painite, "multicannon" for Multi-Cannon) still comes close.
            let sim = align(&said, toks).max(word_sim(&joined, &toks.concat()));
            if sim > best.sim {
                best.sim = sim;
                best.value = value.clone();
                best.said = written.clone();
                best.exact = joined == toks.concat();
                if best.exact {
                    break;
                }
            }
        }
        best
    }

    /// All phrases scored, best first.
    pub fn rank(&self, transcript: &str) -> Vec<Match> {
        let u = normalise(transcript);
        let misheard: Option<Vec<String>> = u.iter().any(|w| MISHEARD.iter().any(|(h, _)| h == w)).then(|| {
            u.iter()
                .map(|w| MISHEARD.iter().find(|(h, _)| h == w).map(|(_, r)| r.to_string()).unwrap_or_else(|| w.clone()))
                .collect()
        });
        // Each (slot, start, length) is scored once per transcript.
        let memo = RefCell::new(HashMap::<(String, bool, usize, usize), Fill>::new());
        let alt_words = misheard.clone().unwrap_or_default();
        let fill = |alt: bool, name: &str, i: usize, k: usize| {
            let words = if alt { &alt_words } else { &u };
            memo.borrow_mut()
                .entry((name.to_string(), alt, i, k))
                .or_insert_with(|| self.fill(name, &words[i..i + k]))
                .clone()
        };
        let fill_u = |name: &str, i: usize, k: usize| fill(false, name, i, k);
        let fill_alt = |name: &str, i: usize, k: usize| fill(true, name, i, k);
        let mut v: Vec<Match> = self
            .phrases
            .iter()
            .enumerate()
            .map(|(i, (p, toks))| {
                let (mut score, mut filled) = phrase_score(&u, toks, &fill_u);
                // An ask-only transcript stays ask-only against a personal phrase too: a learned
                // "Ill stop" must not make "I'll stop" run (astromech-1c66).
                if misheard.is_some() && i >= self.builtin {
                    score = score.min(MISHEARD_CAP);
                }
                if let Some(alt) = &misheard {
                    let (s2, f2) = phrase_score(alt, toks, &fill_alt);
                    let s2 = s2.min(MISHEARD_CAP);
                    if s2 > score {
                        score = s2;
                        filled = f2;
                    }
                }
                let phrase = match &filled {
                    Some(f) => match toks.iter().find_map(|t| slot_name(t)) {
                        Some(name) => p.text.replace(&format!("{{{name}}}"), &f.said),
                        None => p.text.clone(),
                    },
                    None => p.text.clone(),
                };
                Match {
                    command: p.command,
                    phrase,
                    slot: filled.as_ref().map(|f| f.value.clone()),
                    slot_exact: filled.as_ref().is_some_and(|f| f.exact),
                    slot_text: filled.map(|f| f.text),
                    score,
                    decision: Decision::NoMatch,
                }
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
                slot_text: None,
                slot_exact: false,
                score: 0.0,
                decision: Decision::NoMatch,
            };
        };
        // When the best heard its slot exactly, another command whose slot is only a near match
        // ("Krait" against the commodity list) isn't a rival: it could only ever ask.
        let rival = |m: &&Match| m.command != best.command && !(best.slot_exact && m.slot.is_some() && !m.slot_exact);
        let runner = ranked.iter().find(rival).map(|m| m.score).unwrap_or(0.0);
        let clear = best.score - runner >= MARGIN;
        best.decision = if best.score >= EXECUTE && clear {
            Decision::Execute
        } else if best.score >= ASK {
            Decision::Ask
        } else {
            Decision::NoMatch
        };
        // A slot filled by anything but one of its exact phrasings always asks.
        if best.decision == Decision::Execute && best.slot.is_some() && !best.slot_exact {
            best.decision = Decision::Ask;
        }
        best
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

    /// A few market commodities as the core passes them (spoken form, canonical name).
    fn trade() -> Grammar {
        g().with_slot(
            "commodity",
            [
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
            None,
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
        // "Did you mean …?" names the slot the way it was said.
        assert_eq!(g.best("Where can I sell LTDs?").phrase, "where can i sell LTDs");
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
        for t in [
            "Where should I sell my cargo?",
            "sell my cargo",
            "Where to sell my cargo.",
            "Best place to sell the cargo",
        ] {
            let m = g.best(t);
            assert_eq!(
                (m.command, m.slot.as_deref(), m.decision),
                ("trade.sell_cargo", None, Decision::Execute),
                "{t}"
            );
        }
        // The status question is unchanged.
        assert_eq!((g.best("cargo").command, g.best("cargo").decision), ("status.cargo", Decision::Execute));
        // Not a commodity: at most "Did you mean …?", never a lookup on a guess.
        assert_ne!(g.best("Where can I sell the weather in Paris").decision, Decision::Execute);
        assert_eq!(g.best("What's the weather like in Paris tomorrow").decision, Decision::NoMatch);
    }

    #[test]
    fn multi_word_slots_resolve_to_their_value() {
        let g = Grammar::mvp(&["Sol", "Shinrarta Dezhra"]).with_slot(
            "service",
            [
                ("interstellar factors", "interstellar_factors"),
                ("factors", "interstellar_factors"),
                ("bartender", "bartender"),
            ],
            None,
        );
        let m = g.best("Where's the nearest Interstellar Factors?");
        assert_eq!(
            (m.command, m.slot.as_deref(), m.decision),
            ("find.service", Some("interstellar_factors"), Decision::Execute)
        );
        assert_eq!(m.slot_text.as_deref(), Some("interstellar factors"));
        assert_eq!(m.phrase, "where's the nearest interstellar factors");
        let m = g.best("nearest factors");
        assert_eq!((m.slot.as_deref(), m.decision), (Some("interstellar_factors"), Decision::Execute));
        // close but not one of its phrasings: ask
        let m = g.best("nearest interstellar factor");
        assert_eq!(
            (m.command, m.slot.as_deref(), m.decision),
            ("find.service", Some("interstellar_factors"), Decision::Ask)
        );
        // a multi-word system
        let m = g.best("plot route to Shinrarta Dezhra");
        assert_eq!(
            (m.command, m.slot.as_deref(), m.decision),
            ("plot_route", Some("Shinrarta Dezhra"), Decision::Execute)
        );
        // a slot with no values never matches well (at most a short "nearest ..." phrase is
        // offered as "Did you mean …?")
        let m = Grammar::mvp(&["Sol"]).best("nearest bartender");
        assert!(m.command != "find.service" && m.decision != Decision::Execute, "{m:?}");
    }

    #[test]
    fn free_words_ride_along_in_the_slot_text() {
        fn class_rating(w: &str) -> bool {
            w.len() == 2 && w.as_bytes()[0].is_ascii_digit()
        }
        let table: &[(&str, &[&str])] = &[("find.thing", &["nearest {thing}"])];
        let g = Grammar::new(table, &[]).with_slot(
            "thing",
            [("frame shift drive", "Frame Shift Drive")],
            Some(class_rating),
        );
        let m = g.best("nearest 6A frame shift drive");
        assert_eq!(
            (m.slot.as_deref(), m.slot_text.as_deref()),
            (Some("Frame Shift Drive"), Some("6a frameshift drive"))
        );
        assert_eq!(m.decision, Decision::Execute);
        // only free words: nothing to match
        assert_eq!(g.best("nearest 6a").decision, Decision::NoMatch);
    }

    #[test]
    fn class_and_rating_said_apart_stay_together() {
        assert_eq!(normalise("nearest 5 A FSD"), ["nearest", "5a", "fsd"]);
        assert_eq!(normalise("five a frame shift drive"), ["5a", "frameshift", "drive"]);
        assert_eq!(normalise("buy a krait"), ["buy", "krait"], "\"a\" is still filler elsewhere");
        // A rating said with "rated" or "grade" keeps its letter, as one word.
        assert_eq!(normalise("nearest A-rated FSD"), ["nearest", "arated", "fsd"]);
        assert_eq!(normalise("nearest a rated fsd"), ["nearest", "arated", "fsd"]);
        assert_eq!(normalise("an A grade FSD"), ["arated", "fsd"]);
        assert_eq!(normalise("a grade A FSD"), ["arated", "fsd"]);
        assert_eq!(normalise("class five A FSD"), ["class", "5a", "fsd"]);
        assert_eq!(normalise("Python Mk II"), normalise("python mark two"));
        assert_eq!(normalise("Kestrel Mk II"), ["kestrel", "mkii"]);
        assert_eq!(normalise("Cobra MkIII"), normalise("cobra mark 3"));
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
            ("what's my powerplay rank", "status.powerplay"),
            ("Power play rank", "status.powerplay"),
            ("How many merits?", "status.powerplay"),
            ("Who controls this system?", "status.system_power"),
            ("system powerplay", "status.system_power"),
            ("What earns merits here?", "status.merits_here"),
            ("how do I earn merits here", "status.merits_here"),
            ("merits here", "status.merits_here"),
            ("powerplay activities", "status.merits_here"),
            ("What are my merits?", "status.powerplay"),
            ("how many merits do I have", "status.powerplay"),
        ] {
            let m = g.best(t);
            assert_eq!((m.command, m.decision), (c, Decision::Execute), "{t:?} -> {m:?}");
        }
        assert_ne!(g.best("close cargo scoop").command, "panel.close");
        for (t, c) in [
            ("Nearest crystalline shards.", "sites.crystalline_shards"),
            ("where are the nearest crystalline shards", "sites.crystalline_shards"),
            ("nearest crystalline structures", "sites.crystalline_shards"),
            ("nearest brain trees", "sites.brain_trees"),
            ("nearest crashed ship", "sites.crashed_ships"),
            ("nearest geological sites", "sites.geological"),
            ("nearest raw farm", "sites.raw_farms"),
        ] {
            let m = g.best(t);
            assert_eq!((m.command, m.decision), (c, Decision::Execute), "{t:?} -> {m:?}");
        }
        assert_eq!(g.best("close cargo scoop").command, "cargo_scoop.close");
    }

    /// "sir" is a polite extra word, like "please": "Yes, sir." confirms (voice eval,
    /// astromech-bztt), and it never makes a command out of anything else.
    #[test]
    fn sir_is_filler() {
        let g = g();
        for (t, c) in [
            ("Yes, sir.", "confirm"),
            ("yes please", "confirm"),
            ("Affirmative, sir.", "confirm"),
            ("No, sir.", "cancel"),
            ("Gear down, sir.", "landing_gear.down"),
            ("All stop, sir.", "throttle.zero"),
        ] {
            let m = g.best(t);
            assert_eq!((m.command, m.decision), (c, Decision::Execute), "{t:?} -> {m:?}");
        }
        for t in ["Sir.", "Sir, yes sir, what a beautiful nebula", "Yes sir, I think the coffee is cold"] {
            let m = g.best(t);
            assert_eq!(m.decision, Decision::NoMatch, "{t:?} -> {m:?}");
        }
        // "Thank you" already asks ("you" may be "gear"); "sir" doesn't change that.
        assert_eq!(g.best("Thank you, sir."), g.best("Thank you."));
        assert_ne!(g.best("Thank you, sir.").decision, Decision::Execute);
        // Names that only contain "sir" keep it.
        assert_eq!(normalise("Sirius"), ["sirius"]);
    }

    /// "all stop" heard as "I'll stop." or "Old stopper." (voice eval, astromech-y296): close
    /// enough to ask, never to press the throttle on an everyday phrase.
    #[test]
    fn all_stop_mishearings_ask_and_never_execute() {
        let g = g();
        for t in ["I'll stop.", "I’ll stop.", "Old stopper.", "Old starper."] {
            let m = g.best(t);
            assert_eq!((m.command, m.decision), ("throttle.zero", Decision::Ask), "{t:?} -> {m:?}");
            assert_eq!(m.phrase, "all stop", "{t:?}");
            // Held by the cap, not by luck of the edit distance.
            assert!(m.score <= MISHEARD_CAP, "{t:?} -> {m:?}");
        }
        for t in ["All stop.", "Full stop.", "Zero throttle."] {
            let m = g.best(t);
            assert_eq!((m.command, m.decision), ("throttle.zero", Decision::Execute), "{t:?} -> {m:?}");
        }
        // Everyday sentences with "I'll" or "old" press nothing.
        for t in [
            "I'll stop here.",
            "I'll stop for fuel later",
            "I'll be right back",
            "That's an old station",
            "old stop",
            "I'll",
            "Old.",
            "I'll take it",
            "I'll go",
        ] {
            let m = g.best(t);
            assert_ne!(m.decision, Decision::Execute, "{t:?} -> {m:?}");
        }
        // One extra word still asks, as "all stop here" does; a question presses nothing.
        for t in ["I'll stop here.", "I'll stop now", "I'll stop please"] {
            let m = g.best(t);
            assert_eq!((m.command, m.decision), ("throttle.zero", Decision::Ask), "{t:?} -> {m:?}");
            assert!(m.score <= MISHEARD_CAP, "{t:?} -> {m:?}");
        }
        assert_eq!(g.best("I'll be right back").decision, Decision::NoMatch);
    }

    #[test]
    fn route_phrases_execute_and_stay_apart() {
        let g = g();
        for (t, c) in [
            ("Next waypoint.", "route.next"),
            ("Copy the next waypoint", "route.next"),
            ("Skip waypoint", "route.skip"),
            ("previous waypoint", "route.back"),
            ("Clear route.", "route.clear"),
            ("Paste.", "galaxy_map.paste"),
            ("Paste it", "galaxy_map.paste"),
            ("next system", "status.jumps"),
            ("go back", "panel.close"),
            ("cancel", "cancel"),
        ] {
            let m = g.best(t);
            assert_eq!((m.command, m.decision), (c, Decision::Execute), "{t:?} -> {m:?}");
        }
    }

    /// "paste" heard as "Pays." (voice eval, astromech-8fey): asks, never sends Ctrl+V on its own.
    #[test]
    fn pays_asks_for_paste() {
        let g = g();
        for t in ["Pays.", "pays"] {
            let m = g.best(t);
            assert_eq!(
                (m.command, m.phrase.as_str(), m.decision),
                ("galaxy_map.paste", "paste", Decision::Ask),
                "{t:?}"
            );
            assert!(m.score <= MISHEARD_CAP, "{t:?} -> {m:?}");
        }
        assert_eq!(g.best("Paste.").decision, Decision::Execute);
        // Everyday sentences with "pays" press nothing.
        for t in ["Crime pays.", "It pays well", "Who pays the most for gold", "That pays off", "pays it"] {
            let m = g.best(t);
            assert_ne!(m.decision, Decision::Execute, "{t:?} -> {m:?}");
        }
        assert_eq!(g.best("Crime pays.").decision, Decision::NoMatch);
        assert_eq!(g.best("It pays well").decision, Decision::NoMatch);
    }

    #[test]
    fn personal_alias_executes_next_time() {
        let mut g = g();
        let m = g.best("Definitely hard points.");
        assert_eq!((m.command, m.decision), ("hardpoints.deploy", Decision::Ask));
        assert!(g.add_alias("Definitely hard points.", "hardpoints.deploy"));
        let m = g.best("Definitely hard points.");
        assert_eq!((m.command, m.decision), ("hardpoints.deploy", Decision::Execute));
    }

    /// A phrase with a word from the mishearing table only ever asks: it is never added as a
    /// personal phrase (astromech-1c66).
    #[test]
    fn ask_only_phrases_are_never_aliases() {
        let mut g = g();
        for (t, c) in [
            ("I'll stop.", "throttle.zero"),
            ("I’ll stop", "throttle.zero"),
            ("Old stopper.", "throttle.zero"),
            ("Pays.", "galaxy_map.paste"),
            ("You're down.", "landing_gear.down"),
        ] {
            assert!(is_ask_only(t), "{t:?}");
            assert!(!g.add_alias(t, c), "{t:?}");
            let m = g.best(t);
            assert_eq!((m.command, m.decision), (c, Decision::Ask), "{t:?} -> {m:?}");
        }
        for t in ["Definitely hard points.", "all stop", "paste", "Ill stop", "gear down"] {
            assert!(!is_ask_only(t), "{t:?}");
        }
    }

    /// A learned neighbour of an ask-only phrase does not make the ask-only phrase run: "Ill stop"
    /// learned, "I'll stop" still asks (astromech-1c66 review R1).
    #[test]
    fn a_learned_neighbour_does_not_unlock_an_ask_only_phrase() {
        let mut g = g();
        assert!(g.add_alias("Ill stop", "throttle.zero"));
        assert!(g.add_alias("Hold stop", "throttle.zero"));
        assert_eq!(g.best("Ill stop").decision, Decision::Execute, "the learned phrase itself runs");
        for t in ["I'll stop", "I'll stop.", "I’ll stop.", "I'll stop now", "Old stop"] {
            let m = g.best(t);
            assert_eq!((m.command, m.decision), ("throttle.zero", Decision::Ask), "{t:?} -> {m:?}");
            assert!(m.score <= MISHEARD_CAP, "{t:?} -> {m:?}");
        }
        assert!(g.add_alias("Pace it", "galaxy_map.paste"));
        assert_eq!(g.best("Pays it").decision, Decision::Ask);
    }

    /// Fuzzy near-misses of "all stop" that are not in the mishearing table stay under EXECUTE
    /// (astromech-rcbi): "ill" and "al" are one letter from "all".
    #[test]
    fn all_stop_fuzzy_near_misses_never_execute() {
        let g = g();
        for t in ["ill stop", "Ill stop.", "Al stop.", "al stop"] {
            let m = g.best(t);
            assert_eq!(m.command, "throttle.zero", "{t:?} -> {m:?}");
            assert_ne!(m.decision, Decision::Execute, "{t:?} -> {m:?}");
            assert!(m.score < EXECUTE, "{t:?} -> {m:?}");
        }
    }

    #[test]
    fn a_personal_alias_can_be_removed_and_a_builtin_phrase_cannot() {
        let mut g = g();
        g.add_alias("Definitely hard points.", "hardpoints.deploy");
        g.add_alias("Definitely hard points.", "hardpoints.deploy");
        g.add_alias("Gear down.", "landing_gear.down");
        assert_eq!(g.best("Definitely hard points.").decision, Decision::Execute);
        assert!(g.remove_alias(" Definitely hard points. "));
        let m = g.best("Definitely hard points.");
        assert_eq!(m.decision, Decision::Ask, "back to asking: both copies are gone");
        assert!(!g.remove_alias("Definitely hard points."));
        // An alias spelled like a grammar phrase leaves the grammar's phrase alone.
        assert!(g.remove_alias("Gear down."));
        let m = g.best("Gear down.");
        assert_eq!((m.command, m.decision), ("landing_gear.down", Decision::Execute));
        assert!(!g.remove_alias("lights on"), "not an alias");
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
