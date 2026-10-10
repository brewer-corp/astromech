//! The ship status panel, drawn with egui. Mostly custom painting so we can
//! judge how far egui goes beyond its default look.

use egui::{
    Align2, Color32, CornerRadius, FontData, FontDefinitions, FontFamily, FontId, Pos2, Rect,
    Sense, Shape, Stroke, StrokeKind, Ui, UiBuilder, Vec2, epaint::Mesh, pos2, vec2,
};
use hud_model::*;
use std::sync::Arc;

fn c(rgb: [u8; 3]) -> Color32 {
    Color32::from_rgb(rgb[0], rgb[1], rgb[2])
}
fn ca(rgb: [u8; 3], a: f32) -> Color32 {
    Color32::from_rgba_unmultiplied(rgb[0], rgb[1], rgb[2], (a * 255.0) as u8)
}

pub fn install_fonts(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();
    let add = |fonts: &mut FontDefinitions, name: &str, bytes: &'static [u8]| {
        fonts.font_data.insert(name.into(), Arc::new(FontData::from_static(bytes)));
        fonts.families.insert(FontFamily::Name(name.into()), vec![name.into()]);
    };
    add(&mut fonts, "display", include_bytes!("../../assets/fonts/Rajdhani-SemiBold.ttf"));
    add(&mut fonts, "display-bold", include_bytes!("../../assets/fonts/Rajdhani-Bold.ttf"));
    add(&mut fonts, "body", include_bytes!("../../assets/fonts/Barlow-Regular.ttf"));
    add(&mut fonts, "body-med", include_bytes!("../../assets/fonts/Barlow-Medium.ttf"));
    add(&mut fonts, "mono", include_bytes!("../../assets/fonts/ShareTechMono-Regular.ttf"));
    fonts.families.get_mut(&FontFamily::Proportional).unwrap().insert(0, "body".into());
    fonts.families.get_mut(&FontFamily::Monospace).unwrap().insert(0, "mono".into());
    ctx.set_fonts(fonts);

    let mut style = (*ctx.global_style()).clone();
    style.visuals = egui::Visuals::dark();
    style.visuals.panel_fill = c(BG);
    style.visuals.window_fill = c(BG);
    style.visuals.extreme_bg_color = c(BG);
    style.spacing.scroll.bar_width = 4.0;
    style.spacing.scroll.floating = true;
    style.visuals.widgets.inactive.bg_fill = ca(ACCENT, 0.35);
    style.visuals.widgets.hovered.bg_fill = c(ACCENT);
    style.visuals.widgets.active.bg_fill = c(ACCENT);
    ctx.set_global_style(style);
}

fn font(name: &str, size: f32) -> FontId {
    FontId::new(size, FontFamily::Name(name.into()))
}

/// Polygon with cut (chamfered) top-left and bottom-right corners: the
/// "Elite" panel shape.
fn chamfer(r: Rect, cut: f32) -> Vec<Pos2> {
    vec![
        pos2(r.left() + cut, r.top()),
        pos2(r.right(), r.top()),
        pos2(r.right(), r.bottom() - cut),
        pos2(r.right() - cut, r.bottom()),
        pos2(r.left(), r.bottom()),
        pos2(r.left(), r.top() + cut),
    ]
}

fn panel(ui: &Ui, r: Rect, accent: bool) {
    let p = ui.painter();
    let stroke = Stroke::new(1.0, if accent { ca(ACCENT, 0.55) } else { c(LINE) });
    p.add(Shape::convex_polygon(chamfer(r, 10.0), ca(PANEL, 0.92), stroke));
}

fn section_title(ui: &Ui, pos: Pos2, text: &str) -> f32 {
    let p = ui.painter();
    p.rect_filled(Rect::from_min_size(pos + vec2(0.0, 3.0), vec2(3.0, 12.0)), 0.0, c(ACCENT));
    let r = p.text(pos + vec2(10.0, 0.0), Align2::LEFT_TOP, text, font("display-bold", 15.0), c(TEXT_DIM));
    p.line_segment(
        [pos2(r.right() + 10.0, r.center().y), pos2(r.right() + 60.0, r.center().y)],
        Stroke::new(1.0, ca(ACCENT, 0.4)),
    );
    20.0
}

