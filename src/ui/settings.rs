use egui::*;
use crate::ui::console::{Console, Light, Sign};

pub fn show_settings_panel(console: &mut Console, ctx: &egui::Context) {
    TopBottomPanel::bottom("bottom_panel")
        .exact_height(50.0)
        .show(ctx, |ui| {
            egui::Grid::new("settings_row")
                .num_columns(9)
                .show(ui, |ui| {

                    ui.allocate_ui_with_layout(
                        egui::Vec2::new(60.0, 0.0),
                        egui::Layout::centered_and_justified(egui::Direction::TopDown),
                        |ui| {
                            let toggle_label = if console.on { "ON" } else { "OFF" };
                            let mut on_ui = console.on;
                            ui.set_enabled(console.device_connected);
                            let toggle_response = ui.toggle_value(&mut on_ui, toggle_label);
                            if toggle_response.changed() {
                                if on_ui && console.device_connected {
                                    let _ = console.command_sender.send(crate::ui::console::DeviceCommand::On(true));
                                } else if !on_ui && console.device_connected {
                                    let _ = console.command_sender.send(crate::ui::console::DeviceCommand::On(false));
                                }
                            }
                        },
                    );

                    ui.add_sized([80.0, 0.0], egui::Label::new("Display Range:"));
                    let slider_response = ui.add_sized(
                        [200.0, 0.0],
                        egui::Slider::new(&mut console.display_range, 10..=100),
                    );

                    if slider_response.changed() {
                        if let Some(ref state) = console.device_state {
                            let mut new_settings = state.settings.clone();
                            if new_settings.values.len() > 1 && console.device_connected {
                                new_settings.values[1] = console.display_range as u16;
                                let _ = console.command_sender.send(crate::ui::console::DeviceCommand::SetSettings(new_settings));
                            }
                        }
                    }
  
                    ui.allocate_ui_with_layout(
                        egui::Vec2::new(60.0, 0.0),
                        egui::Layout::centered_and_justified(egui::Direction::TopDown),
                        |ui| {
                            let combo_resp = egui::ComboBox::from_label("X")
                                .selected_text(match console.x_sign {
                                    Sign::Plus => "+",
                                    Sign::Minus => "-",
                                })
                                .show_ui(ui, |ui| {
                                    let plus_resp = ui.selectable_value(&mut console.x_sign, Sign::Plus, "+");
                                    let minus_resp = ui.selectable_value(&mut console.x_sign, Sign::Minus, "-");
                                    plus_resp | minus_resp
                                });

                            if let Some(resp) = combo_resp.inner {
                                if resp.changed() {
                                    console.send_xy_settings();
                                }
                            }
                        }
                    );

                    ui.allocate_ui_with_layout(
                        egui::Vec2::new(60.0, 0.0),
                        egui::Layout::centered_and_justified(egui::Direction::TopDown),
                        |ui| {
                            let combo_resp = egui::ComboBox::from_label("Y")
                                .selected_text(match console.y_sign {
                                    Sign::Plus => "+",
                                    Sign::Minus => "-",
                                })
                                .show_ui(ui, |ui| {
                                    let plus_resp = ui.selectable_value(&mut console.y_sign, Sign::Plus, "+");
                                    let minus_resp = ui.selectable_value(&mut console.y_sign, Sign::Minus, "-");
                                    plus_resp | minus_resp
                                });


                                if let Some(resp) = combo_resp.inner {
                                   if resp.changed() {
                                        console.send_xy_settings();
                                    }
                                }
                        }
                    );

                    let interchange_resp = ui.add_sized(
                        [50.0, 0.0],
                        egui::Checkbox::new(&mut console.x_y_interchange, "Intechange"),
                    );


                    let interchange_changed = interchange_resp.changed();
                    if interchange_changed {
                        console.send_xy_settings();
                    }

                    ui.allocate_ui_with_layout(
                        egui::Vec2::new(60.0, 0.0),
                        egui::Layout::centered_and_justified(egui::Direction::TopDown),
                        |ui| {
                            ui.radio_value(&mut console.light, Light::Mono, "Mono");
                        },
                    );

                    ui.allocate_ui_with_layout(
                        egui::Vec2::new(50.0, 0.0),
                        egui::Layout::centered_and_justified(egui::Direction::TopDown),
                        |ui| {
                            let mut light_ui = console.light;
                            let rgb_response = ui.radio_value(&mut light_ui, Light::RGB, "RGB");
                            if rgb_response.changed() && console.device_connected {
                                if let Some(ref state) = console.device_state {
                                    let mut new_settings = state.settings.clone();
                                    new_settings.light = if light_ui == Light::RGB { 3 } else { 1 };
                                    let _ = console.command_sender.send(crate::ui::console::DeviceCommand::SetSettings(new_settings));
                                }
                            }
                        },
                    );
                    
                    ui.add_sized([50.0, 0.0], egui::Label::new("Channel:"));
                    ui.add_sized(
                        [30.0, 0.0],
                        egui::DragValue::new(&mut console.channel).clamp_range(1..=512),
                    );
                });
        });
}
