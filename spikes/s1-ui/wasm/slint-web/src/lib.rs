//! The SAME .slint HUD (slint-demo/ui/hud.slint), compiled to wasm (winit web backend, FemtoVG/WebGL).
slint::include_modules!();
use slint::{Model, VecModel};
use std::rc::Rc;
use wasm_bindgen::prelude::*;

fn now() -> f64 {
    web_time::SystemTime::now().duration_since(web_time::UNIX_EPOCH).unwrap().as_secs_f64()
}

#[wasm_bindgen(start)]
pub fn start() {
    let ui = Hud::new().unwrap();
    let log = Rc::new(VecModel::<LogItem>::default());
    ui.set_log(slint::ModelRc::from(log.clone()));
    let start = now();
    let weak = ui.as_weak();
    let timer = slint::Timer::default();
    timer.start(slint::TimerMode::Repeated, std::time::Duration::from_millis(33), move || {
        let Some(ui) = weak.upgrade() else { return };
        let s = hud_model::snapshot(now() - start);
        ui.set_clock(s.clock.clone().into());
        ui.set_gear(Tog { on: s.gear.on, pending: s.gear.pending });
        ui.set_shields(s.shields);
        ui.set_pulse(s.pulse);
        // (abbreviated binding: enough to prove the .slint file + model compile and run on wasm)
        while log.row_count() < s.log.len() {
            let e = &s.log[log.row_count()];
            log.push(LogItem { time: e.time.clone().into(), speaker: 0, text: e.text.into() });
        }
    });
    std::mem::forget(timer);
    ui.show().unwrap();
    std::mem::forget(ui);
}
