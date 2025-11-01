use crate::{model::DeviceMode, ui::app::App};
use eframe::egui::*;
use crate::model;

pub fn show_mode_buttons(console: &mut App, ctx: &eframe::egui::Context) {
    TopBottomPanel::top("top_panel").show(ctx, | ui| {
        ui.horizontal(|ui| {
             mode_toggle(ui, console, DeviceMode::Dmx, "DMX");
            mode_toggle(ui, console, DeviceMode::RandomPlayback, "Random");
            mode_toggle(ui, console, DeviceMode::AnimationPlayback, "Animation");
            mode_toggle(ui, console, DeviceMode::LineGeometryPlayback, "Line");
            mode_toggle(ui, console, DeviceMode::ChristmasPlayback, "Christmas");
            mode_toggle(ui, console, DeviceMode::OutdoorPlayback, "Outdoor");
            mode_toggle(ui, console, DeviceMode::TextPlayback, "Text");
            mode_toggle(ui, console, DeviceMode::Program, "Program");
            mode_toggle(ui, console, DeviceMode::Draw, "Draw");
        });
    });
}

fn handle_mode_change(console: &mut App, mode: DeviceMode) {
    console.set_playback(mode, None);
}

fn mode_toggle(ui: &mut egui::Ui, console: &mut App, mode: DeviceMode, label: &str) {
    let mut is_selected = console.mode == mode;
    if ui.toggle_value(&mut is_selected, label).changed() && is_selected {
        handle_mode_change(console, mode);
    }
}
