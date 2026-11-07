//! The browser preview's fixtures (app/src/preview/fixtures/) are the real core's replies, dumped
//! from a headless core in a world, and this test fails when one drifts (decision astromech-1zhg:
//! fixtures are recorded inputs, never invented replies).
//!
//! [`fixtures`] lists every file in that folder. For each file, or each top-level key of a
//! composite one, it gives the window command and args the headless core answers it with, in a
//! world under `worlds/preview/` (synthetic journals and recorded network) at a mark of its
//! steps, or why the core doesn't produce it. The test calls each as the window does
//! (`origin: "app_window"`), masks the volatile fields (the file's own value is kept), and
//! compares the reply with the file. A hand edit to a dumped fixture fails it.
//!
//! Refresh the files after a deliberate change to the core or a world with
//! `ASTROMECH_UPDATE_FIXTURES=1 cargo test -p astromech-core --test preview_fixtures`, then check
//! the diff, and that `pnpm check` and `pnpm test` in app/ still pass.

mod support;

use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use support::{world, WorldCore};

/// Where a part of a fixture comes from.
enum From {
    /// The core's reply to `cmd` with `args` in the world `worlds/preview/<world>`, once its steps
    /// have run to the mark `at` (none: before any step), at `pick` in the reply (a JSON pointer;
    /// "" is all of it). A string `"$/<pointer>"` in `args` stands for the file's value there (the
    /// sample's own input, such as the plan it asks for). `keep`: pointers into the part whose
    /// value is volatile (a path in the world's temp folder, a label); the file's is kept.
    Core {
        world: &'static str,
        at: Option<&'static str>,
        cmd: &'static str,
        args: Value,
        pick: &'static str,
        keep: &'static [&'static str],
    },
    /// Not a reply but the sample's own input to one (`"$/…"` above), kept as written.
    Input(&'static str),
    /// Not produced by the core, and why.
    Not(&'static str),
}

/// A part of a fixture: `key` is a JSON pointer into the file ("" is the whole file).
struct Part {
    key: &'static str,
    from: From,
}

fn core(world: &'static str, at: Option<&'static str>, cmd: &'static str, args: Value) -> From {
    From::Core { world, at, cmd, args, pick: "", keep: &[] }
}

fn whole(from: From) -> Vec<Part> {
    vec![Part { key: "", from }]
}

/// ships.json's build plans and their stats.
const CASES: [(&str, &str); 6] = [
    ("/cases/0/plan", "/cases/0/stats"),
    ("/cases/1/plan", "/cases/1/stats"),
    ("/cases/2/plan", "/cases/2/stats"),
    ("/cases/3/plan", "/cases/3/stats"),
    ("/cases/4/plan", "/cases/4/stats"),
    ("/cases/5/plan", "/cases/5/stats"),
];

/// voice.json's phrases, by key, and what is said.
const VOICE: [(&str, &str); 6] = [
    ("/banana bread", "banana bread"),
    ("/engage supercruise", "engage supercruise"),
    ("/gear down", "gear down"),
    ("/how much fuel", "how much fuel"),
    ("/lights on", "lights on"),
    ("/youre down", "you're down"),
];

/// sites.json's kinds, by key.
const SITES: [(&str, &str); 5] = [
    ("/brain_trees", "brain_trees"),
    ("/crashed_ships", "crashed_ships"),
    ("/crystalline_shards", "crystalline_shards"),
    ("/geological", "geological"),
    ("/raw_farms", "raw_farms"),
];

/// Every fixture file, by name, and where each part of it comes from.
fn fixtures() -> BTreeMap<&'static str, Vec<Part>> {
    let mut t = BTreeMap::new();
    t.insert("logbook.json", whole(core("history", Some("imported"), "logbook", json!({"query": "", "cmdr": null}))));
    t.insert("session.json", whole(core("history", Some("imported"), "session_summary", Value::Null)));
    t.insert(
        "mining.json",
        whole(From::Core {
            world: "mining",
            at: Some("mined"),
            cmd: "status",
            args: Value::Null,
            pick: "/mining",
            keep: &[],
        }),
    );
    t.insert(
        "materials.json",
        whole(From::Core {
            world: "materials",
            at: None,
            cmd: "materials_inventory",
            args: Value::Null,
            pick: "",
            keep: &["/source"],
        }),
    );
    t.insert("unlocks.json", whole(core("unlocks", None, "engineering_unlocks", Value::Null)));
    t.insert(
        "engineering.json",
        vec![
            Part {
                key: "/fleet",
                from: From::Core {
                    world: "engineering",
                    at: None,
                    cmd: "engineering_fleet",
                    args: Value::Null,
                    pick: "",
                    keep: &["/source"],
                },
            },
            Part { key: "/plan_input", from: From::Input("the plan the sample's numbers are for") },
            Part { key: "/plan", from: core("engineering", None, "engineering_plan", json!({"plan": "$/plan_input"})) },
            Part { key: "/engineers", from: core("engineering", None, "engineering_engineers", Value::Null) },
            Part { key: "/trip", from: core("engineering", None, "engineering_trip", json!({"plan": "$/plan_input"})) },
        ],
    );
    // The Ships page: the synthetic combat Krait (its own world) and its stats for each sample
    // plan; the card stats for it and for the engineering world's ships.
    let mut ships = vec![
        Part {
            key: "/fleet",
            from: From::Core {
                world: "krait",
                at: None,
                cmd: "engineering_fleet",
                args: Value::Null,
                pick: "",
                keep: &["/source"],
            },
        },
        Part {
            key: "/headlines/1",
            from: From::Core {
                world: "krait",
                at: None,
                cmd: "engineering_headlines",
                args: Value::Null,
                pick: "/1",
                keep: &[],
            },
        },
        Part {
            key: "/headlines/3",
            from: From::Core {
                world: "engineering",
                at: None,
                cmd: "engineering_headlines",
                args: Value::Null,
                pick: "/3",
                keep: &[],
            },
        },
        Part {
            key: "/headlines/7",
            from: From::Core {
                world: "engineering",
                at: None,
                cmd: "engineering_headlines",
                args: Value::Null,
                pick: "/7",
                keep: &[],
            },
        },
    ];
    for (plan, stats) in CASES {
        ships.push(Part { key: plan, from: From::Input("a build plan the Ships page's sample has stats for") });
        ships.push(Part {
            key: stats,
            from: core("krait", None, "engineering_stats", json!({ "plan": format!("${plan}") })),
        });
    }
    t.insert("ships.json", ships);
    // The voice tester's dry runs, by phrase: Elite running, the gear up (flags: in the ship),
    // 24.5 t of a 32 t tank, Key_L on the gear's secondary and Shift+K on the lights.
    t.insert(
        "voice.json",
        VOICE
            .iter()
            .map(|(key, said)| Part {
                key,
                from: From::Core {
                    world: "voice",
                    at: Some("running"),
                    cmd: "voice_test",
                    args: json!(said),
                    pick: "/result",
                    keep: &[],
                },
            })
            .collect(),
    );
    // The keybinds pages: the example preset, a copy and a keyboard-only variant, a VKB pair
    // connected, and one edit already made (OrbitLinesToggle's secondary cleared by the core's
    // binds_set_slot, with its backup). The backup's and the presets' times are their files' (the
    // world's copies, made at its start).
    t.insert(
        "sample.json",
        vec![
            Part {
                key: "/state",
                from: From::Core {
                    world: "binds",
                    at: None,
                    cmd: "binds_state",
                    args: Value::Null,
                    pick: "",
                    keep: &["/backups/*/created", "/presets/*/files/*/modified"],
                },
            },
            Part { key: "/view", from: core("binds", None, "binds_view", json!({"path": "$/view/path"})) },
        ],
    );
    t.insert(
        "station.json",
        vec![
            Part { key: "/here", from: core("station", Some("seen"), "docked_here", Value::Null) },
            // A query every module of the two stations seen matches: the preview searches this
            // book itself.
            Part { key: "/book", from: core("station", Some("seen"), "outfitting_search", json!({"query": "a"})) },
        ],
    );
    // Lookups around Sol (the sol world: Jameson docked at Abraham Lincoln in an Asp), answered
    // from Spansh's recorded pages.
    t.insert("nearby.json", whole(core("sol", None, "galaxy_nearby", json!({"query": "famine"}))));
    t.insert(
        "nearby-hge.json",
        whole(From::Not(
            "no recorded Spansh answer to the Federation-faction (HGE) search is committed: the sample is the famine \
             page relabelled",
        )),
    );
    t.insert(
        "hotspots.json",
        whole(core(
            "sol",
            None,
            "mining_hotspots",
            json!({"want": {"ore": "painite", "ring_type": null, "reserve": null}}),
        )),
    );
    t.insert(
        "stations.json",
        whole(core("sol", None, "stations_nearby", json!({"want": {"kind": "service", "service": "black_market"}}))),
    );
    t.insert(
        "stations-module.json",
        whole(core(
            "sol",
            None,
            "stations_nearby",
            json!({"want": {"kind": "module", "name": "Frame Shift Drive", "class": 5, "rating": "A", "mount": null}}),
        )),
    );
    t.insert(
        "stations-ship.json",
        whole(core("sol", None, "stations_nearby", json!({"want": {"kind": "ship", "ship": "Krait MkII"}}))),
    );
    t.insert(
        "sites.json",
        SITES
            .iter()
            .map(|(key, kind)| Part { key, from: core("sol", None, "galaxy_sites", json!({ "kind": kind })) })
            .collect(),
    );
    // Trade around Sol: the sol world's commander has 10 MCr, 200 t of space and the mined
    // hold (Cargo.json), and Ardent Insight's and Spansh's answers are recorded.
    let within = |ly: u32| {
        json!({"pad": null, "pad_set": false, "exclude_carriers": false, "exclude_odyssey": false, "max_age_days": null,
               "max_arrival_ls": null, "max_distance_ly": ly, "min_volume": null})
    };
    let sol = |cmd, args| core("sol", None, cmd, args);
    let picked = |cmd, args, pick| From::Core { world: "sol", at: None, cmd, args, pick, keep: &[] };
    t.insert(
        "trade.json",
        vec![
            Part { key: "/sell", from: sol("trade_search", json!({"side": "sell", "commodities": ["painite"], "filters": within(40)})) },
            Part { key: "/buy", from: sol("trade_search", json!({"side": "buy", "commodities": ["tritium"], "filters": within(40)})) },
            Part { key: "/anywhere/sell", from: sol("trade_anywhere", json!({"side": "sell", "commodity": "painite"})) },
            // Stations only: the preview applies the filters to its sample, so it holds the
            // answer that still has offers with fleet carriers left out.
            Part {
                key: "/anywhere/buy",
                from: sol(
                    "trade_anywhere",
                    json!({"side": "buy", "commodity": "tritium", "filters": {"pad": null, "pad_set": false, "exclude_carriers": true,
                        "exclude_odyssey": false, "max_age_days": null, "max_arrival_ls": null, "max_distance_ly": null, "min_volume": null}}),
                ),
            },
            Part { key: "/cargo", from: picked("trade_cargo", json!({"filters": within(40)}), "/plan") },
            Part { key: "/hold", from: picked("trade_cargo", json!({"filters": within(40)}), "/hold") },
            Part {
                key: "/list",
                from: picked(
                    "colonisation_buy",
                    json!({"filters": within(20), "needs": [
                        {"symbol": "steel", "name": "Steel", "tonnes": 4912},
                        {"symbol": "cmmcomposite", "name": "CMM Composite", "tonnes": 3820},
                        {"symbol": "ceramiccomposites", "name": "Ceramic Composites", "tonnes": 512},
                        {"symbol": "insulatingmembrane", "name": "Insulating Membrane", "tonnes": 310}]}),
                    "/plan",
                ),
            },
            Part { key: "/route/defaults", from: sol("trade_route_defaults", Value::Null) },
            Part {
                key: "/route/single",
                from: picked(
                    "trade_route",
                    json!({"ask": {"kind": "single", "station": "Abraham Lincoln", "capacity": 200, "credits": 10_000_000, "max_hop_ly": 31.5}}),
                    "/route",
                ),
            },
            Part {
                key: "/route/loop",
                from: From::Not("no recorded Spansh answer to a loop search: spansh-trade-result.json is a single hop"),
            },
        ],
    );
    // The Route page: the ship, and routes plotted from Sol by Spansh's recorded answers.
    t.insert(
        "plot.json",
        vec![
            Part { key: "/ship", from: sol("route_plot_ship", Value::Null) },
            Part {
                key: "/exact",
                from: picked(
                    "route_plot",
                    json!({"ask": {"plotter": "exact", "to": "Maia", "efficiency": 60, "refuel_every_scoopable": true}}),
                    "/plotted",
                ),
            },
            Part {
                key: "/neutron",
                from: picked(
                    "route_plot",
                    json!({"ask": {"plotter": "neutron", "to": "Colonia", "efficiency": 60, "refuel_every_scoopable": false}}),
                    "/plotted",
                ),
            },
            Part {
                key: "/local",
                from: From::Not(
                    "a neutron route plotted on this PC needs the neutron star file installed, which a world doesn't \
                     seed: astromech-galaxy's dump-local-route example plots it over tests/fixtures/neutron-sample.bin",
                ),
            },
        ],
    );
    // Colony candidates around Greae Phoea EM-K d9-675, where the colonies world's commander is,
    // from Spansh's answers recorded there on 17 Oct 2026.
    t.insert(
        "colonies.json",
        ["any", "earth_like", "landable", "rings"]
            .into_iter()
            .zip(["/any", "/earth_like", "/landable", "/rings"])
            .map(|(interest, key)| Part {
                key,
                from: core("colonies", None, "colony_candidates", json!({ "interest": interest })),
            })
            .collect(),
    );
    // Powerplay: the powerplay world's commander, pledged to Li Yong-Rui, at Lembava (the
    // targets, from Spansh's recorded answers) and then at Piscium Sector EB-X b1-3. The week's
    // merits and its end are counted to the wall clock's week (astromech-powerplay
    // `Standing::view`), so the file's are kept.
    t.insert(
        "powerplay-targets.json",
        [("/acquire", "acquire"), ("/reinforce", "reinforce"), ("/undermine", "undermine")]
            .into_iter()
            .map(|(key, target)| Part {
                key,
                from: core(
                    "powerplay",
                    Some("lembava"),
                    "powerplay_targets",
                    json!({"target": target, "within_ly": 40.0}),
                ),
            })
            .collect(),
    );
    t.insert(
        "powerplay.json",
        whole(From::Core {
            world: "powerplay",
            at: Some("piscium"),
            cmd: "powerplay_overview",
            args: Value::Null,
            pick: "",
            keep: &["/standing/week_merits", "/standing/week_ends"],
        }),
    );
    // Two colonisation projects in Wolf 359, one built, from the colonisation world's synthetic
    // depot visits, with 90 t of Insulating Membrane aboard a 784 t Type-9.
    t.insert("colonisation.json", whole(core("colonisation", None, "colonisation_projects", Value::Null)));
    // In game in Synthetic Sector AB-C d1 (the exobiology world): two bodies with biological
    // signals, A 2 mapped with its genera, and a first Stratum Tectonicas sample with the
    // commander about 210 m from it.
    let bio = |key: &'static str, pick: &'static str| Part {
        key,
        from: From::Core {
            world: "exobiology",
            at: Some("sampling"),
            cmd: "exobiology",
            args: Value::Null,
            pick,
            keep: &[],
        },
    };
    t.insert(
        "exobiology.json",
        vec![
            bio("/ok", "/ok"),
            bio("/system", "/system"),
            bio("/bodies", "/bodies"),
            bio("/sampling", "/sampling"),
            Part {
                key: "/known",
                from: From::Not(
                    "other players' reports come from Spansh's known-biology search, which the session \
                     makes in the background on arrival; it is recorded only for Luhman 16 \
                     (spansh-bodies-biology.json), not for the sample's synthetic system, so the reports \
                     are hand-written for the sample's two bodies",
                ),
            },
        ],
    );
    // The explorer world's commander, in game, scans a whole unexplored system, Synuefe AB-C
    // d1-2 (the exploration crate's synthetic lines); the history counts the unsold data.
    t.insert(
        "exploration.json",
        vec![
            Part { key: "/system", from: core("explorer", Some("explored"), "exploration_system", Value::Null) },
            Part { key: "/unsold", from: core("explorer", Some("explored"), "exploration_unsold", Value::Null) },
        ],
    );
    // Docked at Test Gateway in LHS 2435 (the market world: the market crate's Market.json and
    // Cargo.json), its prices against Ardent Insight's recorded nearby ones.
    t.insert(
        "eddn-log.json",
        whole(From::Not(
            "the EDDN outbox stamps its entries and schedules its retries on the wall clock (astromech-eddn \
             worker.rs), off the world's clock, so a world can't settle its log; the sample shows each outcome",
        )),
    );
    t.insert("market-compare.json", whole(core("market", None, "market_compare", Value::Null)));
    t.insert(
        "odyssey.json",
        vec![
            Part {
                key: "/view",
                from: From::Core {
                    world: "odyssey",
                    at: None,
                    cmd: "odyssey_view",
                    args: Value::Null,
                    pick: "",
                    keep: &["/source"],
                },
            },
            Part { key: "/picks", from: From::Input("the picks the sample's shopping list is for") },
            Part { key: "/shopping", from: core("odyssey", None, "odyssey_shopping", json!({"picks": "$/picks"})) },
        ],
    );
    t
}

