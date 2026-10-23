//! Which plotter a route uses (astromech_galaxy::plot::plot_route): a neutron route is plotted on
//! this PC when the neutron star file is there and readable, else by Spansh (a recorded job),
//! and a file that can't be read is said. The kept file is loaded again when a download replaces
//! it. No test here touches the network.

use astromech_data::spansh::Polling;
use astromech_data::{Cache, Client, Limiter, Request, Response, Transport, TransportError};
use astromech_galaxy::neutron::{Kept, FILE_NAME};
use astromech_galaxy::plot::{plot_route, Ask, Plotter, Ship, NEUTRON};
use astromech_galaxy::store::Store;
use astromech_galaxy::{Galaxy, Here};
use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn fixture(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures").join(name)
}

macro_rules! spansh_fixture {
    ($name:literal) => {
        include_str!(concat!("../../astromech-data/tests/fixtures/", $name))
    };
}

/// Spansh: the system names it knows (autocomplete), and a neutron route job recorded on
/// 16 Oct 2026 (Sol to Colonia). Logs every request.
struct Spansh {
    names: &'static str,
    jobs: Mutex<VecDeque<(u16, &'static str)>>,
    log: Mutex<Vec<String>>,
}

impl Spansh {
    fn new(names: &'static str) -> Arc<Self> {
        let jobs = [
            (202, spansh_fixture!("spansh-route-queued.json")),
            (202, spansh_fixture!("spansh-route-pending.json")),
            (200, spansh_fixture!("spansh-route-result.json")),
        ];
        Arc::new(Spansh { names, jobs: Mutex::new(jobs.into()), log: Mutex::default() })
    }

    fn urls(&self) -> Vec<String> {
        self.log.lock().unwrap().clone()
    }
}

impl Transport for Spansh {
    fn send(&self, url: &str, _req: &Request) -> Result<Response, TransportError> {
        self.log.lock().unwrap().push(url.to_string());
        if url.contains("system_names") {
            return Ok(Response { status: 200, body: self.names.to_string() });
        }
        let (status, body) = self.jobs.lock().unwrap().pop_front().expect("no more route answers recorded");
        Ok(Response { status, body: body.to_string() })
    }
}

fn client(t: &Arc<Spansh>) -> Client {
    Client::with_transport(Cache::in_memory().unwrap(), t.clone()).limiter(Limiter::none())
}

/// The local star map knows Somewhere, 100 ly past Jackson's Lighthouse (157, −27, −70) from Sol.
fn galaxy(t: &Arc<Spansh>) -> Galaxy {
    let mut s = Store::in_memory().unwrap();
    s.insert_systems([(42, "Somewhere", [257.0, -27.0, -70.0])]).unwrap();
    Galaxy::with_remote(s, client(t))
}

const NO_WAIT: Polling = Polling { every: Duration::ZERO, timeout: Duration::from_secs(5) };

fn ship() -> Ship {
    Ship {
        name: "Asp Explorer".into(),
        fsd: astromech_ship::Fsd {
            optimal_mass: 1470.0,
            max_fuel_per_jump: 8.0,
            fuel_multiplier: 0.012,
            fuel_power: 2.45,
            base_mass: 300.0,
            tank_size: 32.0,
            reserve_size: 0.63,
            range_boost: 0.0,
            supercharge_multiplier: 4,
        },
        cargo: 0.0,
        range: 25.0,
        unresolved: vec![],
    }
}

fn sol() -> Here {
    Here { system: "Sol".into(), id64: Some(10477373803), pos: [0.0; 3] }
}

fn ask(plotter: Plotter, to: &str) -> Ask {
    Ask { plotter, to: to.into(), efficiency: 60, refuel_every_scoopable: true }
}

/// A data folder holding `file` as `neutron.bin` (none if `None`).
fn data_dir(file: Option<&str>) -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    if let Some(f) = file {
        std::fs::copy(fixture(f), dir.path().join(FILE_NAME)).unwrap();
    }
    dir
}

fn stops(p: &astromech_galaxy::plot::Plotted) -> Vec<(&str, Option<&str>)> {
    p.waypoints.iter().map(|w| (w.system.as_str(), w.label.as_deref())).collect()
}

