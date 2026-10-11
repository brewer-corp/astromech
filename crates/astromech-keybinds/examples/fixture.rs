//! Writes the browser-preview fixture (app/src/lib/fixtures/sample.json): the app's State and the
//! View of the example preset, as the shell would return them, with a fake VKB pair connected.
//! `cargo run -p astromech-keybinds --example fixture > app/src/lib/fixtures/sample.json`

use astromech_keybinds::api::Api;
use astromech_keybinds::devices::{Controller, Inventory};

fn stick(id: &str, product: &str) -> Controller {
    Controller {
        vid: id[..4].into(),
        pid: id[4..].into(),
        elite_id: id.into(),
        product: product.into(),
        manufacturer: "VKB-Sim © Alex Oz 2021".into(),
        usage: 4,
        kind: "joystick".into(),
        buttons: 128,
        axes: vec!["X".into(), "Y".into(), "Z".into(), "RX".into(), "RY".into(), "RZ".into()],
        hats: 1,
        other_values: vec![],
    }
}

fn main() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let sample = root.join("data/samples/Example DualVKB.4.2.binds");
    let d = tempfile::tempdir().unwrap();
    let b = d.path().join("Bindings");
    std::fs::create_dir_all(&b).unwrap();
    let text = std::fs::read_to_string(&sample).unwrap();
    std::fs::write(b.join("Example DualVKB.4.2.binds"), &text).unwrap();
    std::fs::write(b.join("Example DualVKB.4.1.binds"), &text).unwrap();
    std::fs::write(
        b.join("Keyboard only.4.2.binds"),
        text.replace("PresetName=\"Example DualVKB\"", "PresetName=\"Keyboard only\"")
            .replace("231D3201", "{NoDevice}")
            .replace("231D0200", "{NoDevice}"),
    )
    .unwrap();
    std::fs::write(b.join("StartPreset.4.start"), "Example DualVKB\nExample DualVKB\nExample DualVKB\nKeyboard only")
        .unwrap();
    let api = Api {
        bindings_dir: b.clone(),
        builtin_dirs: vec![],
        inventory: Inventory {
            source: "fixture".into(),
            controllers: vec![
                stick("231D3201", "VKBsim Gladiator EVO OT  L SEM"),
                stick("231D0200", "VKBsim Gladiator EVO  R"),
            ],
            ..Default::default()
        },
        data_dir: d.path().join("data"),
    };
    let p = b.join("Example DualVKB.4.2.binds");
    // One edit so there is a backup to show.
    let e = astromech_keybinds::api::SlotEdit {
        path: p.clone(),
        action: "OrbitLinesToggle".into(),
        slot: "Secondary".into(),
        analogue: false,
        binding: None,
    };
    api.set_slot(&e).unwrap();
    let v = serde_json::json!({ "state": api.state(), "view": api.view(&p).unwrap() });
    let fake = r"C:\Users\you\AppData\Local\Frontier Developments\Elite Dangerous\Options\Bindings";
    let s = serde_json::to_string_pretty(&v).unwrap();
    let bs = b.display().to_string();
    let jfake = fake.replace('\\', "\\\\");
    let s = s
        .replace(
            &format!("{bs}/{}/", astromech_keybinds::edit::BACKUP_DIR),
            &format!("{jfake}\\\\{}\\\\", astromech_keybinds::edit::BACKUP_DIR),
        )
        .replace(&format!("{bs}/"), &format!("{jfake}\\\\"))
        .replace(&bs, &jfake);
    // Timestamps vary per run; pin them so the fixture is stable.
    let re_ts = |s: &str| -> String {
        let mut out = String::new();
        let mut rest = s;
        while let Some(i) = rest.find("\"20") {
            out.push_str(&rest[..i]);
            let tail = &rest[i..];
            let end = tail[1..].find('"').map(|j| j + 2).unwrap_or(tail.len());
            let lit = &tail[..end];
            if lit.len() >= 17 && lit.as_bytes()[5] == b'-' {
                out.push_str("\"2026-10-11 10:30\"");
            } else {
                out.push_str(lit);
            }
            rest = &tail[end..];
        }
        out + rest
    };
    let s = re_ts(&s);
    let s = regex_lite_backup_names(&s);
    println!("{s}");
}

/// Backup file names carry a timestamp: pin it.
fn regex_lite_backup_names(s: &str) -> String {
    let mut out = String::new();
    for line in s.lines() {
        if let Some(i) = line.find(".binds.20") {
            let end = line[i..].find(".bak").map(|j| i + j).unwrap_or(line.len());
            out.push_str(&line[..i]);
            out.push_str(".binds.20261011-103000-000");
            out.push_str(&line[end..]);
        } else {
            out.push_str(line);
        }
        out.push('\n');
    }
    out
}
