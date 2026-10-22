//! One parser test per typed event, on the synthetic lines in `fixtures/events.jsonl`, plus the
//! raw-JSON guarantee the EDDN sender relies on.

use astromech_journal::{parse_line, ExtraEvent as E, JournalEvent, JournalLine};

const FIXTURE: &str = include_str!("fixtures/events.jsonl");

/// The fixture line for the `nth` occurrence of `event` (0-based).
fn line(event: &str, nth: usize) -> JournalLine {
    FIXTURE
        .lines()
        .map(|l| parse_line(l).expect("fixture line parses"))
        .filter(|l| l.name == event)
        .nth(nth)
        .unwrap_or_else(|| panic!("no fixture line for {event}"))
}

fn ex(event: &str) -> E {
    match line(event, 0).event {
        JournalEvent::Extra(e) => e,
        other => panic!("{event} parsed as {other:?}"),
    }
}

#[test]
fn market_outfitting_shipyard_point_at_the_station() {
    for (name, ctor) in [("Market", 0), ("Outfitting", 1), ("Shipyard", 2)] {
        let s = match (ctor, ex(name)) {
            (0, E::Market(s)) | (1, E::Outfitting(s)) | (2, E::Shipyard(s)) => s,
            (_, other) => panic!("{name} parsed as {other:?}"),
        };
        assert_eq!(s.market_id, Some(128000001));
        assert_eq!(s.station_name, "Test Orbital");
        assert_eq!(s.station_type.as_deref(), Some("Coriolis"));
        assert_eq!(s.star_system, "Test System");
    }
}

#[test]
fn cargo_inventory() {
    let E::Cargo { vessel, count, inventory } = ex("Cargo") else { panic!() };
    assert_eq!(vessel.as_deref(), Some("Ship"));
    assert_eq!(count, Some(12));
    assert_eq!(inventory[0].name_localised.as_deref(), Some("Gold"));
    assert!(!inventory[0].stolen);
    assert!(inventory[1].stolen);
    assert_eq!(inventory[1].mission_id, Some(77));
}

#[test]
fn market_buy_and_sell() {
    let E::MarketTrade(b) = ex("MarketBuy") else { panic!() };
    assert!(!b.sell);
    assert_eq!((b.commodity.as_str(), b.count, b.price, b.total), ("gold", 10, 9000, 90000));
    let E::MarketTrade(s) = ex("MarketSell") else { panic!() };
    assert!(s.sell && s.stolen_goods && !s.black_market);
    assert_eq!((s.price, s.total, s.avg_price_paid), (9500, 95000, Some(9000)));
}

#[test]
fn carrier_jump_stays_a_jump_and_keeps_the_dock() {
    match line("CarrierJump", 0).event {
        JournalEvent::FsdJump { from_carrier, star_system, system_address, star_pos, .. } => {
            assert!(from_carrier);
            assert_eq!(star_system, "Test Two");
            assert_eq!(system_address, Some(2));
            assert_eq!(star_pos, Some([1.5, 2.5, 3.5]));
        }
        other => panic!("{other:?}"),
    }
}

#[test]
fn jump_planning_events() {
    let E::FsdTarget { name, remaining_jumps, star_class, .. } = ex("FSDTarget") else { panic!() };
    assert_eq!((name.as_str(), remaining_jumps, star_class.as_deref()), ("Test Two", Some(3), Some("K")));
    let E::StartJump { jump_type, star_system, .. } = ex("StartJump") else { panic!() };
    assert_eq!((jump_type.as_str(), star_system.as_deref()), ("Hyperspace", Some("Test Two")));
    assert_eq!(ex("NavRoute"), E::NavRoute);
    assert_eq!(ex("NavRouteClear"), E::NavRouteClear);
}

#[test]
fn docking_granted_and_denied() {
    let E::Docking { granted, landing_pad, reason, market_id, .. } = ex("DockingGranted") else { panic!() };
    assert!(granted);
    assert_eq!((landing_pad, reason, market_id), (Some(12), None, Some(128000001)));
    let E::Docking { granted, reason, .. } = ex("DockingDenied") else { panic!() };
    assert!(!granted);
    assert_eq!(reason.as_deref(), Some("NoSpace"));
}

