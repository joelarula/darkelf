use darkelf::util;
use eframe::{egui};
use std::sync::{Arc, Mutex};
use darkelf::dmx::device::DmxDevice;
use darkelf::dmx::model::Fixture;


pub struct DmxApp {
    pub device: Option<Arc<DmxDevice>>,
    pub status_message: Arc<Mutex<String>>,
    pub fixture: Fixture,
}

fn main() {
    util::setup_logging();
    let fixture: Fixture = {
        let file = std::fs::File::open("assets/fixtures/laser_light_8340.json").expect("Fixture file not found");
        serde_json::from_reader(file).expect("Failed to parse fixture JSON")
    };
    let app = DmxApp::new(fixture.clone());
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 600.0])
            .with_min_inner_size([400.0, 600.0]),
        ..Default::default()
    };
    let _ = eframe::run_native(
        fixture.name.as_str(),
        native_options,
        Box::new(|_cc| Box::new(app)),
    );
}

impl DmxApp {
    pub fn new(fixture: Fixture) -> Self {
        let status_message = Arc::new(Mutex::new(String::from("=== DMX Laser Device Setup ===")));
        let dmx_ports = darkelf::dmx::controller::scan_dmx_ports();
        let port = "COM4";
        let dmx_channel = 1;
        let device = if dmx_ports.contains(&port.to_string()) {
            match DmxDevice::new(port, dmx_channel, fixture.clone()) {
                Ok(dev) => {
                    if dev.start().is_ok() {
                        let mut status = status_message.lock().unwrap();
                        *status = format!("Created and started DMX device on {} channel {}", port, dmx_channel);
                        Some(Arc::new(dev))
                    } else {
                        let mut status = status_message.lock().unwrap();
                        *status = "Failed to start DMX device".to_string();
                        None
                    }
                }
                Err(e) => {
                    let mut status = status_message.lock().unwrap();
                    *status = format!("Failed to create DMX device: {}", e);
                    None
                }
            }
        } else {
            let mut status = status_message.lock().unwrap();
            *status = format!("{} not found in available DMX ports: {:?}", port, dmx_ports);
            None
        };
        let status_message = Arc::new(Mutex::new(String::from("Ready")));
        DmxApp {
            device,
            status_message,
            fixture,
        }
    }
}

impl eframe::App for DmxApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            let status = self.status_message.lock().unwrap().clone();
            ui.label(format!("Status: {}", status));
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(device) = &self.device {
                for (i, channel) in self.fixture.channels.iter().enumerate() {
                    self.dmx_field_fixture(ui, channel, i + 1, device);
                }
            }
        });
    }
}

impl DmxApp {
        fn dmx_field_fixture(&self, ui: &mut egui::Ui, channel: &darkelf::dmx::model::Channel, rel_channel: usize, device: &Arc<DmxDevice>) {
            let label = &channel.name;
            // Compose tooltip from channel capabilities
            let tooltip = channel.capabilities.iter().map(|cap| {
                let range = format!("{}-{}", cap.dmx_range[0], cap.dmx_range[1]);
                let name = cap.menu_name.as_ref().unwrap_or(&cap.type_);
                let desc = cap.description.as_deref().unwrap_or("");
                format!("{}: {}\n{}", range, name, desc)
            }).collect::<Vec<_>>().join("\n\n");

            // Use device getter for channel value
            if let Some(value) = device.get_dmx_channel(rel_channel) {
                let mut value_str = value.to_string();
                ui.horizontal(|ui| {
                    ui.add_sized([100.0, 0.0], egui::Label::new(label));
                    let text_response = ui.add_sized(
                        [50.0, 0.0],
                        egui::TextEdit::singleline(&mut value_str)
                            .char_limit(3)
                    );
                    let mut slider_value = value;
                    let slider = egui::Slider::new(&mut slider_value, 0..=255).step_by(1.0);
                    let slider_response = ui.add_sized([100.0, 0.0], slider);
                    let slider_response = slider_response.on_hover_text(tooltip);
                    if slider_response.changed() {
                        value_str = slider_value.to_string();
                        device.set_dmx_channel(rel_channel, slider_value).ok();
                    }
                    if text_response.changed() {
                        if let Ok(mut val) = value_str.parse::<i32>() {
                            val = val.clamp(0, 255);
                            device.set_dmx_channel(rel_channel, val as u8).ok();
                        }
                    }
                });
            }
        }


}


