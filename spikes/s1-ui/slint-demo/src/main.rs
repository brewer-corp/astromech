//! S1 UI spike: Slint ship status panel.
//!
//!   helm-s1-slint                desktop window + VR panel rendered offscreen at 30 Hz on the SAME wgpu device
//!                                (custom "hybrid" platform that wraps Slint's winit backend)
//!   helm-s1-slint --no-vr        desktop window only, via the public BackendSelector API
//!   helm-s1-slint --offscreen [N] [--png out.png] [--t SECONDS] [--software]
//!                                headless: FemtoVG-on-wgpu into a texture + readback (default),
//!                                or Slint's CPU software renderer straight into a pixel buffer
//!
//! Standalone demo: reads nothing, writes nothing (except --png), never touches the game.

slint::include_modules!();

use hud_model::{FsdState, OFFSCREEN_H, OFFSCREEN_W, Ptt, Snapshot, Speaker, stats_ms};
use slint::platform::femtovg_renderer::FemtoVGWGPURenderer;
use slint::platform::software_renderer::{MinimalSoftwareWindow, PremultipliedRgbaColor, RepaintBufferType};
use slint::platform::{Platform, WindowAdapter, WindowEvent};
use slint::{ComponentHandle, Model, PhysicalSize, VecModel};
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::time::Instant;

// ---------------------------------------------------------------- state -> UI

fn tog(t: hud_model::Toggle) -> Tog {
    Tog { on: t.on, pending: t.pending }
}

struct Bound {
    ui: Hud,
    log: Rc<VecModel<LogItem>>,
}

impl Bound {
    fn new(ui: Hud) -> Self {
        let log = Rc::new(VecModel::default());
        ui.set_log(slint::ModelRc::from(log.clone()));
        Self { ui, log }
    }
    fn apply(&self, s: &Snapshot) {
        let ui = &self.ui;
        ui.set_commander(s.commander.into());
        ui.set_ship_type(s.ship_type.into());
        ui.set_ship_sub(format!("{} · {}", s.ship_name, s.ship_ident).into());
        ui.set_system(s.system.into());
        ui.set_body(s.body.into());
        ui.set_clock(s.clock.clone().into());
        ui.set_gear(tog(s.gear));
        ui.set_lights(tog(s.lights));
        ui.set_hardpoints(tog(s.hardpoints));
        ui.set_scoop(tog(s.cargo_scoop));
        ui.set_night_vision(tog(s.night_vision));
        ui.set_fa(tog(s.flight_assist));
        ui.set_fsd_label(s.fsd.label().into());
        let (kind, frac) = match s.fsd {
            FsdState::Ready => (0, 1.0),
            FsdState::Charging | FsdState::Jumping => (1, s.fsd_progress),
            FsdState::Cooldown => (2, s.fsd_progress),
            FsdState::MassLocked => (3, 0.0),
        };
        ui.set_fsd_kind(kind);
        ui.set_fsd_frac(frac);
        ui.set_fuel(s.fuel_main / s.fuel_capacity);
        ui.set_fuel_text(format!("{:.1} / {:.0} t", s.fuel_main, s.fuel_capacity).into());
        ui.set_reservoir(s.fuel_reservoir);
        ui.set_shields(s.shields);
        ui.set_hull(s.hull);
        ui.set_heat(s.heat);
        ui.set_ptt(match s.ptt {
            Ptt::Idle => 0,
            Ptt::Listening => 1,
            Ptt::Thinking => 2,
        });
        ui.set_ptt_label(s.ptt.label().into());
        ui.set_mic_level(s.mic_level);
        ui.set_pulse(s.pulse);
        let have = self.log.row_count();
        if have < s.log.len() {
            for e in &s.log[have..] {
                self.log.push(LogItem {
                    time: e.time.clone().into(),
                    speaker: match e.speaker {
                        Speaker::Cmdr => 0,
                        Speaker::Copilot => 1,
                        Speaker::System => 2,
                    },
                    text: e.text.into(),
                });
            }
            ui.invoke_scroll_bottom();
        }
    }
}

