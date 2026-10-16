//! Test helpers shared by the schema slices: the live EDDN schema files, fixtures, and a game
//! context to build messages in.
//!
//! The schema files in `tests/fixtures/schemas/` are copied unchanged from EDCD/EDDN `schemas/`
//! on the `live` branch (commit `4ad669bb7bbe1eae080e4c354e786dca4db91f35`, read 16 Oct 2026;
//! BSD-3-Clause, see the `LICENSE` beside them). When EDDN changes a schema, copy the new file
//! over and update the commit here.

use crate::message::build;
use crate::{GameContext, SchemaSpec, Software};
use serde_json::{json, Value};
use std::path::PathBuf;

fn fixtures() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures")
}

/// A JSON file under `tests/fixtures/`, e.g. `"Market.json"`.
pub fn fixture(rel: &str) -> Value {
    let path = fixtures().join(rel);
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    serde_json::from_str(&text).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

/// Every way `upload` (the whole thing: `$schemaRef`, `header`, `message`) breaks the schema in
/// `tests/fixtures/schemas/<schema_file>`, e.g. `"commodity-v3.0.json"`. Formats such as
/// `date-time` are checked too.
pub fn schema_errors(schema_file: &str, upload: &Value) -> Vec<String> {
    let schema = fixture(&format!("schemas/{schema_file}"));
    let validator = jsonschema::draft4::options()
        .should_validate_formats(true)
        .build(&schema)
        .unwrap_or_else(|e| panic!("{schema_file}: {e}"));
    validator.iter_errors(upload).map(|e| format!("{}: {e}", e.instance_path())).collect()
}

pub fn assert_valid(schema_file: &str, upload: &Value) {
    let errors = schema_errors(schema_file, upload);
    assert!(errors.is_empty(), "{schema_file}:\n{}\n{upload:#}", errors.join("\n"));
}

/// The schema file for a spec: `<name>-v<version>.0.json`.
pub fn schema_file(spec: &SchemaSpec) -> String {
    format!("{}-v{}.0.json", spec.name, spec.version)
}

/// The upload the sender would queue for `message`, built with the shared rules.
pub fn upload(ctx: &GameContext, spec: &SchemaSpec, message: Value) -> Value {
    build(spec, ctx, &software(), false, message).unwrap_or_else(|d| panic!("{} message dropped: {d}", spec.name))
}

/// Every path in `paths` is absent from `obj` (dot-separated, `[]` for each array element, as in
/// [`crate::Elision`]).
pub fn assert_absent(obj: &Value, paths: &[&str]) {
    for p in paths {
        assert!(lookup(obj, p).is_empty(), "{p} should be elided: {obj:#}");
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

pub fn software() -> Software {
    Software { name: "astromech".into(), version: "0.1.5".into() }
}

pub const SHINRARTA: u64 = 3932277478106;
pub const SHINRARTA_POS: [f64; 3] = [55.71875, 17.59375, 27.15625];

/// A live Odyssey session in Shinrarta Dezhra.
pub fn live_context() -> GameContext {
    let mut c = GameContext::default();
    c.observe(&json!({"event":"Fileheader","part":1,"gameversion":"4.0.0.1904","build":"r308767/r0 "}));
    c.observe(&json!({"event":"LoadGame","Commander":"Jameson","FID":"F42","Horizons":true,"Odyssey":true}));
    c.observe(&json!({"event":"Location","StarSystem":"Shinrarta Dezhra","SystemAddress":3932277478106u64,
        "StarPos":[55.71875,17.59375,27.15625]}));
    c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schema_errors_reports_violations() {
        let up = json!({"$schemaRef":"x","header":{"uploaderID":"J","softwareName":"a","softwareVersion":"1"},
            "message":{"systemName":"Sol","stationName":"Abraham Lincoln","marketId":1,
                "timestamp":"not a time","ships":["sidewinder"]}});
        let errors = schema_errors("shipyard-v2.0.json", &up);
        assert!(errors.iter().any(|e| e.contains("/message/timestamp")), "{errors:?}");
    }
}
