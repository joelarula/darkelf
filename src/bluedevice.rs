use crate::device::LaserDevice;
use crate::model::{CommandConfig,  DrawData, MainCommandData, PisObject, PlaybackCommand, Point, PlaybackData, Playback, AudioConfig, TextData};
use log::{debug, info, error};
use std::sync::{Arc, Mutex};
use rand;
use crate::model::{ DeviceState, DeviceSettings};
use crate::command::{CommandGenerator, POWER_ON_CMD, POWER_OFF_CMD};
use crate::blue::BlueController;


pub struct BlueLaserDevice {
    random_check: Vec<u8>,
    device_controller: Arc<Mutex<dyn BlueController>>,
	device_info: Arc<Mutex<Option<DeviceState>>>,
    playback_items: std::collections::HashMap<u8, Playback>,
}

impl BlueLaserDevice {



    pub fn new(device_controller: impl BlueController + 'static) -> Self {
        Self {
            random_check: Self::gen_random_check(),
            device_controller: Arc::new(Mutex::new(device_controller)),
            device_info: Arc::new(Mutex::new(None)),
            playback_items: {
            let mut map = std::collections::HashMap::new();
                map.insert(2, Playback { playback_mode: 128, selected_plays: vec![65535, 65535, 65535, 3] });
                map.insert(3, Playback { playback_mode: 128, selected_plays: vec![65535, 65535, 65535, 3] });
                map.insert(5, Playback { playback_mode: 128, selected_plays: vec![65535, 65535, 65535, 3] });
                map.insert(6, Playback { playback_mode: 128, selected_plays: vec![65535, 65535, 65535, 3] });
                map
            },
        }
    }

    pub fn is_connected(&self) -> bool {
        let controller = self.device_controller.lock().unwrap();
        controller.is_connected()
    }
    
    pub async fn connect(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut controller = self.device_controller.lock().unwrap();
        controller.connect().await
    }

    pub async fn setup(&self) {
        debug!("Device: setup");
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
 
        }
        
        let cmd = CommandGenerator::get_query_cmd(&self.random_check);
        debug!("get_query_cmd: {}", cmd);        
        let mut controller = self.device_controller.lock().unwrap();
        if let Err(e) = controller.send(&cmd).await {
            error!("Failed to send command: {:?}", e);
        }
    }

    pub fn is_initialized(&self) -> bool {
        self.device_info.lock().unwrap().is_some()
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
        if let Ok(mut info_lock) = self.device_info.lock() {
            if let Some(ref mut device_data) = *info_lock {
                device_data.device_info.device_on = true;              
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

        if let Err(e) = controller.send(POWER_OFF_CMD).await {
            error!("Failed to send OFF command: {:?}", e);
        }

        if let Ok(mut info_lock) = self.device_info.lock() {
            if let Some(ref mut device_data) = *info_lock {
                device_data.device_info.device_on = false;              
            }
        }
    }

    /// Get a copy of the current device settings
    pub fn get_setting(&self) -> Option<DeviceSettings> {
        self.device_info.lock().unwrap()
            .as_ref()
            .map(|resp| resp.settings.clone())
    }

    pub async fn set_settings(&self, new_settings: DeviceSettings) {
        info!("Setting new device settings: {:?}", new_settings);
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

    pub async fn set_main_command(&self, command: MainCommandData) {
        info!("Setting main command: {:?}", command);
        let cmd = CommandGenerator::pack_main_command(&command);
        let mut controller = self.device_controller.lock().unwrap();
        if let Ok(_) = controller.send(&cmd).await {
            let mut info_lock = self.device_info.lock().unwrap();
            if let Some(ref mut response) = *info_lock {
                response.main_data = command;
            }
        } else {
            error!("Failed to send main command");
        }
    }

    pub async fn draw(&self, points: Vec<Point>, config: PisObject) {
       let cmd = CommandGenerator::get_draw_cmd_str(&points, &config);
       let mut controller = self.device_controller.lock().unwrap();
       let _ = controller.send(&cmd).await;

    }



    /// Set the playback mode on the device
    pub async fn set_playback_mode(&self, command: PlaybackCommand) {
        let command_clone = command.clone();
        if let Some(command_config) = self.command_config_from_main(&command) {
            let cmd = CommandGenerator::get_cmd_str(&command_config);
            let mut controller = self.device_controller.lock().unwrap();
            if let Ok(_) = controller.send(&cmd).await {
                // Update main_data in device_info
                let mut info_lock = self.device_info.lock().unwrap();
                if let Some(ref mut resp) = *info_lock {
                    resp.main_data.device_mode = command_clone.mode;
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
            let cmd = CommandGenerator::get_cmd_str(&command_data);
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

        if let Ok(info_lock) = self.device_info.lock() {
            if let Some(ref device_data) = *info_lock {
                return device_data.device_info.device_on;              
            }
        }
        false
    }

    /// Get a copy of the entire device response
    pub fn get_device_response(&self) -> Option<DeviceState> {
        self.device_info.try_lock().unwrap()
            .as_ref()
            .map(|resp| resp.clone())
    }


    /// Converts a MainCommandData to a CommandConfig with default prj_item
fn command_config_from_main(&self, command: &PlaybackCommand) -> Option<CommandConfig> {
    if let Some(resp) = self.get_device_response() {
        let main = resp.main_data.clone();
        let text_data = TextData {
            tx_color: command.color.unwrap_or(main.color), 
            tx_size: main.text_size_x,
            run_speed: command.playback_speed.unwrap_or(main.run_speed),
            tx_dist: main.text_distance,
            tx_point_time: main.text_point_time,
            run_dir: main.run_direction,
        };
        let mut prj_data = resp.main_data.playback.clone();
        prj_data.audio_config = AudioConfig {
            audio_trigger_mode: if command.audio_mode.unwrap_or(main.audio_mode != 0) { 1 } else { 0 },
            sound_sensitivity: command.audio_sensitivity.unwrap_or(main.sound_value),
        };
        prj_data.playback_items = self.playback_items.iter().map(|(&k, v)| (k as u8, v.clone())).collect();


        if let Some(selected) = &command.selected_shows {
      
            prj_data.playback_items.insert(
                command.mode as u8,
                Playback {
                    playback_mode: if command.tick_playback.unwrap_or(false) { 128 } else { 0 },
                    selected_plays: CommandGenerator::pack_bits_to_prj_selected(&selected),
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

impl LaserDevice for BlueLaserDevice {
    async fn setup(&self) {
        self.setup().await
    }
    
    async fn on(&self) {
        self.on().await
    }
    
    async fn off(&self) {
        self.off().await
    }
    
    fn get_settings(&self) -> Option<DeviceSettings> {
        self.get_setting()
    }
    
    async fn set_settings(&self, new_settings: DeviceSettings) {
        self.set_settings(new_settings).await
    }
    
    async fn draw(&self, points: Vec<Point>, config: PisObject) {
        self.draw(points, config).await
    }
    
    async fn set_playback_mode(&self, command: PlaybackCommand) {
        self.set_playback_mode(command).await
    }
    
    fn is_on(&self) -> bool {
        self.is_on()
    }
    

}


