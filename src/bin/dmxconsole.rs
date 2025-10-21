
use eframe::{egui};
use std::sync::{Arc, Mutex};
use darkelf::dmx::device::DmxLaserDevice;
use darkelf::dmx::device::DmxLaserState;


pub struct DmxApp {
    pub device: Option<Arc<DmxLaserDevice>>,
    pub state: Arc<Mutex<DmxLaserState>>,
}

fn main() {
    let app = DmxApp::default();
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([300.0, 500.0])
            .with_min_inner_size([300.0, 500.0]),
        ..Default::default()
    };
    let _ = eframe::run_native(
        "DMX Laser Device Controller",
        native_options,
        Box::new(|_cc| Box::new(app)),
    );
}

impl Default for DmxApp {
    fn default() -> Self {
        darkelf::util::setup_logging();
        unsafe {
            std::env::set_var("RUST_LOG", "debug");
        }
  
        log::info!("=== DMX Laser Device Setup ===");
        let dmx_ports = darkelf::dmx::controller::scan_dmx_ports();
        let port = "COM4";
        let dmx_channel = 1;
        let device = if dmx_ports.contains(&port.to_string()) {
            match DmxLaserDevice::new(port, dmx_channel) {
                Ok(dev) => {
                    if dev.start().is_ok() {
                        log::info!("Created and started DMX laser device on {} channel {}", port, dmx_channel);
                        Some(Arc::new(dev))
                    } else {
                        log::warn!("Failed to start DMX device");
                        None
                    }
                }
                Err(e) => {
                    log::warn!("Failed to create DMX device: {}", e);
                    None
                }
            }
        } else {
            log::warn!("{} not found in available DMX ports: {:?}", port, dmx_ports);
            None
        };
        let state = Arc::new(Mutex::new(DmxLaserState::default()));
        DmxApp {
            device,
            state,
        }
    }
}

impl eframe::App for DmxApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut state = self.state.lock().unwrap();
            ui.heading("DMX Laser State");

            self.dmx_field(ui, "Master Dimmer:", &mut state.master_dimmer, &self.device, "master_dimmer");
            self.dmx_field(ui, "Color Control:", &mut state.color_control, &self.device, "color_control");
            self.dmx_field(ui, "Color Speed:", &mut state.color_speed, &self.device, "color_speed");
            self.dmx_field(ui, "Pattern Group:", &mut state.pattern_group, &self.device, "pattern_group");
            self.dmx_field(ui, "Pattern Select:", &mut state.pattern_select, &self.device, "pattern_select");
            self.dmx_field(ui, "Dynamic Effects:", &mut state.dynamic_effects, &self.device, "dynamic_effects");
            self.dmx_field(ui, "Effect Speed:", &mut state.effect_speed, &self.device, "effect_speed");
            self.dmx_field(ui, "Pattern Size:", &mut state.pattern_size, &self.device, "pattern_size");
            self.dmx_field(ui, "Size Control:", &mut state.size_control, &self.device, "size_control");
            self.dmx_field(ui, "Rotation:", &mut state.rotation, &self.device, "rotation");
            self.dmx_field(ui, "Vertical Flip:", &mut state.vertical_flip, &self.device, "vertical_flip");
            self.dmx_field(ui, "Horizontal Flip:", &mut state.horizontal_flip, &self.device, "horizontal_flip");
            self.dmx_field(ui, "Horizontal Pos:", &mut state.horizontal_pos, &self.device, "horizontal_pos");
            self.dmx_field(ui, "Vertical Pos:", &mut state.vertical_pos, &self.device, "vertical_pos");
            self.dmx_field(ui, "Wave Effect:", &mut state.wave_effect, &self.device, "wave_effect");
            self.dmx_field(ui, "Manual Drawing:", &mut state.manual_drawing, &self.device, "manual_drawing");
        });
    }
}