#[test]
fn supercruise_transitions() {
    let E::SupercruiseExit { star_system, body, body_id, body_type, system_address } = ex("SupercruiseExit") else {
        panic!()
    };
    assert_eq!(star_system, "Test Two");
    assert_eq!(
        (body.as_deref(), body_id, body_type.as_deref(), system_address),
        (Some("Test Two A 1"), Some(5), Some("Planet"), Some(2))
    );
    let E::SupercruiseEntry { star_system, system_address } = ex("SupercruiseEntry") else { panic!() };
    assert_eq!((star_system.as_str(), system_address), ("Test Two", Some(2)));
}

#[test]
fn fss_events() {
    let E::FssDiscoveryScan { progress, body_count, non_body_count, .. } = ex("FSSDiscoveryScan") else { panic!() };
    assert_eq!((progress, body_count, non_body_count), (0.5, 7, 3));
    let E::FssAllBodiesFound { system_name, count, .. } = ex("FSSAllBodiesFound") else { panic!() };
    assert_eq!((system_name.as_str(), count), ("Test Two", 7));
    match line("FSSSignalDiscovered", 0).event {
        JournalEvent::FssSignal { signal_name, is_station, system_address, high_grade, .. } => {
            assert_eq!(signal_name.as_deref(), Some("Test Carrier T1X-00Z"));
            assert!(is_station && !high_grade);
            assert_eq!(system_address, Some(2));
        }
        other => panic!("{other:?}"),
    }
    let E::FssBodySignals(b) = ex("FSSBodySignals") else { panic!() };
    assert_eq!(b.body_name, "Test Two A 1");
    assert_eq!(b.signals[0].count, 3);
    assert_eq!(b.signals[0].kind_localised.as_deref(), Some("Biological"));
    assert!(b.genuses.is_empty());
}

#[test]
fn saa_events() {
    let E::SaaSignalsFound(b) = ex("SAASignalsFound") else { panic!() };
    assert_eq!(b.body_id, Some(5));
    assert_eq!(b.genuses, vec!["$Codex_Ent_Bacterial_Genus_Name;".to_string()]);
    let E::SaaScanComplete { body_name, probes_used, efficiency_target, .. } = ex("SAAScanComplete") else { panic!() };
    assert_eq!((body_name.as_str(), probes_used, efficiency_target), ("Test Two A 1", 6, 7));
}

#[test]
fn scan_body_and_barycentre() {
    let E::Scan(b) = ex("Scan") else { panic!() };
    assert_eq!(b.scan_type, "Detailed");
    assert_eq!(b.planet_class.as_deref(), Some("High metal content body"));
    assert!(b.landable);
    assert_eq!(b.was_discovered, Some(true));
    assert_eq!(b.was_mapped, Some(false));
    assert_eq!(b.distance_from_arrival_ls, Some(123.4));
    assert_eq!(b.surface_gravity, Some(9.8));
    assert_eq!(b.materials, vec![("iron".to_string(), 18.5), ("nickel".to_string(), 14.0)]);
    assert_eq!((b.ring_count, b.belt_count), (1, 0));
    assert_eq!(b.terraform_state, None, "empty string is no value");
    assert_eq!(b.atmosphere_type.as_deref(), Some("CarbonDioxide"));
    assert_eq!(b.parents, vec![("Null".to_string(), 3), ("Star".to_string(), 1), ("Null".to_string(), 0)]);
    let E::ScanBaryCentre { body_id, system_address, .. } = ex("ScanBaryCentre") else { panic!() };
    assert_eq!((body_id, system_address), (Some(3), Some(2)));
}

#[test]
fn exobiology_events() {
    let E::ScanOrganic { scan_type, species_localised, variant, body, .. } = ex("ScanOrganic") else { panic!() };
    assert_eq!(scan_type, "Log");
    assert_eq!(species_localised.as_deref(), Some("Bacterium Aurasus"));
    assert_eq!(variant.as_deref(), Some("$Codex_Ent_Bacterial_01_M_Name;"));
    assert_eq!(body, Some(5));
    let E::SellOrganicData { market_id, bio_data } = ex("SellOrganicData") else { panic!() };
    assert_eq!(market_id, Some(128000001));
    assert_eq!((bio_data[0].value, bio_data[0].bonus), (1000, 2000));
}

