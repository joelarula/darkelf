use std::thread::sleep;
use std::time::Duration;

use darkelf::controller::DeviceController;
use darkelf::mock::MockController;
use darkelf::util;
use darkelf::device::LaserDevice; 
use anyhow::{anyhow, Ok};

#[tokio::main]
#[test]
async fn test_laser_device() -> Result<(), anyhow::Error> {

    util::setup_logging();

    let controller = MockController::new();
    let _ = controller.connect();
    assert!(controller.is_connected());

    let device = LaserDevice::new(controller);
    device.setup();
    device.off();
    sleep(Duration::from_millis(1000));
    device.on();
    
    Ok(())
}