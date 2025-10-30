use std::sync::Arc;
use std::collections::HashMap;
use crate::model::{DisplayColor, DrawData, Point, ProjectItem, PisObject};
use crate::ui::model::{DeviceCommand, DeviceList, DeviceMessage};
use eframe::egui; 
use tokio::sync::{Mutex, mpsc};
use windows::Devices::Enumeration::DeviceInformation;

use crate::model::{DeviceResponse, PlaybackMode};

use crate::ui::show_selector::show_selector_grid;
use crate::ui::{buttons, playback_settings, settings, statusbar,draw}; 


pub struct Console {  
    pub channel: i32,
    pub display_range: i32,
    pub light: Light,
    pub x_y_interchange: bool,
    pub x_sign: Sign,
    pub y_sign: Sign,
    pub on: bool,
    pub mode: PlaybackMode,
    pub color: DisplayColor,
    pub playback_speed: u8,
    pub sound_sensitivity: u8,
    pub audio_mode: u8,
    pub device_connected: bool,
    pub device_name: Option<String>,
    pub device_state: Option<DeviceResponse>,
    incomming_channel: Arc<Mutex<mpsc::UnboundedReceiver<DeviceMessage>>>,
    pub(crate) command_sender: mpsc::UnboundedSender<DeviceCommand>,
    /// Maps playback mode/item to ProjectItem (py_mode, prj_selected)
    pub playback_selections: HashMap<u8, ProjectItem>, // key: playback mode/item, value: ProjectItem
    pub draw_data: String,
    pub draw_config_data: String,
    pub cached_draw_text: String,
    pub cached_points_result: Option<Result<Vec<Point>, String>>,
    pub cached_config_text: String,
    pub cached_config_result: Option<Result<PisObject, String>>,
    pub text_command: String,
}


impl Console {

        pub fn new(device_channel: Arc<Mutex<mpsc::UnboundedReceiver<DeviceMessage>>>,device_command: mpsc::UnboundedSender<DeviceCommand>) -> Self {
            let mut playback_selections = HashMap::new();
            // Initialize with required playback modes, all bits 0
            use crate::model::PlaybackMode;
            for key in [
                PlaybackMode::LineGeometryPlayback as u8,
                PlaybackMode::AnimationPlayback as u8,
                PlaybackMode::ChristmasPlayback as u8,
                PlaybackMode::OutdoorPlayback as u8,
            ] {
                playback_selections.insert(key, ProjectItem { py_mode: 128, prj_selected: vec![0u16; 4] });
            }
            Self {
                channel: 1,
                display_range: 50,
                sound_sensitivity: 128,
                playback_speed: 50,
                light: Light::Mono,
                x_y_interchange: false,
                x_sign: Sign::Plus,
                y_sign: Sign::Plus,
                on: false,
                mode: PlaybackMode::RandomPlayback,
                color: DisplayColor::RGB,
                audio_mode: 0,
                device_connected: false,
                device_name: None,
                device_state: None,
                incomming_channel: device_channel,
                command_sender: device_command,
                playback_selections,
                draw_data: String::new(),
                draw_config_data: r#"{
    "txPointTime": 55,
    "cnfValus": [
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        3
    ]
}"#.to_string(),
                cached_draw_text: String::new(),
                cached_points_result: None,
                cached_config_text: String::new(),
                cached_config_result: None,
                text_command: String::new(),
            }
        }
    }

#[derive(PartialEq, Copy, Clone)]
pub enum Sign {
    Plus,
    Minus,
}

