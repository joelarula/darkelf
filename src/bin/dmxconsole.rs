
use eframe::{egui};
use std::sync::{Arc, Mutex};
use darkelf::dmx::device::DmxLaserDevice;
use darkelf::dmx::device::DmxLaserState;
use darkelf::dmx::laser_light_8340::{DMX_CHANNELS, DmxChannel,DmxChannelInfo};


pub struct DmxApp {
    pub device: Option<Arc<DmxLaserDevice>>,
    pub state: Arc<Mutex<DmxLaserState>>,
}

fn main() {
    let app = DmxApp::default();
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([300.0, 600.0])
            .with_min_inner_size([300.0, 600.0]),
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
            for channel_enum in DMX_CHANNELS {
                let value = match channel_enum {
                    DmxChannel::Dimmer { .. } => &mut state.master_dimmer,
                    DmxChannel::ColorControl { .. } => &mut state.color_control,
                    DmxChannel::ColorChangeSpeed { .. } => &mut state.color_speed,
                    DmxChannel::PatternSelection { .. } => &mut state.pattern_select,
                    DmxChannel::EffectSpeed { .. } => &mut state.effect_speed,
                    DmxChannel::PatternSize { .. } => &mut state.pattern_size,
                    DmxChannel::SizeControl { .. } => &mut state.size_control,
                    DmxChannel::RotationControl { .. } => &mut state.rotation,
                    DmxChannel::VerticalFlip { .. } => &mut state.vertical_flip,
                    DmxChannel::HorizontalFlip { .. } => &mut state.horizontal_flip,
                    DmxChannel::HorizontalPosition { .. } => &mut state.horizontal_pos,
                    DmxChannel::VerticalPosition { .. } => &mut state.vertical_pos,
                    DmxChannel::WaveEffect { .. } => &mut state.wave_effect,
                    DmxChannel::ManualDrawing { .. } => &mut state.manual_drawing,
                };
                self.dmx_field(ui, channel_enum, value, &self.device);
            }
        });
    }
}

impl DmxApp {
    fn dmx_field(&self, ui: &mut egui::Ui, channel_enum: &DmxChannel, value: &mut u8, device: &Option<Arc<DmxLaserDevice>>) {
        let label = channel_enum.label();
        let channel_num = channel_enum.channel();
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
                    if let Some(field) = get_state_field_by_channel(&mut dev_state, channel_num) {
                        *field = *value;
                    }
                    if channel_num > 0 {
                        let _ = device.set_dmx_channel(channel_num, *value);
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
                        if let Some(field) = get_state_field_by_channel(&mut dev_state, channel_num) {
                            *field = *value;
                        }
                        if channel_num > 0 {
                            let _ = device.set_dmx_channel(channel_num, *value);
                        }
                    }
                }
            }
        });
    }
}


fn get_state_field_by_channel<'a>(state: &'a mut DmxLaserState, channel: u8) -> Option<&'a mut u8> {
    match channel {
        1 => Some(&mut state.master_dimmer),
        2 => Some(&mut state.color_control),
        3 => Some(&mut state.color_speed),
        4 => Some(&mut state.pattern_group),
        5 => Some(&mut state.pattern_select),
        6 => Some(&mut state.dynamic_effects),
        7 => Some(&mut state.effect_speed),
        8 => Some(&mut state.pattern_size),
        9 => Some(&mut state.size_control),
        10 => Some(&mut state.rotation),
        11 => Some(&mut state.vertical_flip),
        12 => Some(&mut state.horizontal_flip),
        13 => Some(&mut state.horizontal_pos),
        14 => Some(&mut state.vertical_pos),
        15 => Some(&mut state.wave_effect),
        16 => Some(&mut state.manual_drawing),
        _ => None,
    }
}