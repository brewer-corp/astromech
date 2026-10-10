//! The SAME egui HUD code (egui-demo/src/hud.rs), compiled to wasm for the web dashboard.
#[path = "../../../egui-demo/src/hud.rs"]
mod hud;

use wasm_bindgen::prelude::*;

struct App {
    start: f64,
}

fn now() -> f64 {
    web_time::SystemTime::now().duration_since(web_time::UNIX_EPOCH).unwrap().as_secs_f64()
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let snap = hud_model::snapshot(now() - self.start);
        hud::draw(ui, &snap);
        ui.ctx().request_repaint_after(std::time::Duration::from_millis(33));
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    use wasm_bindgen::JsCast;
    let canvas = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("hud")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();
    wasm_bindgen_futures::spawn_local(async move {
        eframe::WebRunner::new()
            .start(
                canvas,
                eframe::WebOptions::default(),
                Box::new(|cc| {
                    hud::install_fonts(&cc.egui_ctx);
                    Ok(Box::new(App { start: now() }))
                }),
            )
            .await
            .expect("start eframe");
    });
}
