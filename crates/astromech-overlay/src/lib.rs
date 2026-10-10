//! # astromech-overlay
//!
//! egui panels rendered offscreen into the SteamVR overlay (ARCHITECTURE.md §5.16–5.17, S1):
//! kneeboard, push-to-talk indicator and the confirmation toast. This stub only has the
//! toast view; the OpenVR plumbing comes with the VR phase (feature "vr").

/// A pending "Did you mean …?" or tier-2 confirmation.
#[derive(Debug, Clone, PartialEq)]
pub struct Toast {
    pub text: String,
    pub seconds_left: f32,
}

/// Draw the confirmation toast.
pub fn toast(ui: &mut egui::Ui, t: &Toast) {
    ui.vertical_centered(|ui| {
        ui.heading(&t.text);
        ui.label(format!("Confirm or cancel ({:.0} s)", t.seconds_left.max(0.0)));
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn toast_renders_headless() {
        let ctx = egui::Context::default();
        let t = Toast { text: "Did you mean plot route to Sol?".into(), seconds_left: 5.0 };
        let mut out = ctx.run_ui(egui::RawInput::default(), |ui| toast(ui, &t));
        out.textures_delta.clear();
        assert!(!out.shapes.is_empty());
    }
}
