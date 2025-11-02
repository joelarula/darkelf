use crate::ui::app::App;
use eframe::egui::containers::TopBottomPanel;

pub fn show_status_bar(app: &App, ctx: &eframe::egui::Context) {
    TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
        ui.horizontal(|ui| {
            let status_text = if app.ble_device_connected {
                "Status: Connected"
            } else {
                "Status: Not Connected"
            };
            ui.label(status_text);
            ui.separator();

            if (app.ble_device_connected) {
                let init_text = if app.laser_device_initialized {
                    "Initialized"
                } else {
                    "Not Initialized"
                };

                ui.label(init_text);
                ui.separator();

                if let Some(ref device_info) = app.device_list.selected_device() {
                    ui.label(format!("Device: {:?}", device_info.Name()));
                }
                ui.separator();
                if app.laser_device_initialized {
                    ui.separator();
                    ui.label(format!(
                        "Type: {} | Version: {} | User: {}",
                        app.device_info.device_type,
                        app.device_info.version,
                        app.device_info.user_type
                    ));
                }
            }
        });
    });
}