#[test]
fn exploration_sales() {
    let E::ExplorationSale { multi, systems, base_value, bonus, total_earnings } = ex("SellExplorationData") else {
        panic!()
    };
    assert!(!multi);
    assert_eq!(systems, vec!["Test Two", "Test Three"]);
    assert_eq!((base_value, bonus, total_earnings), (5000, 100, 5100));
    let E::ExplorationSale { multi, systems, total_earnings, .. } = ex("MultiSellExplorationData") else { panic!() };
    assert!(multi);
    assert_eq!((systems, total_earnings), (vec!["Test Two".to_string()], 6000));
}

#[test]
fn codex_settlement_and_beacon() {
    let E::CodexEntry(c) = ex("CodexEntry") else { panic!() };
    assert_eq!(c.entry_id, Some(1100101));
    assert_eq!(c.name_localised.as_deref(), Some("Test Thing"));
    assert!(c.is_new_entry && !c.new_traits_discovered);
    assert_eq!(c.voucher_amount, Some(2500));
    let E::ApproachSettlement { name, market_id, latitude, longitude, .. } = ex("ApproachSettlement") else { panic!() };
    assert_eq!(
        (name.as_str(), market_id, latitude, longitude),
        ("Test Outpost", Some(128000009), Some(10.5), Some(-20.25))
    );
    let E::NavBeaconScan { system_address, num_bodies } = ex("NavBeaconScan") else { panic!() };
    assert_eq!((system_address, num_bodies), (Some(2), 9));
}