/// The folder of the fixtures.
fn folder() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../app/src/preview/fixtures")
}

/// `v` with every `"$/<pointer>"` string replaced by the file's value there.
fn fill(v: &Value, file: &Value) -> Value {
    match v {
        Value::String(s) if s.starts_with("$/") => {
            file.pointer(&s[1..]).unwrap_or_else(|| panic!("args name {s}, which the file doesn't have")).clone()
        }
        Value::Array(a) => Value::Array(a.iter().map(|v| fill(v, file)).collect()),
        Value::Object(o) => Value::Object(o.iter().map(|(k, v)| (k.clone(), fill(v, file))).collect()),
        v => v.clone(),
    }
}

/// The first few paths where `a` and `b` differ, as `path: a ≠ b`.
fn differences(a: &Value, b: &Value, at: String, out: &mut Vec<String>) {
    if out.len() >= 6 {
        return;
    }
    match (a, b) {
        (Value::Object(x), Value::Object(y)) => {
            for k in x.keys().chain(y.keys().filter(|k| !x.contains_key(*k))) {
                let none = Value::Null;
                let (p, q) = (x.get(k), y.get(k));
                if p.is_none() || q.is_none() {
                    let show = |v: Option<&Value>| v.map_or("(missing)".to_string(), short);
                    out.push(format!("{at}/{k}: {} ≠ {}", show(p), show(q)));
                } else {
                    differences(p.unwrap_or(&none), q.unwrap_or(&none), format!("{at}/{k}"), out);
                }
            }
        }
        (Value::Array(x), Value::Array(y)) if x.len() == y.len() => {
            for (i, (p, q)) in x.iter().zip(y).enumerate() {
                differences(p, q, format!("{at}/{i}"), out);
            }
        }
        _ if a != b => out.push(format!("{}: {} ≠ {}", if at.is_empty() { "/" } else { &at }, short(a), short(b))),
        _ => {}
    }
}

