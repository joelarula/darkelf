use log::{debug, info};
use std::sync::{Arc, Mutex};
use rand;

use crate::command::CommandGenerator;
use crate::controller::DeviceController;

pub struct LaserDevice {
    random_check: Vec<u8>,
    device_controller: Arc<Mutex<dyn DeviceController>>,
}

impl LaserDevice {
    /// Create a new LaserDevice instance with initialized random check bytes and device controller
    pub fn new(device_controller: impl DeviceController + 'static) -> Self {
        Self {
            random_check: Self::gen_random_check(),
            device_controller: Arc::new(Mutex::new(device_controller)),
        }
    }

    pub fn setup (&self) {
        info!("LaserDevice: setup");
        let cmd = CommandGenerator::get_query_cmd(&self.random_check);
        info!("get_query_cmd: {}", cmd);
        // Send hex string directly like in JavaScript setCmdData
        let _ = self.device_controller.lock().unwrap().send(&cmd);
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
}
