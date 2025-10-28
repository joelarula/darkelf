use crate::{model::PlaybackMode, ui::console::Console};
use eframe::egui::*;
use crate::model;

pub fn show_mode_buttons(console: &mut Console, ctx: &eframe::egui::Context) {
    TopBottomPanel::top("top_panel").show(ctx, | ui| {
        ui.horizontal(|ui| {
             mode_toggle(ui, console, PlaybackMode::Dmx, "DMX");
            mode_toggle(ui, console, PlaybackMode::RandomPlayback, "Random");
            mode_toggle(ui, console, PlaybackMode::AnimationPlayback, "Animation");
            mode_toggle(ui, console, PlaybackMode::LineGeometryPlayback, "Line");
            mode_toggle(ui, console, PlaybackMode::ChristmasPlayback, "Christmas");
            mode_toggle(ui, console, PlaybackMode::OutdoorPlayback, "Outdoor");
            mode_toggle(ui, console, PlaybackMode::TextPlayback, "Text");
            mode_toggle(ui, console, PlaybackMode::Program, "Program");
            mode_toggle(ui, console, PlaybackMode::Draw, "Draw");
        });
    });
}

fn handle_mode_change(console: &mut Console, mode: PlaybackMode) {
    console.set_playback(mode, None);
}

fn mode_toggle(ui: &mut egui::Ui, console: &mut Console, mode: PlaybackMode, label: &str) {
    let mut is_selected = console.mode == mode;
    if ui.toggle_value(&mut is_selected, label).changed() && is_selected {
        handle_mode_change(console, mode);
    }
}