fn short(v: &Value) -> String {
    let s = v.to_string();
    if s.chars().count() > 80 {
        format!("{}…", s.chars().take(80).collect::<String>())
    } else {
        s
    }
}

/// The world's folders as a Windows PC names them, so a reply's paths read as the installed
/// app's do and don't change with the world's temp folder: `(folder under the temp root, path)`.
const PC_FOLDERS: [(&str, &str); 2] = [
    ("world/bindings", r"C:\Users\you\AppData\Local\Frontier Developments\Elite Dangerous\Options\Bindings"),
    ("world/game", r"C:\Users\you\Saved Games\Frontier Developments\Elite Dangerous"),
];

/// `v` with each path under the world's temp folder `root` named as [`PC_FOLDERS`] does.
fn on_a_pc(v: &mut Value, root: &str) {
    match v {
        Value::String(s) => {
            for (folder, pc) in PC_FOLDERS {
                let under = format!("{root}/{folder}");
                if let Some(rest) = s.strip_prefix(&under).filter(|r| r.is_empty() || r.starts_with('/')) {
                    *s = format!("{pc}{}", rest.replace('/', "\\"));
                }
            }
        }
        Value::Array(a) => a.iter_mut().for_each(|v| on_a_pc(v, root)),
        Value::Object(o) => o.values_mut().for_each(|v| on_a_pc(v, root)),
        _ => {}
    }
}

