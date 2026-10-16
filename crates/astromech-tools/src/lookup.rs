//! Station lookups by voice (read-only, tier 0): what a matched `find.*` command asks for, the
//! values for its grammar slot, and the short spoken answer. The search itself is
//! `astromech_galaxy::stations::find`.

use astromech_galaxy::stations::{Found, Service, Want};

/// Commands that look something up instead of pressing keys.
pub fn is_lookup(command: &str) -> bool {
    command.starts_with("find.")
}

/// Values for the grammar's `{service}` slot: (a way to say it, the service id).
pub fn service_values() -> Vec<(&'static str, String)> {
    Service::ALL.iter().flat_map(|s| s.spoken().iter().map(move |said| (*said, s.id()))).collect()
}

/// What a matched lookup asks for. `slot` is the grammar slot's value; `None` if the command
/// isn't a lookup or the slot doesn't name something it knows.
pub fn want(command: &str, slot: Option<&str>) -> Option<Want> {
    match command {
        "find.service" => Some(Want::Service { service: Service::from_id(slot?)? }),
        _ => None,
    }
}

/// "6 light years", or "in this system".
fn distance(ly: f64) -> String {
    if ly < 0.05 {
        "in this system".into()
    } else if ly < 10.0 {
        format!("{ly:.1} light years")
    } else {
        format!("{ly:.0} light years")
    }
}

/// "450 light seconds", "12 thousand light seconds".
fn arrival(ls: f64) -> String {
    if ls < 10_000.0 {
        format!("{ls:.0} light seconds from the star")
    } else {
        format!("{:.0} thousand light seconds from the star", ls / 1000.0)
    }
}

fn days_old(updated_at: Option<&str>, now_s: i64) -> Option<i64> {
    let t = chrono::DateTime::parse_from_rfc3339(updated_at?).ok()?.timestamp();
    Some((now_s - t) / 86_400)
}

/// Data older than this is called out.
const OLD_DAYS: i64 = 7;

/// The spoken answer to a lookup near `origin` (a system name): the nearest station, how far,
/// how far from the star, and how old the report is when it's more than a week old.
pub fn spoken(want: &Want, origin: &str, found: &Result<Found, String>, now_s: i64) -> String {
    let found = match found {
        Ok(f) => f,
        Err(e) => return e.clone(),
    };
    let label = want.label();
    let Some(h) = found.hits.first() else {
        return match &found.warning {
            Some(w) => w.clone(),
            None => format!("No {label} found near {origin} with your search filters."),
        };
    };
    let mut s = format!("Nearest {label}: {} in {}, {}", h.name, h.system, distance(h.ly));
    if let Some(ls) = h.arrival_ls {
        s += &format!(", {}", arrival(ls));
    }
    s.push('.');
    if let Some(d) = days_old(h.updated_at.as_deref(), now_s).filter(|d| *d > OLD_DAYS) {
        s += &format!(" Reported {d} days ago.");
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;
    use astromech_data::spansh::Reference;
    use astromech_data::{Cache, Client, Limiter, Request, Response, Transport, TransportError};
    use astromech_galaxy::Filters;
    use std::sync::Arc;

    /// Spansh's recorded answers for each service near Sol (astromech-galaxy's fixture).
    const SERVICES: &str = include_str!("../../astromech-galaxy/tests/fixtures/spansh-services.json");

    struct Recorded(&'static str);

    impl Transport for Recorded {
        fn send(&self, _url: &str, _req: &Request) -> Result<Response, TransportError> {
            let all: serde_json::Value = serde_json::from_str(SERVICES).unwrap();
            Ok(Response { status: 200, body: all[self.0].to_string() })
        }
    }

    fn find(page: &'static str, want: &Want, filters: &Filters) -> Result<Found, String> {
        let c = Client::with_transport(Cache::in_memory().unwrap(), Arc::new(Recorded(page))).limiter(Limiter::none());
        astromech_galaxy::stations::find(&c, want, Reference::System("Sol".into()), filters, 1, NOW)
            .map_err(|e| e.to_string())
    }

    const NOW: i64 = 1_792_152_000; // 2026-10-16T12:00:00Z

    #[test]
    fn service_slot_values_name_every_service() {
        let v = service_values();
        for s in Service::ALL {
            assert!(v.iter().any(|(said, id)| *id == s.id() && s.spoken().contains(said)), "{s:?}");
        }
        assert_eq!(
            want("find.service", Some("interstellar_factors")),
            Some(Want::Service { service: Service::InterstellarFactors })
        );
        assert_eq!(want("find.service", Some("nonsense")), None);
        assert_eq!(want("lights.on", None), None);
        assert!(is_lookup("find.service") && !is_lookup("status.fuel"));
    }

    #[test]
    fn spoken_answers_for_recorded_responses() {
        let want = Want::Service { service: Service::GuardianBroker };
        let r = find("guardian_broker", &want, &Filters::default());
        assert_eq!(
            spoken(&want, "Sol", &r, NOW),
            "Nearest Guardian technology broker: Magnus Gateway in EZ Aquarii, 11 light years, 395 light seconds from the star."
        );
        let want = Want::Service { service: Service::Refuel };
        let r = find("refuel", &want, &Filters::default());
        assert_eq!(
            spoken(&want, "Sol", &r, NOW),
            "Nearest refuel: Ehrlich City in Sol, in this system, 228 light seconds from the star."
        );
        // an old report says how old
        let want = Want::Service { service: Service::BlackMarket };
        let r = find("black_market", &want, &Filters::default());
        assert!(
            spoken(&want, "Sol", &r, NOW).ends_with("Reported 2037 days ago."),
            "{}",
            spoken(&want, "Sol", &r, NOW)
        );
        // nothing passes the filters
        let strict = Filters { max_arrival_ls: Some(100), exclude_carriers: true, ..Filters::default() };
        let r = find("black_market", &want, &strict);
        assert_eq!(spoken(&want, "Sol", &r, NOW), "No black market found near Sol with your search filters.");
        // errors are already plain sentences
        assert_eq!(spoken(&want, "Sol", &Err("Can't reach Spansh.".into()), NOW), "Can't reach Spansh.");
    }
}
