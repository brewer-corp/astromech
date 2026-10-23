//! The local neutron plotter (astromech_galaxy::neutron_plot): synthetic star fields where the
//! best route is known, the neutron routes Spansh plotted on 22 Oct 2026 (Sol to Colonia at
//! 50 ly, Sol to Beagle Point at 60 ly ×4 and 80 ly ×6, efficiency 60), and, ignored unless
//! the real neutron star file is given, the same routes and the timings in the module docs.

use astromech_galaxy::neutron::{NeutronStars, Stars};
use astromech_galaxy::neutron_plot::{leg_jumps, plot, route, Route, SOURCE};
use astromech_galaxy::plot::{Ship, NEUTRON};
use astromech_galaxy::{dist, Here};
use std::path::{Path, PathBuf};
use std::time::Instant;

fn fixture(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures").join(name)
}

fn q32(p: [f64; 3]) -> [i32; 3] {
    p.map(|v| (v * 32.0).round() as i32)
}

fn field(ly: &[[f64; 3]]) -> Stars {
    Stars::new(128, ly.iter().map(|p| q32(*p)).collect()).unwrap()
}

/// Where each leg ends (the destination for the last).
fn ends(stars: &Stars, r: &Route, to: [f64; 3]) -> Vec<[f64; 3]> {
    r.legs.iter().map(|l| l.star.map_or(to, |i| stars.pos(i))).collect()
}

fn jumps_per_leg(r: &Route) -> Vec<u32> {
    r.legs.iter().map(|l| l.jumps).collect()
}

/// The fewest (jumps, stops) from `from` to `to`, by Dijkstra over every pair of stars: the
/// reference the search is checked against on small fields.
fn fewest(stars: &Stars, from: [f64; 3], to: [f64; 3], range: f64, boost: f64) -> (u32, u32) {
    let n = stars.len();
    let pos = |i: usize| match i {
        i if i < n => stars.pos(i),
        i if i == n => from,
        _ => to,
    };
    let start = (0..n).find(|&i| stars.pos32(i) == q32(from)).unwrap_or(n);
    let first = |i: usize| if i < n { boost * range } else { range };
    let mut best = vec![(u32::MAX, u32::MAX); n + 2];
    let mut done = vec![false; n + 2];
    best[start] = (0, 0);
    while let Some(u) = (0..n + 2).filter(|&i| !done[i] && best[i].0 != u32::MAX).min_by_key(|&i| best[i]) {
        done[u] = true;
        if u == n + 1 {
            break;
        }
        for v in (0..n).chain([n + 1]) {
            if v != u && v != start {
                let c = best[u];
                let next = (c.0 + leg_jumps(dist(pos(u), pos(v)), first(u), range), c.1 + 1);
                best[v] = best[v].min(next);
            }
        }
    }
    best[n + 1]
}

fn stops(r: &Route) -> u32 {
    r.legs.len() as u32
}

#[test]
fn a_straight_chain_boosts_from_every_star() {
    let stars = field(&[[10.0, 0.0, 0.0], [50.0, 0.0, 0.0], [90.0, 0.0, 0.0], [130.0, 0.0, 0.0]]);
    let to = [170.0, 0.0, 0.0];
    let r = route(&stars, [0.0; 3], to, 10.0, 4.0);
    assert_eq!(jumps_per_leg(&r), [1, 1, 1, 1, 1]);
    let xs: Vec<f64> = ends(&stars, &r, to).iter().map(|p| p[0]).collect();
    assert_eq!(xs, [10.0, 50.0, 90.0, 130.0, 170.0]);
    assert_eq!(r.start, None);
    assert_eq!(r.legs.last().unwrap().star, None, "the last stop is the destination");

    // No neutron stars: the plain route.
    let none = route(&field(&[]), [0.0; 3], to, 10.0, 4.0);
    assert_eq!((none.jumps(), stops(&none)), (17, 1));
}

#[test]
fn takes_a_detour_that_saves_jumps() {
    // Nothing on the straight line (40 jumps); a chain of neutron stars 30 ly off it.
    let chain: Vec<[f64; 3]> = (0..10).map(|i| [f64::from(i) * 40.0, 30.0, 0.0]).collect();
    let stars = field(&chain);
    let to = [400.0, 0.0, 0.0];
    let r = route(&stars, [0.0; 3], to, 10.0, 4.0);
    // 3 jumps to the chain, 9 boosts along it, then 50 ly: a boost and one normal jump.
    assert_eq!(r.jumps(), 14);
    assert_eq!((r.jumps(), stops(&r)), fewest(&stars, [0.0; 3], to, 10.0, 4.0));
    assert_eq!(jumps_per_leg(&r), [3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2]);
}