/// Vertical gradient quad (used for background glow).
fn vgradient(ui: &Ui, r: Rect, top: Color32, bottom: Color32) {
    let mut m = Mesh::default();
    m.colored_vertex(r.left_top(), top);
    m.colored_vertex(r.right_top(), top);
    m.colored_vertex(r.right_bottom(), bottom);
    m.colored_vertex(r.left_bottom(), bottom);
    m.add_triangle(0, 1, 2);
    m.add_triangle(0, 2, 3);
    ui.painter().add(Shape::mesh(m));
}

fn segmented_bar(ui: &Ui, r: Rect, frac: f32, col: Color32, segments: usize) {
    let p = ui.painter();
    let gap = 2.0;
    let w = (r.width() - gap * (segments as f32 - 1.0)) / segments as f32;
    let lit = frac.clamp(0.0, 1.0) * segments as f32;
    for i in 0..segments {
        let x = r.left() + i as f32 * (w + gap);
        let seg = Rect::from_min_size(pos2(x, r.top()), vec2(w, r.height()));
        let fill = if (i as f32) < lit.floor() {
            col
        } else if (i as f32) < lit {
            col.gamma_multiply(0.55)
        } else {
            ca(LINE, 0.7)
        };
        p.rect_filled(seg, 1.0, fill);
    }
}

fn meter(ui: &Ui, r: Rect, label: &str, value: String, frac: f32, col: Color32) {
    let p = ui.painter();
    p.text(r.left_top(), Align2::LEFT_TOP, label, font("display-bold", 14.0), c(TEXT_DIM));
    p.text(r.right_top(), Align2::RIGHT_TOP, value, FontId::monospace(14.0), c(TEXT));
    segmented_bar(ui, Rect::from_min_max(pos2(r.left(), r.top() + 20.0), r.right_bottom()), frac, col, 24);
}

fn toggle_tile(ui: &Ui, r: Rect, label: &str, on_text: &str, off_text: &str, t: Toggle) {
    let p = ui.painter();
    let on = t.on;
    let fill = if on { ca(ACCENT, 0.10) } else { ca(PANEL_HI, 0.9) };
    let stroke = Stroke::new(1.0, if on { ca(ACCENT, 0.7) } else { c(LINE) });
    p.add(Shape::convex_polygon(chamfer(r, 8.0), fill, stroke));
    p.rect_filled(
        Rect::from_min_size(r.left_top() + vec2(0.0, 12.0), vec2(3.0, r.height() - 24.0)),
        0.0,
        if on { c(ACCENT) } else { c(LINE) },
    );
    p.text(r.left_top() + vec2(14.0, 9.0), Align2::LEFT_TOP, label, font("display-bold", 13.0), c(TEXT_DIM));
    let state = if on { on_text } else { off_text };
    p.text(
        r.left_bottom() + vec2(14.0, -9.0),
        Align2::LEFT_BOTTOM,
        state,
        font("display-bold", 22.0),
        if on { c(ACCENT) } else { c(TEXT) },
    );
    // Status dot / verifying marker.
    let dot = r.right_top() + vec2(-14.0, 15.0);
    if t.pending {
        p.text(r.right_top() + vec2(-12.0, 9.0), Align2::RIGHT_TOP, "VERIFYING", font("display-bold", 11.0), c(CYAN));
    } else {
        p.circle_filled(dot, 4.0, if on { c(ACCENT) } else { ca(TEXT_DIM, 0.4) });
        if on {
            p.circle_filled(dot, 8.0, ca(ACCENT, 0.18));
        }
    }
}

