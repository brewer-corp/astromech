//! Colonisation questions by voice (read-only, tier 0): "how is the build going", "what is left
//! to deliver", "where can I buy steel for the depot". The core replays the journals into the
//! tracker's projects (`astromech_colonisation::from_journal_dir`, what the app's Colonisation
//! page shows) and, for where to buy, runs the page's Ardent lookup
//! (`astromech_market::list::plan_search`, filters remembered as `colonisation.buy`); this picks
//! the project and turns the results into short spoken answers.

use astromech_colonisation::shopping::commodity_id;
use astromech_colonisation::{Project, ProjectView};
use astromech_market::list::{ListPlan, Need};

/// A colonisation question the grammar recognises.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ask {
    /// "How is the build going?"
    Progress,
    /// "What is left to deliver?"
    Remaining,
    /// "Where can I buy steel for the depot?"
    Buy,
}

/// The colonisation question behind an `astromech-intent` command id. `colonisation.buy` is
/// also the key the app remembers the where-to-buy filters under.
pub fn from_command(command: &str) -> Option<Ask> {
    match command {
        "colonisation.progress" => Some(Ask::Progress),
        "colonisation.remaining" => Some(Ask::Remaining),
        astromech_market::list::LOOKUP => Some(Ask::Buy),
        _ => None,
    }
}

/// Said when no project is being built.
pub const NO_PROJECT: &str = "No construction project in progress.";

/// Said when the journal has not given the commander's position yet.
pub const NO_POSITION: &str = "Your position is not known yet. It comes from the next jump or login.";

/// Said when the galaxy store could not be opened.
pub const NO_STAR_MAP: &str = "Market lookups are not available: the star map could not be opened.";

/// What the voice tester shows instead of reading the journals.
pub fn describe(ask: Ask, commodity: Option<&str>) -> String {
    match ask {
        Ask::Progress => "Reads the construction project from your journals and says how far along it is.".into(),
        Ask::Remaining => "Reads the construction project from your journals and says what is left to deliver.".into(),
        Ask::Buy => format!(
            "Looks up where to buy {} for the construction project near you on Ardent Insight.",
            commodity.unwrap_or("that")
        ),
    }
}

/// The project a question is about: of those still being built, the one docked at
/// (`docked_market`), else the one last delivered to, else the one the game updated last.
pub fn pick(projects: &[ProjectView], docked_market: Option<i64>) -> Option<&ProjectView> {
    let building = || projects.iter().filter(|v| !v.project.complete && !v.project.failed);
    let last_delivery = |v: &&ProjectView| v.project.contributions.iter().map(|c| c.timestamp.clone()).max();
    building()
        .find(|v| Some(v.project.market_id) == docked_market)
        .or_else(|| building().filter(|v| last_delivery(v).is_some()).max_by_key(last_delivery))
        .or_else(|| building().max_by(|a, b| a.project.depot_updated.cmp(&b.project.depot_updated)))
}

/// The project as said: the station name after its "Planetary Construction Site: " kind.
pub fn spoken_name(p: &Project) -> String {
    match p.station_name.as_deref().filter(|n| !n.starts_with('$')) {
        Some(n) => n.rsplit_once(": ").map_or(n, |(_, name)| name).to_string(),
        None => match &p.system_name {
            Some(s) => format!("The site in {s}"),
            None => "The construction site".into(),
        },
    }
}

/// "12,400".
fn thousands(n: i64) -> String {
    crate::trade::thousands(n)
}

fn tonnes(n: i64) -> String {
    if n == 1 {
        "1 tonne".into()
    } else {
        format!("{} tonnes", thousands(n))
    }
}

/// "Test Hub is 50 percent built. 300 tonnes left to deliver."
pub fn progress(v: &ProjectView) -> String {
    let p = &v.project;
    let pct = (p.progress * 100.0).floor().clamp(0.0, 100.0);
    let mut s = format!("{} is {pct:.0} percent built.", spoken_name(p));
    let left = p.remaining_total();
    if left > 0 {
        s += &format!(" {} left to deliver.", tonnes(left));
    }
    s
}