#[test]
fn crosses_a_gap_no_boost_spans_with_normal_jumps() {
    let stars = field(&[[10.0, 0.0, 0.0], [200.0, 0.0, 0.0]]);
    let to = [240.0, 0.0, 0.0];
    let r = route(&stars, [0.0; 3], to, 10.0, 4.0);
    // 190 ly from the first star: one boosted jump of 40 ly, then 15 of 10 ly.
    assert_eq!(jumps_per_leg(&r), [1, 16, 1]);
    assert_eq!((r.jumps(), stops(&r)), fewest(&stars, [0.0; 3], to, 10.0, 4.0));
}

#[test]
fn a_start_on_a_neutron_star_boosts_the_first_jump() {
    let stars = field(&[[0.0, 0.0, 0.0], [5.0, 5.0, 5.0]]);
    let to = [40.0, 0.0, 0.0];
    let r = route(&stars, [0.0; 3], to, 10.0, 4.0);
    assert_eq!(r.start.map(|i| stars.pos(i)), Some([0.0; 3]));
    assert_eq!((jumps_per_leg(&r), r.legs[0].star), (vec![1], None));
    // From next to it, the first jump is a normal one.
    let r = route(&stars, [0.0, 0.0, 1.0], to, 10.0, 4.0);
    assert_eq!(r.start, None);
    assert_eq!(r.jumps(), 2, "a jump to the star, then a boost");
}

#[test]
fn a_destination_on_a_neutron_star_is_one_stop() {
    let stars = field(&[[10.0, 0.0, 0.0], [50.0, 0.0, 0.0]]);
    let to = [50.0, 0.0, 0.0];
    let r = route(&stars, [0.0; 3], to, 10.0, 4.0);
    assert_eq!(jumps_per_leg(&r), [1, 1]);
    assert_eq!(r.legs[1].star, None);
}

#[test]
fn the_boost_multiplier_sets_the_reach() {
    let stars = field(&[[10.0, 0.0, 0.0]]);
    let to = [70.0, 0.0, 0.0];
    // 60 ly from the star: ×4 reaches 40 ly then two normal jumps; ×6 reaches it.
    assert_eq!(jumps_per_leg(&route(&stars, [0.0; 3], to, 10.0, 4.0)), [1, 3]);
    assert_eq!(jumps_per_leg(&route(&stars, [0.0; 3], to, 10.0, 6.0)), [1, 1]);
}

#[test]
fn leg_jumps_follow_the_model() {
    assert_eq!(leg_jumps(0.0, 40.0, 10.0), 0);
    assert_eq!(leg_jumps(40.0, 40.0, 10.0), 1);
    assert_eq!(leg_jumps(40.1, 40.0, 10.0), 2);
    assert_eq!(leg_jumps(60.0, 40.0, 10.0), 3);
    assert_eq!(leg_jumps(25.0, 10.0, 10.0), 3, "from an ordinary star: ceil(d / R)");
}

/// Random fields (a fixed generator), clustered and sparse, against the reference.
#[test]
fn random_fields_get_the_fewest_jumps_then_stops() {
    let mut seed = 0x2545_f491_4f6c_dd1du64;
    let mut next = move || {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        (seed >> 11) as f64 / (1u64 << 53) as f64
    };
    // Ranges small and large next to the grid's 128 ly cells (a boost of 360 ly spans cells).
    for case in 0..200 {
        let (range, boost) = ([10.0, 23.5, 60.0][case % 3], [4.0, 6.0][case % 2]);
        let scale = range / 10.0;
        let count = 20 + case % 180;
        let spread = [400.0, 60.0 + (case % 4) as f64 * 40.0, 60.0].map(|s| s * scale);
        let ly: Vec<[f64; 3]> = (0..count).map(|_| spread.map(|s| (next() - 0.5) * s)).collect();
        let stars = field(&ly);
        let from = [-200.0 * scale, 0.0, 0.0];
        let to = [200.0 * scale, next() * 20.0 * scale, 0.0];
        let r = route(&stars, from, to, range, boost);
        assert!(r.fewest, "case {case}");
        assert_eq!((r.jumps(), stops(&r)), fewest(&stars, from, to, range, boost), "case {case}");
        let ends = ends(&stars, &r, to);
        let mut at = from;
        for (leg, end) in r.legs.iter().zip(&ends) {
            let first = if at == from { range } else { boost * range };
            assert_eq!(leg.jumps, leg_jumps(dist(at, *end), first, range), "case {case}");
            at = *end;
        }
    }
}

