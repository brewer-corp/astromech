//! Station lookups by voice (read-only, tier 0): what a matched `find.*` command asks for, the
//! values for its grammar slot, and the short spoken answer. The search itself is
//! `astromech_galaxy::stations::find`.

use astromech_announce::session_summary::credits;
use astromech_galaxy::outfitting::{self, ModuleWant};
use astromech_galaxy::stations::{Found, Service, Want};

/// Said when the journal has not given the commander's position yet: the one line every
/// position-based voice lookup (stations, sites, factions, Powerplay targets, colonisation) uses.
pub const NO_POSITION: &str = "Your position is not known yet. It comes from the next jump or login.";

/// Said when a lookup has a command but not what to look for ("nearest" without a service).
pub const NOT_CAUGHT: &str = "Did not catch what to look for.";

/// Said when a lookup fails: the reason (already a sentence) follows.
pub fn failed(why: &str) -> String {
    format!("Could not look that up. {why}")
}

/// Said when the galaxy store could not be opened: the one line every star-map voice lookup uses.
pub const NO_STAR_MAP: &str = "Lookups are not available: the star map could not be opened.";

/// Commands that look something up instead of pressing keys.
pub fn is_lookup(command: &str) -> bool {
    command.starts_with("find.")
}

/// Values for the grammar's `{service}` slot: (a way to say it, the service id).
pub fn service_values() -> Vec<(&'static str, String)> {
    Service::ALL.iter().flat_map(|s| s.spoken().iter().map(move |said| (*said, s.id()))).collect()
}

/// Values for the grammar's `{module}` slot: (a way to say it, the module's name). Its free
/// words are [`outfitting::is_modifier`] (class, rating, mount).
pub fn module_values() -> Vec<(&'static str, &'static str)> {
    outfitting::spoken_modules()
}

/// Values for the grammar's `{ship}` slot: (a way to say it, the ship's name).
pub fn ship_values() -> Vec<(&'static str, &'static str)> {
    outfitting::spoken_ships()
}

/// What a matched lookup asks for. `slot` is the grammar slot's value and `slot_text` the
/// words it took (a module's class, rating and mount are read from them); `None` if the
/// command isn't a lookup or the slot doesn't name something it knows.
pub fn want(command: &str, slot: Option<&str>, slot_text: Option<&str>) -> Option<Want> {
    match command {
        "find.service" => Some(Want::Service { service: Service::from_id(slot?)? }),
        "find.module" => {
            let name = outfitting::module(slot?)?.name.as_str();
            Some(Want::Module(ModuleWant::from_words(name, slot_text.unwrap_or(""))))
        }
        "find.ship" => Some(Want::Ship { ship: outfitting::ship(slot?)?.name.clone() }),
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
    if let Some(p) = h.price {
        s += &format!(" {}.", credits(p));
    }
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
            want("find.service", Some("interstellar_factors"), None),
            Some(Want::Service { service: Service::InterstellarFactors })
        );
        assert_eq!(want("find.service", Some("nonsense"), None), None);
        assert_eq!(want("lights.on", None, None), None);
        assert!(is_lookup("find.service") && !is_lookup("status.fuel"));
    }

    #[test]
    fn module_lookups_read_class_rating_and_mount_from_the_words() {
        let w = want("find.module", Some("Multi-Cannon"), Some("3c gimballed multi cannon")).unwrap();
        assert_eq!(w.label(), "3C Multi-Cannon (gimballed)");
        assert_eq!(
            want("find.module", Some("frame shift drive"), Some("6a fsd")).unwrap().label(),
            "6A Frame Shift Drive"
        );
        assert_eq!(want("find.module", Some("no such module"), Some("5a")), None);
        assert!(module_values().contains(&("fsd", "Frame Shift Drive")));
        assert_eq!(
            want("find.ship", Some("krait mkii"), Some("krait")),
            Some(Want::Ship { ship: "Krait MkII".into() })
        );
        assert!(ship_values().contains(&("conda", "Anaconda")));
    }

    #[test]
    fn ship_answer_says_the_price() {
        const SHIPS: &str = include_str!("../../astromech-galaxy/tests/fixtures/spansh-ships.json");
        struct Page;
        impl Transport for Page {
            fn send(&self, _url: &str, _req: &Request) -> Result<Response, TransportError> {
                let all: serde_json::Value = serde_json::from_str(SHIPS).unwrap();
                Ok(Response { status: 200, body: all["krait_mkii"].to_string() })
            }
        }
        let c = Client::with_transport(Cache::in_memory().unwrap(), Arc::new(Page)).limiter(Limiter::none());
        let w = want("find.ship", Some("Krait MkII"), Some("krait")).unwrap();
        let r = astromech_galaxy::stations::find(&c, &w, Reference::System("Sol".into()), &Filters::default(), 1, NOW)
            .map_err(|e| e.to_string());
        assert_eq!(
            spoken(&w, "Sol", &r, NOW),
            "Nearest Krait MkII: Ehrlich City in Sol, in this system, 228 light seconds from the star. 45.8 million credits."
        );
    }

    #[test]
    fn module_answer_says_the_price() {
        const MODULES: &str = include_str!("../../astromech-galaxy/tests/fixtures/spansh-modules.json");
        struct Page;
        impl Transport for Page {
            fn send(&self, _url: &str, _req: &Request) -> Result<Response, TransportError> {
                let all: serde_json::Value = serde_json::from_str(MODULES).unwrap();
                Ok(Response { status: 200, body: all["multicannon_3c"].to_string() })
            }
        }
        let c = Client::with_transport(Cache::in_memory().unwrap(), Arc::new(Page)).limiter(Limiter::none());
        let w = want("find.module", Some("Multi-Cannon"), Some("3c gimballed")).unwrap();
        let r = astromech_galaxy::stations::find(&c, &w, Reference::System("Sol".into()), &Filters::default(), 1, NOW)
            .map_err(|e| e.to_string());
        assert_eq!(
            spoken(&w, "Sol", &r, NOW),
            "Nearest 3C Multi-Cannon (gimballed): Galileo in Sol, in this system, 504 light seconds from the star. 578 thousand credits."
        );
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
        assert_eq!(spoken(&want, "Sol", &Err("Cannot reach Spansh.".into()), NOW), "Cannot reach Spansh.");
    }
}