/// What is left: tonnes to deliver, what is aboard for it, the loads still to buy at the
/// current ship's capacity, and the commodity most needed.
pub fn remaining(v: &ProjectView) -> String {
    let p = &v.project;
    let list = &v.shopping;
    let left = p.remaining_total();
    if left == 0 {
        return format!("{} needs nothing more.", spoken_name(p));
    }
    let mut s = format!("{} left to deliver to {}.", tonnes(left), spoken_name(p));
    let aboard: i64 = list.lines.iter().map(|l| l.in_cargo).sum();
    if aboard > 0 {
        s += &format!(" {} of it aboard.", tonnes(aboard));
    }
    if let (Some(loads), Some(cap)) = (list.loads.filter(|l| *l > 0), list.capacity) {
        let n = if loads == 1 { "1 more load".to_string() } else { format!("{loads} more loads") };
        s += &format!(" {n} at {}.", tonnes(cap));
    }
    if let Some(top) = list.lines.first().filter(|l| l.to_buy > 0) {
        s += &format!(" Most needed: {}, {}.", line_name(top), tonnes(top.to_buy));
    }
    s
}

fn line_name(l: &astromech_colonisation::shopping::Line) -> String {
    l.name_localised.clone().unwrap_or_else(|| astromech_market::catalog::name_of(&l.symbol))
}

/// What to search for when the commander asks where to buy `commodity` (a catalogue name) for
/// the project: its tonnes still to buy, or why there is nothing to search for.
pub fn need(v: &ProjectView, commodity: &str) -> Result<Need, String> {
    let Some(c) = astromech_market::catalog::find(commodity) else {
        return Err("Which commodity? Name one the project needs.".into());
    };
    let line = v.shopping.lines.iter().find(|l| l.symbol == commodity_id(&c.symbol));
    match line {
        Some(l) if l.to_buy > 0 => Ok(Need { symbol: l.symbol.clone(), name: c.name.clone(), tonnes: l.to_buy }),
        Some(_) => Err(format!("{} for {} is already aboard.", c.name, spoken_name(&v.project))),
        None => Err(format!("{} needs no more {}.", spoken_name(&v.project), c.name)),
    }
}

