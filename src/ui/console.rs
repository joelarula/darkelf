use std::sync::Arc;
use std::collections::HashMap;

use tokio::sync::{Mutex, mpsc};

use crate::model::{DeviceResponse, PlaybackMode};

use crate::ui::show_selector::show_selector_grid;
use crate::ui::{buttons, playback_settings, settings, statusbar}; 

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
    pub mode: PlaybackMode,
    pub device_connected: bool,
    pub device_name: Option<String>,
    pub device_state: Option<DeviceResponse>,
    incomming_channel: Arc<Mutex<mpsc::UnboundedReceiver<DeviceMessage>>>,
    pub(crate) command_sender: mpsc::UnboundedSender<DeviceCommand>,
    /// Maps playback mode/item to a vector of 0/1 bits (length 50)
    pub playback_selections: HashMap<u8, Vec<u8>>, // key: playback mode/item, value: 50 bits (0/1)
}


impl Console {
        pub fn new(device_channel: Arc<Mutex<mpsc::UnboundedReceiver<DeviceMessage>>>,device_command: mpsc::UnboundedSender<DeviceCommand>) -> Self {
            let mut playback_selections = HashMap::new();
            // Initialize with required playback modes, all bits 0
            use crate::model::PlaybackMode;
            for key in [
                PlaybackMode::TimelinePlayback as u8,
                PlaybackMode::AnimationPlayback as u8,
                PlaybackMode::ChristmasBroadcast as u8,
                PlaybackMode::OutdoorPlayback as u8,
            ] {
                playback_selections.insert(key, vec![0u8; 50]);
            }
            Self {
                channel: 1,
                display_range: 50,
               // sound_sensitivity: 128,
               // playback_speed: 50,
                light: Light::Mono,
                x_y_interchange: false,
                x_sign: Sign::Plus,
                y_sign: Sign::Plus,
                on: false,
                mode: PlaybackMode::RandomPlayback,
                device_connected: false,
                device_name: None,
                device_state: None,
                incomming_channel: device_channel,
                command_sender: device_command,
                playback_selections,
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


#[derive(Clone, Debug)]
pub enum DeviceCommand {
    On(bool),
    SetSettings(crate::model::SettingsData),
    SetMode {
        mode: PlaybackMode,
        selected_shows: Option<Vec<u8>>,
    },
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
            }
        }


        buttons::show_mode_buttons(self, ctx);
        statusbar::show_status_bar(self, ctx);
        settings::show_settings_panel(self, ctx);
        
        if matches!(
            self.mode,
            PlaybackMode::TimelinePlayback
                | PlaybackMode::AnimationPlayback
                | PlaybackMode::TextPlayback
                | PlaybackMode::ChristmasBroadcast
                | PlaybackMode::OutdoorPlayback
        ){
            playback_settings::show_playback_settings_ui(ctx);


            // Central panel (fills the remaining space)
            egui::CentralPanel::default().show(ctx, |ui| {
                show_selector_grid(ui, self);
            });

        }else{
            // Central panel (fills the remaining space)
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.label("Hello, dark elf!");
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
}