// ------------------------------------------------- offscreen wgpu window adapter

/// A Slint window that only exists as a wgpu texture (the VR overlay panel).
struct WgpuOffscreen {
    window: slint::Window,
    renderer: FemtoVGWGPURenderer,
    size: Cell<PhysicalSize>,
}

impl WgpuOffscreen {
    fn new(instance: wgpu::Instance, device: wgpu::Device, queue: wgpu::Queue, w: u32, h: u32) -> Rc<Self> {
        let renderer = FemtoVGWGPURenderer::new(instance, device, queue).expect("femtovg wgpu renderer");
        Rc::new_cyclic(|weak: &std::rc::Weak<Self>| Self {
            window: slint::Window::new(weak.clone() as std::rc::Weak<dyn WindowAdapter>),
            renderer,
            size: Cell::new(PhysicalSize::new(w, h)),
        })
    }
}

impl WindowAdapter for WgpuOffscreen {
    fn window(&self) -> &slint::Window {
        &self.window
    }
    fn size(&self) -> PhysicalSize {
        self.size.get()
    }
    fn set_size(&self, size: slint::WindowSize) {
        self.size.set(size.to_physical(1.0));
    }
    fn renderer(&self) -> &dyn slint::platform::Renderer {
        &self.renderer
    }
}

/// GPU target + readback buffer for the VR panel.
struct VrTarget {
    device: wgpu::Device,
    queue: wgpu::Queue,
    texture: wgpu::Texture,
    readback: wgpu::Buffer,
}

impl VrTarget {
    fn new(device: wgpu::Device, queue: wgpu::Queue) -> Self {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("vr-overlay-panel"),
            size: wgpu::Extent3d { width: OFFSCREEN_W, height: OFFSCREEN_H, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::COPY_SRC
                | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        let readback = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("vr-readback"),
            size: (OFFSCREEN_W * OFFSCREEN_H * 4) as u64,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });
        Self { device, queue, texture, readback }
    }

    /// Returns (render ms, readback ms).
    fn render(&self, off: &WgpuOffscreen, out: &mut Vec<u8>) -> (f64, f64) {
        let t0 = Instant::now();
        off.renderer.render_to_texture(&self.texture).expect("render_to_texture");
        let t1 = Instant::now();
        let mut enc = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("readback") });
        enc.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfo {
                texture: &self.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::TexelCopyBufferInfo {
                buffer: &self.readback,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(OFFSCREEN_W * 4),
                    rows_per_image: Some(OFFSCREEN_H),
                },
            },
            wgpu::Extent3d { width: OFFSCREEN_W, height: OFFSCREEN_H, depth_or_array_layers: 1 },
        );
        self.queue.submit(Some(enc.finish()));
        let slice = self.readback.slice(..);
        slice.map_async(wgpu::MapMode::Read, |r| r.expect("map"));
        self.device.poll(wgpu::PollType::wait_indefinitely()).expect("poll");
        {
            let data = slice.get_mapped_range().expect("mapped");
            out.clear();
            out.extend_from_slice(&data);
        }
        self.readback.unmap();
        let t2 = Instant::now();
        let ms = |a: Instant, b: Instant| (b - a).as_secs_f64() * 1000.0;
        (ms(t0, t1), ms(t1, t2))
    }
}

fn init_window(w: &slint::Window) {
    w.dispatch_event(WindowEvent::ScaleFactorChanged { scale_factor: 1.0 });
    w.dispatch_event(WindowEvent::Resized { size: slint::LogicalSize::new(OFFSCREEN_W as f32, OFFSCREEN_H as f32) });
}

// ------------------------------------------------------------------ platforms

/// Offscreen-only platform: every window is the adapter we pre-seed.
struct OffscreenPlatform {
    next: RefCell<Option<Rc<dyn WindowAdapter>>>,
    start: Instant,
}