/// [`on_a_pc`] backwards: the sample's own input (a preset's path) as the world core names it.
fn in_the_world(v: &mut Value, root: &str) {
    match v {
        Value::String(s) => {
            for (folder, pc) in PC_FOLDERS {
                if let Some(rest) = s.strip_prefix(pc).filter(|r| r.is_empty() || r.starts_with('\\')) {
                    *s = format!("{root}/{folder}{}", rest.replace('\\', "/"));
                }
            }
        }
        Value::Array(a) => a.iter_mut().for_each(|v| in_the_world(v, root)),
        Value::Object(o) => o.values_mut().for_each(|v| in_the_world(v, root)),
        _ => {}
    }
}

/// Copy `had`'s value at `path` (pointer segments; `*` is every element or key) over `got`'s,
/// where both have one.
fn keep_from(had: &Value, got: &mut Value, path: &[&str]) {
    let Some((first, rest)) = path.split_first() else {
        *got = had.clone();
        return;
    };
    match (had, got) {
        (Value::Array(h), Value::Array(g)) if *first == "*" => {
            for (h, g) in h.iter().zip(g.iter_mut()) {
                keep_from(h, g, rest);
            }
        }
        (Value::Object(h), Value::Object(g)) if *first == "*" => {
            for (k, g) in g.iter_mut() {
                if let Some(h) = h.get(k) {
                    keep_from(h, g, rest);
                }
            }
        }
        (h, g) => {
            let (Some(h), Some(g)) = (child(h, first), child_mut(g, first)) else { return };
            keep_from(h, g, rest);
        }
    }
}

