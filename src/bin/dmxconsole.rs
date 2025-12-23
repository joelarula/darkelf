use darkelf::util;
use eframe::{egui};
use std::sync::{Arc, Mutex};
use darkelf::dmx::device::DmxDevice;
use darkelf::dmx::model::Fixture;
use egui_taffy::{TuiBuilderLogic, taffy, tid, tui};
use std::path::PathBuf;


pub struct DmxApp {
    pub dmx_ports: Vec<String>,
    pub selected_port: Option<String>,
    pub device: Option<Arc<DmxDevice>>,
    pub status_message: Arc<Mutex<String>>,
    pub fixture: Fixture,
    pub available_fixtures: Vec<(String, PathBuf)>, // (name, path)
    pub selected_fixture_idx: usize,
}

fn main() {
    util::setup_logging();
    
    // Scan available fixtures
    let available_fixtures = scan_fixtures();
    
    // Load first fixture by default
    let fixture: Fixture = if !available_fixtures.is_empty() {
        let file = std::fs::File::open(&available_fixtures[0].1).expect("Fixture file not found");
        serde_json::from_reader(file).expect("Failed to parse fixture JSON")
    } else {
        panic!("No fixtures found in assets/fixtures/");
    };
    
    let app = DmxApp::new(fixture.clone(), available_fixtures);
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([500.0, 400.0])
            .with_min_inner_size([500.0, 400.0]),
            
        ..Default::default()
    };
    let _ = eframe::run_native(
        format!("DMX Console - {}", fixture.name).as_str(),
        native_options,
        Box::new(|_cc| Ok::<Box<dyn eframe::App>, Box<dyn std::error::Error + Send + Sync>>(Box::new(app))),
    );
}

fn scan_fixtures() -> Vec<(String, PathBuf)> {
    let fixtures_dir = PathBuf::from("assets/fixtures");
    let mut fixtures = Vec::new();
    
    if let Ok(entries) = std::fs::read_dir(&fixtures_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Ok(file) = std::fs::File::open(&path) {
                    if let Ok(fixture) = serde_json::from_reader::<_, Fixture>(file) {
                        fixtures.push((fixture.name.clone(), path));
                    }
                }
            }
        }
    }
    
    fixtures.sort_by(|a, b| a.0.cmp(&b.0));
    fixtures
}

impl DmxApp {
    fn render_channel_column(&self, tui: &mut egui_taffy::Tui, channels: &[darkelf::dmx::model::Channel], offset: usize, device: &Arc<DmxDevice>) {
        for (i, channel) in channels.iter().enumerate() {
            let label = &channel.name;
            let tooltip = channel.capabilities.iter().map(|cap| {
                let range = format!("{}-{}", cap.dmx_range[0], cap.dmx_range[1]);
                let name = cap.menu_name.as_ref().unwrap_or(&cap.type_);
                let desc = cap.description.as_deref().unwrap_or("");
                format!("{}: {}\n{}", range, name, desc)
            }).collect::<Vec<_>>().join("\n\n");
            if let Some(mut value) = device.get_dmx_channel(offset + i + 1) {
                tui.style(taffy::Style {
                    flex_direction: taffy::FlexDirection::Row,
                    align_items: Some(taffy::AlignItems::Center),
                    gap: taffy::style_helpers::length(8.0),
                    ..Default::default()
                }).add(|tui| {
                    tui.style(taffy::Style {
                        size: taffy::Size {
                            width: taffy::style_helpers::length(120.0),
                            height: taffy::style_helpers::auto(),
                        },
                        ..Default::default()
                    }).label(label);
                    let drag = egui::DragValue::new(&mut value)
                        .range(0..=255)
                        .speed(1.0)
                        .suffix("");
                    if tui.ui_add(drag).on_hover_text(tooltip).changed() {
                        device.set_dmx_channel(offset + i + 1, value).ok();
                    }
                });
            }
        }
    }
    fn select_and_init_port(&mut self, idx: usize) {
        if idx < self.dmx_ports.len() {
            let new_port = self.dmx_ports[idx].clone();
            if self.selected_port.as_ref() != Some(&new_port) {
                self.selected_port = Some(new_port.clone());
                // Initialize device on port selection
                let dmx_channel = 1;
                match DmxDevice::new(&new_port, dmx_channel, self.fixture.clone()) {
                    Ok(dev) => {
                        if dev.start().is_ok() {
                            let mut status = self.status_message.lock().unwrap();
                            *status = format!("Created and started DMX device on {} channel {}", new_port, dmx_channel);
                            self.device = Some(Arc::new(dev));
                        } else {
                            let mut status = self.status_message.lock().unwrap();
                            *status = "Failed to start DMX device".to_string();
                            self.device = None;
                        }
                    }
                    Err(e) => {
                        let mut status = self.status_message.lock().unwrap();
                        *status = format!("Failed to create DMX device: {}", e);
                        self.device = None;
                    }
                }
            }
        }
    }
    
