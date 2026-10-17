//! Surface guidance against known bearings and distances on bodies of different radii.
//!
//! Earth cases are the usual published great-circle examples (mean radius 6371 km): London to
//! Paris is 343.6 km on an initial bearing of 148.1°, New York JFK to London Heathrow 5540 km
//! on 51.4°. The rest are exact by construction (a quarter meridian, the equator, a short step
//! of latitude whose length is radius × angle) or were cross-checked with the spherical law of
//! cosines.

use astromech_exobiology::surface::{bearing_deg, distance_m, guidance, LatLon};

const EARTH: f64 = 6_371_000.0;
const MOON: f64 = 1_737_400.0;
/// A small rocky body (Luhman 16 B 3 is 2964 km).
const ROCK: f64 = 2_964_202.5;
/// A tiny icy moonlet.
const TINY: f64 = 300_000.0;

fn p(lat: f64, lon: f64) -> LatLon {
    LatLon::new(lat, lon)
}

#[test]
fn known_bearings_and_distances() {
    // (from, to, radius, distance m, tolerance m, bearing °)
    let table = [
        (p(51.5074, -0.1278), p(48.8566, 2.3522), EARTH, 343_556.0, 5.0, 148.116),
        (p(40.6413, -73.7781), p(51.47, -0.4543), EARTH, 5_540_011.0, 5.0, 51.353),
        // A quarter meridian: north, a quarter of the circumference.
        (p(0.0, 0.0), p(90.0, 0.0), MOON, MOON * std::f64::consts::FRAC_PI_2, 0.01, 0.0),
        // Along the equator: east and west, radius × angle.
        (p(0.0, 10.0), p(0.0, 11.0), EARTH, EARTH * 1f64.to_radians(), 0.01, 90.0),
        (p(0.0, 11.0), p(0.0, 10.0), EARTH, EARTH * 1f64.to_radians(), 0.01, 270.0),
        // Due south.
        (p(10.0, 20.0), p(9.0, 20.0), TINY, TINY * 1f64.to_radians(), 0.01, 180.0),
        // Sampling scale: a hundredth of a degree north on a 2964 km body is about 517 m.
        (p(10.0, 20.0), p(10.01, 20.0), ROCK, 517.351, 0.01, 0.0),
        // Across the date line in the south: the short way round, a little south of east.
        (p(-45.0, 170.0), p(-45.0, -170.0), MOON, 427_742.5, 1.0, 97.107),
        // A few metres to the south-west on a 1000 km body.
        (p(30.0, 40.0), p(29.99, 39.99), 1_000_000.0, 230.89, 0.01, 220.897),
    ];
    for (a, b, r, d, tol, brg) in table {
        let g = guidance(a, b, r);
        assert!((g.distance_m - d).abs() <= tol, "{a:?} -> {b:?} on {r} m: {} m, expected {d}", g.distance_m);
        assert!((g.bearing_deg - brg).abs() <= 0.01, "{a:?} -> {b:?}: bearing {}, expected {brg}", g.bearing_deg);
    }
}

#[test]
fn distance_scales_with_the_radius_and_bearing_does_not() {
    let (a, b) = (p(12.0, -33.0), p(12.3, -32.6));
    let small = distance_m(a, b, TINY);
    let big = distance_m(a, b, EARTH);
    assert!((big / small - EARTH / TINY).abs() < 1e-9);
    assert_eq!(bearing_deg(a, b), guidance(a, b, TINY).bearing_deg);
}

#[test]
fn the_same_point_is_zero_and_antipodes_are_half_way_round() {
    let a = p(23.5, 101.25);
    assert_eq!(distance_m(a, a, ROCK), 0.0);
    assert_eq!(bearing_deg(a, a), 0.0);
    let d = distance_m(p(0.0, 0.0), p(0.0, 180.0), TINY);
    assert!((d - TINY * std::f64::consts::PI).abs() < 1e-6);
    let d = distance_m(p(40.0, 10.0), p(-40.0, -170.0), ROCK);
    // Haversine loses a little precision at the antipode: centimetres on a 9300 km half circle.
    assert!((d - ROCK * std::f64::consts::PI).abs() < 0.1);
}

#[test]
fn bearings_stay_in_range() {
    for lat in [-80.0, -10.0, 0.0, 33.3, 89.0] {
        for dlon in [-179.0, -90.0, -0.001, 0.001, 45.0, 179.9] {
            let b = bearing_deg(p(lat, 0.0), p(lat + 0.5, dlon));
            assert!((0.0..360.0).contains(&b), "{b}");
        }
    }
}
