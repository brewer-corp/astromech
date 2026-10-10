//! S1 UI spike: iced ship status panel.
//!
//!   helm-s1-iced                         desktop window
//!   helm-s1-iced --offscreen [N] [--png out.png] [--t SECONDS] [--software]
//!                                        headless: iced's Headless renderer (wgpu 27, own device; or tiny-skia)
//!
//! Standalone demo: reads nothing, writes nothing (except --png), never touches the game.

mod hud;

use hud_model::{OFFSCREEN_H, OFFSCREEN_W, Snapshot, stats_ms};
use iced::advanced::renderer::Headless;
use iced::{Size, Subscription, window};
use std::time::Instant;

struct App {
    start: Instant,
    snap: Snapshot,
}

#[derive(Debug, Clone)]
enum Msg {
    Tick,
}

fn boot() -> App {
    App { start: Instant::now(), snap: hud_model::snapshot(0.0) }
}

fn update(app: &mut App, _m: Msg) {
    app.snap = hud_model::snapshot(app.start.elapsed().as_secs_f64());
}

fn view(app: &App) -> iced::Element<'_, Msg> {
    hud::view(&app.snap, "iced 0.14 · wgpu 27 · VR texture not wired (separate device)".into())
}

fn subscription(_app: &App) -> Subscription<Msg> {
    // ~30 fps is enough; frames() fires every vsync, so throttle via time check in update would also work.
    window::frames().map(|_| Msg::Tick)
}

fn arg_value(args: &[String], k: &str) -> Option<String> {
    args.iter().position(|a| a == k).and_then(|i| args.get(i + 1).cloned())
}

fn write_png(path: &str, rgba: &[u8]) {
    let f = std::fs::File::create(path).expect("create png");
    let mut e = png::Encoder::new(std::io::BufWriter::new(f), OFFSCREEN_W, OFFSCREEN_H);
    e.set_color(png::ColorType::Rgba);
    e.set_depth(png::BitDepth::Eight);
    e.write_header().unwrap().write_image_data(rgba).unwrap();
}

fn offscreen_main(args: &[String]) {
    let frames: usize = args
        .iter()
        .position(|a| a == "--offscreen")
        .and_then(|i| args.get(i + 1))
        .and_then(|s| s.parse().ok())
        .unwrap_or(300);
    let png = arg_value(args, "--png");
    let t_png: f64 = arg_value(args, "--t").and_then(|s| s.parse().ok()).unwrap_or(42.0);
    let backend = if args.iter().any(|a| a == "--software") { "tiny-skia" } else { "wgpu" };

    {
        let mut fs = iced_graphics::text::font_system().write().expect("font system");
        for f in hud::FONTS {
            fs.load_font(std::borrow::Cow::Borrowed(f));
        }
    }
    let t_init = Instant::now();
    let mut renderer: iced::Renderer =
        pollster::block_on(<iced::Renderer as Headless>::new(iced::Font::DEFAULT, 16.into(), Some(backend)))
            .expect("headless renderer");
    println!("renderer: iced Headless '{}' (backend={backend})", renderer.name());
    println!("init: {:.1} ms", t_init.elapsed().as_secs_f64() * 1000.0);
    let theme = iced::Theme::Dark;
    let style = iced::advanced::renderer::Style { text_color: iced::Color::WHITE };
    let size = Size::new(OFFSCREEN_W as f32, OFFSCREEN_H as f32);
    let mut cache = iced_runtime::user_interface::Cache::default();
    let bg = iced::Color::from_rgb8(hud_model::BG[0], hud_model::BG[1], hud_model::BG[2]);

    let mut frame = |t: f64, cache: &mut iced_runtime::user_interface::Cache| -> (Vec<u8>, f64, f64, f64) {
        let t0 = Instant::now();
        let snap = hud_model::snapshot(t);
        let mut ui = iced_runtime::UserInterface::build(
            hud::view::<()>(&snap, String::new()),
            size,
            std::mem::take(cache),
            &mut renderer,
        );
        let t1 = Instant::now();
        ui.draw(&mut renderer, &theme, &style, iced::mouse::Cursor::Unavailable);
        *cache = ui.into_cache();
        let t2 = Instant::now();
        let px = renderer.screenshot(Size::new(OFFSCREEN_W, OFFSCREEN_H), 1.0, bg);
        let t3 = Instant::now();
        let ms = |a: Instant, b: Instant| (b - a).as_secs_f64() * 1000.0;
        (px, ms(t0, t1), ms(t1, t2), ms(t2, t3))
    };
    let f0 = Instant::now();
    let _ = frame(0.0, &mut cache);
    println!("first frame: {:.2} ms", f0.elapsed().as_secs_f64() * 1000.0);
    let (mut tot, mut b, mut d, mut s) = (vec![], vec![], vec![], vec![]);
    let mut last = Vec::new();
    for i in 0..frames {
        let (px, a, bb, cc) = frame(5.0 + i as f64 / 30.0, &mut cache);
        b.push(a);
        d.push(bb);
        s.push(cc);
        tot.push(a + bb + cc);
        last = px;
    }
    assert_eq!(last.len(), (OFFSCREEN_W * OFFSCREEN_H * 4) as usize);
    println!("total/frame:        {}", stats_ms(&mut tot));
    println!("  build+layout:     {}", stats_ms(&mut b));
    println!("  draw (record):    {}", stats_ms(&mut d));
    println!("  render+readback:  {}", stats_ms(&mut s));
    if let Some(path) = png {
        let mut px = Vec::new();
        for k in 0..3 {
            px = frame(t_png + k as f64 * 0.001, &mut cache).0;
        }
        write_png(&path, &px);
        println!("wrote {path}");
    }
}

fn main() -> iced::Result {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--offscreen") {
        offscreen_main(&args);
        return Ok(());
    }
    let mut app = iced::application(boot, update, view)
        .title("helm S1 · iced ship status")
        .window_size(Size::new(1024.0, 768.0))
        .theme(|_: &App| iced::Theme::Dark)
        .subscription(subscription);
    for f in hud::FONTS {
        app = app.font(f);
    }
    let _ = window::Settings::default;
    app.run()
}
