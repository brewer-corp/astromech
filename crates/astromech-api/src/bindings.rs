//! The registry's TypeScript types for the window (astromech-se44): each command's args, reply
//! and progress types, exported with ts-rs to `app/src/lib/api/types/`, and
//! `app/src/lib/api/commands.ts`, which maps every command name to
//! `{ args, reply, exposure, progress? }` in those types.
//!
//! [`render`] builds the files from a [`Registry`] whose commands all have types;
//! [`check_or_write`] compares them with the checked-in files, or writes them when
//! `ASTROMECH_UPDATE_BINDINGS` is set. The core's test runs it over the whole registry
//! (CONTRIBUTING.md names the command); this crate's tests run it over the commands it registers.
//!
//! Large integers are `number` (JSON numbers are read as `number` in the window), and a
//! `serde_json::Value` is ts-rs's `JsonValue`; [`json_values`] lists where one is reached.

use crate::{Exposure, Registry, Ty};
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::path::{Path, PathBuf};
use ts_rs::{Config, TypeVisitor, TS};

/// The env var that makes [`check_or_write`] write the files instead of comparing them.
pub const UPDATE: &str = "ASTROMECH_UPDATE_BINDINGS";

/// Where the files go below `app/src/lib/api/`: ts-rs's output.
pub const TYPES_DIR: &str = "types";

/// The command map, below `app/src/lib/api/`.
pub const COMMANDS_FILE: &str = "commands.ts";

/// The settings every file is made with.
pub fn config() -> Config {
    Config::new().with_large_int("number")
}

/// A walk over a type and everything it uses, collecting the files ts-rs writes for them.
pub struct Walk<'a> {
    cfg: &'a Config,
    seen: HashSet<std::any::TypeId>,
    /// Each exportable type's file (relative to [`TYPES_DIR`]): the Rust type and the contents.
    files: BTreeMap<PathBuf, (String, String)>,
    /// Two Rust types that would be written to one file.
    clashes: BTreeSet<(PathBuf, String, String)>,
    json: bool,
}

impl<'a> Walk<'a> {
    fn of(cfg: &'a Config, ty: Ty) -> Self {
        let mut w = Self { cfg, seen: HashSet::new(), files: BTreeMap::new(), clashes: BTreeSet::new(), json: false };
        (ty.walk)(&mut w);
        w
    }
}

impl TypeVisitor for Walk<'_> {
    fn visit<T: TS + 'static + ?Sized>(&mut self) {
        if !self.seen.insert(std::any::TypeId::of::<T>()) {
            return;
        }
        let path = T::output_path();
        if path.as_deref().is_some_and(|p| p.starts_with("serde_json")) {
            self.json = true;
        }
        if let Some(p) = path {
            let rust = rust_name::<T>();
            let text = T::export_to_string(self.cfg).unwrap_or_else(|e| format!("// ts-rs failed: {e}\n"));
            // ts-rs writes serde_json's Value as its own JsonValue: two Rust types, one text.
            if let Some((other, _)) = self.files.get(&p).filter(|(r, t)| *r != rust && *t != text) {
                self.clashes.insert((p.clone(), other.clone(), rust.clone()));
            }
            self.files.insert(p, (rust, text));
        }
        T::visit_dependencies(self);
        T::visit_generics(self);
    }
}

/// `T`'s Rust path without its generic arguments.
fn rust_name<T: ?Sized>() -> String {
    let n = std::any::type_name::<T>();
    n.split('<').next().unwrap_or(n).to_string()
}

/// Where a `serde_json::Value` (`JsonValue`) is reached: `(command, "args" | "reply" |
/// "progress")`, for the allow-list in the core's test.
pub fn json_values(reg: &Registry) -> Vec<(String, &'static str)> {
    let cfg = config();
    let mut out = vec![];
    for (name, _, types) in reg.commands() {
        let Some(t) = types else { continue };
        for (part, ty) in [("args", Some(t.args)), ("reply", Some(t.reply)), ("progress", t.progress)] {
            if ty.is_some_and(|ty| Walk::of(&cfg, ty).json) {
                out.push((name.to_string(), part));
            }
        }
    }
    out
}

