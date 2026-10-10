//! The ship status panel in iced: widgets for layout/text, small canvas
//! programs for the HUD decorations (chamfered panels, segmented bars, mic).

use hud_model::*;
use iced::font::{Family, Weight};
use iced::mouse;
use iced::widget::canvas::{self, Frame, Geometry, Path, Stroke};
use iced::widget::{canvas as cv, column, container, row, scrollable, space, stack, text};
use iced::{Border, Color, Element, Fill, Font, Point, Rectangle, Renderer, Size, Theme};

pub const RAJ: Font = Font { family: Family::Name("Rajdhani"), weight: Weight::Bold, ..Font::DEFAULT };
pub const BODY: Font = Font { family: Family::Name("Barlow"), ..Font::DEFAULT };
pub const MONO: Font = Font { family: Family::Name("Share Tech Mono"), ..Font::DEFAULT };

pub const FONTS: [&[u8]; 5] = [
    include_bytes!("../../assets/fonts/Rajdhani-SemiBold.ttf"),
    include_bytes!("../../assets/fonts/Rajdhani-Bold.ttf"),
    include_bytes!("../../assets/fonts/Barlow-Regular.ttf"),
    include_bytes!("../../assets/fonts/Barlow-Medium.ttf"),
    include_bytes!("../../assets/fonts/ShareTechMono-Regular.ttf"),
];

fn c(rgb: [u8; 3]) -> Color {
    Color::from_rgb8(rgb[0], rgb[1], rgb[2])
}
fn ca(rgb: [u8; 3], a: f32) -> Color {
    Color::from_rgba8(rgb[0], rgb[1], rgb[2], a)
}

type El<'a, M> = Element<'a, M, Theme, Renderer>;

// ------------------------------------------------------------ canvas programs

struct Backdrop;
impl<M> canvas::Program<M> for Backdrop {
    type State = ();
    fn draw(&self, _: &(), r: &Renderer, _: &Theme, b: Rectangle, _: mouse::Cursor) -> Vec<Geometry> {
        let mut f = Frame::new(r, b.size());
        f.fill_rectangle(Point::ORIGIN, b.size(), c(BG));
        let grid = ca(LINE, 0.22);
        let mut x = 0.0;
        while x < b.width {
            f.fill_rectangle(Point::new(x, 0.0), Size::new(1.0, b.height), grid);
            x += 32.0;
        }
        let mut y = 0.0;
        while y < b.height {
            f.fill_rectangle(Point::new(0.0, y), Size::new(b.width, 1.0), grid);
            y += 32.0;
        }
        let g = iced::gradient::Linear::new(std::f32::consts::PI)
            .add_stop(0.0, ca(ACCENT, 0.10))
            .add_stop(1.0, ca(ACCENT, 0.0));
        let glow = Path::rectangle(Point::ORIGIN, Size::new(b.width, 220.0));
        f.fill(&glow, canvas::Fill::from(canvas::Gradient::Linear(
            canvas::gradient::Linear::new(Point::new(0.0, 0.0), Point::new(0.0, 220.0))
                .add_stop(0.0, ca(ACCENT, 0.10))
                .add_stop(1.0, ca(ACCENT, 0.0)),
        )));
        let _ = g;
        vec![f.into_geometry()]
    }
}

struct Chamfer {
    fill: Color,
    stroke: Color,
    cut: f32,
    bar: Option<Color>,
    underline: Option<Color>,
}
impl<M> canvas::Program<M> for Chamfer {
    type State = ();
    fn draw(&self, _: &(), r: &Renderer, _: &Theme, b: Rectangle, _: mouse::Cursor) -> Vec<Geometry> {
        let mut f = Frame::new(r, b.size());
        let (w, h, k) = (b.width - 0.5, b.height - 0.5, self.cut);
        let p = Path::new(|p| {
            p.move_to(Point::new(k, 0.5));
            p.line_to(Point::new(w, 0.5));
            p.line_to(Point::new(w, h - k));
            p.line_to(Point::new(w - k, h));
            p.line_to(Point::new(0.5, h));
            p.line_to(Point::new(0.5, k));
            p.close();
        });
        f.fill(&p, self.fill);
        f.stroke(&p, Stroke::default().with_width(1.0).with_color(self.stroke));
        if let Some(bar) = self.bar {
            f.fill_rectangle(Point::new(0.0, 12.0), Size::new(3.0, b.height - 24.0), bar);
        }
        if let Some(u) = self.underline {
            f.fill_rectangle(Point::new(0.0, b.height - 2.0), Size::new(b.width * 0.4, 2.0), u);
        }
        vec![f.into_geometry()]
    }
}

