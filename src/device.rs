use crate::model::{CommandConfig, MainCommandData, PlaybackCommand, ProjectData, ProjectItem, PublicData, TextData};
use log::{debug, info, error};
use std::sync::{Arc, Mutex};
use rand;
use crate::model::{ DeviceResponse, PlaybackMode, SettingsData};
use crate::command::{CommandGenerator, POWER_ON_CMD, POWER_OFF_CMD};
use crate::blue::BlueController;

pub struct LaserDevice {
    random_check: Vec<u8>,
    device_controller: Arc<Mutex<dyn BlueController>>,
	device_info: Arc<Mutex<Option<DeviceResponse>>>,
    playback_items: std::collections::HashMap<u8, ProjectItem>,
}

impl LaserDevice {



    /// Create a new LaserDevice instance with initialized random check bytes and device controller
    pub fn new(device_controller: impl BlueController + 'static) -> Self {
        Self {
            random_check: Self::gen_random_check(),
            device_controller: Arc::new(Mutex::new(device_controller)),
            device_info: Arc::new(Mutex::new(None)),
            playback_items: {
            let mut map = std::collections::HashMap::new();
                map.insert(2, ProjectItem { py_mode: 128, prj_selected: vec![65535, 65535, 65535, 3] });
                map.insert(3, ProjectItem { py_mode: 128, prj_selected: vec![65535, 65535, 65535, 3] });
                map.insert(5, ProjectItem { py_mode: 128, prj_selected: vec![65535, 65535, 65535, 3] });
                map.insert(6, ProjectItem { py_mode: 128, prj_selected: vec![65535, 65535, 65535, 3] });
                map
            },
        }
    }

    pub async fn setup(&self) {
        debug!("LaserDevice: setup");
        {
            // Clone Arc fields for the callback
            let device_info = self.device_info.clone();
            let random_check = self.random_check.clone();

            let mut controller = self.device_controller.lock().unwrap();
            controller.set_receiver_callback(Box::new(move |data| {
                info!("Received data: {}", data);
                // First verify response using random check
                let (success, _) = CommandGenerator::check_received_data(&data, &random_check);
                if success {
                    // Then parse full device response
                    if let Some(response) = CommandGenerator::parse_device_response(&data) {
                        info!("DeviceResponse: {:#?}", response);
                        if let Ok(mut info) = device_info.lock() {
                            *info = Some(response);
                        }
                    }
                } else {
                    info!("Invalid or unverified device response");
                }
            }));
            // Lock is released here when controller goes out of scope
        }
        
        let cmd = CommandGenerator::get_query_cmd(&self.random_check);
        debug!("get_query_cmd: {}", cmd);        
        let mut controller = self.device_controller.lock().unwrap();
        if let Err(e) = controller.send(&cmd).await {
            error!("Failed to send command: {:?}", e);
        }
    }

    pub async fn on(&self) {
        info!("LaserDevice: turning on");
        let mut controller = self.device_controller.lock().unwrap();
        if !controller.is_connected() {
            error!("Cannot turn on - device not connected");
            return;
        }
        // Send power on command
        if let Err(e) = controller.send(POWER_ON_CMD).await {
            error!("Failed to send ON command: {:?}", e);
        }
        // Update device_info to reflect device is on
        if let Ok(mut info) = self.device_info.lock() {
            if let Some(ref mut resp) = *info {
                if let Some(ref mut dev_info) = resp.device_info {
                    dev_info.device_on = true;
                }
            }
        }
    }

    pub async fn off(&self) {
        info!("LaserDevice: turning off");
        let mut controller = self.device_controller.lock().unwrap();
        if !controller.is_connected() {
            error!("Cannot turn off - device not connected");
            return;
        }
        // Send power off command
        if let Err(e) = controller.send(POWER_OFF_CMD).await {
            error!("Failed to send OFF command: {:?}", e);
        }
        // Update device_info to reflect device is off
        if let Ok(mut info) = self.device_info.lock() {
            if let Some(ref mut resp) = *info {
                if let Some(ref mut dev_info) = resp.device_info {
                    dev_info.device_on = false;
                }
            }
        }
    }

