//! Reading rules: rewrite a line of callout text into what the TTS engine should be given.
//!
//! Kokoro phonemises with espeak-ng, which reads "BY-F" as a word, "d12-39" as noise and "19 M"
//! as "nineteen em". [`normalise`] is a pure `&str -> String` pass run on every line just before
//! it is rendered, so every spoken line benefits whatever produced it. It never changes what the
//! line says, only how it is written; the on-screen/console text stays as authored.
//!
//! Rules (each is covered by the table test below):
//! - Procedural names (`Synuefe BY-F d12-39`): the two-letter and one-letter blocks are spelled
//!   (`B Y F`); the mass code is the letter, then its numbers, with the dash spoken
//!   (`D 12 dash 39`). An upper-case letter is used because espeak says a lone capital by its
//!   letter name, whereas a lone lower-case `d` can be read as a word fragment. Pilots say the
//!   dash too, and it keeps the two numbers from running into "twelve thirty-nine".
//! - Catalogue prefixes (`HIP`, `HD`, `LHS`, `Col`, ...): initialisms are spelled, words that
//!   people say as words (HIP, Gliese, Collinder) are written as that word.
//! - Ore and commodity names: `LTD` is expanded; a few names espeak mis-stresses are respelled.
//! - Numbers: `43.5%` becomes `43.5 percent`; `19 M` becomes `19 million`; `Cr` is credits;
//!   `ly`, `ls`, `t`, `MJ` and friends become words, singular for exactly 1; ranges (`40-60%`)
//!   read `40 to 60 percent`. Thousands separators are dropped.

/// Catalogue and survey prefixes, matched whole-word and case-sensitively.
const PREFIXES: &[(&str, &str)] = &[
    ("HIP", "Hip"),
    ("HD", "H D"),
    ("HR", "H R"),
    ("LHS", "L H S"),
    ("LP", "L P"),
    ("LTT", "L T T"),
    ("GJ", "G J"),
    ("NGC", "N G C"),
    ("IC", "I C"),
    ("TYC", "T Y C"),
    ("PSR", "P S R"),
    ("2MASS", "two mass"),
    ("Col", "Collinder"),
    ("Gliese", "Gleeza"),
];

/// Ore and commodity names (case-insensitive, whole word). `LTD` is a player abbreviation.
/// Respellings are for names espeak is likely to stress or split wrongly; they are
/// best-effort and should be re-checked by ear when the voice changes.
const NAMES: &[(&str, &str)] = &[
    ("ltd", "Low Temperature Diamonds"),
    ("ltds", "Low Temperature Diamonds"),
    ("grandidierite", "Grandy-deer-ite"),
    ("serendibite", "Serren-dib-ite"),
    ("rhodplumsite", "Rodd-plum-site"),
    ("musgravite", "Musgrave-ite"),
    ("benitoite", "Ben-ee-toe-ite"),
];

/// Words after a number that scale it.
const SCALES: &[(&str, &str)] = &[("M", "million"), ("B", "billion"), ("K", "thousand"), ("k", "thousand")];

/// Units and currency after a number: (written, singular, plural).
const UNITS: &[(&str, &str, &str)] = &[
    ("%", "percent", "percent"),
    ("Cr", "credit", "credits"),
    ("CR", "credit", "credits"),
    ("ly", "light year", "light years"),
    ("ls", "light second", "light seconds"),
    ("t", "tonne", "tonnes"),
    ("MJ", "megajoule", "megajoules"),
    ("km", "kilometre", "kilometres"),
    ("m", "metre", "metres"),
    ("km/s", "kilometre per second", "kilometres per second"),
    ("m/s", "metre per second", "metres per second"),
];

