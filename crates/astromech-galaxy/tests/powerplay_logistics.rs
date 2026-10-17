//! Where to collect and deliver power commodities, against a recorded Spansh answer
//! (`fixtures/spansh-powerplay-reinforcement.json`: Li Yong-Rui's systems around Lembava,
//! recorded 16 Oct 2026, trimmed to the system fields).

use astromech_data::{Body, Cache, Client, Limiter, Request, Response, Transport, TransportError};
use astromech_galaxy::{Galaxy, Here, Logistics, Sphere, Store, Target};
use serde_json::Value;
use std::sync::{Arc, Mutex};

const LYR: &str = "Li Yong-Rui";
const LEMBAVA: [f64; 3] = [-43.25, -64.34375, -77.6875];

#[derive(Default)]
struct Recorded(Mutex<Vec<Value>>);

/// Li Yong-Rui's systems that Spansh should not have returned: one far beyond the search radius
/// and one that isn't in a controlled state.
fn strays() -> [Value; 2] {
    let stray = |name: &str, id: i64, state: &str, x: f64| {
        serde_json::json!({ "name": name, "id64": id, "x": x, "y": -64.3, "z": -77.7, "controlling_power": LYR,
            "power": [LYR], "power_state": state })
    };
    [stray("Too Far", 90, "Stronghold", 400.0), stray("Not Controlled", 91, "Unoccupied", -40.0)]
}

impl Transport for Recorded {
    fn send(&self, url: &str, req: &Request) -> Result<Response, TransportError> {
        assert_eq!(url, "https://spansh.co.uk/api/systems/search", "only Spansh is asked");
        let Body::Json(b) = &req.body else { panic!("{req:?}") };
        self.0.lock().unwrap().push(b.clone());
        let mut body: Value =
            serde_json::from_str(include_str!("fixtures/spansh-powerplay-reinforcement.json")).unwrap();
        body["results"].as_array_mut().unwrap().extend(strays());
        Ok(Response { status: 200, body: body.to_string() })
    }
}

fn galaxy(rec: Arc<Recorded>) -> Galaxy {
    let client = Client::with_transport(Cache::in_memory().unwrap(), rec).limiter(Limiter::none());
    let s = Store::in_memory().unwrap();
    s.add_coverage(Sphere { center: LEMBAVA, radius: 100.0 }, "test", "t").unwrap();
    Galaxy::with_remote(s, client)
}

/// An acquisition target of Li Yong-Rui's in the same fixture.
fn yj_a() -> Here {
    Here { system: "Piscium Sector YJ-A b2".into(), id64: Some(5068195440001), pos: [-46.71875, -70.5625, -85.09375] }
}

fn lookup(work: Target, at: &Here) -> (Logistics, Arc<Recorded>) {
    let rec = Arc::new(Recorded::default());
    let l = galaxy(rec.clone()).powerplay_logistics(work, LYR, at, 40.0, 10).unwrap();
    (l, rec)
}

fn names(p: &[astromech_galaxy::LogisticsPoint]) -> Vec<&str> {
    p.iter().map(|p| p.name.as_str()).collect()
}

#[test]
fn acquisition_collects_at_fortified_within_20_ly_and_strongholds_within_30() {
    let (l, rec) = lookup(Target::Acquire, &yj_a());
    let sent = rec.0.lock().unwrap()[0].clone();
    assert_eq!(sent["filters"]["controlling_power"]["value"], serde_json::json!([LYR]));
    assert_eq!(sent["filters"]["power_state"]["value"], serde_json::json!(["Fortified", "Stronghold"]));
    assert_eq!(sent["filters"]["distance"]["max"], "30");

    // Exploited NLTT 7805 supplies nothing; the rest are in range.
    assert_eq!(names(&l.collect), ["Lembava", "Pikum", "Piscium Sector BV-Y b3", "Aasgananu"]);
    assert!(l.collect.iter().all(|c| c.commodity.as_deref() == Some("Sirius Franchise Package")));
    assert!(l.collect.windows(2).all(|w| w[0].ly <= w[1].ly));
    for c in &l.collect {
        let limit = if c.state.as_deref() == Some("Fortified") { 20.0 } else { 30.0 };
        assert!(c.ly <= limit && c.limit_ly == Some(limit), "{c:?}");
    }
    assert_eq!(names(&l.deliver), ["Piscium Sector YJ-A b2"]);
    // Data for acquisition goes to the same supporting systems.
    assert_eq!(names(&l.deliver_data), names(&l.collect));
    assert!(l.warnings.iter().any(|w| w.contains("Power Contact in the target")));
}

#[test]
fn a_fortified_system_beyond_20_ly_is_not_a_source() {
    // 22 ly from Piscium Sector BV-Y b3 (Fortified), but Lembava's stronghold is within 30.
    let far = Here { system: "Far".into(), id64: Some(1), pos: [-41.6875, -63.34375, -94.47] };
    let (l, _) = lookup(Target::Acquire, &far);
    assert!(!names(&l.collect).contains(&"Piscium Sector BV-Y b3"), "{:?}", l.collect);
    assert!(names(&l.collect).contains(&"Lembava"));
}

#[test]
fn reinforcement_comes_from_other_strongholds_and_data_goes_to_the_target() {
    let at = Here {
        system: "Piscium Sector BV-Y b3".into(),
        id64: Some(7267487196553),
        pos: [-41.6875, -63.34375, -72.46875],
    };
    let (l, _) = lookup(Target::Reinforce, &at);
    assert_eq!(names(&l.collect), ["Aasgananu", "Pikum", "Lembava"]);
    assert!(l.collect.iter().all(|c| c.state.as_deref() == Some("Stronghold")));
    assert_eq!(l.collect[0].commodity.as_deref(), Some("Sirius Industrial Equipment"));
    assert_eq!(names(&l.deliver_data), ["Piscium Sector BV-Y b3"]);
}

#[test]
fn a_stronghold_does_not_supply_itself() {
    let lembava = Here { system: "Lembava".into(), id64: Some(3824408316259), pos: LEMBAVA };
    let (l, _) = lookup(Target::Reinforce, &lembava);
    assert!(!names(&l.collect).contains(&"Lembava"));
}

#[test]
fn undermining_data_goes_to_any_controlled_system() {
    let enemy = Here { system: "Piscium Sector EB-X b1-3".into(), id64: Some(2), pos: [-40.0, -66.0, -80.0] };
    let (l, _) = lookup(Target::Undermine, &enemy);
    assert!(l.collect.iter().all(|c| c.state.as_deref() == Some("Stronghold")));
    assert!(names(&l.deliver_data).contains(&"NLTT 7805"), "Exploited systems take data: {:?}", l.deliver_data);
    assert!(l.deliver_data.iter().all(|d| d.uncertain.is_some()));
    // Spansh also returned a system 360 ly away and one that isn't controlled: neither is offered.
    assert!(!names(&l.deliver_data).contains(&"Too Far") && !names(&l.deliver_data).contains(&"Not Controlled"));
    assert!(!names(&l.collect).contains(&"Too Far"));
}
