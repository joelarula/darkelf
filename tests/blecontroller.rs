use darkelf::{ util, winblue};
use log::{error, info};
use windows::Devices::Enumeration::DeviceInformation;
use anyhow::{anyhow, Ok};

#[tokio::main]
#[test]
async fn test_connector_scan() -> Result<(), anyhow::Error> {

    util::setup_logging();

    let devices: Vec<DeviceInformation> = winblue::scan_laser_devices()
        .await
        .map_err(|e| { 
            error!("Failed to scan for devices: {:?}", e); 
            anyhow!(e.to_string())
        })?;
    info!("Found {:?} devices", devices.len());
    assert!(!devices.is_empty(), "No devices found");
    Ok(())
}

#[tokio::main]
#[test]
async fn test_make_connection() -> Result<(), anyhow::Error> {
   util::setup_logging();

    let devices: Vec<DeviceInformation> = winblue::scan_laser_devices()
        .await
        .map_err(|e| { 
            error!("Failed to scan for devices: {:?}", e); 
            anyhow!(e.to_string())
        })?;

    let mut controller = crate::winblue::WinBlueController::new(devices.get(0))
        .await
        .map_err(|e| anyhow!(e.to_string()))?;
    controller.connect()
        .await
        .map_err(|e| anyhow!(e.to_string()))?;
    assert!(controller.is_connected(), "Controller is not connected");


   Ok(())
}


#[tokio::main]
#[test]
async fn test_read_characteristics() -> Result<(), anyhow::Error> {
   
    util::setup_logging();

    let devices: Vec<DeviceInformation> = winblue::scan_laser_devices()
        .await
        .map_err(|e| { 
            error!("Failed to scan for devices: {:?}", e); 
            anyhow!(e.to_string())
        })?;

    let mut controller = crate::winblue::WinBlueController::new(devices.get(0))
        .await
        .map_err(|e| anyhow!(e.to_string()))?;
    controller.connect()
        .await
        .map_err(|e| anyhow!(e.to_string()))?;
    assert!(controller.is_connected(), "Controller is not connected");

    controller.discover_characteristics()
        .await
        .map_err(|e| anyhow!(e.to_string()))?; 


   Ok(())
}