
use std::env;
use std::time::Duration;

use anyhow::{anyhow, Ok};
use darkelf::{ blue, laser::{CommandGenerator, LaserController, SettingParams}, util, winblue::{self, WinBlueController}};
use log::{debug, error, info};
use windows::Devices::Enumeration::DeviceInformation;
use tokio::time::sleep;

#[tokio::main]
#[test]
async fn test_toggle_color_mode() -> Result<(), anyhow::Error> {
    // Ensure debug logs are shown
    unsafe {
        env::set_var("RUST_LOG", "debug");
    }
    util::setup_logging();
    let blecontroller = setup_controller().await?;
    let mut laser = LaserController::new(blecontroller).await.map_err(|e: Box<dyn std::error::Error>| anyhow!(e.to_string()))?;

    // Create CommandGenerator
    let generator = CommandGenerator::new();

    // Helper function to create settings with specified mode
    let create_settings = |mode: u8| {
        SettingParams::new(
            [0x00, 0x00, 0xFF, 0xFF, 0xFF], // Full RGB on for visibility
            0x01,                           // ch (type)
            0x32,                          // xy: 50 (0x32)
            0xFF,                           // light (full intensity)
            mode,                           // cfg: Mode (03 or 01)
            0x00,                           // lang
        )
    };





    // Step 1: Send initialization command (from test1arrays.c, e.g., pkt446)
    let init_cmd = "E0E1E2E38BCE183AE4E5E6E7";
    laser.send(init_cmd, None).await.map_err(|e| anyhow!(e))?;
    sleep(Duration::from_millis(100)).await; // Wait for device to process

    // Send the sequence of commands matching frames 272, 277, 327, 328, and 333
    // but with lower intensity values (25 instead of 50-55)

    // Send alternating commands between mode 03 and 01
    // First pair - Mode 03 then 01
    let cmd = generator.get_setting_cmd(create_settings(0x03));
    laser.send(&cmd, None).await.map_err(|e| anyhow!(e))?;
    sleep(Duration::from_millis(1500)).await; // ~1.57s delay from capture

    let cmd = generator.get_setting_cmd(create_settings(0x01));
    laser.send(&cmd, None).await.map_err(|e| anyhow!(e))?;
    sleep(Duration::from_millis(1000)).await; // Minimum 1 second delay

    // Second pair
    let cmd = generator.get_setting_cmd(create_settings(0x03));
    laser.send(&cmd, None).await.map_err(|e| anyhow!(e))?;
    sleep(Duration::from_millis(8600)).await; // ~8.64s delay from capture

    let cmd = generator.get_setting_cmd(create_settings(0x01));
    laser.send(&cmd, None).await.map_err(|e| anyhow!(e))?;
    sleep(Duration::from_millis(4200)).await; // ~4.20s delay from capture

    // Third pair
    let cmd = generator.get_setting_cmd(create_settings(0x03));
    laser.send(&cmd, None).await.map_err(|e| anyhow!(e))?;

    Ok(())
}



async fn setup_controller() -> Result<crate::winblue::WinBlueController, anyhow::Error> {
    let devices: Vec<DeviceInformation> = winblue::scan_laser_devices()
        .await
        .map_err(|e| {
            error!("Failed to scan for devices: {:?}", e);
            anyhow!(e.to_string())
        })?;
    if devices.is_empty() {
        return Err(anyhow!("No devices found"));
    }
    let mut controller = crate::winblue::WinBlueController::new(devices.get(0))
        .await
        .map_err(|e| anyhow!(e.to_string()))?;
    controller.connect()
        .await
        .map_err(|e| anyhow!(e.to_string()))?;
    if !controller.is_connected() {
        return Err(anyhow!("Controller is not connected"));
    }
    Ok(controller)
}