impl Platform for OffscreenPlatform {
    fn create_window_adapter(&self) -> Result<Rc<dyn WindowAdapter>, slint::PlatformError> {
        self.next.borrow_mut().take().ok_or_else(|| "no offscreen adapter seeded".into())
    }
    fn duration_since_start(&self) -> std::time::Duration {
        self.start.elapsed()
    }
}

/// Hybrid platform: the desktop window comes from Slint's own winit backend,
/// the VR panel from our offscreen adapter. Both render on the same wgpu device.
struct HybridPlatform {
    winit: i_slint_backend_winit::Backend,
    next_offscreen: RefCell<Option<Rc<dyn WindowAdapter>>>,
}

impl Platform for HybridPlatform {
    fn create_window_adapter(&self) -> Result<Rc<dyn WindowAdapter>, slint::PlatformError> {
        if let Some(a) = self.next_offscreen.borrow_mut().take() {
            return Ok(a);
        }
        self.winit.create_window_adapter()
    }
    fn run_event_loop(&self) -> Result<(), slint::PlatformError> {
        self.winit.run_event_loop()
    }
    fn process_events(
        &self,
        timeout: Option<std::time::Duration>,
        token: i_slint_core::InternalToken,
    ) -> Result<std::ops::ControlFlow<()>, slint::PlatformError> {
        self.winit.process_events(timeout, token)
    }
    fn bind_context(&self, ctx: i_slint_core::SlintContextWeak, token: i_slint_core::InternalToken) {
        self.winit.bind_context(ctx, token)
    }
    fn new_event_loop_proxy(&self) -> Option<Box<dyn slint::platform::EventLoopProxy>> {
        self.winit.new_event_loop_proxy()
    }
    fn duration_since_start(&self) -> std::time::Duration {
        self.winit.duration_since_start()
    }
    fn set_clipboard_text(&self, text: &str, c: slint::platform::Clipboard) {
        self.winit.set_clipboard_text(text, c)
    }
    fn clipboard_text(&self, c: slint::platform::Clipboard) -> Option<String> {
        self.winit.clipboard_text(c)
    }
}

fn make_wgpu() -> (wgpu::Instance, wgpu::Adapter, wgpu::Device, wgpu::Queue) {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle_from_env());
    let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions::default()))
        .expect("no wgpu adapter");
    let (device, queue) =
        pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor::default())).expect("device");
    (instance, adapter, device, queue)
}