fn child<'a>(v: &'a Value, k: &str) -> Option<&'a Value> {
    match v {
        Value::Array(a) => a.get(k.parse::<usize>().ok()?),
        Value::Object(o) => o.get(k),
        _ => None,
    }
}

fn child_mut<'a>(v: &'a mut Value, k: &str) -> Option<&'a mut Value> {
    match v {
        Value::Array(a) => a.get_mut(k.parse::<usize>().ok()?),
        Value::Object(o) => o.get_mut(k),
        _ => None,
    }
}

/// Set `ptr` in `v` to `new` ("" replaces `v`).
fn put(v: &mut Value, ptr: &str, new: Value) {
    *v.pointer_mut(ptr).unwrap_or_else(|| panic!("no {ptr} to write")) = new;
}

/// Where the parts at `keys` (JSON pointers, "" for the whole file) don't cover `file` exactly
/// once: at every level a key descends into, each key or index the file has there needs a part
/// at or under it, and every part must name something the file has. A hand-added entry inside a
/// composite part (a fourth headline, a key beside the dumped ones) is caught.
fn uncovered(file: &Value, keys: &[&str]) -> Vec<String> {
    #[derive(Default)]
    struct Node {
        part: bool,
        under: BTreeMap<String, Node>,
    }
    let mut root = Node::default();
    for key in keys {
        let mut n = &mut root;
        for seg in key.split('/').skip(1) {
            n = n.under.entry(seg.to_string()).or_default();
        }
        n.part = true;
    }
    fn walk(n: &Node, v: Option<&Value>, at: &str, out: &mut Vec<String>) {
        let Some(v) = v else {
            out.push(format!("a part names {}, which the file doesn't have", if at.is_empty() { "/" } else { at }));
            return;
        };
        if n.part {
            if !n.under.is_empty() {
                out.push(format!("{} is a part and has parts under it", if at.is_empty() { "/" } else { at }));
            }
            return;
        }
        let have: Vec<String> = match v {
            Value::Object(o) => o.keys().cloned().collect(),
            Value::Array(a) => (0..a.len()).map(|i| i.to_string()).collect(),
            _ => vec![],
        };
        for k in have.iter().filter(|k| !n.under.contains_key(*k)) {
            out.push(format!("{at}/{k} has no part in the table"));
        }
        for (k, u) in &n.under {
            walk(u, child(v, k), &format!("{at}/{k}"), out);
        }
    }
    let mut out = vec![];
    walk(&root, Some(file), "", &mut out);
    out
}

