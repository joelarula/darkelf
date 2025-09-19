use log::{debug, info, error};
use std::sync::{Arc, Mutex};
use rand;

use crate::model::DeviceInfo;
use crate::command::{CommandGenerator};
use crate::blue::BlueController;

pub struct LaserDevice {
    random_check: Vec<u8>,
    device_controller: Arc<Mutex<dyn BlueController>>,
    device_info: Arc<Mutex<DeviceInfo>>,
}

impl LaserDevice {
    /// Create a new LaserDevice instance with initialized random check bytes and device controller
    pub fn new(device_controller: impl BlueController + 'static) -> Self {
        Self {
            random_check: Self::gen_random_check(),
            device_controller: Arc::new(Mutex::new(device_controller)),
            device_info: Arc::new(Mutex::new(DeviceInfo {
                device_on: false,
                device_type: String::new(),
                version: String::new(),
                user_type: String::new(),
            })),
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
                        if let Some(new_info) = response.device_info {
                            info!(
                                "Device info updated - Power: {}, Type: {}, Version: {}, User Type: {}", 
                                if new_info.device_on { "ON" } else { "OFF" },
                                new_info.device_type,
                                new_info.version, 
                                new_info.user_type
                            );
                            
                            // Update device state with parsed info
                            if let Ok(mut info) = device_info.lock() {
                                *info = new_info;
                            }
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

    pub fn on(&self) {
        info!("LaserDevice: on");
    }

    pub fn off(&self) {
        info!("LaserDevice: off");
    }

    /// Generate random verification bytes
    fn gen_random_check() -> Vec<u8> {
        let bytes: Vec<u8> = (0..4).map(|_| rand::random::<u8>()).collect();
        info!("Generated random check bytes: {:02X?}", bytes);
        bytes
    }

    /// Get the current device power state
    pub fn is_on(&self) -> bool {
        self.device_info.lock().unwrap().device_on
    }

    /// Get the device type
    pub fn get_device_type(&self) -> String {
        self.device_info.lock().unwrap().device_type.clone()
    }

    /// Get the firmware version
    pub fn get_version(&self) -> String {
        self.device_info.lock().unwrap().version.clone()
    }

    /// Get the user type
    pub fn get_user_type(&self) -> String {
        self.device_info.lock().unwrap().user_type.clone()
    }
    
    /// Get a copy of the entire device info
    pub fn get_device_info(&self) -> DeviceInfo {
        self.device_info.lock().unwrap().clone()
    }
}
