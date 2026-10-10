//! Console, HTML and JSON output, with Windows-username redaction.

use crate::doctor::{Confidence, Report};

pub struct Redactor {
    pairs: Vec<(String, String)>,
    user: Option<String>,
}

impl Redactor {
    pub fn from_env() -> Self {
        let mut pairs = vec![];
        for (var, label) in
            [("LOCALAPPDATA", "%LOCALAPPDATA%"), ("APPDATA", "%APPDATA%"), ("USERPROFILE", "%USERPROFILE%")]
        {
            if let Ok(v) = std::env::var(var) {
                if v.len() > 3 {
                    pairs.push((v, label.to_string()));
                }
            }
        }
        let user = std::env::var("USERNAME").ok().filter(|u| u.len() >= 2);
        Self::new(pairs, user)
    }
    pub fn new(pairs: Vec<(String, String)>, user: Option<String>) -> Self {
        Self { pairs, user }
    }
    pub fn apply(&self, s: &str) -> String {
        let mut out = s.to_string();
        for (from, to) in &self.pairs {
            out = replace_ci(&out, from, to);
        }
        if let Some(u) = &self.user {
            for (pre, rep) in [
                ("\\users\\", "\\Users\\<user>"),
                ("/users/", "/Users/<user>"),
                ("/home/", "/home/<user>"),
                ("\\\\users\\\\", "\\\\Users\\\\<user>"),
            ] {
                out = replace_ci(&out, &format!("{pre}{u}"), rep);
            }
        }
        out
    }
    pub fn apply_json(&self, v: &mut serde_json::Value) {
        match v {
            serde_json::Value::String(s) => *s = self.apply(s),
            serde_json::Value::Array(a) => a.iter_mut().for_each(|x| self.apply_json(x)),
            serde_json::Value::Object(o) => o.values_mut().for_each(|x| self.apply_json(x)),
            _ => {}
        }
    }
}

fn replace_ci(hay: &str, needle: &str, rep: &str) -> String {
    if needle.is_empty() {
        return hay.to_string();
    }
    let lh = hay.to_lowercase();
    let ln = needle.to_lowercase();
    // Only safe when lowercasing preserves byte offsets (ASCII-ish paths); otherwise exact match.
    if lh.len() != hay.len() || ln.len() != needle.len() {
        return hay.replace(needle, rep);
    }
    let mut out = String::with_capacity(hay.len());
    let mut i = 0;
    while let Some(p) = lh[i..].find(&ln) {
        out.push_str(&hay[i..i + p]);
        out.push_str(rep);
        i += p + ln.len();
    }
    out.push_str(&hay[i..]);
    out
}

fn esc(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

fn conf_label(c: &Confidence) -> &'static str {
    match c {
        Confidence::High => "HIGH",
        Confidence::Medium => "MED",
        Confidence::Low => "LOW",
    }
}