/// The coverage check sees a hand-added entry at any depth a part descends into, and a part
/// naming something the file lacks.
#[test]
fn a_hand_added_entry_beside_the_parts_is_caught() {
    let ships = json!({"fleet": {}, "headlines": {"1": {}, "3": {}}, "cases": [{"plan": 1, "stats": 2}]});
    let parts = ["/fleet", "/headlines/1", "/headlines/3", "/cases/0/plan", "/cases/0/stats"];
    assert_eq!(uncovered(&ships, &parts), Vec::<String>::new());
    let mut added = ships.clone();
    added["headlines"]["9"] = json!({"dps": 1});
    added["cases"][0]["note"] = json!("by hand");
    added["cases"].as_array_mut().unwrap().push(json!({"plan": 2, "stats": 3}));
    assert_eq!(
        uncovered(&added, &parts),
        [
            "/cases/1 has no part in the table",
            "/cases/0/note has no part in the table",
            "/headlines/9 has no part in the table"
        ]
    );
    assert_eq!(
        uncovered(&json!({"fleet": {}}), &["/fleet", "/gone"]),
        ["a part names /gone, which the file doesn't have"]
    );
    assert_eq!(uncovered(&json!({"a": 1}), &[""]), Vec::<String>::new());
    assert_eq!(uncovered(&json!({"a": {"b": 1}}), &["/a", "/a/b"]), ["/a is a part and has parts under it"]);
}

