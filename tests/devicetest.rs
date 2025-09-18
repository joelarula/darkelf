use std::env;
use std::thread::sleep;
use std::time::Duration;

use darkelf::blue::BlueController as _;
use darkelf::winblue::{ self, WinBlueController};
use darkelf::mock::MockController;
use darkelf::util;
use darkelf::device::LaserDevice;
use darkelf::command::CommandGenerator;
use anyhow::{anyhow, Ok};
use windows::Devices::Enumeration::DeviceInformation;
use log::{error, info};

#[tokio::main]
#[test]
async fn test_laser_device_mock() -> Result<(), anyhow::Error> {

    util::setup_logging();
    unsafe {
        env::set_var("RUST_LOG", "debug");
    }
    let mut controller = MockController::new();
    let _ = controller.connect();
    assert!(controller.is_connected());
    let mut device: LaserDevice = LaserDevice::new(controller);
    test_laser_device_functionality(&mut device).await;

    Ok(())
}


#[tokio::main]
#[test]
async fn test_laser_device() -> Result<(), anyhow::Error> {

    util::setup_logging();
        unsafe {
        env::set_var("RUST_LOG", "debug");
    }
    let devices: Vec<DeviceInformation> = winblue::scan_laser_devices()
        .await
        .map_err(|e| { 
            error!("Failed to scan for devices: {:?}", e); 
            anyhow!(e.to_string())
        })?;

    let mut controller = WinBlueController::new(devices.get(0)).await
        .map_err(|e| anyhow!(e.to_string()))?;

    let _ = controller.connect().await
        .map_err(|e| anyhow!(e.to_string()))?;

    assert!(controller.is_connected());
    let mut device: LaserDevice = LaserDevice::new(controller);
    test_laser_device_functionality(&mut device).await;
    
    Ok(())

}


async fn test_laser_device_functionality(device: &mut LaserDevice) {
    // Initialize the device
    device.setup().await;
    
    // Give some time for the setup command to complete
    sleep(Duration::from_millis(500));
    
    // Send the off command and wait for it to complete
    device.off();
    sleep(Duration::from_millis(500));
    
    // Send the on command
    device.on();
    
    // Final wait to ensure all commands complete
    sleep(Duration::from_millis(500));
}




#[test]
fn test_check_received_data() {
    use std::println;
    util::setup_logging();
    unsafe {
        env::set_var("RUST_LOG", "debug");
    }

    // Test data from the log
    let random_data = [0xED, 0x00, 0x05, 0xD5];
    
    // These are the expected values from actual device response
    let expected = [0x88, 0x7F, 0x42, 0x82];
    println!("Expected verification bytes: {:02X?}", expected);
    
    let received_data = "E0E1E2E3B0B1B2B3FFB4B5B6B7C0C1C2C306000994943838A5007000000000512E80FFFFFFFFFFFFFFFF80000000000000000080FFFFFFFFFFFFFFFF80FFFFFFFFFFFFFFFF0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000C4C5C6C7000102030001003000646464030000000000000004050607D0D1D2D38100F52000000000000000000000003200FFD4D5D6D7000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000D4D5D6D7887F4282FF000200E4E5E6E7";
    println!("Response verification part: {}", &received_data[received_data.len() - 24..received_data.len() - 16]);
    
    // Print expected verification bytes
    println!("Expected verification bytes: {:02X?}", expected);
    
    // Get verification part from response (8 bytes)
    let response_verify = &received_data[received_data.len() - 24..received_data.len() - 16];
    println!("Response verification part: {}", response_verify);
    
    // Parse received verification bytes
    let mut received = Vec::with_capacity(4);
    for i in 0..4 {
        let hex_pair = &response_verify[i*2..i*2+2];
        let value = u8::from_str_radix(hex_pair, 16).unwrap();
        received.push(value);
    }
    println!("Received verification bytes: {:02X?}", received);
    
    // Execute the verification
    let (success, device_info) = CommandGenerator::check_received_data(received_data, &random_data);
    
    // Verify the results
    assert!(success, "Verification should pass");
    
    let device_info = device_info.expect("Device info should be present");
    assert_eq!(device_info.device_on, true, "Device should be on");
    assert_eq!(device_info.device_type, "02", "Device type should be '02'");
    assert_eq!(device_info.version, "00", "Version should be '00'");
    assert_eq!(device_info.user_type, "FF", "User type should be 'FF'");
       

}