pub fn console(r: &Report) -> String {
    let mut o = String::new();
    o += &format!("{}  ({})\n", r.tool, r.generated_at);
    o += &format!("Bindings folder: {}{}\n", r.bindings_dir, if r.bindings_dir_found { "" } else { "  [NOT FOUND]" });
    if r.installs.is_empty() {
        o += "Game install: not found (DeviceMappings.xml unavailable; use --game-dir)\n";
    }
    for i in &r.installs {
        o += &format!(
            "Game install ({}): {}  [{} device mappings, {} built-in presets]\n",
            i.source,
            i.product_dir,
            i.device_mapping_count,
            i.builtin_presets.len()
        );
    }
    o += &format!(
        "\nDevices ({}): {} controller(s), {} keyboard(s), {} mouse/mice, XInput slots {:?}\n",
        r.devices.source,
        r.devices.controllers.len(),
        r.devices.keyboards.len(),
        r.devices.mice.len(),
        r.devices.xinput_slots
    );
    for c in &r.devices.controllers {
        o += &format!(
            "  {}  {:<40} {:<22} {:>3} buttons, axes {}, {} hats\n",
            c.elite_id,
            trunc(&c.product, 40),
            c.kind,
            c.buttons,
            c.axes.join("/"),
            c.hats
        );
    }
    for f in &r.device_flags {
        o += &format!("  ! {f}\n");
    }
    o += "\nActive presets:\n";
    if r.start_presets.is_empty() {
        o += "  (no StartPreset file)\n";
    }
    for s in &r.start_presets {
        o += &format!(
            "  {}{}\n",
            s.file_name,
            if s.probably_effective { "  (probably the one the game uses)" } else { "" }
        );
        for l in &s.lines {
            o += &format!("    {:<10} {:<28} -> {}\n", l.context, l.preset, l.resolves_to);
        }
    }
    o += &format!(
        "\nCurrent version: {} ({})\n",
        r.current_version.clone().unwrap_or("?".into()),
        r.current_version_basis
    );
    o += &format!("\nBinds files ({}):\n", r.files.len());
    for f in &r.files {
        o += &format!(
            "  {:<40} \"{}\"\n      {}\n",
            trunc(&f.rel_path, 40),
            f.preset_name.clone().unwrap_or("-".into()),
            f.verdict
        );
        for x in f.reasons.iter().filter(|x| x.confidence != Confidence::Low).take(4) {
            o += &format!("      [{}] {}\n", conf_label(&x.confidence), x.title);
        }
        for d in &f.devices {
            for p in d.problems.iter().take(3) {
                o += &format!("      ! {}: {}\n", d.name, p);
            }
            if d.problems.len() > 3 {
                o += &format!("      ! {}: ...and {} more (see HTML report)\n", d.name, d.problems.len() - 3);
            }
        }
    }
    for n in &r.near_misses {
        o += &format!("  ! {}: {}\n", n.path, n.why);
    }
    if let Some(b) = &r.binding_loading_errors {
        o += &format!("\nBindingLoadingErrors.log ({}):\n", b.modified);
        for l in b.lines.iter().take(15) {
            o += &format!("  | {l}\n");
        }
    }
    for l in &r.client_logs {
        o += &format!("\n{} ({} matching lines{}):\n", l.file, l.lines.len(), if l.truncated { "+" } else { "" });
        for x in l.lines.iter().take(8) {
            o += &format!("  | {x}\n");
        }
    }
    for n in &r.notes {
        o += &format!("\nNote: {n}");
    }
    o += "\n";
    o
}

fn trunc(s: &str, n: usize) -> String {
    if s.chars().count() <= n {
        s.to_string()
    } else {
        format!("{}…", s.chars().take(n - 1).collect::<String>())
    }
}

