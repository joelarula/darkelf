use std::sync::Arc;

use tokio::sync::{Mutex, mpsc};

use crate::model::DeviceResponse;


pub enum DeviceMessage {
    DeviceResponse(DeviceResponse),
    DeviceName(String),
    DeviceStatus(bool),
}

pub struct Console {  
    pub channel: i32,
    pub display_range: i32,
    pub light: Light,
    pub x_y_interchange: bool,
    pub x_sign: Sign,
    pub y_sign: Sign,
    pub on: bool,
    pub mode: i32,
    pub device_connected: bool,
    pub device_name: Option<String>,
    pub device_state: Option<DeviceResponse>,
    incomming_channel: Arc<Mutex<mpsc::UnboundedReceiver<DeviceMessage>>>,
    command_sender: mpsc::UnboundedSender<DeviceCommand>,
}


impl Console {
        pub fn new(device_channel: Arc<Mutex<mpsc::UnboundedReceiver<DeviceMessage>>>,device_command: mpsc::UnboundedSender<DeviceCommand>) -> Self {
            Self {
                channel: 1,
                display_range: 50,
                light: Light::Mono,
                x_y_interchange: false,
                x_sign: Sign::Plus,
                y_sign: Sign::Plus,
                on: false,
                mode: 1,
                device_connected: false,
                device_name: None,
                device_state: None,
                incomming_channel: device_channel,
                command_sender: device_command,
            }
        }
    }

#[derive(PartialEq)]
pub enum Sign {
    Plus,
    Minus,
}

impl Default for Sign {
    fn default() -> Self {
        Sign::Plus
    }
}

#[derive(PartialEq)]
pub enum Light {
    Mono,
    RGB,
}

impl Default for Light {
    fn default() -> Self {
        Light::Mono
    }
}


#[derive(Clone, Debug)]
pub enum DeviceCommand {
    On(bool),
    SetSettings(crate::model::SettingsData),
}



impl eframe::App for Console {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

        if let Ok(mut rx) = self.incomming_channel.try_lock() {
            while let Ok(msg) = rx.try_recv() {
                match msg {
                    DeviceMessage::DeviceResponse(resp) => {
                        self.device_state = Some(resp);

                        self.on = self.device_state
                            .as_ref()
                            .and_then(|resp| resp.device_info.as_ref().map(|info| info.device_on))
                            .unwrap_or(false);
                     
                        self.display_range = self.device_state
                            .as_ref()
                            .map(|resp| resp.settings.values[1] as i32)
                            .unwrap_or(50);
                    }
                    DeviceMessage::DeviceName(name) => {
                        self.device_name = Some(name);
                    }
                    DeviceMessage::DeviceStatus(status) => {
                        self.device_connected = status;
                        log::info!("Device connection status changed: {}", status);
                    }
                }
            }
        }

