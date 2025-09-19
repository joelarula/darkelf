use log::{debug, info, error};
use std::sync::{Arc, Mutex};
use rand;
use crate::model::{SettingsData, DeviceResponse,MainCommandData};
use crate::command::{CommandGenerator, POWER_ON_CMD, POWER_OFF_CMD};
use crate::blue::BlueController;

pub struct LaserDevice {
    random_check: Vec<u8>,
    device_controller: Arc<Mutex<dyn BlueController>>,
	device_info: Arc<Mutex<Option<DeviceResponse>>>,
}

impl LaserDevice {



    /// Create a new LaserDevice instance with initialized random check bytes and device controller
    pub fn new(device_controller: impl BlueController + 'static) -> Self {
        Self {
            random_check: Self::gen_random_check(),

            device_controller: Arc::new(Mutex::new(device_controller)),
            device_info: Arc::new(Mutex::new(None)),
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
    }

    /// Get a copy of the current device settings
    pub fn get_setting(&self) -> Option<SettingsData> {
        self.device_info.lock().unwrap()
            .as_ref()
            .map(|resp| resp.settings.clone())
    }

    pub async fn set_settings(&self, new_settings: SettingsData) {
        let mut info_lock = self.device_info.lock().unwrap();
        if let Some(ref mut response) = *info_lock {
            response.settings = new_settings;
            // Generate the settings command string
            let cmd = CommandGenerator::get_setting_cmd(&response.settings);
            // Send the command to the device
            let mut controller = self.device_controller.lock().unwrap();
            if let Err(e) = controller.send(&cmd).await {
                error!("Failed to send settings command: {:?}", e);
            }
        }
    }


        /// Get a copy of the current main command data
    pub fn get_command_data(&self) -> Option<MainCommandData> {
        self.device_info.lock().unwrap()
            .as_ref()
            .map(|resp| resp.main_data.clone())
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
        self.device_info.lock().unwrap().clone()
    }
}