pub fn draw(ui: &mut Ui, s: &Snapshot) {
    let full = ui.max_rect();
    let p = ui.painter().clone();

    // Background, faint grid and a warm top glow.
    p.rect_filled(full, 0.0, c(BG));
    let grid = ca(LINE, 0.22);
    let mut x = full.left();
    while x < full.right() {
        p.line_segment([pos2(x, full.top()), pos2(x, full.bottom())], Stroke::new(1.0, grid));
        x += 32.0;
    }
    let mut y = full.top();
    while y < full.bottom() {
        p.line_segment([pos2(full.left(), y), pos2(full.right(), y)], Stroke::new(1.0, grid));
        y += 32.0;
    }
    vgradient(ui, Rect::from_min_size(full.min, vec2(full.width(), 220.0)), ca(ACCENT, 0.10), ca(ACCENT, 0.0));

    let m = 16.0;
    let inner = full.shrink(m);

    // ---------- Header ----------
    let header = Rect::from_min_size(inner.min, vec2(inner.width(), 70.0));
    panel(ui, header, true);
    p.line_segment(
        [header.left_bottom() + vec2(0.0, 0.5), header.left_bottom() + vec2(header.width() * 0.4, 0.5)],
        Stroke::new(2.0, c(ACCENT)),
    );
    let mut hx = header.left() + 22.0;
    let col = |p: &egui::Painter, x: f32, k: &str, v: &str, sub: &str| -> f32 {
        p.text(pos2(x, header.top() + 11.0), Align2::LEFT_TOP, k, font("display-bold", 12.0), c(ACCENT_DIM));
        let r = p.text(pos2(x, header.top() + 24.0), Align2::LEFT_TOP, v, font("display-bold", 22.0), c(TEXT));
        let r2 = p.text(pos2(x, header.top() + 48.0), Align2::LEFT_TOP, sub, font("body", 13.0), c(TEXT_DIM));
        r.width().max(r2.width())
    };
    hx += col(&p, hx, "COMMANDER", s.commander, "Elite · Combat: Dangerous") + 40.0;
    let ship_sub = format!("{} · {}", s.ship_name, s.ship_ident);
    hx += col(&p, hx, "SHIP", s.ship_type, &ship_sub) + 40.0;
    col(&p, hx, "LOCATION", s.system, s.body);
    p.text(header.right_top() + vec2(-20.0, 14.0), Align2::RIGHT_TOP, &s.clock, FontId::monospace(24.0), c(ACCENT));
    let live = Rect::from_min_size(header.right_top() + vec2(-128.0, 44.0), vec2(108.0, 16.0));
    p.rect(live, 2.0, ca(GREEN, 0.10), Stroke::new(1.0, ca(GREEN, 0.5)), StrokeKind::Inside);
    p.circle_filled(live.left_center() + vec2(9.0, 0.0), 3.0, c(GREEN));
    p.text(live.left_center() + vec2(17.0, 0.0), Align2::LEFT_CENTER, "JOURNAL LIVE", font("display-bold", 11.0), c(GREEN));

    // ---------- Footer: PTT ----------
    let footer = Rect::from_min_max(pos2(inner.left(), inner.bottom() - 64.0), inner.max);
    panel(ui, footer, s.ptt != Ptt::Idle);
    let (ptt_col, ring) = match s.ptt {
        Ptt::Listening => (c(ACCENT), true),
        Ptt::Thinking => (c(CYAN), false),
        Ptt::Idle => (c(TEXT_DIM), false),
    };
    let mic = footer.left_center() + vec2(36.0, 0.0);
    if ring {
        let k = s.pulse;
        p.circle_stroke(mic, 14.0 + 12.0 * k, Stroke::new(2.0, ptt_col.gamma_multiply(1.0 - k)));
    }
    p.circle_filled(mic, 16.0, ptt_col.gamma_multiply(0.18));
    p.circle_stroke(mic, 16.0, Stroke::new(1.5, ptt_col));
    // simple mic glyph
    p.rect_filled(Rect::from_center_size(mic - vec2(0.0, 3.0), vec2(7.0, 12.0)), 3.5, ptt_col);
    p.line_segment([mic + vec2(0.0, 5.0), mic + vec2(0.0, 9.0)], Stroke::new(1.5, ptt_col));
    p.text(mic + vec2(30.0, -10.0), Align2::LEFT_CENTER, s.ptt.label(), font("display-bold", 20.0), ptt_col);
    p.text(mic + vec2(30.0, 11.0), Align2::LEFT_CENTER, "Hold JOY 14 (or Caps Lock) to talk · offline STT", font("body", 13.0), c(TEXT_DIM));
    // level meter
    let lm = Rect::from_min_size(footer.left_center() + vec2(380.0, -12.0), vec2(220.0, 24.0));
    let bars = 28;
    for i in 0..bars {
        let f = i as f32 / bars as f32;
        let h = 4.0 + 20.0 * (0.5 + 0.5 * ((f * 9.0 + s.pulse * 6.28).sin())) * s.mic_level;
        let bx = lm.left() + i as f32 * (lm.width() / bars as f32);
        let br = Rect::from_center_size(pos2(bx + 2.5, lm.center().y), vec2(4.0, h.max(3.0)));
        p.rect_filled(br, 1.0, if s.mic_level > 0.0 { ca(ACCENT, 0.4 + 0.6 * s.mic_level) } else { ca(LINE, 0.8) });
    }
    let ks = Rect::from_min_size(footer.right_center() + vec2(-200.0, -14.0), vec2(180.0, 28.0));
    p.rect(ks, 2.0, ca(RED, 0.08), Stroke::new(1.0, ca(RED, 0.6)), StrokeKind::Inside);
    p.text(ks.center(), Align2::CENTER_CENTER, "KILL SWITCH ARMED", font("display-bold", 14.0), c(RED));

    // ---------- Body ----------
    let body = Rect::from_min_max(pos2(inner.left(), header.bottom() + 12.0), pos2(inner.right(), footer.top() - 12.0));
    let left_w = (body.width() * 0.56).floor();
    let left = Rect::from_min_size(body.min, vec2(left_w, body.height()));
    let right = Rect::from_min_max(pos2(left.right() + 12.0, body.top()), body.max);

    panel(ui, left, false);
    let mut y = left.top() + 14.0;
    y += section_title(ui, pos2(left.left() + 16.0, y), "SHIP SYSTEMS") + 8.0;
    let gx = left.left() + 16.0;
    let gw = left.width() - 32.0;
    let tw = (gw - 2.0 * 10.0) / 3.0;
    let th = 64.0;
    let tiles: [(&str, &str, &str, Toggle); 6] = [
        ("LANDING GEAR", "DEPLOYED", "RETRACTED", s.gear),
        ("SHIP LIGHTS", "ON", "OFF", s.lights),
        ("HARDPOINTS", "DEPLOYED", "STOWED", s.hardpoints),
        ("CARGO SCOOP", "OPEN", "CLOSED", s.cargo_scoop),
        ("NIGHT VISION", "ON", "OFF", s.night_vision),
        ("FLIGHT ASSIST", "ON", "OFF", s.flight_assist),
    ];
    for (i, (l, on, off, t)) in tiles.iter().enumerate() {
        let r = Rect::from_min_size(pos2(gx + (i % 3) as f32 * (tw + 10.0), y + (i / 3) as f32 * (th + 10.0)), vec2(tw, th));
        toggle_tile(ui, r, l, on, off, *t);
    }
    y += 2.0 * th + 10.0 + 18.0;

    // FSD tile
    let fsd = Rect::from_min_size(pos2(gx, y), vec2(gw, 62.0));
    let fsd_col = match s.fsd {
        FsdState::Ready => c(GREEN),
        FsdState::Charging | FsdState::Jumping => c(ACCENT),
        FsdState::Cooldown => c(CYAN),
        FsdState::MassLocked => c(RED),
    };
    p.add(Shape::convex_polygon(chamfer(fsd, 8.0), ca(PANEL_HI, 0.9), Stroke::new(1.0, c(LINE))));
    p.text(fsd.left_top() + vec2(14.0, 9.0), Align2::LEFT_TOP, "FRAME SHIFT DRIVE", font("display-bold", 13.0), c(TEXT_DIM));
    p.text(fsd.left_bottom() + vec2(14.0, -9.0), Align2::LEFT_BOTTOM, s.fsd.label(), font("display-bold", 22.0), fsd_col);
    let fb = Rect::from_min_max(fsd.center_top() + vec2(-20.0, 30.0), fsd.right_bottom() + vec2(-14.0, -14.0));
    let fsd_frac = match s.fsd {
        FsdState::Ready => 1.0,
        FsdState::MassLocked => 0.0,
        _ => s.fsd_progress,
    };
    segmented_bar(ui, fb, fsd_frac, fsd_col, 30);
    let pct = format!("{:>3.0}%", fsd_frac * 100.0);
    p.text(fsd.right_top() + vec2(-14.0, 10.0), Align2::RIGHT_TOP, pct, FontId::monospace(13.0), c(TEXT_DIM));
    y += 62.0 + 18.0;

    y += section_title(ui, pos2(left.left() + 16.0, y), "VITALS") + 8.0;
    let mh = 34.0;
    let colw = (gw - 16.0) / 2.0;
    let m = |i: usize, j: usize| Rect::from_min_size(pos2(gx + j as f32 * (colw + 16.0), y + i as f32 * (mh + 14.0)), vec2(colw, mh));
    meter(ui, m(0, 0), "FUEL", format!("{:.1} / {:.0} t", s.fuel_main, s.fuel_capacity), s.fuel_main / s.fuel_capacity, c(ACCENT));
    meter(ui, m(0, 1), "RESERVOIR", format!("{:.0}%", s.fuel_reservoir * 100.0), s.fuel_reservoir, ca(ACCENT, 0.7));
    meter(ui, m(1, 0), "SHIELDS", format!("{:.0}%", s.shields * 100.0), s.shields, c(CYAN));
    meter(ui, m(1, 1), "HULL", format!("{:.0}%", s.hull * 100.0), s.hull, c(GREEN));
    let heat_col = if s.heat > 0.6 { c(RED) } else { c(ACCENT) };
    meter(ui, m(2, 0), "HEAT", format!("{:.0}%", s.heat * 100.0), s.heat, heat_col);

    // ---------- Comms log ----------
    panel(ui, right, false);
    section_title(ui, pos2(right.left() + 16.0, right.top() + 14.0), "COMMS");
    let counter = format!("{} entries", s.log.len());
    p.text(right.right_top() + vec2(-16.0, 16.0), Align2::RIGHT_TOP, counter, FontId::monospace(12.0), c(TEXT_DIM));
    let log_rect = Rect::from_min_max(right.min + vec2(14.0, 44.0), right.max - vec2(8.0, 10.0));
    let mut child = ui.new_child(UiBuilder::new().max_rect(log_rect));
    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .stick_to_bottom(true)
        .show(&mut child, |ui| {
            ui.set_width(log_rect.width() - 10.0);
            for e in &s.log {
                log_entry(ui, e);
            }
        });
}

