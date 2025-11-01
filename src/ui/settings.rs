use eframe::egui::*;
use crate::ui::{self, app::{App, Light, Sign}, model::{self, DeviceCommand}};

pub fn show_settings_panel(app: &mut App, ctx: &eframe::egui::Context) {

    let settings = &app.device_state.as_ref().unwrap().settings;

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
                            let toggle_label = if app.on { "ON" } else { "OFF" };
                            let mut on_ui = app.on;
                            ui.set_enabled(app.ble_device_connected);
                            let toggle_response = ui.toggle_value(&mut on_ui, toggle_label);
                            if toggle_response.changed() {
                                if on_ui && app.ble_device_connected {
                                    let _ = app.command_sender.send(DeviceCommand::On(true));
                                } else if !on_ui && app.ble_device_connected {
                                    let _ = app.command_sender.send(DeviceCommand::On(false));
                                }
                            }
                        },
                    );

                    ui.add_sized([80.0, 0.0], egui::Label::new("Display Range:"));
                    let slider_response = ui.add_sized(
                        [200.0, 0.0],
                        egui::Slider::new(&mut app.display_range, 10..=100),
                    );

                    if slider_response.changed() {
                        if let Some(ref state) = app.device_state {
                            let mut new_settings = state.settings.clone();
                            if app.ble_device_connected {
                                new_settings.display_range = app.display_range as u8;
                                let _ = app.command_sender.send(DeviceCommand::SetSettings(new_settings));
                            }
                        }
                    }
  
                    ui.allocate_ui_with_layout(
                        egui::Vec2::new(60.0, 0.0),
                        egui::Layout::centered_and_justified(egui::Direction::TopDown),
                        |ui| {
                            let combo_resp = egui::ComboBox::from_label("X")
                                .selected_text(match app.x_sign {
                                    Sign::Plus => "+",
                                    Sign::Minus => "-",
                                })
                                .show_ui(ui, |ui| {
                                    let plus_resp = ui.selectable_value(&mut app.x_sign, Sign::Plus, "+");
                                    let minus_resp = ui.selectable_value(&mut app.x_sign, Sign::Minus, "-");
                                    plus_resp | minus_resp
                                });

                            if let Some(resp) = combo_resp.inner {
                                if resp.changed() {
                                    app.send_xy_settings();
                                }
                            }
                        }
                    );

                    ui.allocate_ui_with_layout(
                        egui::Vec2::new(60.0, 0.0),
                        egui::Layout::centered_and_justified(egui::Direction::TopDown),
                        |ui| {
                            let combo_resp = egui::ComboBox::from_label("Y")
                                .selected_text(match app.y_sign {
                                    Sign::Plus => "+",
                                    Sign::Minus => "-",
                                })
                                .show_ui(ui, |ui| {
                                    let plus_resp = ui.selectable_value(&mut app.y_sign, Sign::Plus, "+");
                                    let minus_resp = ui.selectable_value(&mut app.y_sign, Sign::Minus, "-");
                                    plus_resp | minus_resp
                                });


                                if let Some(resp) = combo_resp.inner {
                                   if resp.changed() {
                                        app.send_xy_settings();
                                    }
                                }
                        }
                    );

                    let interchange_resp = ui.add_sized(
                        [50.0, 0.0],
                        egui::Checkbox::new(&mut app.x_y_interchange, "Intechange"),
                    );


                    let interchange_changed = interchange_resp.changed();
                    if interchange_changed {
                        app.send_xy_settings();
                    }

                    ui.allocate_ui_with_layout(
                        egui::Vec2::new(60.0, 0.0),
                        egui::Layout::centered_and_justified(egui::Direction::TopDown),
                        |ui| {
                            ui.radio_value(&mut app.light, Light::Mono, "Mono");
                        },
                    );

                    ui.allocate_ui_with_layout(
                        egui::Vec2::new(50.0, 0.0),
                        egui::Layout::centered_and_justified(egui::Direction::TopDown),
                        |ui| {
                            let mut light_ui = app.light;
                            let rgb_response = ui.radio_value(&mut light_ui, Light::RGB, "RGB");
                            if rgb_response.changed() && app.ble_device_connected {
                                if let Some(ref state) = app.device_state {
                                    let mut new_settings = state.settings.clone();
                                    new_settings.beams = if light_ui == Light::RGB { 3 } else { 1 };
                                    let _ = app.command_sender.send(DeviceCommand::SetSettings(new_settings));
                                }
                            }
                        },
                    );
                    
                    //ui.add_sized([50.0, 0.0], egui::Label::new("Channel:"));
                    //ui.add_sized(
                    //    [30.0, 0.0],
                    //    egui::DragValue::new(&mut console.channel).clamp_range(1..=512),
                    //);
                });
        });
}
