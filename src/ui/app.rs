use std::sync::Arc;
use std::collections::HashMap;
use crate::dmx::device::DmxDevice;
use crate::dmx::model::Fixture;
use crate::blue::model::{DeviceInfo, DeviceSettings, DrawData, MainCommandData, Playback, PlaybackMode};
use crate::ui::model::{DeviceCommand, DeviceMessage};
use crate::blue::winblue::DeviceList;
use eframe::egui; 
use egui::Mesh;
use tokio::sync::{Mutex, mpsc};

use crate::blue::model::{DeviceMode};

use crate::ui::playback_selector::show_selector_grid;
use crate::ui::{buttons, dmx, draw, playback_settings, settings, statusbar}; 


pub struct App {  
    pub device_info: DeviceInfo,
    pub settings : DeviceSettings,
    pub command_data: MainCommandData,

    pub on: bool,
    pub mode: DeviceMode,

    pub ble_device_connected: bool,

    incomming_channel: Arc<Mutex<mpsc::UnboundedReceiver<DeviceMessage>>>,
    pub(crate) command_sender: mpsc::UnboundedSender<DeviceCommand>,


    pub draw_data: String,
    pub cached_draw_text: String,
    pub cached_points_result: Option<DrawData>,
    pub cached_error_result: Option<String>,
    pub mesh: Option<Mesh>,
    pub text_command: String,
    pub laser_device_initialized: bool,
    pub device_list: DeviceList,
    pub fixture: Option<Fixture>,
    pub dmx_ports: Vec<String>,
    pub selected_dmx_port: Option<String>,
    pub dmx_device: Option<Arc<DmxDevice>>
}


impl App {

        pub fn new(device_channel: Arc<Mutex<mpsc::UnboundedReceiver<DeviceMessage>>>,device_command: mpsc::UnboundedSender<DeviceCommand>) -> Self {
            let mut playback_selections = HashMap::new();
            // Initialize with required playback modes, all bits 0
            use crate::blue::model::DeviceMode;
            for key in [
                DeviceMode::LineGeometryPlayback as u8,
                DeviceMode::AnimationPlayback as u8,
                DeviceMode::ChristmasPlayback as u8,
                DeviceMode::OutdoorPlayback as u8,
            ] {
                playback_selections.insert(key, Playback { playback_mode: PlaybackMode::LoopPlay, selected_plays: vec![0u16; 4], selected_play: 1 });
            }
            Self {
                device_info: DeviceInfo::default(),
                settings : DeviceSettings::default(),
                command_data: MainCommandData::default(),
                on: false,
                mode: DeviceMode::RandomPlayback,
                ble_device_connected: false,
                incomming_channel: device_channel,
                command_sender: device_command,
                draw_data: String::new(),
                cached_draw_text: String::new(),
                cached_points_result: None,
                cached_error_result: None,
                mesh: None,
                text_command: String::new(),
                laser_device_initialized: false,
                device_list: DeviceList { devices: Vec::new(), selected_index: None },
                fixture: None,
                dmx_ports: Vec::new(),
                selected_dmx_port: None,
                dmx_device: None,
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
pub enum Beam {
    Mono,
    RGB,
}

impl Default for Beam {
    fn default() -> Self {
        Beam::Mono
    }
}




impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

        if let Ok(mut rx) = self.incomming_channel.try_lock() {
            while let Ok(msg) = rx.try_recv() {
                match msg {
                    DeviceMessage::ConnectionStatus(status) => {
                        self.ble_device_connected = status;
                    }
                    DeviceMessage::DeviceList(device_list) => {
                        self.device_list = device_list;
                    }
                    DeviceMessage::SetupStatus(status) => {
                        self.laser_device_initialized = status;
                    }
                    DeviceMessage::DeviceInfo(info) => {
                        self.device_info = info;
                    }
                    DeviceMessage::DeviceSettings(device_settings) => {
                        self.settings = device_settings;
                    },
                    DeviceMessage::DeviceCommand(main_command_data) => {
                        self.command_data = main_command_data;
                    }
                }
            }
        }

        buttons::show_mode_buttons(self, ctx);
        statusbar::show_status_bar(self, ctx);
        settings::show_settings_panel(self, ctx);
        
        if matches!(self.mode,DeviceMode::Dmx){
            dmx::show_dmx(self,ctx);
        }

        if matches!(
            self.mode,
            DeviceMode::RandomPlayback 
                | DeviceMode::LineGeometryPlayback
                | DeviceMode::AnimationPlayback
                | DeviceMode::ChristmasPlayback
                | DeviceMode::OutdoorPlayback
        ){
            playback_settings::show_playback_settings_ui(self, ctx);
            egui::CentralPanel::default().show(ctx, |ui| {
                show_selector_grid(ui, self);
            });

        }
        
        if matches!(self.mode,DeviceMode::Draw){
            draw::show_draw_ui(self,ctx);
        }

        if matches!(self.mode,DeviceMode::TextPlayback){
            crate::ui::text::show_text_ui(self,ctx);
        }

        if matches!(self.mode,DeviceMode::RandomPlayback){
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.label("Random Playback!");
            });
        }

    }
}