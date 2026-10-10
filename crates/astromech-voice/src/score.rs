//! Transcript scoring: normalisation and word error rate.

use serde::Serialize;

const ONES: [&str; 20] = [
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];
const TENS: [&str; 10] = ["", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"];

fn number_words(n: u32) -> Option<Vec<String>> {
    match n {
        0..=19 => Some(vec![ONES[n as usize].to_string()]),
        20..=99 => {
            let mut v = vec![TENS[(n / 10) as usize].to_string()];
            if !n.is_multiple_of(10) {
                v.push(ONES[(n % 10) as usize].to_string());
            }
            Some(v)
        }
        100 => Some(vec!["one".into(), "hundred".into()]),
        _ => None,
    }
}

/// Lowercase, turn `%` into "percent", drop punctuation, spell out 0..=100, split hyphenated
/// number words ("fifty-five" -> "fifty five").
pub fn normalize(s: &str) -> Vec<String> {
    let s = s.to_lowercase().replace('%', " percent ");
    let cleaned: String = s.chars().map(|c| if c.is_alphanumeric() || c == '\'' { c } else { ' ' }).collect();
    let mut out = Vec::new();
    for w in cleaned.split_whitespace() {
        let w = w.trim_matches('\'');
        if w.is_empty() {
            continue;
        }
        match w.parse::<u32>().ok().and_then(number_words) {
            Some(ws) => out.extend(ws),
            None => out.push(w.to_string()),
        }
    }
    out
}

fn edit_distance(a: &[String], b: &[String]) -> usize {
    let mut prev: Vec<usize> = (0..=b.len()).collect();
    for (i, x) in a.iter().enumerate() {
        let mut cur = vec![i + 1; b.len() + 1];
        for (j, y) in b.iter().enumerate() {
            cur[j + 1] = (prev[j] + usize::from(x != y)).min(prev[j + 1] + 1).min(cur[j] + 1);
        }
        prev = cur;
    }
    prev[b.len()]
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Score {
    pub exact: bool,
    /// Same words once spaces are ignored ("hard points" == "hardpoints").
    pub loose: bool,
    pub word_errors: usize,
    pub ref_words: usize,
    pub wer: f64,
}

pub fn score(expected: &str, got: &str) -> Score {
    let r = normalize(expected);
    let h = normalize(got);
    let e = edit_distance(&r, &h);
    let loose = r.concat() == h.concat();
    Score {
        exact: e == 0,
        loose,
        word_errors: e,
        ref_words: r.len(),
        wer: if r.is_empty() { 0.0 } else { e as f64 / r.len() as f64 },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalisation() {
        assert_eq!(normalize("Set speed to 50%."), ["set", "speed", "to", "fifty", "percent"]);
        assert_eq!(normalize("Plot route to Sol!"), ["plot", "route", "to", "sol"]);
        assert_eq!(normalize(" twenty-one "), ["twenty", "one"]);
        assert_eq!(normalize("21"), ["twenty", "one"]);
        assert_eq!(normalize("don't"), ["don't"]);
    }

    #[test]
    fn scoring() {
        assert!(score("set speed to fifty percent", "Set speed to 50%.").exact);
        let s = score("plot route to Sol", "plot root to sole");
        assert_eq!((s.word_errors, s.ref_words), (2, 4));
        assert_eq!(s.wer, 0.5);
        assert!(!score("deploy hardpoints", "Deploy hard points.").exact);
        assert!(score("deploy hardpoints", "Deploy hard points.").loose);
        let s = score("lights on", "");
        assert_eq!(s.word_errors, 2);
        let s = score("lights on", "the lights are on");
        assert_eq!(s.word_errors, 2);
    }
}