/// A dense field (about 80 neutron stars within each boost's reach) over 4,000 ly: too much for
/// plain A* within its work, so the guided search plots it, close to the straight-line bound.
#[test]
fn a_long_dense_route_is_guided_and_near_the_bound() {
    let mut seed = 0x9e37_79b9_7f4a_7c15u64;
    let mut next = move || {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        (seed >> 11) as f64 / (1u64 << 53) as f64
    };
    let spread = [4400.0, 400.0, 400.0];
    let ly: Vec<[f64; 3]> = (0..200_000).map(|_| spread.map(|s| (next() - 0.5) * s)).collect();
    let stars = field(&ly);
    let (from, to) = ([-2000.0, 0.0, 0.0], [2000.0, 0.0, 0.0]);
    let r = route(&stars, from, to, 10.0, 4.0);
    assert!(!r.fewest, "plain A* finished; the field is too small to test the guided search");
    let bound = (dist(from, to) / 40.0).ceil() as u32;
    assert!(
        r.jumps() >= bound && f64::from(r.jumps()) <= f64::from(bound) * 1.15,
        "{} jumps, bound {bound}",
        r.jumps()
    );
    let points: Vec<[f64; 3]> = [from].into_iter().chain(ends(&stars, &r, to)).collect();
    for (i, (leg, w)) in r.legs.iter().zip(points.windows(2)).enumerate() {
        let first = if i == 0 { 10.0 } else { 40.0 };
        assert_eq!(leg.jumps, leg_jumps(dist(w[0], w[1]), first, 10.0));
    }
    // No stop that saves no jumps: going past it costs more.
    for (i, w) in points.windows(3).enumerate() {
        let first = if i == 0 { 10.0 } else { 40.0 };
        let past = leg_jumps(dist(w[0], w[2]), first, 10.0);
        assert!(past > r.legs[i].jumps + r.legs[i + 1].jumps, "stop {i} saves nothing");
    }
}

/// A route Spansh plotted: its stops (position, neutron star, Spansh's jumps to reach it).
struct Recorded {
    range: f64,
    boost: f64,
    stops: Vec<([f64; 3], bool, u32)>,
}

fn recorded(name: &str) -> Recorded {
    let v: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(fixture(name)).unwrap()).unwrap();
    let num = |v: &serde_json::Value| v.as_str().map_or_else(|| v.as_f64().unwrap(), |s| s.parse().unwrap());
    let stops = v["result"]["system_jumps"]
        .as_array()
        .unwrap()
        .iter()
        .map(|j| {
            let p = ["x", "y", "z"].map(|a| j[a].as_f64().unwrap());
            (p, j["neutron_star"].as_bool().unwrap(), j["jumps"].as_u64().unwrap() as u32)
        })
        .collect();
    Recorded { range: num(&v["parameters"]["range"]), boost: num(&v["parameters"]["supercharge_multiplier"]), stops }
}

const RECORDED: [&str; 3] = [
    "spansh-neutron-sol-colonia-50-4.json",
    "spansh-neutron-sol-beagle-60-4.json",
    "spansh-neutron-sol-beagle-80-6.json",
];

impl Recorded {
    fn spansh_jumps(&self) -> u32 {
        self.stops.iter().map(|s| s.2).sum()
    }

    /// The jumps of each of Spansh's legs in the model.
    fn model_legs(&self) -> Vec<u32> {
        self.stops
            .windows(2)
            .map(|w| {
                let first = if w[0].1 { self.boost * self.range } else { self.range };
                leg_jumps(dist(w[0].0, w[1].0), first, self.range)
            })
            .collect()
    }

    fn endpoints(&self) -> ([f64; 3], [f64; 3]) {
        (self.stops[0].0, self.stops.last().unwrap().0)
    }
}