/// The files for `reg`'s commands, by path below `app/src/lib/api/`: one per exported type in
/// [`TYPES_DIR`], and [`COMMANDS_FILE`]. An error names each command registered without types
/// and each type name used by two different types.
pub fn render(reg: &Registry) -> Result<BTreeMap<PathBuf, String>, String> {
    let cfg = config();
    let mut files: BTreeMap<PathBuf, (String, String)> = BTreeMap::new();
    let mut errors = vec![];
    let mut lines = vec![];
    for (name, exposure, types) in reg.commands() {
        let Some(t) = types else {
            errors.push(format!("{name} is registered without types (Registry::add); use add_typed or add_request"));
            continue;
        };
        let parts = [("args", Some(t.args)), ("reply", Some(t.reply)), ("progress", t.progress)];
        let mut fields = vec![];
        for (part, ty) in parts {
            let Some(ty) = ty else { continue };
            let deep = Walk::of(&cfg, ty);
            let mut clashes = deep.clashes.clone();
            for (p, (rust, text)) in deep.files {
                if let Some((other, _)) = files.get(&p).filter(|(r, t)| *r != rust && *t != text) {
                    clashes.insert((p.clone(), other.clone(), rust.clone()));
                }
                files.insert(p, (rust, text));
            }
            for (p, a, b) in clashes {
                errors.push(format!("{} would be both {a} and {b}; rename one with #[ts(rename)]", p.display()));
            }
            fields.push(format!("{part}: {}", (ty.name)(&cfg)));
        }
        let exposure = match exposure {
            Exposure::Window => "window",
            Exposure::Local => "local",
        };
        fields.insert(2, format!("exposure: \"{exposure}\""));
        lines.push(format!("  {name}: {{ {} }};", fields.join("; ")));
    }
    if !errors.is_empty() {
        errors.sort();
        errors.dedup();
        return Err(errors.join("\n"));
    }
    let mut out: BTreeMap<PathBuf, String> =
        files.into_iter().map(|(p, (_, t))| (Path::new(TYPES_DIR).join(p), t)).collect();
    let mut ts = String::from(
        "// Generated from the command registry by astromech-api's bindings (astromech-se44). Do not edit:\n\
         // `ASTROMECH_UPDATE_BINDINGS=1 cargo test -p astromech-core --lib bindings` writes it (CONTRIBUTING.md).\n",
    );
    // Import each exported type the lines name: a file's name is its type's.
    let named: BTreeSet<&str> =
        lines.iter().flat_map(|l| l.split(|c: char| !(c.is_alphanumeric() || c == '_'))).collect();
    for p in out.keys() {
        let Some(ident) = p.file_stem().and_then(|s| s.to_str()).filter(|i| named.contains(i)) else { continue };
        let module = p.with_extension("").display().to_string().replace('\\', "/");
        ts += &format!("import type {{ {ident} }} from \"./{module}\";\n");
    }
    ts += "\n/** Every command: what the window sends as `args` (a `string` is the request's text), the\n \
           * reply's `value`, who may call it, and the progress values it sends before the reply. */\n";
    ts += "export type Commands = {\n";
    for l in lines {
        ts += &l;
        ts += "\n";
    }
    ts += "};\n";
    out.insert(PathBuf::from(COMMANDS_FILE), ts);
    Ok(out)
}

/// The checked-in files below `dir` (`app/src/lib/api/`) that [`render`] owns: everything in
/// [`TYPES_DIR`] and [`COMMANDS_FILE`].
fn checked_in(dir: &Path) -> BTreeMap<PathBuf, String> {
    fn walk(base: &Path, d: &Path, out: &mut BTreeMap<PathBuf, String>) {
        let Ok(rd) = std::fs::read_dir(d) else { return };
        for e in rd.flatten() {
            let p = e.path();
            if p.is_dir() {
                walk(base, &p, out);
            } else if let Ok(t) = std::fs::read_to_string(&p) {
                out.insert(p.strip_prefix(base).unwrap_or(&p).to_path_buf(), t);
            }
        }
    }
    let mut out = BTreeMap::new();
    walk(dir, &dir.join(TYPES_DIR), &mut out);
    if let Ok(t) = std::fs::read_to_string(dir.join(COMMANDS_FILE)) {
        out.insert(PathBuf::from(COMMANDS_FILE), t);
    }
    out
}

