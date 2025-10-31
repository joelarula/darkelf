use crate::ui::console::Console;
use eframe::egui::*;
use eframe::egui::containers::TopBottomPanel;

pub fn show_status_bar(console: &Console, ctx: &eframe::egui::Context) {
    TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
        ui.horizontal(|ui| {
            let status_text = if console.device_connected {
                "Status: Connected"
            } else {
                "Status: Not Connected"
            };
            ui.label(status_text);
            ui.separator();
            if let Some(ref name) = console.device_name {
                ui.label(format!("Device: {}", name));
            } else {
                ui.label("Other info");
            }
            if console.device_connected {
                if let Some(ref state) = console.device_state {
                    let (device_type, version, user_type)  =
                    (
                        state.device_info.device_type.clone(),
                        state.device_info.version.clone(),
                        state.device_info.user_type.clone(),
                    );
          
                    ui.separator();
                    ui.label(format!("Type: {} | Version: {} | User: {}", device_type, version, user_type));
                }
            }
        });
    });
}