/// Spansh's legs against the model. Every stop between the ends is a neutron star. Where a leg is
/// one boosted jump (within k·R) or the first leg from the start, Spansh and the model mostly
/// agree; where normal jumps follow a boost, Spansh often counts one or two more than
/// `ceil((d − k·R) / R)`: it allows for real jumps falling short of R (the stars between aren't on
/// the line), which the model doesn't (fuel and real stars are astromech-zrbk). So Spansh's count
/// is never lower than the model's, and the routes are compared in the model.
#[test]
fn spansh_counts_its_legs_as_the_model_does_or_more() {
    // Total jumps: Spansh's own, and its route in the model. (`total_jumps` in the result is
    // the number of legs, not this.)
    let want = [(166, 160), (467, 438), (190, 181)];
    for (name, (spansh, model)) in RECORDED.into_iter().zip(want) {
        let r = recorded(name);
        assert!(r.stops[1..r.stops.len() - 1].iter().all(|s| s.1), "{name}: a stop that isn't a neutron star");
        let legs = r.model_legs();
        let kr = r.boost * r.range;
        for (w, m) in r.stops.windows(2).zip(&legs) {
            let spansh = w[1].2;
            assert!(spansh >= *m, "{name}: Spansh {spansh} below the model's {m}");
            if w[0].1 && dist(w[0].0, w[1].0) <= kr {
                assert_eq!(spansh, 1, "{name}: a boost within k·R");
            }
            assert!(spansh <= m + 2, "{name}: Spansh {spansh} vs the model's {m}");
        }
        assert_eq!((r.spansh_jumps(), legs.iter().sum::<u32>()), (spansh, model), "{name}");
    }
}

/// Over only the neutron stars Spansh used, the local search finds a route no longer (in the
/// model) than Spansh's.
#[test]
fn the_search_does_no_worse_than_spansh_on_its_own_stars() {
    for name in RECORDED {
        let r = recorded(name);
        let stars = field(&r.stops[1..r.stops.len() - 1].iter().map(|s| s.0).collect::<Vec<_>>());
        let (from, to) = r.endpoints();
        let local = route(&stars, from, to, r.range, r.boost);
        let spansh: u32 = r.model_legs().iter().sum();
        assert!(local.jumps() <= spansh, "{name}: {} jumps against Spansh's {spansh}", local.jumps());
    }
}

fn ship(range: f64, boost: u8) -> Ship {
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
            supercharge_multiplier: boost,
        },
        cargo: 0.0,
        range,
        unresolved: vec![],
    }
}

fn here(system: &str, pos: [f64; 3]) -> Here {
    Here { system: system.into(), id64: None, pos }
}

#[test]
fn plots_with_the_stops_names_and_labels() {
    let file = NeutronStars::open(&fixture("neutron-sample.bin")).unwrap();
    // Jackson's Lighthouse is at (157, −27, −70): 7 jumps of 25 ly from Sol, then 100 ly on is
    // one boosted jump. Straight there is 11.
    let (from, to) = (here("Sol", [0.0; 3]), here("Somewhere", [257.0, -27.0, -70.0]));
    let p = plot(&file, &from, &to, &ship(25.0, 4)).unwrap();
    assert_eq!(p.source, SOURCE);
    assert_eq!(p.source, "neutron plotter (this PC)");
    assert_eq!(p.name, "Sol to Somewhere");
    let stops: Vec<(&str, Option<&str>)> =
        p.waypoints.iter().map(|w| (w.system.as_str(), w.label.as_deref())).collect();
    assert_eq!(stops, [("Sol", None), ("Jackson's Lighthouse", Some(NEUTRON)), ("Somewhere", None)]);
    assert_eq!((p.jumps, p.neutrons, p.refuels, p.range), (8, 1, 0, 25.0));
    assert_eq!((p.local, p.fewest, p.fallback), (true, Some(true), None));
    assert!((p.ly - dist(from.pos, to.pos)).abs() < 1e-9);

    // From the lighthouse itself: its first jump is boosted, and it is labelled.
    let from = here("Jackson's Lighthouse", [157.0, -27.0, -70.0]);
    let p = plot(&file, &from, &to, &ship(25.0, 4)).unwrap();
    let stops: Vec<(&str, Option<&str>)> =
        p.waypoints.iter().map(|w| (w.system.as_str(), w.label.as_deref())).collect();
    assert_eq!(stops, [("Jackson's Lighthouse", Some(NEUTRON)), ("Somewhere", None)]);
    assert_eq!((p.jumps, p.neutrons), (1, 1));

    assert!(plot(&file, &from, &to, &ship(0.0, 4)).unwrap_err().contains("can't jump"));
}