/// Where to buy `need`, from the page's lookup (stations nearest first): the nearest station
/// stocking all of it, else the one stocking the most, with its stock and price, and the data's
/// age when older than a day.
pub fn buy(need: &Need, plan: &ListPlan, max_ly: u32, now_s: i64) -> String {
    let still = format!("{} of {} still to buy.", tonnes(need.tonnes), need.name);
    let sellers = || plan.stations.iter().filter_map(|s| s.lines.first().map(|l| (s, l)));
    let best = sellers().find(|(_, l)| l.stock >= need.tonnes).or_else(|| sellers().max_by_key(|(_, l)| l.stock));
    let Some((st, line)) = best else {
        return format!("{still} No market within {max_ly} light years sells it with your filters.");
    };
    let ly = st.hop.ly.round() as i64;
    let far = if ly == 1 { "1 light year".to_string() } else { format!("{ly} light years") };
    let mut s = format!(
        "{still} Nearest: {} in {}, {far}. {} in stock, {} credits.",
        st.station,
        st.system,
        thousands(line.stock),
        thousands(line.price)
    );
    if let Ok(t) = chrono::DateTime::parse_from_rfc3339(&st.updated_at) {
        match (now_s - t.timestamp()) / 86_400 {
            ..=0 => {}
            1 => s += " Reported yesterday.",
            d => s += &format!(" Reported {d} days ago."),
        }
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;
    use astromech_colonisation::shopping::{shopping_list, Hold};
    use astromech_colonisation::{replay_lines, Tracker};

    /// Two projects: Test Hub (docked there first, 50% built), then Far Port (delivered to last).
    const JOURNAL: &str = r#"
{"timestamp":"2026-10-01T09:00:00Z","event":"Loadout","Ship":"type9","ShipID":7,"CargoCapacity":400,"Modules":[]}
{"timestamp":"2026-10-01T10:00:00Z","event":"Docked","StationName":"Planetary Construction Site: Test Hub","StationType":"SurfaceStation","StarSystem":"Test System","SystemAddress":1001,"MarketID":3900000001}
{"timestamp":"2026-10-01T10:30:00Z","event":"ColonisationConstructionDepot","MarketID":3900000001,"ConstructionProgress":0.5,"ConstructionComplete":false,"ConstructionFailed":false,"ResourcesRequired":[{"Name":"$steel_name;","Name_Localised":"Steel","RequiredAmount":800,"ProvidedAmount":500,"Payment":4000},{"Name":"$aluminium_name;","Name_Localised":"Aluminium","RequiredAmount":200,"ProvidedAmount":50,"Payment":3000}]}
{"timestamp":"2026-10-01T10:40:00Z","event":"ColonisationContribution","MarketID":3900000001,"Contributions":[{"Name":"$steel_name;","Amount":100}]}
{"timestamp":"2026-10-02T10:00:00Z","event":"Docked","StationName":"Orbital Construction Site: Far Port","StationType":"SpaceConstructionDepot","StarSystem":"Far System","SystemAddress":1003,"MarketID":3900000003}
{"timestamp":"2026-10-02T10:00:05Z","event":"ColonisationConstructionDepot","MarketID":3900000003,"ConstructionProgress":0.1,"ConstructionComplete":false,"ConstructionFailed":false,"ResourcesRequired":[{"Name":"$cmmcomposite_name;","Name_Localised":"CMM Composite","RequiredAmount":5000,"ProvidedAmount":100,"Payment":6000}]}
{"timestamp":"2026-10-02T10:10:00Z","event":"ColonisationContribution","MarketID":3900000003,"Contributions":[{"Name":"$cmmcomposite_name;","Amount":100}]}
"#;

    fn views(cargo: &[(&str, i64)]) -> Vec<ProjectView> {
        let mut t = Tracker::new();
        replay_lines(&mut t, JOURNAL.lines().filter(|l| !l.trim().is_empty()));
        let mut hold = Hold { capacity: Some(400), ..Hold::default() };
        hold.cargo = cargo.iter().map(|(n, c)| (n.to_string(), *c)).collect();
        t.projects().map(|p| ProjectView { project: p.clone(), shopping: shopping_list(p, &hold) }).collect()
    }

    #[test]
    fn the_project_is_the_one_docked_at_else_the_last_delivered_to() {
        let v = views(&[]);
        let name = |p: Option<&ProjectView>| p.map(|v| spoken_name(&v.project));
        assert_eq!(name(pick(&v, Some(3900000001))), Some("Test Hub".into()));
        assert_eq!(name(pick(&v, None)), Some("Far Port".into()), "delivered to last");
        assert_eq!(name(pick(&v, Some(42))), Some("Far Port".into()), "docked elsewhere");
        let mut done = v.clone();
        done.iter_mut().for_each(|v| v.project.complete = true);
        assert_eq!(name(pick(&done, None)), None);
    }

    #[test]
    fn progress_and_what_is_left() {
        let v = views(&[("steel", 120)]);
        let hub = pick(&v, Some(3900000001)).unwrap();
        assert_eq!(progress(hub), "Test Hub is 50 percent built. 450 tonnes left to deliver.");
        // 300 steel and 150 aluminium left; 120 steel aboard; 330 to buy at 400 a load.
        assert_eq!(
            remaining(hub),
            "450 tonnes left to deliver to Test Hub. 120 tonnes of it aboard. 1 more load at 400 tonnes. \
             Most needed: Steel, 180 tonnes."
        );
        let port = pick(&v, None).unwrap();
        assert_eq!(
            remaining(port),
            "4,900 tonnes left to deliver to Far Port. 13 more loads at 400 tonnes. Most needed: CMM Composite, 4,900 tonnes."
        );
    }

    #[test]
    fn where_to_buy_needs_a_commodity_the_project_still_needs() {
        let v = views(&[("aluminium", 150)]);
        let hub = pick(&v, Some(3900000001)).unwrap();
        assert_eq!(need(hub, "Steel"), Ok(Need { symbol: "steel".into(), name: "Steel".into(), tonnes: 300 }));
        assert_eq!(need(hub, "Aluminium"), Err("Aluminium for Test Hub is already aboard.".into()));
        assert_eq!(need(hub, "Gold"), Err("Test Hub needs no more Gold.".into()));
    }

    #[test]
    fn names_without_a_station() {
        let mut v = views(&[]);
        v[0].project.station_name = None;
        assert_eq!(spoken_name(&v[0].project), "The site in Test System");
        v[0].project.system_name = None;
        assert_eq!(spoken_name(&v[0].project), "The construction site");
    }

    #[test]
    fn commands() {
        assert_eq!(from_command("colonisation.progress"), Some(Ask::Progress));
        assert_eq!(from_command("colonisation.remaining"), Some(Ask::Remaining));
        assert_eq!(from_command("colonisation.buy"), Some(Ask::Buy));
        assert_eq!(from_command("trade.buy"), None);
    }
}