    fn load_fixture(&mut self, idx: usize) {
        if idx < self.available_fixtures.len() {
            let path = &self.available_fixtures[idx].1;
            if let Ok(file) = std::fs::File::open(path) {
                if let Ok(fixture) = serde_json::from_reader::<_, Fixture>(file) {
                    self.fixture = fixture.clone();
                    self.selected_fixture_idx = idx;
                    
                    // Reinitialize device with new fixture
                    if let Some(port) = &self.selected_port {
                        let dmx_channel = 1;
                        match DmxDevice::new(port, dmx_channel, fixture) {
                            Ok(dev) => {
                                if dev.start().is_ok() {
                                    let mut status = self.status_message.lock().unwrap();
                                    *status = format!("Loaded fixture: {}", self.fixture.name);
                                    self.device = Some(Arc::new(dev));
                                } else {
                                    let mut status = self.status_message.lock().unwrap();
                                    *status = "Failed to start DMX device with new fixture".to_string();
                                }
                            }
                            Err(e) => {
                                let mut status = self.status_message.lock().unwrap();
                                *status = format!("Failed to create DMX device: {}", e);
                            }
                        }
                    }
                }
            }
        }
    }
    
    pub fn new(fixture: Fixture, available_fixtures: Vec<(String, PathBuf)>) -> Self {
        let status_message = Arc::new(Mutex::new(String::from("=== DMX Laser Device Setup ===")));
        let dmx_ports = darkelf::dmx::controller::scan_dmx_ports();
        let mut app = DmxApp {
            device: None,
            status_message,
            fixture,
            dmx_ports,
            selected_port: None,
            available_fixtures,
            selected_fixture_idx: 0,
        };
        if app.dmx_ports.len() == 1 {
            app.select_and_init_port(0);
        }
        app
    }
}

impl eframe::App for DmxApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            let status = self.status_message.lock().unwrap().clone();
            ui.label(format!("Status: {}", status));
        });
        // DMX port and fixture dropdown above status bar
        egui::TopBottomPanel::bottom("port_selector_bar").show(ctx, |ui| {
            let mut fixture_changed = None;
            ui.horizontal(|ui| {
                // Fixture selector
                ui.label("Fixture:");
                let current_fixture_name = &self.fixture.name;
                egui::ComboBox::from_id_source("fixture_selector")
                    .selected_text(current_fixture_name)
                    .show_ui(ui, |ui| {
                        for (idx, (name, _path)) in self.available_fixtures.iter().enumerate() {
                            if ui.selectable_value(&mut self.selected_fixture_idx, idx, name).changed() {
                                fixture_changed = Some(idx);
                            }
                        }
                    });
                
                ui.separator();
                
                // DMX Port selector
                ui.label("DMX Port:");
                let mut selected_idx = self.selected_port.as_ref().and_then(|port| self.dmx_ports.iter().position(|p| p == port)).unwrap_or(0);
                let mut port_changed = None;
                egui::ComboBox::from_id_source("dmx_port_selector")
                    .selected_text(self.selected_port.clone().unwrap_or_else(|| "Select port".to_string()))
                    .show_ui(ui, |ui| {
                        for (idx, port) in self.dmx_ports.iter().enumerate() {
                            if ui.selectable_value(&mut selected_idx, idx, port).changed() {
                                port_changed = Some(idx);
                            }
                        }
                    });
                
                if let Some(idx) = port_changed {
                    self.select_and_init_port(idx);
                }
                
                if self.dmx_ports.len() == 1 && self.selected_port.is_none() {
                    self.select_and_init_port(0);
                }
            });
            
            if let Some(idx) = fixture_changed {
                self.load_fixture(idx);
            }
        });


        let channels = &self.fixture.channels;
        let mid = channels.len() / 2;
        let (left, right) = channels.split_at(mid);
        egui::CentralPanel::default().show(ctx, |ui| {
            tui(ui, ui.id().with("channels_panel"))
                .reserve_available_space()
                .style(taffy::Style {
                    flex_direction: taffy::FlexDirection::Row,
                    gap: taffy::style_helpers::length(12.0),
                    padding: taffy::style_helpers::length(12.0),
                    ..Default::default()
                })
                .show(|tui| {
                    // Organize channels in rows: left column
                    tui.style(taffy::Style {
                        flex_direction: taffy::FlexDirection::Column,
                        gap: taffy::style_helpers::length(8.0),
                        ..Default::default()
                    }).add(|tui| {
                        if let Some(device) = &self.device {
                            self.render_channel_column(tui, left, 0, device);
                        }
                    });
                    // Organize channels in rows: right column
                    tui.style(taffy::Style {
                        flex_direction: taffy::FlexDirection::Column,
                        gap: taffy::style_helpers::length(8.0),
                        ..Default::default()
                    }).add(|tui| {
                        if let Some(device) = &self.device {
                            self.render_channel_column(tui, right, mid, device);
                        }
                    });
                });
        });
    }
}
