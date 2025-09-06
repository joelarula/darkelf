use darkelf::{ble, winble};
use log::error;
use windows::Devices::Enumeration::DeviceInformation;
use anyhow::{anyhow, Ok};

#[tokio::main]
#[test]
async fn test_ble_connector_scan() -> Result<(), anyhow::Error> {

    let devices: Vec<DeviceInformation> = winble::scan_laser_devices()
        .await
        .map_err(|e| { error!("Failed to scan for devices: {:?}", e); anyhow!(e) })?;
    assert!(!devices.is_empty(), "No devices found");
    Ok(())
}