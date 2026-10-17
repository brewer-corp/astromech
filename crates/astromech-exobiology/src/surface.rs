//! Surface guidance: the bearing and distance from one point on a body to another, along the
//! surface (great circle on a sphere of the body's radius).
//!
//! Positions are degrees of latitude (north positive) and longitude (east positive), as
//! Status.json gives them; the radius is in metres (Status.json's `PlanetRadius`). The bearing
//! is the initial heading to fly or walk, in degrees clockwise from north (0 to 360), which is
//! how the game's compass and `Heading` count. Altitude is ignored: sample distances and colony
//! ranges are measured along the ground.

/// A point on a body's surface, in degrees.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LatLon {
    pub lat: f64,
    pub lon: f64,
}

impl LatLon {
    pub fn new(lat: f64, lon: f64) -> Self {
        Self { lat, lon }
    }
}

/// Distance along the surface from `a` to `b` on a body of `radius_m` metres, in metres.
///
/// Haversine formula: well conditioned for the short distances sampling cares about (metres).
/// Near the antipode it loses precision to a few centimetres, which no use here notices.
pub fn distance_m(a: LatLon, b: LatLon, radius_m: f64) -> f64 {
    let (p1, p2) = (a.lat.to_radians(), b.lat.to_radians());
    let dp = p2 - p1;
    let dl = (b.lon - a.lon).to_radians();
    let h = (dp / 2.0).sin().powi(2) + p1.cos() * p2.cos() * (dl / 2.0).sin().powi(2);
    let h = h.clamp(0.0, 1.0);
    2.0 * radius_m * h.sqrt().atan2((1.0 - h).sqrt())
}

/// Initial bearing from `a` to `b`, degrees clockwise from north in `[0, 360)`. Independent of
/// the body's size. The same point gives 0.
pub fn bearing_deg(a: LatLon, b: LatLon) -> f64 {
    let (p1, p2) = (a.lat.to_radians(), b.lat.to_radians());
    let dl = (b.lon - a.lon).to_radians();
    let y = dl.sin() * p2.cos();
    let x = p1.cos() * p2.sin() - p1.sin() * p2.cos() * dl.cos();
    if x == 0.0 && y == 0.0 {
        return 0.0;
    }
    let deg = y.atan2(x).to_degrees().rem_euclid(360.0);
    // rem_euclid can round up to exactly 360 for a tiny negative angle.
    if deg >= 360.0 {
        0.0
    } else {
        deg
    }
}

/// Bearing and distance together: where `to` is from `from`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Guidance {
    pub bearing_deg: f64,
    pub distance_m: f64,
}

pub fn guidance(from: LatLon, to: LatLon, radius_m: f64) -> Guidance {
    Guidance { bearing_deg: bearing_deg(from, to), distance_m: distance_m(from, to, radius_m) }
}