impl Default for Sign {
    fn default() -> Self {
        Sign::Plus
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum Light {
    Mono,
    RGB,
}

impl Default for Light {
    fn default() -> Self {
        Light::Mono
    }
}




impl eframe::App for Console {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

        let mut device_response: Option<DeviceResponse> = None;
        let mut xy: Option<u8> = None;
        if let Ok(mut rx) = self.incomming_channel.try_lock() {
            while let Ok(msg) = rx.try_recv() {
                match msg {
                    DeviceMessage::DeviceResponse(resp) => {
                        // Extract xy value before moving resp
                        xy = Some(resp.settings.xy);
                        device_response = Some(resp);
                        break; // Only process one DeviceResponse per update
                    }
                    DeviceMessage::DeviceName(name) => {
                        self.device_name = Some(name);
                    }
                    DeviceMessage::DeviceStatus(status) => {
                        self.device_connected = status;
                        log::info!("Device connection status changed: {}", status);
                    }
                    DeviceMessage::DeviceList(device_list) => todo!(),
                    DeviceMessage::DeviceInfo(device_information) => todo!(),
                    DeviceMessage::SetupStatus(_) => todo!(),
                }
            }
        }

        if let Some(device_state) = device_response {
            self.device_state = Some(device_state.clone());
            if let Some(xy_val) = xy {
                self.parse_xy_map(&xy_val);
            }
            if let Some(device_state_ref) = self.device_state.as_ref() {
                self.on = device_state_ref
                    .device_info
                    .as_ref()
                    .map(|info| info.device_on)
                    .unwrap_or(false);

                self.display_range = device_state_ref
                    .settings
                    .values[1] as i32;

                self.light = if device_state_ref.settings.light == 1 {
                    Light::Mono
                } else {
                    Light::RGB
                };

                // Update playback_selections from prj_data
                if let Some(prj_data) = device_state_ref.prj_data.as_ref() {
                    for (&mode, item) in prj_data.prj_item.iter() {
                        self.playback_selections.insert(
                            mode as u8,
                            item.clone(),
                        );
                    }
                }
            }
        }


        buttons::show_mode_buttons(self, ctx);
        statusbar::show_status_bar(self, ctx);
        settings::show_settings_panel(self, ctx);
        
        if matches!(
            self.mode,
            PlaybackMode::LineGeometryPlayback
                | PlaybackMode::AnimationPlayback
                | PlaybackMode::ChristmasPlayback
                | PlaybackMode::OutdoorPlayback
        ){
            playback_settings::show_playback_settings_ui(ctx);


            // Central panel (fills the remaining space)
            egui::CentralPanel::default().show(ctx, |ui| {
                show_selector_grid(ui, self);
            });

        }
        
        if matches!(self.mode,PlaybackMode::Draw){
            draw::show_draw_ui(self,ctx);
        }

        if matches!(self.mode,PlaybackMode::TextPlayback){
            crate::ui::text::show_text_ui(self,ctx);
        }

        if matches!(self.mode,PlaybackMode::RandomPlayback){
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.label("Random Playback!");
            });
        }


    }
}


impl Console {
   
    pub fn set_playback(&mut self, mode: PlaybackMode, selected_shows: Option<Vec<u8>>) {
        self.mode = mode;
        let _ = self.command_sender.send(DeviceCommand::SetMode { mode, selected_shows });
    }
    
    pub fn parse_xy_map(&mut self, xy_map: &u8)  {
        // Map: 0-3 normal, 4-7 interchange
        self.x_y_interchange = *xy_map >= 4;
        let idx = *xy_map % 4;
        // Order: 0: X+Y+, 1: X+Y-, 2: X-Y-, 3: X-Y+
        self.x_sign = if idx == 0 || idx == 1 { Sign::Plus } else { Sign::Minus };
        self.y_sign = if idx == 0 || idx == 3 { Sign::Plus } else { Sign::Minus };
    }

        /// Calculates xy value from UI widget states (inverse of parse_xy_map)
    pub fn calc_xy_value(&self) -> u8 {
        // Map: 0-3 normal, 4-7 interchange
        let base = if self.x_y_interchange { 4 } else { 0 };
        // Order: 0: X+Y+, 1: X+Y-, 2: X-Y-, 3: X-Y+
        let idx = match (self.x_sign, self.y_sign) {
            (Sign::Plus, Sign::Plus) => 0,
            (Sign::Plus, Sign::Minus) => 1,
            (Sign::Minus, Sign::Minus) => 2,
            (Sign::Minus, Sign::Plus) => 3,
        };
        base + idx
    }


    // Helper function to send SetSettings with updated xy value
    pub fn send_xy_settings(&mut self) {
        if let Some(device_state) = &self.device_state {
            let mut new_settings = device_state.settings.clone();
            new_settings.xy = self.calc_xy_value();
            let _ = self.command_sender.send(crate::ui::console::DeviceCommand::SetSettings(new_settings));
        }
    }
}
