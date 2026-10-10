use s5_status_latency::binds::{self, BindsDoc, StartPreset};
use s5_status_latency::keys::{self, KeyLookup, ScanKey};
use std::path::PathBuf;

fn fixture(name: &str) -> String {
    std::fs::read_to_string(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures").join(name)).unwrap()
}

fn k(scan: u16, extended: bool) -> ScanKey {
    ScanKey { scan, extended }
}

// ---------- scancode table ----------

#[test]
fn common_keys_map_to_set1_scancodes() {
    let ok = |n: &str| match keys::lookup(n) {
        KeyLookup::Ok(k) => k,
        other => panic!("{n}: {other:?}"),
    };
    assert_eq!(ok("Key_L"), k(0x26, false));
    assert_eq!(ok("Key_Insert"), k(0x52, true));
    assert_eq!(ok("Key_Delete"), k(0x53, true));
    assert_eq!(ok("Key_Home"), k(0x47, true));
    assert_eq!(ok("Key_UpArrow"), k(0x48, true));
    assert_eq!(ok("Key_DownArrow"), k(0x50, true));
    assert_eq!(ok("Key_LeftArrow"), k(0x4B, true));
    assert_eq!(ok("Key_RightArrow"), k(0x4D, true));
    assert_eq!(ok("Key_Numpad_Enter"), k(0x1C, true));
    assert_eq!(ok("Key_Enter"), k(0x1C, false));
    assert_eq!(ok("Key_RightControl"), k(0x1D, true));
    assert_eq!(ok("Key_LeftControl"), k(0x1D, false));
    assert_eq!(ok("Key_RightAlt"), k(0x38, true));
    assert_eq!(ok("Key_LeftAlt"), k(0x38, false));
    assert_eq!(ok("Key_Numpad_Divide"), k(0x35, true));
    assert_eq!(ok("Key_Slash"), k(0x35, false));
    assert_eq!(ok("Key_Numpad_8"), k(0x48, false)); // same make code as Up, but not extended
    assert_eq!(ok("Key_Numpad_Multiply"), k(0x37, false));
    assert_eq!(ok("Key_SYSRQ"), k(0x37, true));
    assert_eq!(ok("Key_Apps"), k(0x5D, true));
    assert_eq!(ok("Key_F12"), k(0x58, false));
    assert_eq!(ok("Key_OEM_102"), k(0x56, false));
}

#[test]
fn pause_media_and_layout_specific_keys_are_not_guessed() {
    assert!(matches!(keys::lookup("Key_Pause"), KeyLookup::Refused(_)));
    assert!(matches!(keys::lookup("Key_VolumeUp"), KeyLookup::Refused(_)));
    assert!(matches!(keys::lookup("Key_ä"), KeyLookup::Unknown));
    assert!(matches!(keys::lookup("Key_Return"), KeyLookup::Unknown));
    assert_eq!(keys::lookup("Key_Hash"), KeyLookup::Unverified(k(0x2B, false)));
    assert_eq!(keys::lookup("Key_LessThan"), KeyLookup::Unverified(k(0x56, false)));
}

/// Cross-check against COVAS:NEXT's keymap.json (DirectInput DIK values, MIT licensed).
#[test]
fn table_agrees_with_covas_keymap() {
    let v: serde_json::Value = serde_json::from_str(&fixture("covas_keymap.json")).unwrap();
    let map = v.as_object().unwrap();
    let mut compared = 0;
    for (name, key) in keys::table() {
        if let Some(dik) = map.get(name).and_then(|x| x.as_u64()) {
            assert_eq!(key.dik() as u64, dik, "{name}: ours {key} vs COVAS DIK {dik:#X}");
            compared += 1;
        }
    }
    assert!(compared >= 100, "only {compared} keys compared");
    // Every named (non "??") COVAS key we don't map must be one we refuse or deliberately skip.
    for (name, _) in map {
        if name.starts_with("Key_") && !matches!(keys::lookup(name), KeyLookup::Ok(_)) {
            assert!(
                matches!(keys::lookup(name), KeyLookup::Refused(_)),
                "COVAS knows {name} but we neither map nor refuse it"
            );
        }
    }
}

#[test]
fn scancode_override_parsing() {
    assert_eq!(keys::parse_scancode("0x26"), Some(0x26));
    assert_eq!(keys::parse_scancode("26h"), Some(0x26));
    assert_eq!(keys::parse_scancode("38"), Some(38));
    assert_eq!(keys::parse_scancode("0x80"), None);
    assert_eq!(keys::parse_scancode("0"), None);
    assert_eq!(keys::parse_scancode("L"), None);
}

// ---------- StartPreset ----------

#[test]
fn startpreset_single_line_legacy() {
    let sp = StartPreset::parse("StartPreset.start".into(), "Custom").unwrap();
    assert_eq!(sp.ship_preset(), "Custom");
    assert!(sp.all_same());
}

#[test]
fn startpreset_four_lines_uses_ship_line() {
    let sp = StartPreset::parse("StartPreset.4.start".into(), "\u{feff}GeneralP\r\nShipP\r\nSrvP\r\nFootP\r\n").unwrap();
    assert_eq!(sp.ship_preset(), "ShipP");
    assert!(!sp.all_same());
    let ctx = sp.context_names();
    assert_eq!(ctx[0], ("General", "GeneralP"));
    assert_eq!(ctx[3], ("OnFoot", "FootP"));
}

#[test]
fn startpreset_identical_lines() {
    let sp = StartPreset::parse("x".into(), "Custom\nCustom\nCustom\nCustom\n").unwrap();
    assert_eq!(sp.ship_preset(), "Custom");
    assert!(sp.all_same());
}

#[test]
fn startpreset_empty_is_error() {
    assert!(StartPreset::parse("x".into(), "\n \n").is_err());
}

#[test]
fn finds_newest_startpreset_and_highest_binds_version() {
    let d = tempfile::tempdir().unwrap();
    let p = d.path();
    std::fs::write(p.join("StartPreset.start"), "Old").unwrap();
    std::thread::sleep(std::time::Duration::from_millis(20));
    std::fs::write(p.join("StartPreset.4.start"), "Custom\nCustom\nCustom\nCustom").unwrap();
    for f in ["Custom.binds", "Custom.3.0.binds", "Custom.4.0.binds", "Custom.4.2.binds", "Custom.4.10.binds.bak", "CustomX.4.9.binds", "Old.4.2.binds"] {
        std::fs::write(p.join(f), "<Root/>").unwrap();
    }
    let sp = binds::find_start_preset(p).unwrap();
    assert_eq!(sp.ship_preset(), "Custom");
    let b = binds::find_binds_file(p, "Custom").unwrap();
    assert_eq!(b.file_name().unwrap(), "Custom.4.2.binds");
}

#[test]
fn binds_version_parsing() {
    assert_eq!(binds::binds_version("Custom.4.2.binds", "Custom"), Some((4, 2)));
    assert_eq!(binds::binds_version("Custom.4.10.binds", "Custom"), Some((4, 10)));
    assert_eq!(binds::binds_version("custom.3.0.binds", "Custom"), Some((3, 0)));
    assert_eq!(binds::binds_version("Custom.binds", "Custom"), Some((0, 0)));
    assert_eq!(binds::binds_version("HCS Custom.4.2.binds", "HCS Custom"), Some((4, 2)));
    assert_eq!(binds::binds_version("CustomX.4.2.binds", "Custom"), None);
    assert_eq!(binds::binds_version("Custom.4.2.binds.bak", "Custom"), None);
}

#[test]
fn missing_binds_explains_builtin_presets() {
    let d = tempfile::tempdir().unwrap();
    let e = binds::find_binds_file(d.path(), "KeyboardMouseOnly").unwrap_err().to_string();
    assert!(e.contains("built-in"), "{e}");
    assert!(e.contains("--binds-file"), "{e}");
}

// ---------- .binds parsing ----------

#[test]
fn covas_default_preset_lights_gear_nightvision() {
    let doc = BindsDoc::parse(&fixture("covas_DefaultKbdM.4.2.binds")).unwrap();
    assert_eq!(doc.preset_name.as_deref(), Some("Custom"));
    assert_eq!(doc.version.as_deref(), Some("4.2"));
    assert_eq!(doc.keyboard_layout.as_deref(), Some("en-US"));

    let lights = binds::select_keyboard_chord("ShipSpotLightToggle", &doc.action("ShipSpotLightToggle").unwrap()).unwrap();
    assert_eq!(lights.source, "Primary");
    assert_eq!(lights.key_name, "Key_Insert");
    assert_eq!(lights.key, k(0x52, true));
    assert!(lights.modifiers.is_empty());

    let gear = binds::select_keyboard_chord("LandingGearToggle", &doc.action("LandingGearToggle").unwrap()).unwrap();
    assert_eq!(gear.key, k(0x26, false));

    let nv = binds::select_keyboard_chord("NightVisionToggle", &doc.action("NightVisionToggle").unwrap());
    let msg = nv.unwrap_err().to_string();
    assert!(msg.contains("no usable keyboard binding"), "{msg}");
    assert!(msg.contains("--scancode"), "{msg}");
}

#[test]
fn every_keyboard_binding_in_covas_preset_maps() {
    let xml = fixture("covas_DefaultKbdM.4.2.binds");
    let doc = roxmltree::Document::parse(&xml).unwrap();
    let mut n = 0;
    for node in doc.descendants().filter(|n| n.attribute("Device") == Some("Keyboard")) {
        let key = node.attribute("Key").unwrap();
        if key.is_empty() || key == "Key_Invalid" {
            continue;
        }
        assert!(matches!(keys::lookup(key), KeyLookup::Ok(_)), "unmapped key {key}");
        n += 1;
    }
    assert!(n > 50);
}

#[test]
fn synthetic_secondary_with_modifiers_when_primary_is_joystick() {
    let doc = BindsDoc::parse(&fixture("synthetic_custom.4.2.binds")).unwrap();
    let c = binds::select_keyboard_chord("ShipSpotLightToggle", &doc.action("ShipSpotLightToggle").unwrap()).unwrap();
    assert_eq!(c.source, "Secondary");
    assert_eq!(c.key, k(0x31, false));
    assert_eq!(
        c.modifiers,
        vec![("Key_LeftAlt".to_string(), k(0x38, false)), ("Key_RightControl".to_string(), k(0x1D, true))]
    );
    assert!(c.notes[0].contains("not the keyboard"));
}

#[test]
fn synthetic_extended_primary() {
    let doc = BindsDoc::parse(&fixture("synthetic_custom.4.2.binds")).unwrap();
    let c = binds::select_keyboard_chord("NightVisionToggle", &doc.action("NightVisionToggle").unwrap()).unwrap();
    assert_eq!(c.key, k(0x1C, true));
}

#[test]
fn synthetic_mouse_only_is_rejected() {
    let doc = BindsDoc::parse(&fixture("synthetic_custom.4.2.binds")).unwrap();
    let e = binds::select_keyboard_chord("LandingGearToggle", &doc.action("LandingGearToggle").unwrap()).unwrap_err();
    assert!(e.to_string().contains("'Mouse'"), "{e}");
}

#[test]
fn synthetic_joystick_modifier_is_rejected() {
    let doc = BindsDoc::parse(&fixture("synthetic_custom.4.2.binds")).unwrap();
    let e = binds::select_keyboard_chord("HardpointsToggle", &doc.action("HardpointsToggle").unwrap()).unwrap_err();
    assert!(e.to_string().contains("modifier Joy_1"), "{e}");
}

#[test]
fn synthetic_unsupported_keys_are_rejected_not_guessed() {
    let doc = BindsDoc::parse(&fixture("synthetic_custom.4.2.binds")).unwrap();
    let e = binds::select_keyboard_chord("CargoScoop", &doc.action("CargoScoop").unwrap()).unwrap_err().to_string();
    assert!(e.contains("Key_Pause"), "{e}");
    assert!(e.contains("Key_ä"), "{e}");
}

#[test]
fn unverified_mapping_is_flagged() {
    let doc = BindsDoc::parse(&fixture("synthetic_custom.4.2.binds")).unwrap();
    let mut ab = doc.action("UI_Up").unwrap();
    ab.primary = None;
    let c = binds::select_keyboard_chord("UI_Up", &ab).unwrap();
    assert_eq!(c.key, k(0x2B, false));
    assert!(c.notes.iter().any(|n| n.contains("unverified")));
}

/// Optional: a real user preset from EliteAPI's test files (not redistributed here; it has no licence file).
/// Run with S5_EXTRA_BINDS=/path/to/"HCS Custom.4.2.binds".
#[test]
fn optional_external_binds_file() {
    let Ok(p) = std::env::var("S5_EXTRA_BINDS") else { return };
    let doc = BindsDoc::parse(&std::fs::read_to_string(p).unwrap()).unwrap();
    for a in ["ShipSpotLightToggle", "NightVisionToggle", "LandingGearToggle"] {
        let c = binds::select_keyboard_chord(a, &doc.action(a).unwrap()).unwrap();
        println!("{a}: {} [{}]", c.describe(), c.source);
    }
    let lights = binds::select_keyboard_chord("ShipSpotLightToggle", &doc.action("ShipSpotLightToggle").unwrap()).unwrap();
    assert_eq!(lights.source, "Secondary");
    assert_eq!(lights.modifiers.len(), 2);
}

#[test]
fn stats_percentiles() {
    use s5_status_latency::stats::summarize;
    let s = summarize(&[Some(10.0), Some(30.0), None, Some(20.0), Some(40.0)]);
    assert_eq!(s.n, 4);
    assert_eq!(s.misses, 1);
    assert_eq!(s.min, Some(10.0));
    assert_eq!(s.median, Some(25.0));
    assert_eq!(s.p95, Some(40.0));
    assert_eq!(s.max, Some(40.0));
    let e = summarize(&[None, None]);
    assert_eq!(e.n, 0);
    assert_eq!(e.median, None);
}