/// The loaded file holds no handle (so a download can replace it), and won't read names from a
/// file that replaced it.
#[test]
fn names_come_only_from_the_file_that_was_loaded() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("neutron.bin");
    std::fs::copy(fixture("neutron-sample.bin"), &path).unwrap();
    let file = NeutronStars::open(&path).unwrap();
    let first = file.name(0).unwrap();

    let newer = dir.path().join("neutron.bin.part");
    std::fs::copy(fixture("neutron-synthetic.bin"), &newer).unwrap();
    std::fs::rename(&newer, &path).unwrap();
    let err = file.names(&[0]).unwrap_err().to_string();
    assert!(err.contains("changed") && err.contains("Plot the route again"), "{err}");

    // The same build again (same header and length) reads as before.
    std::fs::copy(fixture("neutron-sample.bin"), &path).unwrap();
    assert_eq!(file.name(0).unwrap(), first);
}

/// Linux: the process's resident memory now and at its peak since the last reset, MB.
fn memory() -> (u64, u64) {
    let status = std::fs::read_to_string("/proc/self/status").unwrap_or_default();
    let kb = |key: &str| {
        status.lines().find_map(|l| l.strip_prefix(key)).and_then(|v| v.split_whitespace().next()?.parse().ok())
    };
    (kb("VmRSS:").unwrap_or(0) / 1024, kb("VmHWM:").unwrap_or(0) / 1024)
}

const SOL: [f64; 3] = [0.0, 0.0, 0.0];
const COLONIA: [f64; 3] = [-9530.5, -910.28125, 19808.125];
const BEAGLE_POINT: [f64; 3] = [-1111.5625, -134.21875, 65269.75];

/// On the real file (`NEUTRON_BIN=<path>`): the recorded routes against Spansh's, then the timed
/// routes in the module docs. `NEUTRON_CASE=<n>` runs only timed route n (0–3), to measure its
/// peak memory alone.
#[test]
#[ignore = "needs the real neutron star file (NEUTRON_BIN)"]
fn on_the_real_file() {
    let path = std::env::var("NEUTRON_BIN").expect("NEUTRON_BIN");
    let t = Instant::now();
    let file = NeutronStars::open(path.as_ref()).unwrap();
    println!("loaded {} neutron stars in {:?}, peak {} MB", file.stars().len(), t.elapsed(), memory().1);
    let only: Option<usize> = std::env::var("NEUTRON_CASE").ok().map(|c| c.parse().unwrap());
    if only.is_none() {
        for name in RECORDED {
            let r = recorded(name);
            let (from, to) = r.endpoints();
            let local = route(file.stars(), from, to, r.range, r.boost);
            let model: u32 = r.model_legs().iter().sum();
            println!(
                "{name}: local {} jumps, {} stops; Spansh {} jumps ({model} in the model), {} stops",
                local.jumps(),
                stops(&local),
                r.spansh_jumps(),
                r.stops.len() - 1
            );
            assert!(local.jumps() <= model, "{name}");
        }
    }
    let timed = [
        ("Sol to Colonia", COLONIA, 60.0, 4.0),
        ("Sol to Colonia", COLONIA, 80.0, 6.0),
        ("Sol to Beagle Point", BEAGLE_POINT, 60.0, 4.0),
        ("Sol to Beagle Point", BEAGLE_POINT, 80.0, 6.0),
    ];
    for (i, (name, to, range, boost)) in timed.into_iter().enumerate() {
        if only.is_some_and(|o| o != i) {
            continue;
        }
        let _ = std::fs::write("/proc/self/clear_refs", "5"); // resets the peak
        let before = memory().0;
        let t = Instant::now();
        let r = route(file.stars(), SOL, to, range, boost);
        let search = t.elapsed();
        let peak = memory().1;
        let t = Instant::now();
        let ids: Vec<usize> = r.legs.iter().filter_map(|l| l.star).collect();
        file.names(&ids).unwrap();
        println!(
            "{name}, R = {range}, k = {boost}: {} jumps, {} stops; search {search:?}, names {:?}; peak {peak} MB ({before} MB before)",
            r.jumps(),
            stops(&r),
            t.elapsed()
        );
        assert!(search.as_secs_f64() < 2.0, "{name}");
    }
}