    /// Get a copy of the current device settings
    pub fn get_setting(&self) -> Option<SettingsData> {
        self.device_info.lock().unwrap()
            .as_ref()
            .map(|resp| resp.settings.clone())
    }

pub async fn set_settings(&self, new_settings: SettingsData) {
    let cmd = CommandGenerator::get_setting_cmd(&new_settings);
    let mut controller = self.device_controller.lock().unwrap();
    if let Ok(_) = controller.send(&cmd).await {
        let mut info_lock = self.device_info.lock().unwrap();
        if let Some(ref mut response) = *info_lock {
            response.settings = new_settings;
        }
    } else {
        error!("Failed to send settings command");
    }
}


    /// Set the playback mode on the device
    pub async fn set_playback_mode(&self, command: PlaybackCommand) {
        let command_clone = command.clone();
        if let Some(command_config) = self.command_config_from_main(&command) {
            let cmd = CommandGenerator::get_cmd_str(&command_config, None);
            let mut controller = self.device_controller.lock().unwrap();
            if let Ok(_) = controller.send(&cmd).await {
                // Update main_data in device_info
                let mut info_lock = self.device_info.lock().unwrap();
                if let Some(ref mut resp) = *info_lock {
                    resp.main_data.current_mode = command_clone.mode as u8;
                }
            } else {
                error!("Failed to send playback mode command");
            }
        } else {
            error!("Failed to generate command config for playback mode");
        }
    }


        /// Get a copy of the current main command data
    pub fn get_command_data(&self) -> Option<MainCommandData> {
        self.device_info.lock().unwrap()
            .as_ref()
            .map(|resp| resp.main_data.clone())
    }


    
    /// Set the main command data and send the corresponding command to the device
    pub async fn set_command_data(&self, command_data: CommandConfig) {
        let mut info_lock = self.device_info.lock().unwrap();
        if let Some(ref mut response) = *info_lock {

          //  response.main_data = command_data.main_data;
            let cmd = CommandGenerator::get_cmd_str(&command_data, None);
            // Send the command to the device
            let mut controller = self.device_controller.lock().unwrap();
            if let Err(e) = controller.send(&cmd).await {
                error!("Failed to send main command: {:?}", e);
            }
        }
    }

    /// Generate random verification bytes
    fn gen_random_check() -> Vec<u8> {
        let bytes: Vec<u8> = (0..4).map(|_| rand::random::<u8>()).collect();
        info!("Generated random check bytes: {:02X?}", bytes);
        bytes
    }

    /// Get the current device power state
    pub fn is_on(&self) -> bool {
        self.device_info.lock().unwrap()
            .as_ref()
            .and_then(|resp| resp.device_info.as_ref())
            .map(|info| info.device_on)
            .unwrap_or(false)
    }

    /// Get a copy of the entire device response
    pub fn get_device_response(&self) -> Option<DeviceResponse> {
        self.device_info.try_lock().unwrap()
            .as_ref()
            .map(|resp| resp.clone())
    }


    /// Converts a MainCommandData to a CommandConfig with default prj_item
fn command_config_from_main(&self, command: &PlaybackCommand) -> Option<CommandConfig> {
    if let Some(resp) = self.get_device_response() {
        let main = resp.main_data.clone();
        let text_data = TextData {
            tx_color: main.text_color,
            tx_size: main.text_size,
            run_speed: main.run_speed,
            tx_dist: main.text_distance,
            tx_point_time: main.text_point_time,
            run_dir: main.run_direction,
        };
        let mut prj_data = resp.prj_data.clone().unwrap_or_else(|| ProjectData {
            public: PublicData {
                rd_mode: main.audio_mode,
                sound_val: main.sound_value,
            },
            prj_item: self.playback_items.iter().map(|(&k, v)| (k as i32, v.clone())).collect(),
        });

        if let Some(selected) = &command.selected_shows {
      
            prj_data.prj_item.insert(
                command.mode as i32,
                ProjectItem {
                    py_mode: 128,
                    prj_selected: CommandGenerator::pack_bits_to_prj_selected(&selected),
                },
            );
        }
        Some(CommandConfig {
            cur_mode: command.mode as u8,
            text_data,
            prj_data,
        })
    } else {
        None
    }
    }

}