/// Rewrite `text` for the speech engine. Whitespace is collapsed to single spaces.
pub fn normalise(text: &str) -> String {
    let toks: Vec<&str> = text.split_whitespace().collect();
    let mut out: Vec<String> = Vec::with_capacity(toks.len());
    let mut i = 0;
    while i < toks.len() {
        let (pre, core, post) = split_punct(toks[i]);
        i += 1;
        let mut words = if let Some((num, attached, is_range)) = number_part(core) {
            // Number, possibly with a suffix glued on ("43.5%", "12t") or in the next tokens.
            let mut suffix: Vec<&str> = Vec::new();
            let mut tail_post = post;
            if !attached.is_empty() {
                suffix.push(attached);
            } else if post.is_empty() {
                // Up to two following tokens may be scale then unit ("19 M Cr") or one unit.
                while suffix.len() < 2 && i < toks.len() {
                    let (npre, ncore, npost) = split_punct(toks[i]);
                    let ok = npre.is_empty()
                        && match suffix.len() {
                            0 => scale(ncore).is_some() || unit(ncore).is_some(),
                            _ => unit(ncore).is_some() && scale(suffix[0]).is_some(),
                        };
                    if !ok {
                        break;
                    }
                    suffix.push(ncore);
                    i += 1;
                    tail_post = npost;
                    if !npost.is_empty() {
                        break;
                    }
                }
            }
            // "98-132" is a range only when a unit follows; otherwise it is an ID ("LP 98-132").
            if is_range && !suffix.iter().any(|s| unit(s).is_some()) {
                out.push(format!("{pre}{core}{tail_post}"));
                continue;
            }
            let one = num == "1";
            let mut w = vec![num];
            let mut scaled = false;
            for s in suffix {
                if let Some(word) = scale(s) {
                    w.push(word.to_string());
                    scaled = true;
                } else if let Some((sing, plur)) = unit(s) {
                    w.push((if one && !scaled { sing } else { plur }).to_string());
                } else {
                    w.push(s.to_string());
                }
            }
            let joined = w.join(" ");
            out.push(format!("{pre}{joined}{tail_post}"));
            continue;
        } else {
            vec![word(core)]
        };
        let last = words.pop().unwrap_or_default();
        words.push(format!("{pre}{last}{post}"));
        out.extend(words);
    }
    out.join(" ")
}

fn scale(s: &str) -> Option<&'static str> {
    SCALES.iter().find(|(k, _)| *k == s).map(|(_, v)| *v)
}

fn unit(s: &str) -> Option<(&'static str, &'static str)> {
    UNITS.iter().find(|(k, _, _)| *k == s).map(|(_, a, b)| (*a, *b))
}

/// Leading `(`/quotes and trailing sentence punctuation, kept out of the rules.
fn split_punct(t: &str) -> (&str, &str, &str) {
    let start = t.len() - t.trim_start_matches(['(', '"', '\'', '[']).len();
    let rest = &t[start..];
    let end = rest.trim_end_matches(['.', ',', ';', ':', '!', '?', ')', '"', '\'', ']']).len();
    // Keep a bare "%" or similar whole: only strip when something remains.
    if end == 0 {
        return (&t[..start], "", rest);
    }
    (&t[..start], &rest[..end], &rest[end..])
}

/// Rules for a single non-numeric word.
fn word(w: &str) -> String {
    if let Some((_, v)) = PREFIXES.iter().find(|(k, _)| *k == w) {
        return v.to_string();
    }
    if let Some((_, v)) = NAMES.iter().find(|(k, _)| k.eq_ignore_ascii_case(w)) {
        return v.to_string();
    }
    let b = w.as_bytes();
    // Procedural block "BY-F".
    if b.len() == 4
        && b[0].is_ascii_uppercase()
        && b[1].is_ascii_uppercase()
        && b[2] == b'-'
        && b[3].is_ascii_uppercase()
    {
        return format!("{} {} {}", b[0] as char, b[1] as char, b[3] as char);
    }
    // Mass code "d12" or "d12-39": letter a-h, then numbers.
    if b.len() >= 2 && (b'a'..=b'h').contains(&b[0]) {
        let digits = &w[1..];
        let mut parts = digits.splitn(2, '-');
        let first = parts.next().unwrap_or("");
        let second = parts.next();
        let all_digits = |s: &str| !s.is_empty() && s.bytes().all(|c| c.is_ascii_digit());
        if all_digits(first) && second.is_none_or(all_digits) {
            let l = (b[0] as char).to_ascii_uppercase();
            return match second {
                Some(s) => format!("{l} {first} dash {s}"),
                None => format!("{l} {first}"),
            };
        }
    }
    w.to_string()
}

/// A number (or `a-b` range) at the start of a token and whatever is glued after it.
/// Returns the number text, with thousands commas removed, and the leftover suffix.
fn number_part(t: &str) -> Option<(String, &str, bool)> {
    let b = t.as_bytes();
    let (n1, e1) = scan_number(t, 0)?;
    // Range "40-60": the second number must follow directly.
    if b.get(e1) == Some(&b'-') || t[e1..].starts_with('\u{2013}') {
        let dash = if b[e1] == b'-' { 1 } else { '\u{2013}'.len_utf8() };
        if let Some((n2, e2)) = scan_number(t, e1 + dash) {
            return Some((format!("{n1} to {n2}"), &t[e2..], true));
        }
    }
    let rest = &t[e1..];
    // A glued suffix must be a known scale or unit, else this is not a quantity ("3rd", "2MASS").
    if rest.is_empty() || scale(rest).is_some() || unit(rest).is_some() {
        return Some((n1, rest, false));
    }
    None
}

