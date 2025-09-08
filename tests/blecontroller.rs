use darkelf::{ble, winble,util};
use log::{error, info};
use windows::Devices::Enumeration::DeviceInformation;
use anyhow::{anyhow, Ok};

#[tokio::main]
#[test]
async fn test_connector_scan() -> Result<(), anyhow::Error> {

    util::setup_logging();

    let devices: Vec<DeviceInformation> = winble::scan_laser_devices()
        .await
        .map_err(|e| { 
            error!("Failed to scan for devices: {:?}", e); 
            anyhow!(e.to_string())
        })?;
    info!("Found {:?} devices", devices.len());
    assert!(!devices.is_empty(), "No devices found");
    Ok(())
}

async fn test_make_connection() -> Result<(), anyhow::Error> {
   util::setup_logging();

    let devices: Vec<DeviceInformation> = winble::scan_laser_devices()
        .await
        .map_err(|e| { 
            error!("Failed to scan for devices: {:?}", e); 
            anyhow!(e.to_string())
        })?;

    let controller = crate::winble::WinBleController::new(devices.get(0));
    controller.connect().await?;
    assert!(controller.is_connected(), "Controller is not connected");

   Ok(())
}