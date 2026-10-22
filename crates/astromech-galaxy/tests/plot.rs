//! Plotting neutron and exact routes for the current ship, on Spansh jobs recorded on 16 and
//! 17 Oct 2026 (astromech-data's fixtures: Sol to Colonia by neutron, Sol to Maia exact). No
//! test here touches the network.

use astromech_data::spansh::Polling;
use astromech_data::{Body, Cache, Client, Limiter, Request, Response, Transport, TransportError};
use astromech_engineering::{Fleet, FleetBuilder};
use astromech_galaxy::plot::{plot, Ask, Plotter, Ship};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::time::Duration;

macro_rules! fixture {
    ($name:literal) => {
        include_str!(concat!("../../astromech-data/tests/fixtures/", $name))
    };
}

/// Answers the submit, then each poll in turn; logs every request.
struct Recorded {
    submit: (u16, &'static str),
    polls: Mutex<VecDeque<(u16, &'static str)>>,
    log: Mutex<Vec<(String, Request)>>,
}

impl Recorded {
    fn new(submit: (u16, &'static str), polls: &[(u16, &'static str)]) -> Arc<Self> {
        Arc::new(Recorded { submit, polls: Mutex::new(polls.iter().copied().collect()), log: Mutex::default() })
    }

    fn form(&self) -> Vec<(String, String)> {
        match &self.log.lock().unwrap()[0].1.body {
            Body::Form(f) => f.clone(),
            b => panic!("the submit is a form post, not {b:?}"),
        }
    }
}

impl Transport for Recorded {
    fn send(&self, url: &str, req: &Request) -> Result<Response, TransportError> {
        self.log.lock().unwrap().push((url.to_string(), req.clone()));
        let (status, body) = if url.contains("/api/results/") {
            self.polls.lock().unwrap().pop_front().expect("no more polls recorded")
        } else {
            self.submit
        };
        Ok(Response { status, body: body.to_string() })
    }
}

fn client(rec: &Arc<Recorded>) -> Client {
    Client::with_transport(Cache::in_memory().unwrap(), rec.clone()).limiter(Limiter::none())
}

const NO_WAIT: Polling = Polling { every: Duration::ZERO, timeout: Duration::from_secs(5) };

/// A fleet whose current ship is the synthetic Asp Explorer loadout (FSD optimal mass 1470 t
/// from its `Modifiers`, 32 t tank).
fn asp_fleet() -> Fleet {
    let raw = include_str!("../../astromech-engineering/tests/links/loadout-asp.json");
    let line: serde_json::Value = serde_json::from_str(raw).unwrap();
    let l = astromech_journal::parse_line(&line.to_string()).unwrap();
    let mut b = FleetBuilder::default();
    b.apply(&l.timestamp, &l.event);
    b.build("test")
}

fn ask(plotter: Plotter, to: &str) -> Ask {
    Ask { plotter, from: "Sol".into(), to: to.into(), efficiency: 60, refuel_every_scoopable: true }
}

#[test]
fn the_current_ship_gives_the_range_and_drive() {
    let fleet = asp_fleet();
    let ship = Ship::current(&fleet, 0.0).unwrap();
    assert_eq!(ship.name, "Fixture Asp");
    assert_eq!((ship.fsd.optimal_mass, ship.fsd.max_fuel_per_jump, ship.fsd.tank_size), (1470.0, 5.0, 32.0));
    let s = &fleet.ships[0];
    let jump = astromech_ship::stats(&s.ship_type, &s.loadout).unwrap().jump.unwrap();
    assert!((ship.range - jump.unladen).abs() < 1e-9, "empty hold: the full-tank unladen range");
    assert!(Ship::current(&fleet, 16.0).unwrap().range < ship.range, "cargo shortens the jump");

    let empty = FleetBuilder::default().build("test");
    assert_eq!(Ship::current(&empty, 0.0).unwrap_err(), "No ship in the journal yet. Board one in game.");
}

#[test]
fn an_exact_route_has_every_jump_and_its_refuel_stops() {
    let rec = Recorded::new(
        (202, fixture!("spansh-exact-submit.json")),
        &[(202, fixture!("spansh-exact-pending.json")), (200, fixture!("spansh-exact-result.json"))],
    );
    let ship = Ship::current(&asp_fleet(), 0.0).unwrap();
    let p = plot(&client(&rec), &ask(Plotter::Exact, "Maia"), &ship, NO_WAIT).unwrap();

    // The drive went to Spansh as numbers from the loadout.
    let form = rec.form();
    let field = |k: &str| form.iter().find(|(f, _)| f == k).map(|(_, v)| v.clone()).unwrap();
    assert_eq!(rec.log.lock().unwrap()[0].0, "https://spansh.co.uk/api/generic/route");
    assert_eq!((field("source"), field("destination")), ("Sol".into(), "Maia".into()));
    assert_eq!(
        (field("optimal_mass"), field("max_fuel_per_jump"), field("tank_size")),
        ("1470".into(), "5".into(), "32".into())
    );
    assert_eq!((field("fuel_multiplier"), field("fuel_power")), ("0.012".into(), "2.45".into()));
    assert_eq!(field("supercharge_multiplier"), "4", "the Asp's ordinary drive");
    assert_eq!(
        (field("internal_tank_size"), field("cargo"), field("refuel_every_scoopable")),
        ("0.63".into(), "0".into(), "1".into())
    );
    let base: f64 = field("base_mass").parse().unwrap();
    assert!((base - (ship.fsd.base_mass * 100.0).round() / 100.0).abs() < 1e-9);

    assert_eq!((p.name.as_str(), p.source), ("Sol to Maia", "exact plotter"));
    assert_eq!(p.waypoints.len(), 10);
    assert_eq!(p.jumps, 9);
    assert_eq!((p.refuels, p.neutrons), (3, 1));
    assert!((p.ly - 452.0).abs() < 1.0, "{}", p.ly);
    let labels: Vec<(&str, Option<&str>)> =
        p.waypoints.iter().map(|w| (w.system.as_str(), w.label.as_deref())).filter(|(_, l)| l.is_some()).collect();
    assert_eq!(
        labels,
        [
            ("Tascheter Sector MN-T b3-6", Some("fuel")),
            ("Hyades Sector DB-W b2-6", Some("fuel")),
            ("98 k Tauri", Some("neutron boost and fuel")),
        ]
    );
    assert_eq!(p.waypoints.last().unwrap().system, "Maia");
}

#[test]
fn a_neutron_route_stops_at_the_neutron_stars() {
    let rec = Recorded::new(
        (202, fixture!("spansh-route-queued.json")),
        &[(202, fixture!("spansh-route-pending.json")), (200, fixture!("spansh-route-result.json"))],
    );
    let mut ship = Ship::current(&asp_fleet(), 0.0).unwrap();
    assert_eq!(ship.fsd.supercharge_multiplier, 4);
    ship.fsd.supercharge_multiplier = 6; // as with a Mk II drive
    let p = plot(&client(&rec), &ask(Plotter::Neutron, "Colonia"), &ship, NO_WAIT).unwrap();

    let form = rec.form();
    assert!(form.contains(&("range".into(), format!("{}", (ship.range * 100.0).round() / 100.0))), "{form:?}");
    assert!(form.contains(&("efficiency".into(), "60".into())));
    assert!(form.contains(&("supercharge_multiplier".into(), "6".into())), "the drive's boost: {form:?}");
    assert_eq!(p.source, "neutron plotter");
    assert_eq!(p.jumps, 128, "Spansh's total, not the stops");
    let stops: Vec<(&str, Option<&str>)> =
        p.waypoints.iter().map(|w| (w.system.as_str(), w.label.as_deref())).collect();
    assert_eq!(stops, [("Sol", None), ("PSR J1752-2806", Some("neutron boost")), ("Colonia", None)]);
}

#[test]
fn plot_failures_are_sentences() {
    let ship = Ship::current(&asp_fleet(), 0.0).unwrap();
    let rec = Recorded::new((400, r#"{"error":"Could not find finishing system"}"#), &[]);
    let e = plot(&client(&rec), &ask(Plotter::Neutron, "Nowhere"), &ship, NO_WAIT).unwrap_err();
    assert_eq!(e, "Spansh turned the request down: Could not find finishing system.");

    let rec =
        Recorded::new((202, fixture!("spansh-exact-submit.json")), &[(500, fixture!("spansh-exact-failed.json"))]);
    let e = plot(&client(&rec), &ask(Plotter::Exact, "Nowhere"), &ship, NO_WAIT).unwrap_err();
    assert_eq!(e, "Spansh turned the request down: Unable to find route.");

    let rec = Recorded::new((202, "{}"), &[]);
    assert_eq!(
        plot(&client(&rec), &ask(Plotter::Exact, " "), &ship, NO_WAIT).unwrap_err(),
        "Enter where the route starts and where it ends."
    );
    assert!(rec.log.lock().unwrap().is_empty(), "nothing sent without a destination");
}