/// Digits with optional single `.` fraction and `,ddd` groups, from `start`. Returns the
/// number with commas removed and the end offset.
fn scan_number(t: &str, start: usize) -> Option<(String, usize)> {
    let b = t.as_bytes();
    let mut i = start;
    let mut s = String::new();
    while i < b.len() {
        if b[i].is_ascii_digit() {
            s.push(b[i] as char);
        } else if b[i] == b','
            && i > start
            && !s.contains('.')
            && b.len() >= i + 4
            && b[i + 1..i + 4].iter().all(u8::is_ascii_digit)
            && !b.get(i + 4).is_some_and(u8::is_ascii_digit)
        {
            // Thousands separator: dropped, so espeak reads one number.
        } else if b[i] == b'.' && b.get(i + 1).is_some_and(u8::is_ascii_digit) && !s.is_empty() && !s.contains('.') {
            s.push('.');
        } else {
            break;
        }
        i += 1;
    }
    if s.is_empty() || !s.as_bytes()[0].is_ascii_digit() {
        return None;
    }
    Some((s, i))
}

#[cfg(test)]
mod tests {
    use super::normalise;

    /// Each row: line as authored, line as handed to TTS.
    const CASES: &[(&str, &str)] = &[
        // Procedural names: letters spelled, mass code as letter + digits.
        ("Synuefe BY-F d12-39", "Synuefe B Y F D 12 dash 39"),
        ("Eorld Byoi AB-C d1", "Eorld Byoi A B C D 1"),
        ("Jumping to Synuefe BY-F d12-39.", "Jumping to Synuefe B Y F D 12 dash 39."),
        ("Col 285 Sector KC-V c2-5", "Collinder 285 Sector K C V C 2 dash 5"),
        // Catalogue prefixes.
        ("HIP 12345", "Hip 12345"),
        ("HD 164595, LHS 3447", "H D 164595, L H S 3447"),
        ("LP 98-132 and 12-39 LTT 1445", "L P 98-132 and 12-39 L T T 1445"),
        ("Gliese 876", "Gleeza 876"),
        ("Col 285", "Collinder 285"),
        // Ore and commodities.
        ("Low Temperature Diamonds 43.5%", "Low Temperature Diamonds 43.5 percent"),
        ("LTD and LTDs", "Low Temperature Diamonds and Low Temperature Diamonds"),
        ("Void Opals, Painite, Bromellite", "Void Opals, Painite, Bromellite"),
        ("Grandidierite", "Grandy-deer-ite"),
        // Percentages.
        ("Painite 43.5%", "Painite 43.5 percent"),
        ("core 12 %", "core 12 percent"),
        ("yield 40-60%", "yield 40 to 60 percent"),
        ("(7%)", "(7 percent)"),
        // Large numbers and credits.
        ("worth 19 M", "worth 19 million"),
        ("worth 19M.", "worth 19 million."),
        ("1.2 B", "1.2 billion"),
        ("19 M Cr", "19 million credits"),
        ("1,234,567 Cr", "1234567 credits"),
        ("1 Cr", "1 credit"),
        ("5 K", "5 thousand"),
        // Units.
        ("12.5 ly", "12.5 light years"),
        ("1 ly away", "1 light year away"),
        ("3-5 ls", "3 to 5 light seconds"),
        ("720 ls,", "720 light seconds,"),
        ("carrying 12t", "carrying 12 tonnes"),
        ("1 t", "1 tonne"),
        ("40 MJ", "40 megajoules"),
        ("250 m/s", "250 metres per second"),
        // Left alone.
        ("3rd of 2 ships", "3rd of 2 ships"),
        ("Fuel 12 remaining", "Fuel 12 remaining"),
        ("The M type star", "The M type star"),
        ("", ""),
    ];

    #[test]
    fn reading_rules_table() {
        let mut bad = Vec::new();
        for (input, want) in CASES {
            let got = normalise(input);
            if got != *want {
                bad.push(format!("{input:?}\n   want {want:?}\n   got  {got:?}"));
            }
        }
        assert!(bad.is_empty(), "{} mismatches:\n{}", bad.len(), bad.join("\n"));
    }
}