fn log_entry(ui: &mut Ui, e: &LogEntry) {
    let w = ui.available_width();
    let (tag_col, text_col) = match e.speaker {
        Speaker::Cmdr => (c(ACCENT), c(TEXT)),
        Speaker::Copilot => (c(CYAN), c(TEXT)),
        Speaker::System => (c(TEXT_DIM), c(TEXT_DIM)),
    };
    let text_x = 50.0;
    let galley = ui.painter().layout(e.text.to_string(), font("body", 15.0), text_col, w - text_x);
    let h = 18.0 + galley.size().y + 8.0;
    let (rect, _) = ui.allocate_exact_size(Vec2::new(w, h), Sense::hover());
    let p = ui.painter();
    let tag = Rect::from_min_size(rect.left_top() + vec2(0.0, 2.0), vec2(42.0, 15.0));
    p.rect(tag, 2.0, tag_col.gamma_multiply(0.14), Stroke::new(1.0, tag_col.gamma_multiply(0.7)), StrokeKind::Inside);
    p.text(tag.center(), Align2::CENTER_CENTER, e.speaker.label(), font("display-bold", 11.0), tag_col);
    p.text(rect.left_top() + vec2(text_x, 2.0), Align2::LEFT_TOP, &e.time, FontId::monospace(12.0), c(TEXT_DIM));
    p.galley(rect.left_top() + vec2(text_x, 18.0), galley, text_col);
    if e.speaker == Speaker::Cmdr {
        p.line_segment([rect.left_top() + vec2(0.0, 20.0), rect.left_bottom() + vec2(0.0, -6.0)], Stroke::new(1.0, ca(ACCENT, 0.25)));
    }
    let _ = CornerRadius::ZERO;
}