        // Top panel (auto height)
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
               ui.horizontal(|ui| {
                let mut is_dmx = self.mode == 1;
                if ui.toggle_value(&mut is_dmx, "DMX").changed() && is_dmx {
                    self.mode = 1;
                }
                let mut is_random = self.mode == 2;
                if ui.toggle_value(&mut is_random, "Random").changed() && is_random {
                    self.mode = 2;
                }
                let mut is_animation = self.mode == 3;
                if ui.toggle_value(&mut is_animation, "Animation").changed() && is_animation {
                    self.mode = 3;
                }
                let mut is_line = self.mode == 4;
                if ui.toggle_value(&mut is_line, "Line").changed() && is_line {
                    self.mode = 4;
                }
                let mut is_christmas = self.mode == 5;
                if ui.toggle_value(&mut is_christmas, "Christmas").changed() && is_christmas {
                    self.mode = 5;
                }
                let mut is_outdoor = self.mode == 6;
                if ui.toggle_value(&mut is_outdoor, "Outdoor").changed() && is_outdoor {
                    self.mode = 6;
                }
            });
        });

        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let status_text = if self.device_connected {
                    "Status: Connected"
                } else {
                    "Status: Not Connected"
                };
                ui.label(status_text);
                // You can add more widgets here (progress bar, text, etc)
                ui.separator();
                if self.device_name.is_some() {
                    if let Some(ref name) = self.device_name {
                        ui.label(format!("Device: {}", name));
                    } else {
                        ui.label("Device: Unknown");
                    }
                } else {
                    ui.label("Other info");
                }
                 

                if self.device_connected {
                    if let Some(ref state) = self.device_state {
                        
                        let (device_type, version, user_type) = if let Some(ref info) = state.device_info {
                            (
                                info.device_type.clone(),
                                info.version.clone(),
                                info.user_type.clone(),
                            )
                        
                        } else {
                            ("".to_string(), "".to_string(), "".to_string())
                        };
                        ui.separator();
                        ui.label(format!("Type: {} | Version: {} | User: {}", device_type, version, user_type));
                    }
                }
            });
        });

        // Bottom panel (fixed height)
        egui::TopBottomPanel::bottom("bottom_panel")
            .exact_height(50.0)
            .show(ctx, |ui| {
                egui::Grid::new("settings_row")
                    .num_columns(9)
                    .show(ui, |ui| {

                        ui.allocate_ui_with_layout(
                            egui::Vec2::new(60.0, 0.0),
                            egui::Layout::centered_and_justified(egui::Direction::TopDown),
                            |ui| {
                                let toggle_label = if self.on { "ON" } else { "OFF" };
                                let mut on_ui = self.on;
                                let toggle_response = ui.toggle_value(&mut on_ui, toggle_label);
                                if toggle_response.changed() {
                                    if on_ui {
                                        let _ = self.command_sender.send(DeviceCommand::On(true));
                                    } else {
                                        let _ = self.command_sender.send(DeviceCommand::On(false));
                                    }
                                }
                            },
                        );
                        ui.add_sized([80.0, 0.0], egui::Label::new("Display Range:"));
                        let slider_response = ui.add_sized(
                            [200.0, 0.0],
                            egui::Slider::new(&mut self.display_range, 10..=100),
                        );
                        if slider_response.changed() {
                            // Send updated settings to device
                            if let Some(ref state) = self.device_state {
                                let mut new_settings = state.settings.clone();
                                if new_settings.values.len() > 1 {
                                    new_settings.values[1] = self.display_range as u16;
                                    let _ = self.command_sender.send(DeviceCommand::SetSettings(new_settings));
                                }
                            }
                        }

                        ui.allocate_ui_with_layout(
                            egui::Vec2::new(60.0, 0.0), // fixed width, flexible height
                            egui::Layout::centered_and_justified(egui::Direction::TopDown),
                            |ui| {
                                egui::ComboBox::from_label("X")
                                    .selected_text(match self.x_sign {
                                        Sign::Plus => "+",
                                        Sign::Minus => "-",
                                    })
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(&mut self.x_sign, Sign::Plus, "+");
                                        ui.selectable_value(&mut self.x_sign, Sign::Minus, "-");
                                    });
                            },
                        );

                        ui.allocate_ui_with_layout(
                            egui::Vec2::new(60.0, 0.0), // fixed width, flexible height
                            egui::Layout::centered_and_justified(egui::Direction::TopDown),
                            |ui| {
                                egui::ComboBox::from_label("Y")
                                    .selected_text(match self.y_sign {
                                        Sign::Plus => "+",
                                        Sign::Minus => "-",
                                    })
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(&mut self.y_sign, Sign::Plus, "+");
                                        ui.selectable_value(&mut self.y_sign, Sign::Minus, "-");
                                    });
                            },
                        );

                        ui.add_sized(
                            [50.0, 0.0],
                            egui::Checkbox::new(&mut self.x_y_interchange, "Intechange"),
                        );

                        // Center both radios vertically in the cell
                        // -- First Radio: Fixed width, vertically centered --
                        ui.allocate_ui_with_layout(
                            egui::Vec2::new(60.0, 0.0), // fixed width, flexible height
                            egui::Layout::centered_and_justified(egui::Direction::TopDown),
                            |ui| {
                                ui.radio_value(&mut self.light, Light::Mono, "Mono");
                            },
                        );

                        // -- Second Radio: Fixed width, vertically centered --
                        ui.allocate_ui_with_layout(
                            egui::Vec2::new(50.0, 0.0),
                            egui::Layout::centered_and_justified(egui::Direction::TopDown),
                            |ui| {
                                ui.radio_value(&mut self.light, Light::RGB, "RGB");
                            },
                        );

                        ui.add_sized([50.0, 0.0], egui::Label::new("Channel:"));
                        ui.add_sized(
                            [30.0, 0.0],
                            egui::DragValue::new(&mut self.channel).clamp_range(1..=512),
                        );

                    });
            });

        // Central panel (fills the remaining space)
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Hello, dark elf!");
        });
    }
}
