//! Validation against the live EDDN schema files in `tests/schemas/`, copied unchanged from
//! EDCD/EDDN `schemas/` (live branch, commit 4ad669bb7bbe1eae080e4c354e786dca4db91f35, read
//! 16 Oct 2026; BSD-3-Clause, see `tests/schemas/LICENSE-EDDN`).

#![allow(dead_code)]

use astromech_eddn::message::build;
use astromech_eddn::{GameContext, SchemaSpec, Software};
use jsonschema::{Draft, Validator};
use serde_json::{json, Value};
use std::path::Path;

/// The validator for `tests/schemas/<file>` (e.g. `journal-v1.0.json`), draft 4 with formats.
pub fn validator(file: &str) -> Validator {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/schemas").join(file);
    let schema: Value = serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
    jsonschema::options().with_draft(Draft::Draft4).should_validate_formats(true).build(&schema).unwrap()
}

/// The schema file for a spec: `<name>-v<version>.0.json`.
pub fn schema_file(spec: &SchemaSpec) -> String {
    format!("{}-v{}.0.json", spec.name, spec.version)
}

pub fn errors(file: &str, upload: &Value) -> Vec<String> {
    validator(file).iter_errors(upload).map(|e| format!("{e} at {}", e.instance_path())).collect()
}

pub fn assert_valid(spec: &SchemaSpec, upload: &Value) {
    let errs = errors(&schema_file(spec), upload);
    assert!(errs.is_empty(), "{} rejects the message: {errs:#?}\n{upload:#}", spec.name);
}

/// A live session: Fileheader, LoadGame (both flags) and a Location in Shinrarta Dezhra.
pub fn live() -> GameContext {
    let mut c = GameContext::default();
    c.observe(&json!({"timestamp":"2026-10-16T12:00:00Z","event":"Fileheader","part":1,"language":"English/UK","Odyssey":true,"gameversion":"4.2.1.0","build":"r321234/r0 "}));
    c.observe(&json!({"timestamp":"2026-10-16T12:00:05Z","event":"LoadGame","FID":"F0000000","Commander":"Jameson","Horizons":true,"Odyssey":true,"Ship":"Cobra_MkIII","Credits":1000,"Loan":0,"gameversion":"4.2.1.0","build":"r321234/r0 "}));
    c.observe(&shinrarta_location());
    c
}

pub const SHINRARTA: u64 = 3932277478106;

pub fn shinrarta_location() -> Value {
    json!({"timestamp":"2026-10-16T12:00:10Z","event":"Location","DistFromStarLS":0.0,"Docked":false,
        "StarSystem":"Shinrarta Dezhra","SystemAddress":SHINRARTA,"StarPos":[55.71875,17.59375,27.15625],
        "Body":"Shinrarta Dezhra A","BodyID":1,"BodyType":"Star"})
}

/// The upload the sender would queue, built with the shared rules.
pub fn upload(ctx: &GameContext, spec: &SchemaSpec, message: Value) -> Value {
    let sw = Software { name: "astromech".into(), version: "0.1.5".into() };
    build(spec, ctx, &sw, false, message).unwrap_or_else(|d| panic!("{} message dropped: {d}", spec.name))
}

/// Every key in `keys` is absent from `obj` (dot paths, `[]` for each array element).
pub fn assert_absent(obj: &Value, keys: &[&str]) {
    for k in keys {
        assert!(lookup(obj, k).is_empty(), "{k} should be elided: {obj:#}");
    }
}

fn lookup<'a>(v: &'a Value, path: &str) -> Vec<&'a Value> {
    let (head, rest) = match path.split_once('.') {
        Some((h, r)) => (h, Some(r)),
        None => (path, None),
    };
    let (key, each) = head.strip_suffix("[]").map_or((head, false), |k| (k, true));
    let Some(child) = v.get(key) else { return Vec::new() };
    let children: Vec<&Value> =
        if each { child.as_array().map(|a| a.iter().collect()).unwrap_or_default() } else { vec![child] };
    match rest {
        None => children,
        Some(r) => children.into_iter().flat_map(|c| lookup(c, r)).collect(),
    }
}