#[test]
fn missions() {
    let E::Missions { active, failed, complete } = ex("Missions") else { panic!() };
    assert_eq!((active.len(), failed.len(), complete.len()), (1, 0, 1));
    assert_eq!((active[0].mission_id, active[0].expires_secs), (1, Some(3600)));
    let E::MissionAccepted(m) = ex("MissionAccepted") else { panic!() };
    assert_eq!((m.mission_id, m.faction.as_str(), m.count, m.reward), (1, "Test Faction", Some(10), Some(50000)));
    assert_eq!(m.destination_station.as_deref(), Some("Test Orbital"));
    assert_eq!(m.expiry.as_deref(), Some("2026-10-12T20:00:00Z"));
    let E::MissionCompleted { mission_id, reward, donation, materials_reward, .. } = ex("MissionCompleted") else {
        panic!()
    };
    assert_eq!((mission_id, reward, materials_reward[0].count), (Some(1), Some(50000), 3));
    assert_eq!(donation, Some(1_000_000), "`Donation` is a string in real lines");
    let l = parse_line(r#"{"timestamp":"t","event":"MissionCompleted","Name":"m","Donated":500,"Donation":"500"}"#)
        .unwrap();
    assert!(matches!(l.event, JournalEvent::Extra(E::MissionCompleted { donation: Some(500), .. })));
    let E::MissionEnded { failed, mission_id, .. } = ex("MissionAbandoned") else { panic!() };
    assert_eq!((failed, mission_id), (false, Some(3)));
    let E::MissionEnded { failed, fine, .. } = ex("MissionFailed") else { panic!() };
    assert_eq!((failed, fine), (true, Some(1000)));
    let l = parse_line(r#"{"timestamp":"t","event":"MissionRedirected","MissionID":1,"Name":"Mission_X"}"#).unwrap();
    assert!(matches!(l.event, JournalEvent::Extra(E::MissionRedirected { ref name, .. }) if name == "Mission_X"));
    let E::MissionRedirected { new_destination_system, new_destination_station, .. } = ex("MissionRedirected") else {
        panic!()
    };
    assert_eq!(
        (new_destination_system.as_deref(), new_destination_station.as_deref()),
        (Some("Test Three"), Some("Test Port"))
    );
}

#[test]
fn bounty_and_voucher() {
    let E::Bounty { rewards, total_reward, victim_faction, target } = ex("Bounty") else { panic!() };
    assert_eq!((rewards[0].faction.as_str(), rewards[0].reward, total_reward), ("Test Faction", 4000, Some(4000)));
    assert_eq!((victim_faction.as_deref(), target.as_deref()), (Some("Pirates"), Some("viper")));
    // The pre-3.3 shape: one faction and reward at the top level.
    let l = parse_line(r#"{"timestamp":"t","event":"Bounty","Faction":"Old","Reward":10}"#).unwrap();
    let JournalEvent::Extra(E::Bounty { rewards, total_reward, .. }) = l.event else { panic!() };
    assert_eq!((rewards[0].faction.as_str(), total_reward), ("Old", Some(10)));
    let E::RedeemVoucher { kind, amount, factions } = ex("RedeemVoucher") else { panic!() };
    assert_eq!((kind.as_str(), amount, factions), ("bounty", 4000, vec!["Test Faction".to_string()]));
}

#[test]
fn mining() {
    let E::ProspectedAsteroid { materials, content, motherlode_material, remaining } = ex("ProspectedAsteroid") else {
        panic!()
    };
    assert_eq!((materials[0].name.as_str(), materials[0].proportion), ("platinum", 22.5));
    assert_eq!(content.as_deref(), Some("$AsteroidMaterialContent_High;"));
    assert_eq!((motherlode_material.as_deref(), remaining), (Some("painite"), Some(100.0)));
    let E::MiningRefined { commodity } = ex("MiningRefined") else { panic!() };
    assert_eq!(commodity, "$platinum_name;");
    let E::LaunchDrone { kind } = ex("LaunchDrone") else { panic!() };
    assert_eq!(kind, "Prospector");
}

#[test]
fn powerplay_family() {
    let E::Powerplay(p) = ex("Powerplay") else { panic!() };
    assert_eq!(
        (p.kind.as_str(), p.rank, p.total_merits, p.time_pledged_secs),
        ("Powerplay", Some(3), Some(1200), Some(86400))
    );
    let E::Powerplay(p) = ex("PowerplayMerits") else { panic!() };
    assert_eq!((p.merits_gained, p.total_merits, p.power.as_deref()), (Some(50), Some(1250), Some("Test Power")));
    let E::Powerplay(p) = ex("PowerplayDefect") else { panic!() };
    assert_eq!((p.from_power.as_deref(), p.to_power.as_deref()), (Some("Test Power"), Some("Other Power")));
    let E::Powerplay(p) = ex("PowerplayRank") else { panic!() };
    assert_eq!((p.power.as_deref(), p.rank), (Some("Other Power"), Some(4)));
}

#[test]
fn system_powerplay_on_jump_and_location() {
    match line("FSDJump", 0).event {
        JournalEvent::FsdJump { powerplay: Some(p), .. } => {
            assert_eq!(p.controlling_power.as_deref(), Some("Test Power"));
            assert_eq!(p.powers, ["Test Power", "Other Power"]);
            assert_eq!(p.state.as_deref(), Some("Fortified"));
            assert_eq!((p.control_progress, p.reinforcement, p.undermining), (Some(0.277808), Some(82), Some(3707)));
            assert!(p.conflict.is_empty());
        }
        other => panic!("{other:?}"),
    }
    match line("Location", 0).event {
        JournalEvent::Location { powerplay: Some(p), .. } => {
            assert_eq!((p.controlling_power.as_deref(), p.state.as_deref()), (None, Some("Unoccupied")));
            assert_eq!((p.conflict[0].power.as_str(), p.conflict[0].progress), ("Test Power", 0.086375));
            assert_eq!(p.conflict[1].power, "Other Power");
        }
        other => panic!("{other:?}"),
    }
    // A system without Powerplay has no fields at all.
    assert!(matches!(line("CarrierJump", 0).event, JournalEvent::FsdJump { powerplay: None, .. }));
}

#[test]
fn colonisation_family() {
    let E::ColonisationConstructionDepot(d) = ex("ColonisationConstructionDepot") else { panic!() };
    assert_eq!((d.market_id, d.progress, d.complete, d.failed), (Some(3900000001), 0.25, false, false));
    assert_eq!((d.resources[0].required, d.resources[0].provided, d.resources[0].payment), (800, 200, 4000));
    assert_eq!(d.resources[0].name_localised.as_deref(), Some("Steel"));
    let E::ColonisationContribution { market_id, contributions } = ex("ColonisationContribution") else { panic!() };
    assert_eq!((market_id, contributions), (Some(3900000001), vec![("$steel_name;".to_string(), 120)]));
}

#[test]
fn carrier_family() {
    let E::CarrierStats(c) = ex("CarrierStats") else { panic!() };
    assert_eq!(
        (c.callsign.as_str(), c.fuel_level, c.balance, c.reserve_balance, c.free_space),
        ("T1X-00Z", Some(500), Some(1000000), Some(500000), Some(20000))
    );
    assert_eq!(c.allow_notorious, Some(false));
    let E::CarrierJumpPlan { cancelled, system_name, departure_time, .. } = ex("CarrierJumpRequest") else { panic!() };
    assert!(!cancelled);
    assert_eq!((system_name.as_deref(), departure_time.as_deref()), (Some("Test Two"), Some("2026-10-10T21:00:00Z")));
    let E::CarrierJumpPlan { cancelled, system_name, .. } = ex("CarrierJumpCancelled") else { panic!() };
    assert!(cancelled && system_name.is_none());
}

#[test]
fn ship_locker() {
    let E::ShipLocker { items } = ex("ShipLocker") else { panic!() };
    assert_eq!(items.len(), 3);
    assert_eq!((items[0].category.as_str(), items[0].name.as_str(), items[0].count), ("Items", "healthpack", 4));
    assert_eq!((items[1].category.as_str(), items[1].mission_id), ("Components", Some(9)));
    assert_eq!(items[2].category, "Data");
}

#[test]
fn statistics_flattens_numeric_counters() {
    let E::Statistics { sections } = ex("Statistics") else { panic!() };
    let bank = sections.iter().find(|(k, _)| k == "Bank_Account").unwrap();
    assert!(bank.1.contains(&("Current_Wealth".to_string(), 123456789.0)));
    assert!(sections.iter().any(|(k, c)| k == "Combat" && c[0] == ("Bounties_Claimed".to_string(), 42.0)));
    assert!(sections.iter().all(|(k, _)| k != "event" && k != "timestamp"));
}

#[test]
fn rank_progress_reputation() {
    let E::Rank(r) = ex("Rank") else { panic!() };
    assert_eq!((r.combat, r.trade, r.explore, r.exobiologist, r.cqc), (Some(5), Some(6), Some(7), Some(2), Some(0)));
    let E::Progress(p) = ex("Progress") else { panic!() };
    assert_eq!((p.combat, p.federation), (Some(10), Some(60)));
    let E::Reputation(r) = ex("Reputation") else { panic!() };
    assert_eq!((r.empire, r.independent, r.alliance), (Some(10.5), Some(-5.0), None));
}

#[test]
fn combat_events() {
    let E::Interdicted(i) = ex("Interdicted") else { panic!() };
    assert!(!i.submitted && !i.is_player);
    assert_eq!((i.interdictor.as_str(), i.faction.as_deref()), ("Test Pirate", Some("Pirates")));
    let E::ShipTargeted(t) = ex("ShipTargeted") else { panic!() };
    assert!(t.target_locked);
    assert_eq!(
        (t.ship.as_deref(), t.scan_stage, t.bounty, t.hull_health),
        (Some("viper"), Some(3), Some(4000), Some(90.5))
    );
    assert_eq!(t.legal_status.as_deref(), Some("Wanted"));
    assert_eq!((t.subsystem.as_deref(), t.subsystem_health), (Some("$int_powerplant;"), Some(55.5)));
    let E::Scanned { scan_type } = ex("Scanned") else { panic!() };
    assert_eq!(scan_type, "Cargo");
}

#[test]
fn fuel_chat_and_surface_events() {
    let E::FuelScoop { scooped, total } = ex("FuelScoop") else { panic!() };
    assert_eq!((scooped, total), (4.5, 28.5));
    let E::ReceiveText { from, message, channel } = ex("ReceiveText") else { panic!() };
    assert_eq!((from.as_str(), message.as_str(), channel.as_str()), ("Test Pilot", "hello", "local"));
    let E::Touchdown(t) = ex("Touchdown") else { panic!() };
    assert!(t.player_controlled && t.on_planet == Some(true));
    assert_eq!((t.latitude, t.longitude), (Some(10.5), Some(-20.25)));
    assert_eq!((t.system_address, t.body.as_deref(), t.body_id), (Some(2), Some("Test Two A 1"), Some(5)));
    let E::Disembark(d) = ex("Disembark") else { panic!() };
    assert!(!d.srv && d.on_planet && !d.on_station);
    assert_eq!((d.body.as_deref(), d.body_id), (Some("Test Two A 1"), Some(5)));
}

#[test]
fn unknown_events_still_fall_through_to_other() {
    let l = parse_line(r#"{"timestamp":"t","event":"SomethingNew","X":1}"#).unwrap();
    assert!(matches!(l.event, JournalEvent::Other { ref event, .. } if event == "SomethingNew"));
    // Powerplay2-era events outside the typed set are still `Other` or `Powerplay`, never an error.
    assert!(parse_line(r#"{"timestamp":"t","event":"ColonisationBeaconDeployed"}"#).is_ok());
}

#[test]
fn every_fixture_line_is_typed_and_keeps_its_raw_json_byte_for_byte() {
    let mut n = 0;
    for original in FIXTURE.lines() {
        let l = parse_line(original).unwrap();
        assert_eq!(l.raw_json, original, "{} was re-serialised", l.name);
        assert!(!matches!(l.event, JournalEvent::Other { .. }), "{} fell through to Other", l.name);
        n += 1;
    }
    assert!(n >= 60);
}

#[test]
fn raw_json_survives_odd_spacing_key_order_unicode_and_number_forms() {
    // Not what serde_json would write back: spaces, reordered keys, escapes, 1.0 and exponent
    // forms, duplicate-looking keys in nested objects.
    let original = "{ \"Message\":\"caf\\u00e9 \\\"quoted\\\"\" ,\"event\":\"ReceiveText\",  \"From\":\"Zo\u{eb}\", \"Channel\":\"npc\",\"timestamp\":\"2026-10-10T20:00:00Z\",\"x\":1.0,\"y\":1E3,\"z\":{\"a\":1,\"a\":2} }";
    let l = parse_line(original).unwrap();
    assert_eq!(l.raw_json, original);
    let JournalEvent::Extra(E::ReceiveText { from, message, .. }) = l.event else { panic!() };
    assert_eq!((from.as_str(), message.as_str()), ("Zo\u{eb}", "caf\u{e9} \"quoted\""));
    // Untyped events keep it too.
    let other = r#"{ "timestamp":"t",  "event":"Whatever", "B":1.50, "A":[1,  2] }"#;
    assert_eq!(parse_line(other).unwrap().raw_json, other);
}

#[test]
fn raw_json_is_kept_when_reading_a_journal_file() {
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path().join("Journal.2026-10-10T200000.01.log");
    let first = r#"{ "timestamp":"t", "event":"Market", "StarSystem":"Test System", "MarketID":1 }"#;
    let second = r#"{"event":"FuelScoop","timestamp":"t","Total":3.0,"Scooped":1.0}"#;
    std::fs::write(&p, format!("{first}\r\n{second}\r\n")).unwrap();
    let lines = astromech_journal::parse_file(&p);
    assert_eq!(lines.iter().map(|l| l.raw_json.as_str()).collect::<Vec<_>>(), vec![first, second]);
}

#[test]
fn a_scans_asteroid_belts_are_not_rings() {
    let l = parse_line(
        r#"{"timestamp":"t","event":"Scan","ScanType":"AutoScan","BodyName":"Sys A","StarType":"K","Rings":[{"Name":"Sys A A Belt","RingClass":"eRingClass_Rocky"},{"Name":"Sys A B Belt","RingClass":"eRingClass_MetalRich"}]}"#,
    )
    .unwrap();
    let JournalEvent::Extra(E::Scan(b)) = l.event else { panic!() };
    assert_eq!((b.ring_count, b.belt_count), (0, 2));
}
