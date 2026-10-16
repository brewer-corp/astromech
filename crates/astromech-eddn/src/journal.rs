//! `journal/1`: the historical "all journal events" schema, for `Docked`, `FSDJump`, `Scan`,
//! `Location`, `SAASignalsFound` and `CarrierJump`. Rules from EDCD `schemas/journal-README.md`
//! and `journal-v1.0.json` (live branch, commit 4ad669bb, read 16 Oct 2026).
//!
//! `CodexEntry` is also allowed by the schema, but its README sends it on `codexentry/1` instead
//! ([`crate::events`]), so it is not sent here.

use crate::message::{Augment, Elision, SchemaSpec, SystemField};
use serde_json::Value;

const fn all(path: &'static str) -> Elision {
    Elision { event: None, path }
}

const fn on(event: &'static str, field: SystemField) -> Augment {
    Augment { event: Some(event), field }
}

pub const JOURNAL: SchemaSpec = SchemaSpec {
    name: "journal",
    version: 1,
    // The README lists these per event (Docked: Wanted, ActiveFine, CockpitBreach; FSDJump:
    // Wanted, BoostUsed, FuelLevel, FuelUsed, JumpDist and the faction keys; Location: Wanted,
    // Latitude, Longitude and the faction keys). The schema disallows every one of them on every
    // event, so they are removed from all of them, with the rest of its disallowed keys.
    elide: &[
        all("ActiveFine"),
        all("CockpitBreach"),
        all("BoostUsed"),
        all("FuelLevel"),
        all("FuelUsed"),
        all("JumpDist"),
        all("Latitude"),
        all("Longitude"),
        all("Wanted"),
        all("IsNewEntry"),
        all("NewTraitsDiscovered"),
        all("Traits"),
        all("VoucherAmount"),
        all("Factions[].HappiestSystem"),
        all("Factions[].HomeSystem"),
        all("Factions[].MyReputation"),
        all("Factions[].SquadronFaction"),
    ],
    // StarSystem only for SAASignalsFound; StarPos only for Docked, Scan and SAASignalsFound.
    augment: &[
        on("SAASignalsFound", SystemField::StarSystem),
        on("SAASignalsFound", SystemField::StarPos),
        on("Docked", SystemField::StarPos),
        on("Scan", SystemField::StarPos),
    ],
    flags: true,
};

/// The `journal/1` message for one journal event (its raw JSON), or `None` when the event isn't
/// sent on `journal/1`.
///
/// The schema requires `StarSystem`, `StarPos` and `SystemAddress`, and the README allows adding
/// only some of them, so an event missing one it can't be given is not sent: the gateway would
/// reject it.
pub fn message(ev: &Value) -> Option<(&'static SchemaSpec, Value)> {
    let needs: &[&str] = match ev.get("event")?.as_str()? {
        "FSDJump" | "Location" | "CarrierJump" => &["StarSystem", "SystemAddress", "StarPos"],
        "Docked" | "Scan" => &["StarSystem", "SystemAddress"],
        "SAASignalsFound" => &["SystemAddress"],
        _ => return None,
    };
    has_all(ev, needs).then(|| (&JOURNAL, ev.clone()))
}

/// Every key in `keys` is present and not `null`.
pub(crate) fn has_all(ev: &Value, keys: &[&str]) -> bool {
    keys.iter().all(|k| ev.get(*k).is_some_and(|v| !v.is_null()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn only_the_six_events_are_sent() {
        for e in ["Docked", "Scan"] {
            assert!(message(&json!({"event":e,"StarSystem":"Sol","SystemAddress":1})).is_some(), "{e}");
        }
        for e in ["FSDJump", "Location", "CarrierJump"] {
            let ev = json!({"event":e,"StarSystem":"Sol","SystemAddress":1,"StarPos":[0.0,0.0,0.0]});
            assert!(message(&ev).is_some(), "{e}");
        }
        assert!(message(&json!({"event":"SAASignalsFound","SystemAddress":1})).is_some());
        for e in ["CodexEntry", "Music", "FSSDiscoveryScan"] {
            assert!(message(&json!({"event":e,"StarSystem":"Sol","SystemAddress":1})).is_none(), "{e}");
        }
    }

    #[test]
    fn events_missing_a_field_that_cannot_be_added_are_not_sent() {
        assert!(message(&json!({"event":"Scan","SystemAddress":1})).is_none(), "StarSystem isn't added for Scan");
        assert!(message(&json!({"event":"FSDJump","StarSystem":"Sol","SystemAddress":1})).is_none());
        assert!(message(&json!({"event":"SAASignalsFound"})).is_none());
        assert!(message(&json!({"event":"Docked","StarSystem":"Sol","SystemAddress":null})).is_none());
    }
}