#[test]
fn every_preview_fixture_is_the_cores_reply_or_says_why_not() {
    let update = std::env::var_os("ASTROMECH_UPDATE_FIXTURES").is_some_and(|v| v == "1");
    let table = fixtures();
    let dir = folder();
    let on_disk: BTreeSet<String> = std::fs::read_dir(&dir)
        .expect("read the fixtures folder")
        .map(|e| e.expect("an entry").file_name().to_string_lossy().into_owned())
        .filter(|n| n.ends_with(".json"))
        .collect();
    let listed: BTreeSet<String> = table.keys().map(|k| k.to_string()).collect();
    let mut problems = vec![];
    for n in on_disk.difference(&listed) {
        problems.push(format!("{n} has no entry in the table"));
    }
    for n in listed.difference(&on_disk) {
        problems.push(format!("the table lists {n}, which isn't in the folder"));
    }

    let mut files: BTreeMap<&str, Value> = BTreeMap::new();
    for name in table.keys().filter(|n| on_disk.contains(**n)) {
        let text = std::fs::read_to_string(dir.join(name)).unwrap_or_else(|e| panic!("read {name}: {e}"));
        files.insert(name, serde_json::from_str(&text).unwrap_or_else(|e| panic!("{name} isn't JSON: {e}")));
    }

    let table: BTreeMap<_, _> = table.into_iter().filter(|(n, _)| files.contains_key(n)).collect();
    // A part the core doesn't produce says why; a composite file's parts cover it ([`uncovered`]).
    for (name, parts) in &table {
        for part in parts {
            if let From::Not(why) | From::Input(why) = &part.from {
                if why.trim().is_empty() {
                    problems.push(format!("{name}{}: no reason why the core doesn't produce it", part.key));
                }
                eprintln!("{name}{}: kept as written: {why}", part.key);
            }
        }
        let keys: Vec<&str> = parts.iter().map(|p| p.key).collect();
        for gap in uncovered(&files[name], &keys) {
            problems.push(format!("{name}: {gap}"));
        }
    }

    // One world core per world, its steps run in the table's order.
    let mut cores: BTreeMap<&str, (WorldCore, Option<&str>, String)> = BTreeMap::new();
    let mut changed: BTreeSet<&str> = BTreeSet::new();
    for (name, parts) in &table {
        for part in parts {
            let From::Core { world: w, at, cmd, args, pick, keep } = &part.from else { continue };
            let (core, reached, root) = cores.entry(w).or_insert_with(|| {
                let core = WorldCore::start(&world(&format!("preview/{w}")));
                // The temp root: the config is `<root>/world/astromech.toml`.
                let config = core.call_window("status", None).expect("status")["config"]
                    .as_str()
                    .expect("a config path")
                    .to_string();
                let root = config.trim_end_matches("/world/astromech.toml").to_string();
                (core, None, root)
            });
            if at != reached {
                let to =
                    at.unwrap_or_else(|| panic!("{name}{}: a part before any step after one at {reached:?}", part.key));
                core.step(to).unwrap_or_else(|e| panic!("{w}: world_step to {to}: {e}"));
                *reached = Some(to);
            }
            let file = &files[name];
            let mut args = fill(args, file);
            in_the_world(&mut args, root);
            let args = if args.is_null() { None } else { Some(args) };
            let reply = core.call_window(cmd, args).unwrap_or_else(|e| panic!("{name}{}: {cmd}: {e}", part.key));
            let mut got = reply
                .pointer(pick)
                .unwrap_or_else(|| panic!("{name}{}: {cmd}'s reply has no {pick}", part.key))
                .clone();
            on_a_pc(&mut got, root);
            let had = file.pointer(part.key).cloned();
            if let Some(had) = &had {
                for k in keep.iter() {
                    keep_from(had, &mut got, &k.split('/').skip(1).collect::<Vec<_>>());
                }
            }
            if had.as_ref() == Some(&got) {
                continue;
            }
            if update {
                put(files.get_mut(name).expect("read above"), part.key, got);
                changed.insert(name);
            } else {
                let mut d = vec![];
                differences(had.as_ref().unwrap_or(&Value::Null), &got, String::new(), &mut d);
                problems.push(format!(
                    "{name}{} is not {w}'s {cmd} reply (fixture ≠ core):\n    {}",
                    part.key,
                    d.join("\n    ")
                ));
            }
        }
    }
    for (w, (core, _, _)) in cores {
        assert!(core.quit().success(), "the {w} world core didn't quit cleanly");
    }
    for name in changed {
        let text = serde_json::to_string_pretty(&files[name]).expect("serialise") + "\n";
        std::fs::write(dir.join(name), text).unwrap_or_else(|e| panic!("write {name}: {e}"));
        eprintln!("rewrote {name}");
    }
    assert!(
        problems.is_empty(),
        "{}\n\nRefresh with ASTROMECH_UPDATE_FIXTURES=1 cargo test -p astromech-core --test preview_fixtures after a deliberate change",
        problems.join("\n")
    );
}