/// Compare `reg`'s files with the ones checked in below `dir` (`app/src/lib/api/`), or, with
/// [`UPDATE`] set, write them there and delete the stale ones. The error lists every file that
/// is missing, different or stale, and how to update them.
pub fn check_or_write(reg: &Registry, dir: &Path) -> Result<(), String> {
    let want = render(reg)?;
    let have = checked_in(dir);
    if std::env::var_os(UPDATE).is_some() {
        for p in have.keys().filter(|p| !want.contains_key(*p)) {
            std::fs::remove_file(dir.join(p)).map_err(|e| format!("removing {}: {e}", p.display()))?;
        }
        for (p, text) in &want {
            let f = dir.join(p);
            if let Some(parent) = f.parent() {
                std::fs::create_dir_all(parent).map_err(|e| format!("creating {}: {e}", parent.display()))?;
            }
            if have.get(p) != Some(text) {
                std::fs::write(&f, text).map_err(|e| format!("writing {}: {e}", f.display()))?;
            }
        }
        return Ok(());
    }
    let mut diffs = vec![];
    for (p, text) in &want {
        match have.get(p) {
            None => diffs.push(format!("missing: {}", p.display())),
            Some(t) if t != text => diffs.push(format!("differs: {}", p.display())),
            Some(_) => {}
        }
    }
    for p in have.keys().filter(|p| !want.contains_key(*p)) {
        diffs.push(format!("stale: {}", p.display()));
    }
    if diffs.is_empty() {
        return Ok(());
    }
    Err(format!(
        "the TypeScript types in {} are not what the registry makes:\n  {}\nupdate them with \
         `ASTROMECH_UPDATE_BINDINGS=1 cargo test -p astromech-core --lib bindings` (CONTRIBUTING.md)",
        dir.display(),
        diffs.join("\n  ")
    ))
}

/// This crate's part of the check: every file [`render`] makes for `reg`'s commands (a part of
/// the whole registry) is checked in below `dir` as it would write it, and `commands.ts` has
/// each command's line. The core's test checks the whole set, stale files included.
pub fn check_part(reg: &Registry, dir: &Path) -> Result<(), String> {
    let want = render(reg)?;
    let have = checked_in(dir);
    let mut diffs = vec![];
    for (p, text) in &want {
        if p == Path::new(COMMANDS_FILE) {
            let all = have.get(p).map(String::as_str).unwrap_or_default();
            let start = text.find("export type Commands = {\n").map_or(0, |i| i + 25);
            for line in text[start..].lines().filter(|l| l.starts_with("  ")) {
                if !all.lines().any(|l| l == line) {
                    diffs.push(format!("{COMMANDS_FILE} lacks or differs on: {}", line.trim()));
                }
            }
            continue;
        }
        match have.get(p) {
            None => diffs.push(format!("missing: {}", p.display())),
            Some(t) if t != text => diffs.push(format!("differs: {}", p.display())),
            Some(_) => {}
        }
    }
    if diffs.is_empty() {
        return Ok(());
    }
    Err(format!(
        "the TypeScript types in {} are not what the registry makes:\n  {}\nupdate them with \
         `ASTROMECH_UPDATE_BINDINGS=1 cargo test -p astromech-core --lib bindings` (CONTRIBUTING.md)",
        dir.display(),
        diffs.join("\n  ")
    ))
}

#[cfg(test)]
#[allow(clippy::disallowed_methods)] // tests are exempt for now (astromech-c7lc)
mod tests {
    use super::*;
    use crate::{Folders, State};
    use std::sync::Arc;

    /// The window's `app/src/lib/api/`.
    pub(crate) fn api_dir() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../app/src/lib/api")
    }

    /// This crate's commands (the core registers its own beside them).
    fn registry() -> Registry {
        let d = std::env::temp_dir();
        let state = Arc::new(State::new(d.join("unused"), crate::state::tests::offline(), None, Folders::default));
        let mut reg = Registry::new();
        crate::commands::register(&state, &mut reg);
        crate::lookups::register(&state, &mut reg);
        reg
    }

    /// The checked-in files have what this crate's commands make; the core's test checks the
    /// whole registry's set (`cargo test -p astromech-core --lib bindings`).
    #[test]
    fn the_checked_in_types_are_what_this_crates_commands_make() {
        let reg = registry();
        if let Err(e) = check_part(&reg, &api_dir()) {
            panic!("{e}");
        }
    }
}
