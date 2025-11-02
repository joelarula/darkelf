use crate::{model::DeviceMode, ui::{app::App, model::DeviceCommand}};
use eframe::egui::*;

pub fn show_mode_buttons(app: &mut App, ctx: &eframe::egui::Context) {
    TopBottomPanel::top("top_panel").show(ctx, | ui| {
        ui.horizontal(|ui| {
            mode_toggle(ui, app, DeviceMode::Dmx, "DMX");
            mode_toggle(ui, app, DeviceMode::RandomPlayback, "Random");
            mode_toggle(ui, app, DeviceMode::AnimationPlayback, "Animation");
            mode_toggle(ui, app, DeviceMode::LineGeometryPlayback, "Line");
            mode_toggle(ui, app, DeviceMode::ChristmasPlayback, "Christmas");
            mode_toggle(ui, app, DeviceMode::OutdoorPlayback, "Outdoor");
            mode_toggle(ui, app, DeviceMode::TextPlayback, "Text");
            mode_toggle(ui, app, DeviceMode::Program, "Program");
            mode_toggle(ui, app, DeviceMode::Draw, "Draw");
        });
    });
}

fn handle_mode_change(app: &mut App, mode: DeviceMode) {
 
    app.mode = mode;
    let mut command = app.command_data.clone();
    command.device_mode = mode;
    let _ = app.command_sender.send(DeviceCommand::SetMainCommand(command));

}

fn mode_toggle(ui: &mut egui::Ui, console: &mut App, mode: DeviceMode, label: &str) {
    let mut is_selected = console.mode == mode;
    if ui.toggle_value(&mut is_selected, label).changed() && is_selected {
        handle_mode_change(console, mode);
    }
}
