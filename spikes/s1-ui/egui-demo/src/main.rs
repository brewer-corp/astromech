//! S1 UI spike: egui (eframe + egui-wgpu) ship status panel.
//!
//!   helm-s1-egui                 desktop window (also renders the VR texture at 30 Hz on the SAME wgpu device)
//!   helm-s1-egui --no-vr         desktop window only
//!   helm-s1-egui --offscreen [N] [--png out.png] [--t SECONDS]
//!                                headless: render N frames at 1024x768 to an RGBA buffer, print timings
//!
//! Standalone demo: reads nothing, writes nothing (except --png), never touches the game.

mod hud;
mod offscreen;

use hud_model::{OFFSCREEN_H, OFFSCREEN_W, stats_ms};
use std::time::Instant;

struct App {
    start: Instant,
    vr: Option<offscreen::OffscreenHud>,
    vr_enabled: bool,
    vr_buf: Vec<u8>,
    vr_last: f64,
    vr_ms: f64,
    vr_frames: u64,
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        let t = self.start.elapsed().as_secs_f64();
        let snap = hud_model::snapshot(t);
        hud::draw(ui, &snap);

        // VR overlay texture, rendered on eframe's own wgpu device (proves the
        // "one shared device" design). ≤30 Hz, as the overlay would.
        if self.vr_enabled && t - self.vr_last >= 1.0 / 30.0 {
            if self.vr.is_none() {
                if let Some(rs) = frame.wgpu_render_state() {
                    self.vr = Some(offscreen::OffscreenHud::new(
                        rs.device.clone(),
                        rs.queue.clone(),
                        OFFSCREEN_W,
                        OFFSCREEN_H,
                    ));
                }
            }
            if let Some(vr) = self.vr.as_mut() {
                let tm = vr.render(t, &mut self.vr_buf);
                self.vr_ms = self.vr_ms * 0.9 + tm.total_ms * 0.1;
                self.vr_frames += 1;
                self.vr_last = t;
            }
        }
        let r = ui.max_rect();
        let label = if self.vr_enabled {
            format!(
                "egui {} · VR texture {}x{} on shared wgpu device · {:.1} ms/frame · {} frames",
                env!("CARGO_PKG_VERSION"),
                OFFSCREEN_W,
                OFFSCREEN_H,
                self.vr_ms,
                self.vr_frames
            )
        } else {
            "egui · VR texture off".into()
        };
        ui.painter().text(
            r.right_bottom() + egui::vec2(-18.0, -2.0),
            egui::Align2::RIGHT_BOTTOM,
            label,
            egui::FontId::monospace(10.0),
            egui::Color32::from_gray(90),
        );
        // Redraw at ~30 fps while animating; a real app would redraw on state change only.
        ui.ctx().request_repaint_after(std::time::Duration::from_millis(33));
    }
}

fn arg_value(args: &[String], k: &str) -> Option<String> {
    args.iter().position(|a| a == k).and_then(|i| args.get(i + 1).cloned())
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

    let t_init = Instant::now();
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle_from_env());
    let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions::default()))
        .expect("no wgpu adapter");
    let info = adapter.get_info();
    let (device, queue) =
        pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor::default())).expect("device");
    let mut hud = offscreen::OffscreenHud::new(device, queue, OFFSCREEN_W, OFFSCREEN_H);
    println!("adapter: {} ({:?}, {:?})", info.name, info.backend, info.device_type);
    println!("init: {:.1} ms", t_init.elapsed().as_secs_f64() * 1000.0);

    let mut buf = Vec::new();
    let first = hud.render(0.0, &mut buf);
    println!("first frame (font atlas upload etc.): {:.2} ms", first.total_ms);
    let (mut tot, mut ui, mut tess, mut gpu) = (vec![], vec![], vec![], vec![]);
    for i in 0..frames {
        let t = 5.0 + i as f64 / 30.0;
        let tm = hud.render(t, &mut buf);
        tot.push(tm.total_ms);
        ui.push(tm.ui_ms);
        tess.push(tm.tess_ms);
        gpu.push(tm.gpu_readback_ms);
    }
    println!("total/frame:        {}", stats_ms(&mut tot));
    println!("  ui build:         {}", stats_ms(&mut ui));
    println!("  tessellate:       {}", stats_ms(&mut tess));
    println!("  gpu+readback:     {}", stats_ms(&mut gpu));
    assert_eq!(buf.len(), (OFFSCREEN_W * OFFSCREEN_H * 4) as usize);
    if let Some(path) = png {
        // Render a couple of frames at t so the scroll area settles to the bottom.
        for k in 0..3 {
            hud.render(t_png + k as f64 * 0.001, &mut buf);
        }
        offscreen::write_png(&path, OFFSCREEN_W, OFFSCREEN_H, &buf);
        println!("wrote {path}");
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--offscreen") {
        return offscreen_main(&args);
    }
    let vr_enabled = !args.iter().any(|a| a == "--no-vr");
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("helm S1 · egui ship status")
            .with_inner_size([1024.0, 768.0])
            .with_min_inner_size([800.0, 600.0]),
        renderer: eframe::Renderer::Wgpu,
        ..Default::default()
    };
    eframe::run_native(
        "helm-s1-egui",
        options,
        Box::new(move |cc| {
            hud::install_fonts(&cc.egui_ctx);
            Ok(Box::new(App {
                start: Instant::now(),
                vr: None,
                vr_enabled,
                vr_buf: Vec::new(),
                vr_last: -1.0,
                vr_ms: 0.0,
                vr_frames: 0,
            }))
        }),
    )
    .expect("eframe");
}
