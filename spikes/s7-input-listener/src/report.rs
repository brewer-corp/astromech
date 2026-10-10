//! Console lines, HTML and JSON output, with Windows-username redaction (from k0-binds-doctor).

use crate::model::Event;
use crate::summary::{Report, Verdict};

pub struct Redactor {
    pairs: Vec<(String, String)>,
    user: Option<String>,
}

impl Redactor {
    pub fn from_env() -> Self {
        let mut pairs = vec![];
        for (var, label) in [("LOCALAPPDATA", "%LOCALAPPDATA%"), ("APPDATA", "%APPDATA%"), ("USERPROFILE", "%USERPROFILE%")] {
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
            for (pre, rep) in [("\\users\\", "\\Users\\<user>"), ("/users/", "/Users/<user>"), ("/home/", "/home/<user>"), ("\\\\users\\\\", "\\\\Users\\\\<user>")] {
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

pub fn esc(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

pub fn event_line(e: &Event) -> String {
    let t = format!("{:>8.3}s", e.t_ms / 1000.0);
    let focus = if e.elite_focused { "[ELITE]" } else { "[other]" };
    let what = match (&e.axis, e.down) {
        (Some(a), _) => format!("{} moved {:.0}% -> {:.0}% (range {:.0}-{:.0}%, {:.0} ms)", e.control, a.start, a.end, a.min, a.max, a.duration_ms),
        (None, Some(true)) => format!("{} DOWN", e.control),
        (None, Some(false)) => format!("{} up", e.control),
        _ => e.control.clone(),
    };
    let mut s = format!("{t} {focus} {:<44} {:<36}", trunc(&e.device, 44), what);
    if let Some(n) = &e.elite_name {
        s += &format!(" {n}");
    }
    if e.injected {
        s += " (injected)";
    }
    if !e.actions.is_empty() {
        s += &format!("  => {}", e.actions.join("; "));
    }
    if let Some(n) = &e.note {
        s += &format!("  ! {n}");
    }
    if !e.elite_focused && !e.foreground.is_empty() {
        s += &format!("  (fg: {})", trunc(&e.foreground, 40));
    }
    s
}

fn trunc(s: &str, n: usize) -> String {
    if s.chars().count() <= n { s.to_string() } else { format!("{}…", s.chars().take(n - 1).collect::<String>()) }
}

fn vline(label: &str, v: &Verdict) -> String {
    format!("  {label:<12} {:<13} {}\n", v.status, v.detail)
}

pub fn console_summary(r: &Report) -> String {
    let mut o = String::new();
    o += &format!("\n==== Summary ({:.0} s, {}) ====\n", r.seconds, r.mode);
    o += &format!("Elite focused: {:.1} s of {:.1} s ({} focus switches)\n", r.focus.elite_focused_ms / 1000.0, r.focus.total_ms / 1000.0, r.focus.switches);
    for d in &r.devices {
        if d.events == 0 && d.kind != crate::model::DevKind::Hid {
            continue;
        }
        o += &format!("\n{} {}\n", d.device, d.elite_id.as_ref().map(|i| format!("(Elite id {i}, declares {} buttons, {} hats, axes {})", d.buttons_declared, d.hats_declared, d.axes_declared.join("/"))).unwrap_or_default());
        o += &format!("  events: {} (Elite focused {}, not focused {}{})\n", d.events, d.events_elite_focused, d.events_not_focused, if d.events_injected > 0 { format!(", injected {}", d.events_injected) } else { String::new() });
        if !d.buttons.is_empty() {
            o += &format!("  buttons: {}\n", d.buttons.iter().map(|b| format!("{}x{}", b.elite_name.clone().unwrap_or(b.control.clone()), b.presses)).collect::<Vec<_>>().join(", "));
        }
        if !d.hats.is_empty() {
            o += &format!("  hats: {}\n", d.hats.iter().map(|b| format!("{}x{}", b.elite_name.clone().unwrap_or(b.control.clone()), b.presses)).collect::<Vec<_>>().join(", "));
        }
        if !d.axes.is_empty() {
            o += &format!("  axes: {}\n", d.axes.iter().map(|a| format!("{} {:.0}-{:.0}% ({} move(s)){}", a.elite_name.clone().unwrap_or(a.axis.clone()), a.min, a.max, a.movements, if a.actions.is_empty() { String::new() } else { format!(" => {}", a.actions.join("; ")) })).collect::<Vec<_>>().join(", "));
        }
    }
    for w in &r.above_32_warnings {
        o += &format!("\n! {w}");
    }
    if !r.anomalies.is_empty() {
        o += "\n\nPossible dropped input / gaps:\n";
        for a in &r.anomalies {
            o += &format!("  - {a}\n");
        }
    }
    o += "\n\nVerdict:\n";
    o += &vline("Controllers", &r.verdict_controllers);
    o += &vline("Keyboard", &r.verdict_keyboard);
    o += &vline("Mouse", &r.verdict_mouse);
    o += &format!("\n>>> Background input capture while Elite focused: {} <<<\n", r.verdict.status);
    o
}

pub fn html(r: &Report) -> String {
    let mut h = String::new();
    h += "<!doctype html><html><head><meta charset=\"utf-8\"><title>S7 input listener report</title><style>\
body{font-family:Segoe UI,Arial,sans-serif;background:#111;color:#ddd;margin:24px;max-width:1300px}\
h1,h2{color:#ff8c1a}h3{color:#ffb366;margin-bottom:4px}table{border-collapse:collapse;margin:8px 0;width:100%}\
td,th{border:1px solid #333;padding:3px 7px;text-align:left;vertical-align:top;font-size:13px}th{background:#222}\
.WORKS{color:#7ddc7d;font-weight:bold}.FAILED{color:#ff5c5c;font-weight:bold}.NOT{color:#ffc94d;font-weight:bold}\
.big{font-size:22px;padding:10px;border:2px solid #444;border-radius:6px;display:inline-block}\
.muted{color:#888}.warn{color:#ffc94d}.f{color:#7ddc7d}code{background:#000;color:#cfc;padding:1px 4px}</style></head><body>";
    let cls = |s: &str| if s == "NOT OBSERVED" { "NOT".to_string() } else { s.to_string() };
    h += &format!("<h1>S7 input listener</h1><p class=muted>{} · {} · {}</p><p>{}</p>", esc(&r.tool), esc(&r.generated_at), esc(&r.mode), esc(&r.read_only));
    h += &format!("<p class=big>Background input capture while Elite focused: <span class={}>{}</span></p>", cls(&r.verdict.status), esc(&r.verdict.status));
    h += "<table><tr><th>Class</th><th>Verdict</th><th>Detail</th></tr>";
    for (l, v) in [("Controllers (HID + XInput)", &r.verdict_controllers), ("Keyboard", &r.verdict_keyboard), ("Mouse", &r.verdict_mouse)] {
        h += &format!("<tr><td>{l}</td><td class={}>{}</td><td>{}</td></tr>", cls(&v.status), esc(&v.status), esc(&v.detail));
    }
    h += "</table>";
    h += &format!("<p>Run length {:.1} s · Elite (<code>{}</code>) focused <b>{:.1} s</b> · {} focus switches · Raw Input registration: {} · XInput: {}</p>", r.focus.total_ms / 1000.0, esc(&r.elite_title), r.focus.elite_focused_ms / 1000.0, r.focus.switches, esc(&r.registration), esc(&r.xinput));
    h += "<details><summary>Foreground windows</summary><table>";
    for (t, ms) in &r.focus.titles {
        h += &format!("<tr><td>{}</td><td>{:.1} s</td></tr>", esc(if t.is_empty() { "(none)" } else { t }), ms / 1000.0);
    }
    h += "</table></details>";
    if let Some(b) = &r.binds {
        h += &format!("<p>Binds (read-only): active {} preset <b>{}</b> from {} <code>{}</code> ({}; {} bound slots)</p>", esc(&b.context), esc(&b.preset_name), esc(&b.source), esc(&b.file), esc(&b.start_preset_file), b.bound_slots);
    }
    if let Some(e) = &r.binds_error {
        h += &format!("<p class=warn>Binds not loaded: {}</p>", esc(e));
    }
    h += "<h2>Devices</h2>";
    for d in &r.devices {
        h += &format!("<h3>{}</h3><p class=muted>{:?}{} · events {} (Elite focused {}, not focused {}, injected {})</p>", esc(&d.device), d.kind, d.elite_id.as_ref().map(|i| format!(" · Elite id <code>{i}</code> · declares {} buttons, {} hat(s), axes {}", d.buttons_declared, d.hats_declared, esc(&d.axes_declared.join(", ")))).unwrap_or_default(), d.events, d.events_elite_focused, d.events_not_focused, d.events_injected);
        if !d.buttons.is_empty() || !d.hats.is_empty() {
            h += "<table><tr><th>Control</th><th>Elite name</th><th>Presses</th><th>While Elite focused</th><th>Bound actions (active preset)</th></tr>";
            for b in d.buttons.iter().chain(d.hats.iter()) {
                let warn = b.elite_name.as_deref().and_then(|n| n.strip_prefix("Joy_")).and_then(|n| n.parse::<u32>().ok()).map(|n| n > 32).unwrap_or(false);
                h += &format!("<tr><td>{}</td><td{}><code>{}</code>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>", esc(&b.control), if warn { " class=warn" } else { "" }, esc(b.elite_name.as_deref().unwrap_or("")), if warn { " ⚠ &gt;32" } else { "" }, b.presses, b.presses_elite_focused, esc(&b.actions.join("; ")));
            }
            h += "</table>";
        }
        if !d.axes.is_empty() {
            h += "<table><tr><th>Axis</th><th>Elite name</th><th>Movements</th><th>Range seen</th><th>Bound actions (active preset)</th></tr>";
            for a in &d.axes {
                h += &format!("<tr><td>{}</td><td><code>{}</code></td><td>{}</td><td>{:.0}–{:.0}%</td><td>{}</td></tr>", esc(&a.axis), esc(a.elite_name.as_deref().unwrap_or("(no Elite slot)")), a.movements, a.min, a.max, esc(&a.actions.join("; ")));
            }
            h += "</table>";
        }
    }
    for w in &r.above_32_warnings {
        h += &format!("<p class=warn>⚠ {}</p>", esc(w));
    }
    h += "<h2>Possible dropped input / gaps</h2>";
    if r.anomalies.is_empty() {
        h += "<p class=f>None: every press had a matching release.</p>";
    } else {
        h += "<ul>";
        for a in &r.anomalies {
            h += &format!("<li>{}</li>", esc(a));
        }
        h += "</ul>";
    }
    h += &format!("<p class=muted>Keyboard auto-repeat downs collapsed: {}</p>", r.key_repeats_collapsed);
    h += &format!("<h2>Event log ({})</h2><table><tr><th>Time</th><th>Focus</th><th>Device</th><th>Event</th><th>Elite name</th><th>Bound actions</th><th>Foreground / note</th></tr>", r.events.len());
    for e in &r.events {
        let what = match (&e.axis, e.down) {
            (Some(a), _) => format!("{} {:.0}%→{:.0}% (min {:.0}, max {:.0}, {:.0} ms)", e.control, a.start, a.end, a.min, a.max, a.duration_ms),
            (None, Some(true)) => format!("{} down", e.control),
            (None, Some(false)) => format!("{} up", e.control),
            _ => e.control.clone(),
        };
        h += &format!(
            "<tr><td>{:.3}</td><td class={}>{}</td><td>{}</td><td>{}{}</td><td><code>{}</code></td><td>{}</td><td>{}{}</td></tr>",
            e.t_ms / 1000.0,
            if e.elite_focused { "f" } else { "muted" },
            if e.elite_focused { "Elite" } else { "other" },
            esc(&e.device),
            esc(&what),
            if e.injected { " <span class=muted>(injected)</span>" } else { "" },
            esc(e.elite_name.as_deref().unwrap_or("")),
            esc(&e.actions.join("; ")),
            if e.elite_focused { String::new() } else { esc(&e.foreground) },
            e.note.as_ref().map(|n| format!(" <span class=warn>{}</span>", esc(n))).unwrap_or_default()
        );
    }
    h += "</table>";
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
