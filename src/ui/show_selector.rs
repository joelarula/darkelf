/// Shows a grid of 10x5 on/off buttons labeled 1-50.
use crate::ui::console::Console;
use egui::{Grid, Button};

pub fn show_selector_grid(ui: &mut egui::Ui, console: &mut Console) {
    let mode_key = console.mode as u8;
    ui.horizontal(|ui| {
        if ui.button("Select All").clicked() {
            if let Some(vec) = console.playback_selections.get_mut(&mode_key) {
                for v in vec.iter_mut() { *v = 1; }
            }
        }
        if ui.button("Clear All").clicked() {
            if let Some(vec) = console.playback_selections.get_mut(&mode_key) {
                for v in vec.iter_mut() { *v = 0; }
            }
        }
        if ui.button("Invert").clicked() {
            if let Some(vec) = console.playback_selections.get_mut(&mode_key) {
                for v in vec.iter_mut() { *v = if *v == 0 { 1 } else { 0 }; }
            }
        }
    });

    Grid::new("device_grid")
        .num_columns(10)
        .striped(true)
        .show(ui, |ui| {
            let mut btn_count = 0;
            for _row in 0..5 {
                for _col in 0..10 {
                    let idx = btn_count;
                    if btn_count >= 50 {
                        break;
                    }
                    let is_on = if let Some(vec) = console.playback_selections.get(&mode_key) {
                        vec[idx] != 0
                    } else {
                        false
                    };
                    let label = format!("{}", idx + 1);
                    let button = Button::new(label)
                        .min_size(egui::Vec2::splat(50.0))
                        .fill(if is_on { egui::Color32::LIGHT_GREEN } else { egui::Color32::GRAY });
                    if ui.add(button).clicked() {
                        if let Some(vec) = console.playback_selections.get_mut(&mode_key) {
                            vec[idx] = if is_on { 0 } else { 1 };
                        }
                    }
                    btn_count += 1;
                }
                ui.end_row();
                if btn_count >= 50 {
                    break;
                }
            }
        });
}