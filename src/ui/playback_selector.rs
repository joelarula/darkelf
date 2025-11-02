use crate::model::PlaybackMode;
use crate::ui::app;
/// Shows a grid of 10x5 on/off buttons labeled 1-50.
use crate::ui::{app::App, model::DeviceCommand};
use crate::command::CommandGenerator;
use egui::{Grid, Button};
use log::info;

pub fn show_selector_grid(ui: &mut egui::Ui, mut app: &mut App) {
    let mode_key = app.mode as u8;

    ui.horizontal(|ui| {
        if let Some(item) = app.command_data.playback.playback_items.get(&mode_key) {
    
            ui.label("Mode:");
            let py_mode_val = item.playback_mode;
            let loop_selected = py_mode_val == PlaybackMode::LoopPlay;
            let tick_selected = py_mode_val == PlaybackMode::TickPlay;
            if ui.radio(loop_selected, "Loop mode").clicked() {
                             
                  let mut command = app.command_data.clone();
                  if let Some(item) = command.playback.playback_items.get_mut(&mode_key) {
                    item.playback_mode = PlaybackMode::LoopPlay;
                    info!("Playback mode changed: Loop");
                    let _ = app.command_sender.send(DeviceCommand::SetMainCommand(command));
                  }
            }

            if ui.radio(tick_selected, "Tick Play").clicked() {
                  let mut command = app.command_data.clone();
                  if let Some(item) = command.playback.playback_items.get_mut(&mode_key) {
                    item.playback_mode = PlaybackMode::TickPlay;
                    info!("Playback mode changed: Tick Play");
                    let _ = app.command_sender.send(DeviceCommand::SetMainCommand(command));
                  }
            }

            let mut bits = CommandGenerator::unpack_project_item_bits(item);
            let buttons_enabled = item.playback_mode != PlaybackMode::LoopPlay;
            if ui.button("Select All").clicked() && buttons_enabled {
                for v in bits.iter_mut() { *v = 1; }

                  let mut command = app.command_data.clone();
                  if let Some(item) = command.playback.playback_items.get_mut(&mode_key) {
                    item.selected_plays = CommandGenerator::pack_bits_to_prj_selected(&bits);
	                let _ = app.command_sender.send(DeviceCommand::SetMainCommand(command));
                  }


            }
            if ui.button("Clear All").clicked() && buttons_enabled {
                for v in bits.iter_mut() { *v = 0; }

                let mut command = app.command_data.clone();
                if let Some(item) = command.playback.playback_items.get_mut(&mode_key) {
                    item.selected_plays = CommandGenerator::pack_bits_to_prj_selected(&bits);
	                let _ = app.command_sender.send(DeviceCommand::SetMainCommand(command));
                }
            }
            if ui.button("Invert").clicked() && buttons_enabled {
                for v in bits.iter_mut() { *v = if *v == 0 { 1 } else { 0 }; }

                let mut command = app.command_data.clone();
                  if let Some(item) = command.playback.playback_items.get_mut(&mode_key) {
                    item.selected_plays = CommandGenerator::pack_bits_to_prj_selected(&bits);
	                let _ = app.command_sender.send(DeviceCommand::SetMainCommand(command));
                }
            }
        }



        
    });

    Grid::new("device_grid")
        .num_columns(10)
        .striped(true)
        .show(ui, |ui| {
            let mut btn_count = 0;

            if let Some(item) = app.command_data.playback.playback_items.get(&mode_key) {
                let mut bits = CommandGenerator::unpack_project_item_bits(item);
                let buttons_enabled = item.playback_mode != PlaybackMode::LoopPlay ;
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
                            
                            let mut command = app.command_data.clone();
                            if let Some(item) = command.playback.playback_items.get_mut(&mode_key) {
                                item.selected_plays = CommandGenerator::pack_bits_to_prj_selected(&bits);
                                item.selected_play = (idx + 1) as u16;
	                            let _ = app.command_sender.send(DeviceCommand::SetMainCommand(command));
                            }
                            
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


