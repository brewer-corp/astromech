//! Spoken answers for the trade lookups (T0: read-only, nothing pressed). The core runs the
//! lookup (`astromech-market`, Ardent Insight) and these turn its result into one or two short
//! sentences: the station, its system and distance, the price and the demand or stock. Plain
//! text; data older than a day says how old it is.

use astromech_market::cargo::CargoPlan;
use astromech_market::{Offer, Prices, Side};

/// A trade question the grammar recognises.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ask {
    /// "Where can I sell painite?"
    Sell,
    /// "Where can I buy tritium?"
    Buy,
    /// "Where should I sell my cargo?"
    SellCargo,
}

impl Ask {
    pub fn side(self) -> Side {
        match self {
            Ask::Sell | Ask::SellCargo => Side::Sell,
            Ask::Buy => Side::Buy,
        }
    }

    /// The id the app remembers this question's filters under (voice uses the same ones).
    pub fn lookup(self) -> &'static str {
        match self {
            Ask::Sell | Ask::Buy => self.side().lookup(),
            Ask::SellCargo => astromech_market::cargo::LOOKUP,
        }
    }
}

/// The trade question behind an `astromech-intent` command id.
pub fn from_command(id: &str) -> Option<Ask> {
    match id {
        "trade.sell" => Some(Ask::Sell),
        "trade.buy" => Some(Ask::Buy),
        "trade.sell_cargo" => Some(Ask::SellCargo),
        _ => None,
    }
}

/// What the voice tester shows instead of running the lookup.
pub fn describe(ask: Ask, commodity: Option<&str>) -> String {
    let what = commodity.unwrap_or("that");
    match ask {
        Ask::Sell => format!("Looks up where to sell {what} near you on Ardent Insight."),
        Ask::Buy => format!("Looks up where to buy {what} near you on Ardent Insight."),
        Ask::SellCargo => "Looks up where to sell the cargo in your hold near you on Ardent Insight.".into(),
    }
}

/// "211,587".
pub fn thousands(n: i64) -> String {
    let s = n.unsigned_abs().to_string();
    let mut out = String::new();
    for (i, c) in s.chars().enumerate() {
        if i > 0 && (s.len() - i).is_multiple_of(3) {
            out.push(',');
        }
        out.push(c);
    }
    if n < 0 {
        format!("-{out}")
    } else {
        out
    }
}

fn light_years(ly: f64) -> String {
    let n = ly.round() as i64;
    if n == 1 {
        "1 light year".into()
    } else {
        format!("{n} light years")
    }
}

/// " Reported 3 days ago." for data older than a day, else "".
fn age(updated_at: &str, now_s: i64) -> String {
    let Ok(t) = chrono::DateTime::parse_from_rfc3339(updated_at) else {
        return String::new();
    };
    let days = (now_s - t.timestamp()) / 86_400;
    match days {
        ..=0 => String::new(),
        1 => " Reported yesterday.".into(),
        d => format!(" Reported {d} days ago."),
    }
}

/// Where the best offer is: "Good Landing in LHS 1827, 19 light years".
pub fn place(o: &Offer) -> String {
    format!("{} in {}, {}", o.station, o.system, light_years(o.hop.ly))
}

/// The answer to "where can I sell / buy `name`?" from the lookup's result, searched within
/// `max_ly`. `name` is the commodity as the commander knows it ("Low Temperature Diamonds").
pub fn answer(ask: Ask, name: &str, p: &Prices, max_ly: u32, now_s: i64) -> String {
    let Some(best) = p.offers.first() else {
        return match ask {
            Ask::Buy => format!("No market within {max_ly} light years sells {name} with your filters."),
            _ => format!("No market within {max_ly} light years buys {name} with your filters."),
        };
    };
    let price = thousands(best.price);
    let volume = thousands(best.volume);
    let s = match ask {
        Ask::Buy => format!("Cheapest {name}: {}. {price} credits, {volume} in stock.", place(best)),
        _ => format!("Best price for {name}: {}. {price} credits, demand {volume}.", place(best)),
    };
    s + &age(&best.updated_at, now_s)
}

/// The answer to "where should I sell my cargo?": the best station for the whole load, what
/// it pays and how much of the load it takes.
pub fn cargo_answer(p: &CargoPlan, max_ly: u32, now_s: i64) -> String {
    let carried: i64 = p.load.iter().map(|l| l.tonnes).sum();
    if carried == 0 {
        return if p.kept.is_empty() {
            "Your hold is empty.".into()
        } else {
            "Nothing in your hold to sell: mission cargo, stolen goods and limpets are left out.".into()
        };
    }
    let Some(best) = p.stations.first() else {
        return format!("No market within {max_ly} light years buys your cargo with your filters.");
    };
    let what = match p.load.as_slice() {
        [one] => one.name.clone(),
        _ => "your cargo".into(),
    };
    let share = if best.sold == carried {
        format!("all {} tonnes", thousands(carried))
    } else {
        format!("{} of {} tonnes", thousands(best.sold), thousands(carried))
    };
    let s = format!(
        "Best for {what}: {} in {}, {}. {} credits for {share}.",
        best.station,
        best.system,
        light_years(best.hop.ly),
        thousands(best.total)
    );
    s + &age(&best.updated_at, now_s) + &partial_buyers(p)
}

/// Partial-buyer sentences a cargo answer adds at most: the hold's most valuable ones.
const PARTIAL_BUYERS_SPOKEN: usize = 2;

/// For the commodities no market takes whole (when the hold has more than one): the market that
/// pays the most for what it does take, as the Trade page's "My cargo" shows it. Only the
/// [`PARTIAL_BUYERS_SPOKEN`] worth the most are said, and a station the answer already named is
/// not named again.
fn partial_buyers(p: &CargoPlan) -> String {
    if p.load.len() < 2 {
        return String::new();
    }
    let mut partial: Vec<_> =
        p.per_commodity.iter().filter_map(|c| c.partial.as_ref().map(|o| (c, o, o.volume.min(c.carried)))).collect();
    partial.sort_by_key(|(_, o, takes)| std::cmp::Reverse(takes * o.price));
    let named = p.stations.first().map(|s| s.market_id);
    let mut out = String::new();
    for (c, o, takes) in partial.into_iter().take(PARTIAL_BUYERS_SPOKEN) {
        let at = if Some(o.market_id) == named {
            "the same station".to_string()
        } else {
            format!("{} in {}", o.station, o.system)
        };
        out += &format!(
            " {} partly sells at {at}: {} of {} tonnes, {} credits each.",
            c.name,
            thousands(takes),
            thousands(c.carried),
            thousands(o.price)
        );
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbers_and_empty_answers_read_plainly() {
        assert_eq!(
            (thousands(0), thousands(999), thousands(1000), thousands(211_587)),
            ("0".into(), "999".into(), "1,000".into(), "211,587".into())
        );
        assert_eq!(thousands(-1_234_567), "-1,234,567");
        let none =
            Prices { commodity: "painite".into(), side: Side::Sell, fetched_at: 0, truncated: false, offers: vec![] };
        assert_eq!(
            answer(Ask::Sell, "Painite", &none, 40, 0),
            "No market within 40 light years buys Painite with your filters."
        );
        assert_eq!(
            answer(Ask::Buy, "Painite", &none, 100, 0),
            "No market within 100 light years sells Painite with your filters."
        );
        assert_eq!(
            (from_command("trade.sell"), from_command("trade.buy"), from_command("status.cargo")),
            (Some(Ask::Sell), Some(Ask::Buy), None)
        );
    }
}