pub fn html(r: &Report) -> String {
    let mut h = String::new();
    h += "<!doctype html><html><head><meta charset=\"utf-8\"><title>Binds doctor report</title><style>\
body{font-family:Segoe UI,Arial,sans-serif;background:#111;color:#ddd;margin:24px;max-width:1200px}\
h1,h2{color:#ff8c1a}h3{color:#ffb366;margin-bottom:4px}table{border-collapse:collapse;margin:8px 0;width:100%}\
td,th{border:1px solid #333;padding:4px 8px;text-align:left;vertical-align:top;font-size:14px}th{background:#222}\
.card{border:1px solid #444;border-radius:6px;padding:10px 14px;margin:12px 0;background:#181818}\
.high{color:#ff5c5c;font-weight:bold}.medium{color:#ffc94d;font-weight:bold}.low{color:#9ab}\
.ok{color:#7ddc7d}.missing{color:#ff5c5c}.unknown{color:#ffc94d}code,pre{background:#000;color:#cfc;padding:2px 4px}\
pre{padding:8px;overflow-x:auto;white-space:pre-wrap}.muted{color:#888}</style></head><body>";
    h += &format!(
        "<h1>Elite Dangerous binds doctor</h1><p class=muted>{} · {}</p><p>{}</p>",
        esc(&r.tool),
        esc(&r.generated_at),
        esc(&r.disclaimer)
    );
    h += &format!(
        "<h2>Locations</h2><table><tr><th>Bindings folder</th><td>{}{}</td></tr>",
        esc(&r.bindings_dir),
        if r.bindings_dir_found { "" } else { " <span class=missing>(not found)</span>" }
    );
    for i in &r.installs {
        h += &format!("<tr><th>Game install ({})</th><td>{}<br><span class=muted>DeviceMappings: {} ({} names) · built-in presets: {}</span></td></tr>", esc(&i.source), esc(&i.product_dir), esc(i.device_mappings.as_deref().unwrap_or("not found")), i.device_mapping_count, esc(&i.builtin_presets.join(", ")));
    }
    if r.installs.is_empty() {
        h += "<tr><th>Game install</th><td class=unknown>not found; use --game-dir</td></tr>";
    }
    h += "</table>";
    h += &format!("<h2>Connected devices</h2><p class=muted>Source: {}</p><table><tr><th>Elite id</th><th>Product</th><th>Manufacturer</th><th>Type</th><th>Buttons</th><th>Axes</th><th>Hats</th></tr>", esc(&r.devices.source));
    for c in &r.devices.controllers {
        h += &format!(
            "<tr><td><code>{}</code></td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}{}</td><td>{}</td></tr>",
            c.elite_id,
            esc(&c.product),
            esc(&c.manufacturer),
            esc(&c.kind),
            c.buttons,
            esc(&c.axes.join(", ")),
            if c.other_values.is_empty() {
                String::new()
            } else {
                format!(" <span class=muted>(+{})</span>", esc(&c.other_values.join(", ")))
            },
            c.hats
        );
    }
    h += "</table>";
    h += &format!(
        "<p>Keyboards: {} · Mice: {} · XInput pads in slots: {:?}</p>",
        r.devices.keyboards.len(),
        r.devices.mice.len(),
        r.devices.xinput_slots
    );
    for f in &r.device_flags {
        h += &format!("<p class=medium>⚠ {}</p>", esc(f));
    }
    for n in &r.devices.notes {
        h += &format!("<p class=muted>{}</p>", esc(n));
    }
    h += "<h2>Active presets (StartPreset)</h2>";
    if r.start_presets.is_empty() {
        h += "<p class=muted>No StartPreset*.start file.</p>";
    }
    for s in &r.start_presets {
        h += &format!(
            "<h3>{}{}</h3><table><tr><th>Context (likely)</th><th>Preset</th><th>Resolves to</th></tr>",
            esc(&s.file_name),
            if s.probably_effective {
                " <span class=ok>(probably the one the game uses)</span>"
            } else {
                " <span class=muted>(older/lower-numbered; probably ignored)</span>"
            }
        );
        for l in &s.lines {
            let cls = if l.resolves_to.starts_with("NOTHING") {
                "missing"
            } else if l.resolves_to.starts_with("only") {
                "unknown"
            } else {
                "ok"
            };
            h += &format!(
                "<tr><td>{}</td><td>{}</td><td class={cls}>{}</td></tr>",
                esc(&l.context),
                esc(&l.preset),
                esc(&l.resolves_to)
            );
        }
        h += "</table>";
    }
    h += &format!(
        "<p>Current binds version: <b>{}</b> <span class=muted>({})</span></p>",
        esc(r.current_version.as_deref().unwrap_or("unknown")),
        esc(&r.current_version_basis)
    );
    h += &format!("<h2>Binds files ({})</h2>", r.files.len());
    for f in &r.files {
        h += "<div class=card>";
        h += &format!("<h3>{}</h3><p><b>{}</b></p>", esc(&f.rel_path), esc(&f.verdict));
        h += &format!(
            "<table><tr><th>PresetName</th><td>{}</td><th>Root version</th><td>{}.{}</td><th>File suffix</th><td>{}</td></tr><tr><th>Size</th><td>{} bytes</td><th>Modified</th><td>{}</td><th>Encoding</th><td>{}</td></tr><tr><th>Active in</th><td colspan=5>{}</td></tr></table>",
            esc(f.preset_name.as_deref().unwrap_or("—")),
            esc(f.major_version.as_deref().unwrap_or("?")),
            esc(f.minor_version.as_deref().unwrap_or("?")),
            esc(f.file_suffix.as_deref().unwrap_or("none")),
            f.size,
            esc(&f.modified),
            esc(&format!("{}{}", f.encoding.bom.clone().unwrap_or("UTF-8 (no BOM)".into()), f.encoding.declared.as_ref().map(|d| format!(", declares {d}")).unwrap_or_default())),
            esc(&if f.active_in.is_empty() { "not active".to_string() } else { f.active_in.join(", ") })
        );
        if !f.reasons.is_empty() {
            h += "<p><b>Likely reasons it won't show up (ranked):</b></p><ol>";
            for x in &f.reasons {
                let c = match x.confidence {
                    Confidence::High => "high",
                    Confidence::Medium => "medium",
                    Confidence::Low => "low",
                };
                h += &format!(
                    "<li><span class={c}>[{}]</span> <b>{}</b><br>{}</li>",
                    conf_label(&x.confidence),
                    esc(&x.title),
                    esc(&x.explanation)
                );
            }
            h += "</ol>";
        }
        if !f.devices.is_empty() {
            h += "<table><tr><th>Device in file</th><th>Binds</th><th>Referenced as</th><th>Status</th><th>Matched / problems</th></tr>";
            for d in &f.devices {
                let cls = match d.status.as_str() {
                    "missing" => "missing",
                    "unknown" => "unknown",
                    _ => "ok",
                };
                let mut info = d.matched.clone().map(|m| esc(&m)).unwrap_or_default();
                for p in &d.problems {
                    info += &format!("<br><span class=medium>⚠</span> {}", esc(p));
                }
                h += &format!(
                    "<tr><td><code>{}</code></td><td>{}</td><td>{}</td><td class={cls}>{}</td><td>{info}</td></tr>",
                    esc(&d.name),
                    d.binds,
                    esc(&d.referenced_as),
                    esc(&d.status)
                );
            }
            h += "</table>";
        }
        for fl in &f.flags {
            h += &format!("<p class=muted>• {}</p>", esc(fl));
        }
        h += "</div>";
    }
    if !r.near_misses.is_empty() {
        h += "<h2>Files that look like bindings but won't be read</h2><ul>";
        for n in &r.near_misses {
            h += &format!("<li><code>{}</code>: {}</li>", esc(&n.path), esc(&n.why));
        }
        h += "</ul>";
    }
    h += &format!("<p class=muted>Game-made .backup files in the folder: {}</p>", r.game_backup_files);
    h += "<h2>Game logs</h2>";
    match &r.binding_loading_errors {
        Some(b) => {
            h += &format!(
                "<h3>BindingLoadingErrors.log <span class=muted>({})</span></h3><pre>{}</pre>",
                esc(&b.modified),
                esc(&b.lines.join("\n"))
            )
        }
        None => h += "<p class=muted>No BindingLoadingErrors.log in the Bindings folder.</p>",
    }
    for l in &r.client_logs {
        h += &format!(
            "<h3>{} <span class=muted>({}; binding/device-related lines only{})</span></h3><pre>{}</pre>",
            esc(&l.file),
            esc(&l.modified),
            if l.truncated { ", truncated" } else { "" },
            esc(&l.lines.join("\n"))
        );
    }
    if r.client_logs.is_empty() {
        h += "<p class=muted>No matching lines in game client logs (or no logs found).</p>";
    }
    if !r.notes.is_empty() {
        h += "<h2>Notes</h2><ul>";
        for n in &r.notes {
            h += &format!("<li>{}</li>", esc(n));
        }
        h += "</ul>";
    }
    h += "</body></html>";
    h
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn redaction() {
        let r = Redactor::new(
            vec![("C:\\Users\\Sumeet\\AppData\\Local".into(), "%LOCALAPPDATA%".into())],
            Some("Sumeet".into()),
        );
        assert_eq!(r.apply("c:\\users\\sumeet\\AppData\\Local\\Frontier"), "%LOCALAPPDATA%\\Frontier");
        assert_eq!(r.apply("D:\\x C:\\Users\\SUMEET\\Desktop"), "D:\\x C:\\Users\\<user>\\Desktop");
        assert_eq!(r.apply("Sumeet's stick"), "Sumeet's stick");
        let mut v = serde_json::json!({"a":["C:\\Users\\Sumeet\\x"]});
        r.apply_json(&mut v);
        assert_eq!(v["a"][0], "C:\\Users\\<user>\\x");
    }
}
