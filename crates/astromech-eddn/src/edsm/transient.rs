//! The "transient states" EDSM asks a sender to add to each event sent one at a time, written
//! from the `updateGameStatus` example in EDSM's Journal API v1 docs
//! (<https://www.edsm.net/en/api-journal-v1>, revision of 30 Nov 2022): most events don't say
//! where the commander is or in which ship, so each event goes with `_systemAddress`,
//! `_systemName`, `_systemCoordinates`, `_marketId`, `_stationName` and `_shipId` (null when
//! unknown).

use crate::context::GameContext;
use serde_json::{Map, Value};

#[derive(Debug, Clone, Default)]
pub struct Transient {
    /// Header, commander and beta flag (shared with EDDN's rules).
    pub ctx: GameContext,
    system_address: Value,
    system_name: Value,
    coordinates: Value,
    market_id: Value,
    station_name: Value,
    ship_id: Value,
    /// In another commander's crew: EDSM keeps no log for a crew member (answer 104), so
    /// nothing is sent until the crew session ends.
    crew_member: bool,
}

impl Transient {
    /// Feed every journal event, in order. Returns the event with the transient fields added.
    pub fn apply(&mut self, ev: &Value) -> Value {
        self.ctx.observe(ev);
        let get = |k: &str| ev.get(k).cloned().filter(|v| !v.is_null());
        match ev.get("event").and_then(Value::as_str).unwrap_or_default() {
            "Fileheader" if ev.get("part").and_then(Value::as_u64).unwrap_or(1) <= 1 => {
                // A new game session: nothing from the last one carries over.
                *self = Self { ctx: self.ctx.clone(), ..Self::default() };
            }
            "LoadGame" => self.leave_system(),
            "SetUserShipName" | "ShipyardSwap" | "Loadout" => self.ship_id = get("ShipID").unwrap_or(Value::Null),
            "ShipyardBuy" => self.ship_id = Value::Null,
            "Undocked" => self.leave_station(),
            // EDSM's example handles Location, FSDJump and Docked. A carrier jump moves the
            // commander the same way and carries the same fields, so it's handled with them.
            "Location" | "FSDJump" | "Docked" | "CarrierJump" => {
                let system = get("StarSystem").unwrap_or(Value::Null);
                // Docked has no coordinates: they're only kept within the same system.
                if system != self.system_name {
                    self.coordinates = Value::Null;
                }
                if matches!(system.as_str(), Some("ProvingGround" | "CQC")) {
                    self.system_address = Value::Null;
                    self.system_name = Value::Null;
                    self.coordinates = Value::Null;
                } else {
                    if let Some(a) = get("SystemAddress") {
                        self.system_address = a;
                    }
                    self.system_name = system;
                    if let Some(p) = get("StarPos") {
                        self.coordinates = p;
                    }
                }
                if let Some(m) = get("MarketID") {
                    self.market_id = m;
                }
                if let Some(s) = get("StationName") {
                    self.station_name = s;
                }
            }
            e @ ("JoinACrew" | "QuitACrew") => {
                let captain = ev.get("Captain").and_then(Value::as_str);
                self.crew_member = e == "JoinACrew" && captain != self.ctx.commander.as_deref();
                self.leave_system();
            }
            _ => {}
        }
        let mut out = ev.as_object().cloned().unwrap_or_else(Map::new);
        for (k, v) in [
            ("_systemAddress", &self.system_address),
            ("_systemName", &self.system_name),
            ("_systemCoordinates", &self.coordinates),
            ("_marketId", &self.market_id),
            ("_stationName", &self.station_name),
            ("_shipId", &self.ship_id),
        ] {
            out.insert(k.to_owned(), v.clone());
        }
        Value::Object(out)
    }

    /// In another commander's crew (nothing is sent).
    pub fn crew_member(&self) -> bool {
        self.crew_member
    }

    fn leave_system(&mut self) {
        self.system_address = Value::Null;
        self.system_name = Value::Null;
        self.coordinates = Value::Null;
        self.leave_station();
    }

    fn leave_station(&mut self) {
        self.market_id = Value::Null;
        self.station_name = Value::Null;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn fields(v: &Value) -> Value {
        json!([
            v["_systemAddress"],
            v["_systemName"],
            v["_systemCoordinates"],
            v["_marketId"],
            v["_stationName"],
            v["_shipId"]
        ])
    }

    #[test]
    fn events_carry_where_the_commander_is_and_in_which_ship() {
        let mut t = Transient::default();
        t.apply(&json!({"event":"LoadGame","Commander":"Jameson"}));
        t.apply(&json!({"event":"Loadout","ShipID":7}));
        t.apply(&json!({"event":"Location","StarSystem":"Shinrarta Dezhra","SystemAddress":3932277478106u64,
            "StarPos":[55.71875,17.59375,27.15625],"Docked":true,"MarketID":128666762,"StationName":"Jameson Memorial"}));
        let e = t.apply(&json!({"event":"MarketBuy","Type":"gold","Count":1}));
        assert_eq!(e["event"], "MarketBuy", "the event itself is unchanged");
        assert_eq!(
            fields(&e),
            json!([
                3932277478106u64,
                "Shinrarta Dezhra",
                [55.71875, 17.59375, 27.15625],
                128666762,
                "Jameson Memorial",
                7
            ])
        );
        let e = t.apply(&json!({"event":"Undocked"}));
        assert_eq!(fields(&e)[3], Value::Null);
        assert_eq!(fields(&e)[4], Value::Null);
        assert_eq!(fields(&e)[1], "Shinrarta Dezhra");
        // Docked in another system (no StarPos): the old coordinates don't carry over.
        let e = t.apply(&json!({"event":"Docked","StarSystem":"Sol","SystemAddress":10477373803u64,
            "MarketID":128016640,"StationName":"Abraham Lincoln"}));
        assert_eq!(fields(&e), json!([10477373803u64, "Sol", null, 128016640, "Abraham Lincoln", 7]));
        let e = t.apply(&json!({"event":"ShipyardBuy","ShipType":"sidewinder"}));
        assert_eq!(fields(&e)[5], Value::Null);
        let e = t.apply(&json!({"event":"LoadGame","Commander":"Jameson"}));
        assert_eq!(fields(&e), json!([null, null, null, null, null, null]));
    }

    #[test]
    fn proving_ground_and_cqc_are_no_system() {
        let mut t = Transient::default();
        t.apply(&json!({"event":"FSDJump","StarSystem":"Sol","SystemAddress":10477373803u64,"StarPos":[0,0,0]}));
        let e = t.apply(&json!({"event":"Location","StarSystem":"ProvingGround","SystemAddress":1}));
        assert_eq!(fields(&e), json!([null, null, null, null, null, null]));
    }

    #[test]
    fn another_commanders_crew_is_a_crew_session_until_it_ends() {
        let mut t = Transient::default();
        t.apply(&json!({"event":"Commander","Name":"Jameson"}));
        t.apply(&json!({"event":"JoinACrew","Captain":"Jameson"}));
        assert!(!t.crew_member(), "one's own ship");
        t.apply(&json!({"event":"JoinACrew","Captain":"Someone Else"}));
        assert!(t.crew_member());
        t.apply(&json!({"event":"QuitACrew","Captain":"Someone Else"}));
        assert!(!t.crew_member());
    }
}
