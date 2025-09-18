use log::{debug, info, error};
use std::sync::{Arc, Mutex};
use rand;

use crate::command::CommandGenerator;
use crate::blue::BlueController;

pub struct LaserDevice {
    random_check: Vec<u8>,
    device_controller: Arc<Mutex<dyn BlueController>>,
}

impl LaserDevice {
    /// Create a new LaserDevice instance with initialized random check bytes and device controller
    pub fn new(device_controller: impl BlueController + 'static) -> Self {
        Self {
            random_check: Self::gen_random_check(),
            device_controller: Arc::new(Mutex::new(device_controller)),
        }
    }

    pub async fn setup(&self) {
        debug!("LaserDevice: setup");
        {
            let mut controller = self.device_controller.lock().unwrap();
            controller.set_receiver_callback(Box::new(|data| {
                info!("Received data: {}", data);
            }));
            // Lock is released here when controller goes out of scope
        }
        
        let cmd = CommandGenerator::get_query_cmd(&self.random_check);
        debug!("get_query_cmd: {}", cmd);        
        let mut controller = self.device_controller.lock().unwrap();
       // let init_cmd = "E0E1E2E38BCE183AE4E5E6E70000000000000000";
        //let init_cmdB = "E0E1E2E38BCE183AE4E5E6E7";
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
}
