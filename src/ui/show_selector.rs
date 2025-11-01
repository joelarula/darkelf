/// Shows a grid of 10x5 on/off buttons labeled 1-50.
use crate::ui::app::App;
use crate::command::CommandGenerator;
use egui::{Grid, Button};

pub fn show_selector_grid(ui: &mut egui::Ui, console: &mut App) {
    let mode_key = console.mode as u8;
    ui.horizontal(|ui| {
        if let Some(item) = console.playback_selections.get_mut(&mode_key) {
            // Radio selector for py_mode
            ui.label("Mode:");
            let mut py_mode_val = item.playback_mode;
            let loop_selected = py_mode_val == 0;
            let tick_selected = py_mode_val == 128;
            if ui.radio(loop_selected, "Loop mode").clicked() {
                item.playback_mode = 0;
            }
            if ui.radio(tick_selected, "Tick Play").clicked() {
                item.playback_mode = 128;
            }

            let mut bits = CommandGenerator::unpack_project_item_bits(item);
            let buttons_enabled = item.playback_mode != 0;
            if ui.button("Select All").clicked() && buttons_enabled {
                for v in bits.iter_mut() { *v = 1; }
                item.selected_plays = CommandGenerator::pack_bits_to_prj_selected(&bits);
            }
            if ui.button("Clear All").clicked() && buttons_enabled {
                for v in bits.iter_mut() { *v = 0; }
                item.selected_plays = CommandGenerator::pack_bits_to_prj_selected(&bits);
            }
            if ui.button("Invert").clicked() && buttons_enabled {
                for v in bits.iter_mut() { *v = if *v == 0 { 1 } else { 0 }; }
                item.selected_plays = CommandGenerator::pack_bits_to_prj_selected(&bits);
            }
        }
    });

    Grid::new("device_grid")
        .num_columns(10)
        .striped(true)
        .show(ui, |ui| {
            let mut btn_count = 0;
            if let Some(item) = console.playback_selections.get_mut(&mode_key) {
                let mut bits = CommandGenerator::unpack_project_item_bits(item);
                let buttons_enabled = item.playback_mode != 0;
                for _row in 0..5 {
                    for _col in 0..10 {
                        let idx = btn_count;
                        if btn_count >= 50 {
                            break;
                        }
                        let is_on = bits.get(idx).copied().unwrap_or(0) != 0;
                        let label = format!("{}", idx + 1);
                        let button = Button::new(label)
                            .min_size(egui::Vec2::splat(50.0))
                            .fill(if is_on { egui::Color32::LIGHT_GREEN } else { egui::Color32::GRAY });
                        let response = ui.add(button);
                        if buttons_enabled && response.clicked() {
                            bits[idx] = if is_on { 0 } else { 1 };
                            item.selected_plays = CommandGenerator::pack_bits_to_prj_selected(&bits);
                        }
                        if !buttons_enabled {
                            response.surrender_focus();
                        }
                        btn_count += 1;
                    }
                    ui.end_row();
                    if btn_count >= 50 {
                        break;
                    }
                }
            }
        });
}