#[test]
fn a_neutron_route_is_plotted_on_this_pc_when_the_file_is_there() {
    let dir = data_dir(Some("neutron-sample.bin"));
    let kept = Kept::new(dir.path());
    let t = Spansh::new(r#"{"min_max":[]}"#);
    let g = galaxy(&t);
    let locate = |n: &str| g.locate(n);
    let here = sol();

    // The route starts where the commander is; the destination is on the local star map.
    let p = plot_route(g.remote(), &kept, locate, &ask(Plotter::Neutron, "Somewhere"), Some(&here), &ship(), NO_WAIT)
        .unwrap();
    assert!(p.local);
    assert_eq!(p.fewest, Some(true), "a short route is proven fewest");
    assert_eq!(p.fallback, None);
    assert_eq!(p.source, "neutron plotter (this PC)");
    assert_eq!(stops(&p), [("Sol", None), ("Jackson's Lighthouse", Some(NEUTRON)), ("Somewhere", None)]);
    assert_eq!(p.jumps, 8);
    assert!(t.urls().is_empty(), "no network: {:?}", t.urls());
}

#[test]
fn an_unknown_system_says_so() {
    let dir = data_dir(Some("neutron-sample.bin"));
    let kept = Kept::new(dir.path());
    let t = Spansh::new(r#"{"min_max":[{"id64":1,"name":"Nowhere Near","x":0.0,"y":0.0,"z":0.0}]}"#);
    let g = galaxy(&t);
    let e = plot_route(
        g.remote(),
        &kept,
        |n| g.locate(n),
        &ask(Plotter::Neutron, "Nowhere"),
        Some(&sol()),
        &ship(),
        NO_WAIT,
    )
    .unwrap_err();
    assert_eq!(e, "Spansh does not know a system called Nowhere.");
    assert!(!t.urls().iter().any(|u| u.contains("/api/route")), "no Spansh route asked for");

    let e = plot_route(g.remote(), &kept, |n| g.locate(n), &ask(Plotter::Neutron, "Somewhere"), None, &ship(), NO_WAIT)
        .unwrap_err();
    assert_eq!(e, astromech_galaxy::plot::NOT_HERE);
}

#[test]
fn spansh_plots_when_there_is_no_file() {
    let dir = data_dir(None);
    let kept = Kept::new(dir.path());
    let t = Spansh::new(r#"{"min_max":[]}"#);
    let g = galaxy(&t);
    let p = plot_route(
        g.remote(),
        &kept,
        |n| g.locate(n),
        &ask(Plotter::Neutron, "Colonia"),
        Some(&sol()),
        &ship(),
        NO_WAIT,
    )
    .unwrap();
    assert!(!p.local);
    assert_eq!((p.fewest, p.fallback.as_deref()), (None, None));
    assert_eq!(p.source, "neutron plotter");
    assert_eq!(p.jumps, 128);
    assert_eq!(t.urls()[0], "https://spansh.co.uk/api/route");
}

#[test]
fn a_broken_file_falls_back_to_spansh_and_says_why() {
    // The header is fine (so the page lists the file) but the stars are cut short.
    let dir = data_dir(None);
    let full = std::fs::read(fixture("neutron-sample.bin")).unwrap();
    let path = dir.path().join(FILE_NAME);
    std::fs::write(&path, &full[..100]).unwrap();
    let kept = Kept::new(dir.path());
    assert!(kept.installed().unwrap().is_some(), "before a load, only the header is read");

    let t = Spansh::new(r#"{"min_max":[]}"#);
    let g = galaxy(&t);
    let p = plot_route(
        g.remote(),
        &kept,
        |n| g.locate(n),
        &ask(Plotter::Neutron, "Colonia"),
        Some(&sol()),
        &ship(),
        NO_WAIT,
    )
    .unwrap();
    assert!(!p.local);
    assert_eq!(p.source, "neutron plotter");
    let why = p.fallback.expect("says why");
    assert!(why.contains("cut short"), "{why}");
    assert_eq!(kept.installed().unwrap_err().to_string(), why, "the page's file status says it too");

    // A good file in its place is loaded and used.
    std::fs::write(&path, &full).unwrap();
    assert!(kept.installed().unwrap().is_some());
    let p = plot_route(
        g.remote(),
        &kept,
        |n| g.locate(n),
        &ask(Plotter::Neutron, "Somewhere"),
        Some(&sol()),
        &ship(),
        NO_WAIT,
    )
    .unwrap();
    assert!(p.local);
}

#[test]
fn exact_routes_come_from_spansh_with_the_file_there() {
    let dir = data_dir(Some("neutron-sample.bin"));
    let kept = Kept::new(dir.path());
    let rec = Arc::new(Exact(Mutex::new(
        [
            (202, spansh_fixture!("spansh-exact-submit.json")),
            (202, spansh_fixture!("spansh-exact-pending.json")),
            (200, spansh_fixture!("spansh-exact-result.json")),
        ]
        .into(),
    )));
    let client = Client::with_transport(Cache::in_memory().unwrap(), rec).limiter(Limiter::none());
    let p = plot_route(
        &client,
        &kept,
        |_| panic!("not located"),
        &ask(Plotter::Exact, "Maia"),
        Some(&sol()),
        &ship(),
        NO_WAIT,
    )
    .unwrap();
    assert!(!p.local);
    assert_eq!((p.source, p.jumps), ("exact plotter", 9));
}

struct Exact(Mutex<VecDeque<(u16, &'static str)>>);

impl Transport for Exact {
    fn send(&self, _url: &str, _req: &Request) -> Result<Response, TransportError> {
        let (status, body) = self.0.lock().unwrap().pop_front().expect("no more answers");
        Ok(Response { status, body: body.to_string() })
    }
}

/// The file is kept loaded between plots, and loaded again once a download has replaced it,
/// before names are read from it.
#[test]
fn a_downloaded_file_replaces_the_kept_one() {
    let dir = data_dir(Some("neutron-sample.bin"));
    let kept = Kept::new(dir.path());
    let first = kept.get().unwrap().unwrap();
    assert!(Arc::ptr_eq(&first, &kept.get().unwrap().unwrap()), "kept, not loaded again");
    assert_eq!(first.stars().len(), 35);

    // What neutron::download does: unpack into a partial file, then rename it over the old one.
    let part = dir.path().join("neutron.bin.part");
    std::fs::copy(fixture("neutron-synthetic.bin"), &part).unwrap();
    std::fs::rename(&part, dir.path().join(FILE_NAME)).unwrap();
    assert!(!first.is_current());
    assert!(first.names(&[0]).unwrap_err().to_string().contains("Plot the route again"));

    let now = kept.get().unwrap().unwrap();
    assert!(!Arc::ptr_eq(&first, &now));
    assert!(now.is_current());
    let synthetic = astromech_galaxy::neutron::read_header(&fixture("neutron-synthetic.bin")).unwrap();
    assert_eq!(now.header(), synthetic);
    assert!(now.name(0).is_ok(), "names come from the new file");

    // Deleted: no file, so Spansh plots.
    std::fs::remove_file(dir.path().join(FILE_NAME)).unwrap();
    assert!(kept.get().unwrap().is_none());
}