struct SegBar {
    frac: f32,
    col: Color,
    n: usize,
}
impl<M> canvas::Program<M> for SegBar {
    type State = ();
    fn draw(&self, _: &(), r: &Renderer, _: &Theme, b: Rectangle, _: mouse::Cursor) -> Vec<Geometry> {
        let mut f = Frame::new(r, b.size());
        let gap = 2.0;
        let w = (b.width - gap * (self.n as f32 - 1.0)) / self.n as f32;
        let lit = self.frac.clamp(0.0, 1.0) * self.n as f32;
        for i in 0..self.n {
            let col = if (i as f32) < lit.floor() {
                self.col
            } else if (i as f32) < lit {
                Color { a: self.col.a * 0.55, ..self.col }
            } else {
                ca(LINE, 0.7)
            };
            let p = Path::rounded_rectangle(Point::new(i as f32 * (w + gap), 0.0), Size::new(w, b.height), 1.0.into());
            f.fill(&p, col);
        }
        vec![f.into_geometry()]
    }
}

struct Mic {
    col: Color,
    ring: bool,
    pulse: f32,
}
impl<M> canvas::Program<M> for Mic {
    type State = ();
    fn draw(&self, _: &(), r: &Renderer, _: &Theme, b: Rectangle, _: mouse::Cursor) -> Vec<Geometry> {
        let mut f = Frame::new(r, b.size());
        let m = Point::new(36.0, b.height / 2.0);
        if self.ring {
            let k = self.pulse;
            f.stroke(&Path::circle(m, 14.0 + 12.0 * k), Stroke::default().with_width(2.0).with_color(Color { a: 1.0 - k, ..self.col }));
        }
        f.fill(&Path::circle(m, 16.0), Color { a: 0.18, ..self.col });
        f.stroke(&Path::circle(m, 16.0), Stroke::default().with_width(1.5).with_color(self.col));
        f.fill(&Path::rounded_rectangle(Point::new(m.x - 3.5, m.y - 9.0), Size::new(7.0, 12.0), 3.5.into()), self.col);
        f.fill_rectangle(Point::new(m.x - 0.75, m.y + 5.0), Size::new(1.5, 4.0), self.col);
        vec![f.into_geometry()]
    }
}

struct Level {
    level: f32,
    pulse: f32,
}
impl<M> canvas::Program<M> for Level {
    type State = ();
    fn draw(&self, _: &(), r: &Renderer, _: &Theme, b: Rectangle, _: mouse::Cursor) -> Vec<Geometry> {
        let mut f = Frame::new(r, b.size());
        let n = 28;
        for i in 0..n {
            let fr = i as f32 / n as f32;
            let h = (4.0 + 20.0 * (0.5 + 0.5 * (fr * 9.0 + self.pulse * 6.28).sin()) * self.level).max(3.0);
            let x = i as f32 * (b.width / n as f32);
            let col = if self.level > 0.0 { ca(ACCENT, 0.4 + 0.6 * self.level) } else { ca(LINE, 0.8) };
            f.fill(&Path::rounded_rectangle(Point::new(x, b.height / 2.0 - h / 2.0), Size::new(4.0, h), 1.0.into()), col);
        }
        vec![f.into_geometry()]
    }
}

// ------------------------------------------------------------------ helpers

