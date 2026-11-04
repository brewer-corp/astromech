//! Credits as spoken, one way everywhere: the session summary, the discovery callout and the
//! exploration answers (astromech-54k4). One decimal for millions and billions, dropped when it
//! is zero, so a number reads the same whichever answer says it.

/// "1.5 billion", "34.2 million", "1 million", "578 thousand", "900": the number without the
/// word credits, for "4.4 million mapped" and "1.9 million an hour". Rounds up into the next
/// unit where the shorter one would say a thousand ("999 thousand", then "1 million").
pub fn amount(c: i64) -> String {
    let tenths = |v: f64| format!("{v:.1}").trim_end_matches(".0").to_string();
    if c >= 999_950_000 {
        format!("{} billion", tenths(c as f64 / 1e9))
    } else if c >= 999_500 {
        format!("{} million", tenths(c as f64 / 1e6))
    } else if c >= 10_000 {
        format!("{:.0} thousand", c as f64 / 1e3)
    } else {
        c.to_string()
    }
}

/// "1.5 billion credits", "5.1 million credits", "578 thousand credits", "1 credit".
pub fn credits(c: i64) -> String {
    if c == 1 {
        "1 credit".into()
    } else {
        format!("{} credits", amount(c))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rounds_into_the_next_unit_at_the_boundary_and_drops_a_zero_decimal() {
        for (c, said) in [
            (1, "1 credit"),
            (900, "900 credits"),
            (9_999, "9999 credits"),
            (10_000, "10 thousand credits"),
            (578_400, "578 thousand credits"),
            (999_499, "999 thousand credits"),
            (999_500, "1 million credits"),
            (999_999, "1 million credits"),
            (1_000_000, "1 million credits"),
            (5_100_000, "5.1 million credits"),
            (34_220_000, "34.2 million credits"),
            (999_949_999, "999.9 million credits"),
            (999_950_000, "1 billion credits"),
            (1_460_000_000, "1.5 billion credits"),
            (12_000_000_000, "12 billion credits"),
        ] {
            assert_eq!(credits(c), said, "{c}");
        }
        assert_eq!(amount(4_400_000), "4.4 million");
        assert_eq!(amount(720_000), "720 thousand");
        assert_eq!(amount(1_460_000_000), "1.5 billion");
    }
}
