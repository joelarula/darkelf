use eframe::egui::*;
use log::info;
use crate::ui::{self, app::{App, Beam, Sign}, model::{self, DeviceCommand}};

pub fn show_settings_panel(mut app: &mut App, ctx: &eframe::egui::Context) {

    TopBottomPanel::bottom("settings_panel")
        .exact_height(50.0)
        .show(ctx, |ui| {
            egui::Grid::new("settings_row")
                .num_columns(9)
                .show(ui, |ui| {

                    ui.allocate_ui_with_layout(
                        egui::Vec2::new(60.0, 0.0),
                        egui::Layout::centered_and_justified(egui::Direction::TopDown),
                        |ui| {
                            let toggle_label = if app.device_info.device_on { "ON" } else { "OFF" };
                            let mut on_ui = app.device_info.device_on;
                            ui.add_enabled_ui(app.ble_device_connected, |ui| {
                                let toggle_response = ui.toggle_value(&mut on_ui, toggle_label);
                                if toggle_response.changed() {
                                    if on_ui && app.ble_device_connected {
                                        let _ = app.command_sender.send(DeviceCommand::On(true));
                                    } else if !on_ui && app.ble_device_connected {
                                        let _ = app.command_sender.send(DeviceCommand::On(false));
                                    }
                                }
                            });
                        },
                    );

                    ui.add_sized([80.0, 0.0], egui::Label::new("Display Range:"));
                    let slider_response = ui.add_sized(
                        [200.0, 0.0],
                        egui::Slider::new(&mut app.settings.display_range, 10..=100),
                    );

                    if slider_response.drag_stopped() {
                        info!("Display Range changed to {}", app.settings.display_range);
                        if app.laser_device_initialized {
                                let new_settings = app.settings.clone();
                                let _ = app.command_sender.send(DeviceCommand::SetSettings(new_settings));
                            }
                        
                    }

                    let (mut x_y_interchange, mut x_sign, mut y_sign) = prepare_xy_map(app.settings.xy);

                    ui.allocate_ui_with_layout(
                        egui::Vec2::new(60.0, 0.0),
                        egui::Layout::centered_and_justified(egui::Direction::TopDown),
                        |ui| {
                            let combo_resp = egui::ComboBox::from_label("X")
                                .selected_text(match x_sign {
                                    Sign::Plus => "+",
                                    Sign::Minus => "-",
                                })
                                .show_ui(ui, |ui| {
                                    let plus_resp = ui.selectable_value(&mut x_sign, Sign::Plus, "+");
                                    let minus_resp = ui.selectable_value(&mut x_sign, Sign::Minus, "-");
                                    plus_resp | minus_resp
                                });

                            if let Some(resp) = combo_resp.inner {
                                if resp.changed() {
                                    send_xy_settings(app,x_y_interchange, x_sign, y_sign);
                                }
                            }
                        }
                    );

                    ui.allocate_ui_with_layout(
                        egui::Vec2::new(60.0, 0.0),
                        egui::Layout::centered_and_justified(egui::Direction::TopDown),
                        |ui| {
                            let combo_resp = egui::ComboBox::from_label("Y")
                                .selected_text(match y_sign {
                                    Sign::Plus => "+",
                                    Sign::Minus => "-",
                                })
                                .show_ui(ui, |ui| {
                                    let plus_resp = ui.selectable_value(&mut y_sign, Sign::Plus, "+");
                                    let minus_resp = ui.selectable_value(&mut y_sign, Sign::Minus, "-");
                                    plus_resp | minus_resp
                                });


                                if let Some(resp) = combo_resp.inner {
                                   if resp.changed() {
                                         send_xy_settings(app,x_y_interchange, x_sign, y_sign);
                                    }
                                }
                        }
                    );

                    let interchange_resp = ui.add_sized(
                        [50.0, 0.0],
                        egui::Checkbox::new(&mut x_y_interchange, "Intechange"),
                    );


                    let interchange_changed = interchange_resp.changed();
                    if interchange_changed {
                        send_xy_settings(app,x_y_interchange, x_sign, y_sign);
                    }


                    let mut beam = if app.settings.beams == 1 {
                        Beam::Mono
                    } else {
                        Beam::RGB
                    };

                    ui.allocate_ui_with_layout(
                        egui::Vec2::new(60.0, 0.0),
                        egui::Layout::centered_and_justified(egui::Direction::TopDown),
                        |ui| {
                            let mono_response = ui.radio_value(&mut beam, Beam::Mono, "Mono");
                            if mono_response.changed()  {
                                handle_beam_change(&beam, &mut app);
                            }
                        
                        },
                    );

                    ui.allocate_ui_with_layout(
                        egui::Vec2::new(50.0, 0.0),
                        egui::Layout::centered_and_justified(egui::Direction::TopDown),
                        |ui| {
                            let rgb_response = ui.radio_value(&mut beam, Beam::RGB, "RGB");                          
                            if rgb_response.changed()  {
                                handle_beam_change(&beam, &mut app);   
                            }
                        },
                    );
                    

                });
        });
}
    
fn handle_beam_change(beam: &Beam, app: &mut App){
    app.settings.beams = if *beam == Beam::RGB { 3 } else { 1 };
    info!("Beam mode changed to {}",app.settings.beams);
    let new_settings = app.settings.clone();
    let _ = app.command_sender.send(DeviceCommand::SetSettings(new_settings));
}

fn prepare_xy_map(xy: u8) -> (bool, Sign, Sign) {
        // Map: 0-3 normal, 4-7 interchange
        let x_y_interchange = xy >= 4;
        let idx = xy   % 4;
        // Order: 0: X+Y+, 1: X+Y-, 2: X-Y-, 3: X-Y+
        let x_sign = if idx == 0 || idx == 1 { Sign::Plus } else { Sign::Minus };
        let y_sign = if idx == 0 || idx == 3 { Sign::Plus } else { Sign::Minus };

       (x_y_interchange, x_sign, y_sign)
}

fn calc_xy_value(x_y_interchange: bool, x_sign: Sign, y_sign: Sign) -> u8 {
        // Map: 0-3 normal, 4-7 interchange
        let base = if x_y_interchange { 4 } else { 0 };
        // Order: 0: X+Y+, 1: X+Y-, 2: X-Y-, 3: X-Y+
        let idx = match (x_sign, y_sign) {
            (Sign::Plus, Sign::Plus) => 0,
            (Sign::Plus, Sign::Minus) => 1,
            (Sign::Minus, Sign::Minus) => 2,
            (Sign::Minus, Sign::Plus) => 3,
        };
        base + idx
}



pub fn send_xy_settings( app: &mut App,x_y_interchange: bool, x_sign: Sign, y_sign: Sign) {
                
    app.settings.xy = calc_xy_value(x_y_interchange, x_sign, y_sign);
    info!("Sending XY settings: {:?}",  app.settings.xy);
    let new_settings = app.settings.clone();
    let _ = app.command_sender.send(DeviceCommand::SetSettings(new_settings));
       
}