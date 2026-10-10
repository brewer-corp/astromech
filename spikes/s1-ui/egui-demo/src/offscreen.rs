//! Render the same HUD into an offscreen wgpu texture and read it back to an
//! RGBA8 buffer (what `openvr::Overlay::set_raw_data` would consume).
//! Works on *any* `wgpu::Device`, including the one eframe uses for the window.

use std::time::Instant;

pub const FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8Unorm;

pub struct Timings {
    pub ui_ms: f64,
    pub tess_ms: f64,
    pub gpu_readback_ms: f64,
    pub total_ms: f64,
}

pub struct OffscreenHud {
    device: wgpu::Device,
    queue: wgpu::Queue,
    pub ctx: egui::Context,
    renderer: egui_wgpu::Renderer,
    pub texture: wgpu::Texture,
    view: wgpu::TextureView,
    readback: wgpu::Buffer,
    w: u32,
    h: u32,
}

impl OffscreenHud {
    pub fn new(device: wgpu::Device, queue: wgpu::Queue, w: u32, h: u32) -> Self {
        let ctx = egui::Context::default();
        crate::hud::install_fonts(&ctx);
        let renderer = egui_wgpu::Renderer::new(&device, FORMAT, egui_wgpu::RendererOptions::default());
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("vr-overlay-panel"),
            size: wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: FORMAT,
            // TEXTURE_BINDING so a future zero-copy path could hand this texture
            // (or a shared copy of it) to OpenVR SetOverlayTexture.
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::COPY_SRC
                | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let readback = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("vr-readback"),
            size: (w * h * 4) as u64,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });
        Self { device, queue, ctx, renderer, texture, view, readback, w, h }
    }

    /// Render one frame at time `t` into `out` (RGBA8, w*h*4 bytes).
    pub fn render(&mut self, t: f64, out: &mut Vec<u8>) -> Timings {
        let t0 = Instant::now();
        let snap = hud_model::snapshot(t);
        let raw = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(self.w as f32, self.h as f32),
            )),
            time: Some(t),
            ..Default::default()
        };
        let full = self.ctx.run_ui(raw, |ui| crate::hud::draw(ui, &snap));
        let t1 = Instant::now();
        let ppp = full.pixels_per_point;
        let jobs = self.ctx.tessellate(full.shapes, ppp);
        let t2 = Instant::now();

        for (id, deltas) in &full.textures_delta.set {
            for delta in deltas.iter() {
                self.renderer.update_texture(&self.device, &self.queue, *id, delta);
            }
        }
        let sd = egui_wgpu::ScreenDescriptor { size_in_pixels: [self.w, self.h], pixels_per_point: ppp };
        let mut enc = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("hud") });
        let extra = self.renderer.update_buffers(&self.device, &self.queue, &mut enc, &jobs, &sd);
        {
            let pass = enc.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("hud-pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
            let mut pass = pass.forget_lifetime();
            self.renderer.render(&mut pass, &jobs, &sd);
        }
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
                    bytes_per_row: Some(self.w * 4), // 4096: already 256-aligned
                    rows_per_image: Some(self.h),
                },
            },
            wgpu::Extent3d { width: self.w, height: self.h, depth_or_array_layers: 1 },
        );
        self.queue.submit(extra.into_iter().chain(std::iter::once(enc.finish())));
        let slice = self.readback.slice(..);
        slice.map_async(wgpu::MapMode::Read, |r| r.expect("map"));
        self.device.poll(wgpu::PollType::wait_indefinitely()).expect("poll");
        {
            let data = slice.get_mapped_range().expect("mapped range");
            out.clear();
            out.extend_from_slice(&data);
        }
        self.readback.unmap();
        for id in &full.textures_delta.free {
            self.renderer.free_texture(id);
        }
        let t3 = Instant::now();
        let ms = |a: Instant, b: Instant| (b - a).as_secs_f64() * 1000.0;
        Timings { ui_ms: ms(t0, t1), tess_ms: ms(t1, t2), gpu_readback_ms: ms(t2, t3), total_ms: ms(t0, t3) }
    }
}

pub fn write_png(path: &str, w: u32, h: u32, rgba: &[u8]) {
    let f = std::fs::File::create(path).expect("create png");
    let mut e = png::Encoder::new(std::io::BufWriter::new(f), w, h);
    e.set_color(png::ColorType::Rgba);
    e.set_depth(png::BitDepth::Eight);
    e.write_header().unwrap().write_image_data(rgba).unwrap();
}