// ------------------------------------------------------------------- modes

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
    let software = args.iter().any(|a| a == "--software");

    let platform = Rc::new(OffscreenPlatform { next: RefCell::new(None), start: Instant::now() });
    struct P(Rc<OffscreenPlatform>);
    impl Platform for P {
        fn create_window_adapter(&self) -> Result<Rc<dyn WindowAdapter>, slint::PlatformError> {
            self.0.create_window_adapter()
        }
        fn duration_since_start(&self) -> std::time::Duration {
            self.0.duration_since_start()
        }
    }
    slint::platform::set_platform(Box::new(P(platform.clone()))).unwrap();
    let t_init = Instant::now();
    let mut buf: Vec<u8> = Vec::new();
    let (mut tot, mut apply_ms, mut render_ms, mut rb_ms) = (vec![], vec![], vec![], vec![]);

    if software {
        println!("renderer: Slint software renderer (CPU) -> PremultipliedRgbaColor buffer");
        let win = MinimalSoftwareWindow::new(RepaintBufferType::NewBuffer);
        *platform.next.borrow_mut() = Some(win.clone());
        let b = Bound::new(Hud::new().unwrap());
        b.ui.show().unwrap();
        win.set_size(PhysicalSize::new(OFFSCREEN_W, OFFSCREEN_H));
        init_window(b.ui.window());
        let mut px = vec![PremultipliedRgbaColor::default(); (OFFSCREEN_W * OFFSCREEN_H) as usize];
        let mut frame = |t: f64, buf: &mut Vec<u8>| -> (f64, f64, f64) {
            let t0 = Instant::now();
            slint::platform::update_timers_and_animations();
            b.apply(&hud_model::snapshot(t));
            let t1 = Instant::now();
            win.draw_if_needed(|r| {
                r.render(&mut px, OFFSCREEN_W as usize);
            });
            let t2 = Instant::now();
            buf.clear();
            for p in &px {
                let a = p.alpha as u32;
                let un = |c: u8| if a == 0 { 0 } else { ((c as u32 * 255 + a / 2) / a).min(255) as u8 };
                buf.extend_from_slice(&[un(p.red), un(p.green), un(p.blue), p.alpha]);
            }
            let t3 = Instant::now();
            let ms = |a: Instant, b: Instant| (b - a).as_secs_f64() * 1000.0;
            (ms(t0, t1), ms(t1, t2), ms(t2, t3))
        };
        println!("init: {:.1} ms", t_init.elapsed().as_secs_f64() * 1000.0);
        let f0 = Instant::now();
        frame(0.0, &mut buf);
        println!("first frame: {:.2} ms", f0.elapsed().as_secs_f64() * 1000.0);
        for i in 0..frames {
            let (a, r, c) = frame(5.0 + i as f64 / 30.0, &mut buf);
            apply_ms.push(a);
            render_ms.push(r);
            rb_ms.push(c);
            tot.push(a + r + c);
        }
        println!("total/frame:        {}", stats_ms(&mut tot));
        println!("  apply props:      {}", stats_ms(&mut apply_ms));
        println!("  sw render:        {}", stats_ms(&mut render_ms));
        println!("  to RGBA8 bytes:   {}", stats_ms(&mut rb_ms));
        if let Some(path) = png {
            for k in 0..3 {
                frame(t_png + k as f64 * 0.001, &mut buf);
            }
            write_png(&path, &buf);
            println!("wrote {path}");
        }
        return;
    }

    let (instance, adapter, device, queue) = make_wgpu();
    let info = adapter.get_info();
    println!("renderer: Slint FemtoVG on wgpu 30 -> texture -> readback");
    println!("adapter: {} ({:?}, {:?})", info.name, info.backend, info.device_type);
    let off = WgpuOffscreen::new(instance, device.clone(), queue.clone(), OFFSCREEN_W, OFFSCREEN_H);
    *platform.next.borrow_mut() = Some(off.clone());
    let b = Bound::new(Hud::new().unwrap());
    b.ui.show().unwrap();
    init_window(b.ui.window());
    let target = VrTarget::new(device, queue);
    println!("init: {:.1} ms", t_init.elapsed().as_secs_f64() * 1000.0);
    let frame = |t: f64, buf: &mut Vec<u8>| -> (f64, f64, f64) {
        let t0 = Instant::now();
        slint::platform::update_timers_and_animations();
        b.apply(&hud_model::snapshot(t));
        let a = t0.elapsed().as_secs_f64() * 1000.0;
        let (r, c) = target.render(&off, buf);
        (a, r, c)
    };
    let f0 = Instant::now();
    frame(0.0, &mut buf);
    println!("first frame: {:.2} ms", f0.elapsed().as_secs_f64() * 1000.0);
    for i in 0..frames {
        let (a, r, c) = frame(5.0 + i as f64 / 30.0, &mut buf);
        apply_ms.push(a);
        render_ms.push(r);
        rb_ms.push(c);
        tot.push(a + r + c);
    }
    println!("total/frame:        {}", stats_ms(&mut tot));
    println!("  apply props:      {}", stats_ms(&mut apply_ms));
    println!("  render_to_texture:{}", stats_ms(&mut render_ms));
    println!("  gpu readback:     {}", stats_ms(&mut rb_ms));
    if let Some(path) = png {
        for k in 0..3 {
            frame(t_png + k as f64 * 0.001, &mut buf);
        }
        write_png(&path, &buf);
        println!("wrote {path}");
    }
}