impl DmxApp {
    fn dmx_field(&self, ui: &mut egui::Ui, label: &str, value: &mut u8, device: &Option<Arc<DmxLaserDevice>>, field_name: &str) {
    let mut value_str = value.to_string();
        ui.horizontal(|ui| {
            ui.label(label);
            let text_response = ui.add_sized(
                [40.0, 0.0],
                egui::TextEdit::singleline(&mut value_str)
                    .char_limit(3)
            );
            let slider_response = ui.add_sized(
                [120.0, 0.0],
                egui::Slider::new(value, 0..=255).step_by(1.0),
            );

            // If slider changed, update text field string
            if slider_response.changed() {
                value_str = value.to_string();
                if let Some(device) = device {
                    let mut dev_state = device.get_current_state();
                    match field_name {
                        "master_dimmer" => dev_state.master_dimmer = *value,
                        "color_control" => dev_state.color_control = *value,
                        "color_speed" => dev_state.color_speed = *value,
                        "pattern_group" => dev_state.pattern_group = *value,
                        "pattern_select" => dev_state.pattern_select = *value,
                        "dynamic_effects" => dev_state.dynamic_effects = *value,
                        "effect_speed" => dev_state.effect_speed = *value,
                        "pattern_size" => dev_state.pattern_size = *value,
                        "size_control" => dev_state.size_control = *value,
                        "rotation" => dev_state.rotation = *value,
                        "vertical_flip" => dev_state.vertical_flip = *value,
                        "horizontal_flip" => dev_state.horizontal_flip = *value,
                        "horizontal_pos" => dev_state.horizontal_pos = *value,
                        "vertical_pos" => dev_state.vertical_pos = *value,
                        "wave_effect" => dev_state.wave_effect = *value,
                        "manual_drawing" => dev_state.manual_drawing = *value,
                        _ => {},
                    }
                    let channel = match field_name {
                        "master_dimmer" => 1,
                        "color_control" => 2,
                        "color_speed" => 3,
                        "pattern_group" => 4,
                        "pattern_select" => 5,
                        "dynamic_effects" => 6,
                        "effect_speed" => 7,
                        "pattern_size" => 8,
                        "size_control" => 9,
                        "rotation" => 10,
                        "vertical_flip" => 11,
                        "horizontal_flip" => 12,
                        "horizontal_pos" => 13,
                        "vertical_pos" => 14,
                        "wave_effect" => 15,
                        "manual_drawing" => 16,
                        _ => 0,
                    };
                    if channel > 0 {
                        let _ = device.set_dmx_channel(channel, *value);
                    }
                }
            }

            // If text field changed, update value and slider
            if text_response.changed() {
                if let Ok(mut val) = value_str.parse::<i32>() {
                    val = val.clamp(0, 255);
                    *value = val as u8;
                    value_str = value.to_string();
                    if let Some(device) = device {
                        let mut dev_state = device.get_current_state();
                        match field_name {
                            "master_dimmer" => dev_state.master_dimmer = *value,
                            "color_control" => dev_state.color_control = *value,
                            "color_speed" => dev_state.color_speed = *value,
                            "pattern_group" => dev_state.pattern_group = *value,
                            "pattern_select" => dev_state.pattern_select = *value,
                            "dynamic_effects" => dev_state.dynamic_effects = *value,
                            "effect_speed" => dev_state.effect_speed = *value,
                            "pattern_size" => dev_state.pattern_size = *value,
                            "size_control" => dev_state.size_control = *value,
                            "rotation" => dev_state.rotation = *value,
                            "vertical_flip" => dev_state.vertical_flip = *value,
                            "horizontal_flip" => dev_state.horizontal_flip = *value,
                            "horizontal_pos" => dev_state.horizontal_pos = *value,
                            "vertical_pos" => dev_state.vertical_pos = *value,
                            "wave_effect" => dev_state.wave_effect = *value,
                            "manual_drawing" => dev_state.manual_drawing = *value,
                            _ => {},
                        }
                        let channel = match field_name {
                            "master_dimmer" => 1,
                            "color_control" => 2,
                            "color_speed" => 3,
                            "pattern_group" => 4,
                            "pattern_select" => 5,
                            "dynamic_effects" => 6,
                            "effect_speed" => 7,
                            "pattern_size" => 8,
                            "size_control" => 9,
                            "rotation" => 10,
                            "vertical_flip" => 11,
                            "horizontal_flip" => 12,
                            "horizontal_pos" => 13,
                            "vertical_pos" => 14,
                            "wave_effect" => 15,
                            "manual_drawing" => 16,
                            _ => 0,
                        };
                        if channel > 0 {
                            let _ = device.set_dmx_channel(channel, *value);
                        }
                    }
                }
            }
        });
    }
}