fn panel<'a, M: 'a>(content: impl Into<El<'a, M>>, accent: bool) -> El<'a, M> {
    stack![
        cv(Chamfer {
            fill: ca(PANEL, 0.92),
            stroke: if accent { ca(ACCENT, 0.55) } else { c(LINE) },
            cut: 10.0,
            bar: None,
            underline: None,
        })
        .width(Fill)
        .height(Fill),
        content.into()
    ]
    .into()
}

fn t<'a>(s: impl text::IntoFragment<'a>, font: Font, size: f32, col: Color) -> text::Text<'a, Theme, Renderer> {
    text(s).font(font).size(size).color(col)
}

fn title<'a, M: 'a>(s: &'a str) -> El<'a, M> {
    row![
        container(space()).width(3).height(12).style(|_| container::Style { background: Some(c(ACCENT).into()), ..Default::default() }),
        t(s, RAJ, 15.0, c(TEXT_DIM)),
        container(space()).width(50).height(1).style(|_| container::Style { background: Some(ca(ACCENT, 0.4).into()), ..Default::default() }),
    ]
    .spacing(10)
    .align_y(iced::Center)
    .into()
}

fn chip<'a, M: 'a>(label: &'a str, col: Color, w: f32, h: f32, size: f32) -> El<'a, M> {
    container(t(label, RAJ, size, col))
        .center_x(w)
        .center_y(h)
        .style(move |_| container::Style {
            background: Some(Color { a: 0.12, ..col }.into()),
            border: Border { color: Color { a: 0.6, ..col }, width: 1.0, radius: 2.0.into() },
            ..Default::default()
        })
        .into()
}

fn tile<'a, M: 'a>(label: &'a str, on_t: &'a str, off_t: &'a str, s: Toggle) -> El<'a, M> {
    let on = s.on;
    let marker: El<'a, M> = if s.pending {
        t("VERIFYING", RAJ, 11.0, c(CYAN)).into()
    } else {
        container(space())
            .width(8)
            .height(8)
            .style(move |_| container::Style {
                background: Some(if on { c(ACCENT) } else { ca(TEXT_DIM, 0.4) }.into()),
                border: Border { radius: 4.0.into(), ..Default::default() },
                shadow: if on {
                    iced::Shadow { color: ca(ACCENT, 0.6), offset: iced::Vector::ZERO, blur_radius: 8.0 }
                } else {
                    Default::default()
                },
                ..Default::default()
            })
            .into()
    };
    stack![
        cv(Chamfer {
            fill: if on { ca(ACCENT, 0.10) } else { ca(PANEL_HI, 0.9) },
            stroke: if on { ca(ACCENT, 0.7) } else { c(LINE) },
            cut: 8.0,
            bar: Some(if on { c(ACCENT) } else { c(LINE) }),
            underline: None,
        })
        .width(Fill)
        .height(Fill),
        column![
            row![t(label, RAJ, 13.0, c(TEXT_DIM)), space::horizontal(), marker].align_y(iced::Center),
            space::vertical(),
            t(if on { on_t } else { off_t }, RAJ, 22.0, if on { c(ACCENT) } else { c(TEXT) }),
        ]
        .padding(iced::Padding { top: 8.0, right: 12.0, bottom: 6.0, left: 14.0 })
    ]
    .width(Fill)
    .height(64)
    .into()
}

fn meter<'a, M: 'a>(label: &'a str, value: String, frac: f32, col: Color) -> El<'a, M> {
    column![
        row![t(label, RAJ, 14.0, c(TEXT_DIM)), space::horizontal(), t(value, MONO, 14.0, c(TEXT))],
        cv(SegBar { frac, col, n: 24 }).width(Fill).height(14),
    ]
    .spacing(4)
    .width(Fill)
    .into()
}

// ---------------------------------------------------------------------- view

pub fn view<'a, M: 'a>(s: &'a Snapshot, footer_note: String) -> El<'a, M> {
    let hcol = |k: &'a str, v: String, sub: String| -> El<'a, M> {
        column![t(k, RAJ, 12.0, c(ACCENT_DIM)), t(v, RAJ, 22.0, c(TEXT)), t(sub, BODY, 13.0, c(TEXT_DIM))].into()
    };
    let live = container(
        row![
            container(space()).width(6).height(6).style(|_| container::Style {
                background: Some(c(GREEN).into()),
                border: Border { radius: 3.0.into(), ..Default::default() },
                ..Default::default()
            }),
            t("JOURNAL LIVE", RAJ, 11.0, c(GREEN))
        ]
        .spacing(5)
        .align_y(iced::Center),
    )
    .padding([0, 6])
    .width(108)
    .height(16)
    .center_y(16)
    .style(|_| container::Style {
        background: Some(ca(GREEN, 0.10).into()),
        border: Border { color: ca(GREEN, 0.5), width: 1.0, radius: 2.0.into() },
        ..Default::default()
    });
    let header_content = row![
        hcol("COMMANDER", s.commander.into(), "Elite · Combat: Dangerous".into()),
        hcol("SHIP", s.ship_type.into(), format!("{} · {}", s.ship_name, s.ship_ident)),
        hcol("LOCATION", s.system.into(), s.body.into()),
        space::horizontal(),
        column![t(s.clock.clone(), MONO, 24.0, c(ACCENT)), live].spacing(4).align_x(iced::Right),
    ]
    .spacing(40)
    .padding(iced::Padding { top: 8.0, right: 20.0, bottom: 8.0, left: 22.0 });
    let header = stack![
        cv(Chamfer { fill: ca(PANEL, 0.92), stroke: ca(ACCENT, 0.55), cut: 10.0, bar: None, underline: Some(c(ACCENT)) })
            .width(Fill)
            .height(Fill),
        header_content
    ]
    .height(70);

    // left column
    let fsd_col = match s.fsd {
        FsdState::Ready => c(GREEN),
        FsdState::Charging | FsdState::Jumping => c(ACCENT),
        FsdState::Cooldown => c(CYAN),
        FsdState::MassLocked => c(RED),
    };
    let fsd_frac = match s.fsd {
        FsdState::Ready => 1.0,
        FsdState::MassLocked => 0.0,
        _ => s.fsd_progress,
    };
    let fsd = stack![
        cv(Chamfer { fill: ca(PANEL_HI, 0.9), stroke: c(LINE), cut: 8.0, bar: None, underline: None }).width(Fill).height(Fill),
        row![
            column![t("FRAME SHIFT DRIVE", RAJ, 13.0, c(TEXT_DIM)), space::vertical(), t(s.fsd.label(), RAJ, 22.0, fsd_col)].width(Fill),
            column![
                row![space::horizontal(), t(format!("{:.0}%", fsd_frac * 100.0), MONO, 13.0, c(TEXT_DIM))],
                space::vertical(),
                cv(SegBar { frac: fsd_frac, col: fsd_col, n: 30 }).width(Fill).height(18),
            ]
            .width(Fill)
        ]
        .padding(iced::Padding { top: 8.0, right: 14.0, bottom: 12.0, left: 14.0 })
    ]
    .height(62);
    let heat_col = if s.heat > 0.6 { c(RED) } else { c(ACCENT) };
    let left = panel(
        column![
            title("SHIP SYSTEMS"),
            row![
                tile("LANDING GEAR", "DEPLOYED", "RETRACTED", s.gear),
                tile("SHIP LIGHTS", "ON", "OFF", s.lights),
                tile("HARDPOINTS", "DEPLOYED", "STOWED", s.hardpoints)
            ]
            .spacing(10),
            row![
                tile("CARGO SCOOP", "OPEN", "CLOSED", s.cargo_scoop),
                tile("NIGHT VISION", "ON", "OFF", s.night_vision),
                tile("FLIGHT ASSIST", "ON", "OFF", s.flight_assist)
            ]
            .spacing(10),
            space().height(8),
            fsd,
            space().height(8),
            title("VITALS"),
            row![
                meter("FUEL", format!("{:.1} / {:.0} t", s.fuel_main, s.fuel_capacity), s.fuel_main / s.fuel_capacity, c(ACCENT)),
                meter("RESERVOIR", format!("{:.0}%", s.fuel_reservoir * 100.0), s.fuel_reservoir, ca(ACCENT, 0.7))
            ]
            .spacing(16),
            row![
                meter("SHIELDS", format!("{:.0}%", s.shields * 100.0), s.shields, c(CYAN)),
                meter("HULL", format!("{:.0}%", s.hull * 100.0), s.hull, c(GREEN))
            ]
            .spacing(16),
            row![meter("HEAT", format!("{:.0}%", s.heat * 100.0), s.heat, heat_col), space::horizontal()].spacing(16),
        ]
        .spacing(10)
        .padding(16),
        false,
    );

    // comms log
    let entries = column(s.log.iter().map(|e| {
        let (tag, col, txt) = match e.speaker {
            Speaker::Cmdr => ("CMDR", c(ACCENT), c(TEXT)),
            Speaker::Copilot => ("HELM", c(CYAN), c(TEXT)),
            Speaker::System => ("SYS", c(TEXT_DIM), c(TEXT_DIM)),
        };
        row![
            chip(tag, col, 42.0, 15.0, 11.0),
            column![t(e.time.clone(), MONO, 12.0, c(TEXT_DIM)), t(e.text, BODY, 15.0, txt)].width(Fill)
        ]
        .spacing(8)
        .into()
    }))
    .spacing(8)
    .padding(iced::Padding { right: 12.0, ..Default::default() });
    let right = panel(
        column![
            row![title("COMMS"), space::horizontal(), t(format!("{} entries", s.log.len()), MONO, 12.0, c(TEXT_DIM))],
            scrollable(entries)
                .anchor_bottom()
                .direction(scrollable::Direction::Vertical(scrollable::Scrollbar::new().width(4).scroller_width(4)))
                .height(Fill)
                .width(Fill),
        ]
        .spacing(10)
        .padding(iced::Padding { top: 14.0, right: 8.0, bottom: 10.0, left: 14.0 }),
        false,
    );

    // footer
    let ptt_col = match s.ptt {
        Ptt::Listening => c(ACCENT),
        Ptt::Thinking => c(CYAN),
        Ptt::Idle => c(TEXT_DIM),
    };
    let footer = stack![
        cv(Chamfer { fill: ca(PANEL, 0.92), stroke: if s.ptt != Ptt::Idle { ca(ACCENT, 0.55) } else { c(LINE) }, cut: 10.0, bar: None, underline: None })
            .width(Fill)
            .height(Fill),
        row![
            cv(Mic { col: ptt_col, ring: s.ptt == Ptt::Listening, pulse: s.pulse }).width(66).height(64),
            column![t(s.ptt.label(), RAJ, 20.0, ptt_col), t("Hold JOY 14 (or Caps Lock) to talk · offline STT", BODY, 13.0, c(TEXT_DIM))].width(300),
            cv(Level { level: s.mic_level, pulse: s.pulse }).width(220).height(24),
            space::horizontal(),
            chip("KILL SWITCH ARMED", c(RED), 180.0, 28.0, 14.0),
        ]
        .align_y(iced::Center)
        .padding(iced::Padding { right: 20.0, ..Default::default() })
    ]
    .height(64);

    let body = row![
        container(left).width(iced::FillPortion(56)).height(Fill),
        container(right).width(iced::FillPortion(44)).height(Fill)
    ]
    .spacing(12)
    .height(Fill);

    stack![
        cv(Backdrop).width(Fill).height(Fill),
        column![header, body, footer].spacing(12).padding(16),
        column![space::vertical(), row![space::horizontal(), t(footer_note, MONO, 10.0, Color::from_rgb8(90, 90, 90))]]
            .padding(iced::Padding { right: 18.0, bottom: 2.0, ..Default::default() }),
    ]
    .into()
}