fn desktop_simple() {
    slint::BackendSelector::new()
        .backend_name("winit".into())
        .require_wgpu_30(slint::wgpu_30::WGPUConfiguration::default())
        .select()
        .expect("select wgpu backend");
    let b = Rc::new(Bound::new(Hud::new().unwrap()));
    b.ui.set_footer_note(format!("Slint {} · VR texture off", "1.18").into());
    let start = Instant::now();
    let timer = slint::Timer::default();
    let bb = b.clone();
    timer.start(slint::TimerMode::Repeated, std::time::Duration::from_millis(33), move || {
        bb.apply(&hud_model::snapshot(start.elapsed().as_secs_f64()));
    });
    b.apply(&hud_model::snapshot(0.0));
    b.ui.run().unwrap();
}

fn desktop_hybrid() -> Result<(), String> {
    let (instance, adapter, device, queue) = make_wgpu();
    let winit = i_slint_backend_winit::Backend::builder()
        .with_renderer_name("femtovg-wgpu")
        .request_graphics_api(i_slint_core::graphics::RequestedGraphicsAPI::WGPU30(
            slint::wgpu_30::WGPUConfiguration::Manual {
                instance: instance.clone(),
                adapter,
                device: device.clone(),
                queue: queue.clone(),
            },
        ))
        .build()
        .map_err(|e| format!("winit backend: {e}"))?;
    let platform = HybridPlatform { winit, next_offscreen: RefCell::new(None) };
    let off = WgpuOffscreen::new(instance, device.clone(), queue.clone(), OFFSCREEN_W, OFFSCREEN_H);
    *platform.next_offscreen.borrow_mut() = Some(off.clone());
    slint::platform::set_platform(Box::new(platform)).map_err(|e| format!("{e:?}"))?;

    // VR panel instance (offscreen) first, then the desktop window (winit).
    let vr = Rc::new(Bound::new(Hud::new().map_err(|e| e.to_string())?));
    vr.ui.show().map_err(|e| e.to_string())?;
    init_window(vr.ui.window());
    let target = Rc::new(VrTarget::new(device, queue));

    let desk = Rc::new(Bound::new(Hud::new().map_err(|e| e.to_string())?));
    let start = Instant::now();
    let timer = slint::Timer::default();
    let (d2, vr2, off2, tg2) = (desk.clone(), vr.clone(), off.clone(), target.clone());
    let buf = RefCell::new(Vec::new());
    let ema = Cell::new(0.0f64);
    let frames = Cell::new(0u64);
    timer.start(slint::TimerMode::Repeated, std::time::Duration::from_millis(33), move || {
        let s = hud_model::snapshot(start.elapsed().as_secs_f64());
        d2.apply(&s);
        let t0 = Instant::now();
        vr2.apply(&s);
        tg2.render(&off2, &mut buf.borrow_mut());
        let ms = t0.elapsed().as_secs_f64() * 1000.0;
        ema.set(ema.get() * 0.9 + ms * 0.1);
        frames.set(frames.get() + 1);
        d2.ui.set_footer_note(
            format!(
                "Slint 1.18 · VR texture {}x{} on shared wgpu device · {:.1} ms/frame · {} frames",
                OFFSCREEN_W,
                OFFSCREEN_H,
                ema.get(),
                frames.get()
            )
            .into(),
        );
    });
    desk.apply(&hud_model::snapshot(0.0));
    desk.ui.run().map_err(|e| e.to_string())?;
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--offscreen") {
        return offscreen_main(&args);
    }
    if args.iter().any(|a| a == "--no-vr") {
        return desktop_simple();
    }
    if let Err(e) = desktop_hybrid() {
        eprintln!("hybrid platform failed ({e}); run with --no-vr");
        std::process::exit(1);
    }
}
