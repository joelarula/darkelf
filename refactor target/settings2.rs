use std::env;
use std::time::Duration;

use anyhow::{anyhow, Ok};
use darkelf::{blue, laser::{CommandGenerator, LaserController, LaserPoint, LaserOptions, SettingParams}, util, winblue::WinBlueController};
use log::{debug, error, info};
use windows::Devices::Enumeration::DeviceInformation;
use tokio::time::sleep;

#[tokio::main]
#[test]
async fn test_toggle_color_mode() -> Result<(), anyhow::Error> {
    unsafe {
        env::set_var("RUST_LOG", "debug");
    }
    util::setup_logging();
    let blecontroller = setup_controller().await?;
    let mut laser = LaserController::new(blecontroller).await.map_err(|e: Box<dyn std::error::Error>| anyhow!(e.to_string()))?;

    let generator = CommandGenerator::new();

    let create_settings = |mode: u8| {
        SettingParams::new(
            [0x00, 0x00, 0xFF, 0xFF, 0xFF],
            0x01,
            0x32,
            0xFF,
            mode,
            0x00,
        )
    };

    let init_cmd = "E0E1E2E38BCE183AE4E5E6E70000000000000000";
    laser.send(init_cmd, None).await.map_err(|e| anyhow!(e))?;
    sleep(Duration::from_millis(100)).await;

    let cmd = generator.get_setting_cmd(create_settings(0x03));
    laser.send(&cmd, None).await.map_err(|e| anyhow!(e))?;
    sleep(Duration::from_millis(1500)).await;

    let cmd = generator.get_setting_cmd(create_settings(0x01));
    laser.send(&cmd, None).await.map_err(|e| anyhow!(e))?;
    sleep(Duration::from_millis(1000)).await;

    let cmd = generator.get_setting_cmd(create_settings(0x03));
    laser.send(&cmd, None).await.map_err(|e| anyhow!(e))?;
    sleep(Duration::from_millis(8600)).await;

    let cmd = generator.get_setting_cmd(create_settings(0x01));
    laser.send(&cmd, None).await.map_err(|e| anyhow!(e))?;
    sleep(Duration::from_millis(4200)).await;

    let cmd = generator.get_setting_cmd(create_settings(0x03));
    laser.send(&cmd, None).await.map_err(|e| anyhow!(e))?;

    Ok(())
}

#[tokio::main]
#[test]
async fn test_send_animation() -> Result<(), anyhow::Error> {
    unsafe {
        env::set_var("RUST_LOG", "debug");
    }
    util::setup_logging();
    let blecontroller = setup_controller().await?;
    let mut laser = LaserController::new(blecontroller).await.map_err(|e: Box<dyn std::error::Error>| anyhow!(e.to_string()))?;

    let init_cmd = "E0E1E2E38BCE183AE4E5E6E70000000000000000";
    laser.send(init_cmd, None).await.map_err(|e| anyhow!(e))?;
    sleep(Duration::from_millis(100)).await;

    let points = vec![
        LaserPoint::new(100, 200, 1, [255, 0, 0]),
        LaserPoint::new(150, 250, 0, [0, 255, 0]),
        LaserPoint::new(200, 300, 1, [0, 0, 255]),
        LaserPoint::new(50, 100, 1, [128, 128, 128]),
    ];
    laser.send_animation(&points).await.map_err(|e| anyhow!(e))?;
    sleep(Duration::from_millis(100)).await;

    if let Some(ref mut dmx_port) = laser.dmx_port {
        let mut buffer = [0u8; 512];
        dmx_port.read(&mut buffer).map_err(|e| anyhow!(e.to_string()))?;
        debug!("DMX output: R={}, G={}, B={}, x={:02X}{:02X}, y={:02X}{:02X}, z={}",
               buffer[0], buffer[1], buffer[2], buffer[3], buffer[4], buffer[5], buffer[6], buffer[7]);
        assert_eq!(buffer[0], 128, "Red channel mismatch");
        assert_eq!(buffer[1], 128, "Green channel mismatch");
        assert_eq!(buffer[2], 128, "Blue channel mismatch");
        assert_eq!(buffer[3], 50 & 0xFF, "x low byte mismatch");
        assert_eq!(buffer[4], 50 >> 8, "x high byte mismatch");
        assert_eq!(buffer[5], 100 & 0xFF, "y low byte mismatch");
        assert_eq!(buffer[6], 100 >> 8, "y high byte mismatch");
        assert_eq!(buffer[7], 1, "z mismatch");
    } else {
        return Err(anyhow!("DMX port not initialized"));
    }

    Ok(())
}

#[tokio::main]
#[test]
async fn test_animation_with_notifications() -> Result<(), anyhow::Error> {
    unsafe {
        env::set_var("RUST_LOG", "debug");
    }
    util::setup_logging();
    let blecontroller = setup_controller().await?;
    let mut laser = LaserController::new(blecontroller).await.map_err(|e: Box<dyn std::error::Error>| anyhow!(e.to_string()))?;

    let init_cmd = "E0E1E2E38BCE183AE4E5E6E70000000000000000";
    laser.send(init_cmd, None).await.map_err(|e| anyhow!(e))?;
    sleep(Duration::from_millis(100)).await;

    let points = vec![
        LaserPoint::new(100, 200, 1, [255, 0, 0]),
        LaserPoint::new(150, 250, 1, [0, 255, 0]),
    ];
    laser.send_animation(&points).await.map_err(|e| anyhow!(e))?;
    sleep(Duration::from_millis(1500)).await;

    if laser.blu_rec_content.is_empty() {
        return Err(anyhow!("No notifications received after animation"));
    }

    let hex = laser.blu_rec_content.back().cloned().unwrap_or_default();
    laser.parse_response(&hex).await.map_err(|e| anyhow!(e))?;
    debug!("Parsed options: {:?}", laser.options);
    debug!("Parsed project data: {:?}", laser.project_data);

    if !hex.contains("C0C1C2C3") || !hex.contains("C4C5C6C7") {
        debug!("Notification content: {}", hex);
        return Err(anyhow!("Expected C0C1C2C3...C4C5C6C7 in notification"));
    }

    if let Some(ref mut dmx_port) = laser.dmx_port {
        let mut buffer = [0u8; 512];
        dmx_port.read(&mut buffer).map_err(|e| anyhow!(e.to_string()))?;
        debug!("DMX output: R={}, G={}, B={}, x={:02X}{:02X}, y={:02X}{:02X}, z={}",
               buffer[0], buffer[1], buffer[2], buffer[3], buffer[4], buffer[5], buffer[6], buffer[7]);
        assert_eq!(buffer[1], 255, "Green channel mismatch for last point");
    } else {
        return Err(anyhow!("DMX port not initialized"));
    }

    Ok(())
}

#[tokio::main]
#[test]
async fn test_notification_handling() -> Result<(), anyhow::Error> {
    unsafe {
        env::set_var("RUST_LOG", "debug");
    }
    util::setup_logging();
    let blecontroller = setup_controller().await?;
    let mut laser = LaserController::new(blecontroller).await.map_err(|e: Box<dyn std::error::Error>| anyhow!(e.to_string()))?;

    let init_cmd = "E0E1E2E38BCE183AE4E5E6E70000000000000000";
    laser.send(init_cmd, None).await.map_err(|e| anyhow!(e))?;
    sleep(Duration::from_millis(100)).await;

    let generator = CommandGenerator::new();
    let settings = SettingParams::new(
        [0x00, 0x00, 0xFF, 0xFF, 0xFF],
        0x01,
        0x32,
        0xFF,
        0x03,
        0x00,
    );
    let cmd = generator.get_setting_cmd(settings);
    laser.send(&cmd, None).await.map_err(|e| anyhow!(e))?;
    sleep(Duration::from_millis(1500)).await;

    if laser.blu_rec_content.is_empty() {
        return Err(anyhow!("No notifications received"));
    }

    let hex = laser.blu_rec_content.back().cloned().unwrap_or_default();
    laser.parse_response(&hex).await.map_err(|e| anyhow!(e))?;
    debug!("Parsed options: {:?}", laser.options);
    debug!("Parsed project data: {:?}", laser.project_data);

    assert!(laser.options.tx_color <= 9, "Invalid tx_color: {}", laser.options.tx_color);
    assert!(laser.options.tx_size >= 10 && laser.options.tx_size <= 100, "Invalid tx_size: {}", laser.options.tx_size);

    Ok(())
}

#[tokio::main]
#[test]
async fn test_generate_command() -> Result<(), anyhow::Error> {
    unsafe {
        env::set_var("RUST_LOG", "debug");
    }
    util::setup_logging();
    let blecontroller = setup_controller().await?;
    let mut laser = LaserController::new(blecontroller).await.map_err(|e: Box<dyn std::error::Error>| anyhow!(e.to_string()))?;

    let init_cmd = "E0E1E2E38BCE183AE4E5E6E70000000000000000";
    laser.send(init_cmd, None).await.map_err(|e| anyhow!(e))?;
    sleep(Duration::from_millis(100)).await;

    let generator = CommandGenerator::new();
    let points = vec![
        (0, vec![
            LaserPoint::new(100, 200, 1, [255, 0, 0]),
            LaserPoint::new(150, 250, 1, [0, 255, 0]),
        ]),
        (1, vec![
            LaserPoint::new(200, 300, 1, [0, 0, 255]),
        ]),
    ];
    let options = LaserOptions::new();
    let cmd = generator.generate_command(points, 1.0, options, 0, 0).ok_or_else(|| anyhow!("Failed to generate command"))?;
    
    assert_eq!(cmd.point_count, 3, "Incorrect point count");
    assert!(!cmd.command_data.is_empty(), "Command data is empty");
    debug!("Generated command: {:?}", cmd.to_hex_string());

    for ble_cmd in cmd.to_ble_command() {
        let hex_cmd = hex::encode(&ble_cmd);
        laser.send(&hex_cmd, None).await.map_err(|e| anyhow!(e))?;
        sleep(Duration::from_millis(20)).await;
    }

    Ok(())
}

#[tokio::main]
#[test]
async fn test_settings_variations() -> Result<(), anyhow::Error> {
    unsafe {
        env::set_var("RUST_LOG", "debug");
    }
    util::setup_logging();
    let blecontroller = setup_controller().await?;
    let mut laser = LaserController::new(blecontroller).await.map_err(|e: Box<dyn std::error::Error>| anyhow!(e.to_string()))?;

    let init_cmd = "E0E1E2E38BCE183AE4E5E6E70000000000000000";
    laser.send(init_cmd, None).await.map_err(|e| anyhow!(e))?;
    sleep(Duration::from_millis(100)).await;

    let generator = CommandGenerator::new();
    let settings_variations = vec![
        SettingParams::new([0xFF, 0x00, 0x00, 0xFF, 0xFF], 0x01, 0x64, 0x80, 0x03, 0x01),
        SettingParams::new([0x00, 0xFF, 0x00, 0xFF, 0xFF], 0x02, 0x32, 0xFF, 0x01, 0x00),
        SettingParams::new([0x00, 0x00, 0xFF, 0xFF, 0xFF], 0x03, 0x1E, 0x40, 0x00, 0x02),
    ];

    for settings in settings_variations {
        let cmd = generator.get_setting_cmd(settings);
        debug!("Sending settings command: {}", cmd);
        laser.send(&cmd, None).await.map_err(|e| anyhow!(e))?;
        sleep(Duration::from_millis(1000)).await;
    }

    Ok(())
}

#[tokio::main]
#[test]
async fn test_cleanup() -> Result<(), anyhow::Error> {
    unsafe {
        env::set_var("RUST_LOG", "debug");
    }
    util::setup_logging();
    let blecontroller = setup_controller().await?;
    let mut laser = LaserController::new(blecontroller).await.map_err(|e: Box<dyn std::error::Error>| anyhow!(e.to_string()))?;

    let init_cmd = "E0E1E2E38BCE183AE4E5E6E70000000000000000";
    laser.send(init_cmd, None).await.map_err(|e| anyhow!(e))?;
    sleep(Duration::from_millis(100)).await;

    let generator = CommandGenerator::new();
    let settings = SettingParams::new(
        [0x00, 0x00, 0xFF, 0xFF, 0xFF],
        0x01,
        0x32,
        0xFF,
        0x03,
        0x00,
    );
    let cmd = generator.get_setting_cmd(settings);
    laser.send(&cmd, None).await.map_err(|e| anyhow!(e))?;
    sleep(Duration::from_millis(1000)).await;

    laser.cleanup().await.map_err(|e| anyhow!(e))?;
    assert!(!laser.is_connected(), "Controller should not be connected after cleanup");

    let result = laser.send(&cmd, None).await;
    assert!(result.is_err(), "Send should fail after cleanup");

    Ok(())
}

#[tokio::main]
#[test]
async fn test_scan_dmx_channel_4() -> Result<(), anyhow::Error> {
    unsafe {
        env::set_var("RUST_LOG", "debug");
    }
    util::setup_logging();
    let blecontroller = setup_controller().await?;
    let mut laser = LaserController::new(blecontroller).await.map_err(|e: Box<dyn std::error::Error>| anyhow!(e.to_string()))?;

    let init_cmd = "E0E1E2E38BCE183AE4E5E6E70000000000000000";
    laser.send(init_cmd, None).await.map_err(|e| anyhow!(e))?;
    sleep(Duration::from_millis(100)).await;

    let generator = CommandGenerator::new();
    let options = LaserOptions::new();
    let mut notification_count = 0;

    for x in 0..=255 {
        // Create a single point with varying x (DMX channel 4), fixed y=50, RGB=[255, 0, 0], z=1
        let points = vec![LaserPoint::new(x, 50, 1, [255, 0, 0])];
        let cmd = generator.generate_command(
            vec![(0, points)],
            1.0,
            options.clone(),
            0,
            0,
        ).ok_or_else(|| anyhow!("Failed to generate command for x={}", x))?;

        // Try to use to_ble_command, fall back to manual construction if needed
        let ble_cmd = if !cmd.to_ble_command().is_empty() {
            let ble_cmds = cmd.to_ble_command();
            ble_cmds[0].clone()
        } else {
            debug!("Falling back to manual BLE command construction for x={}", x);
            vec![
                0xE0, 0xE1, 0xE2, 0xE3,                    // Header
                0xC0, 0xC1, 0xC2, 0xC3,                    // Placeholder
                (x & 0xFF) as u8, (x >> 8) as u8,           // x low, high
                (50 & 0xFF) as u8, (50 >> 8) as u8,         // y low, high
                255, 0, 0, 1,                               // RGB, z
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 7-byte padding
            ]
        };
        let hex_cmd = hex::encode(&ble_cmd);
        debug!("Sending command for x={}: {}", x, hex_cmd);
        laser.send(&hex_cmd, None).await.map_err(|e| anyhow!(e))?;

        // Verify DMX output
        if let Some(ref mut dmx_port) = laser.dmx_port {
            let mut buffer = [0u8; 512];
            dmx_port.read(&mut buffer).map_err(|e| anyhow!(e.to_string()))?;
            debug!("DMX output for x={}: R={}, G={}, B={}, x={:02X}{:02X}, y={:02X}{:02X}, z={}",
                   x, buffer[0], buffer[1], buffer[2], buffer[3], buffer[4], buffer[5], buffer[6], buffer[7]);
            assert_eq!(buffer[0], 255, "Red channel mismatch for x={}", x);
            assert_eq!(buffer[1], 0, "Green channel mismatch for x={}", x);
            assert_eq!(buffer[2], 0, "Blue channel mismatch for x={}", x);
            assert_eq!(buffer[3], x as u8, "x low byte (DMX channel 4) mismatch for x={}", x);
            assert_eq!(buffer[4], 0, "x high byte (DMX channel 5) mismatch for x={}", x);
            assert_eq!(buffer[5], 50 & 0xFF, "y low byte (DMX channel 6) mismatch for x={}", x);
            assert_eq!(buffer[6], 50 >> 8, "y high byte (DMX channel 7) mismatch for x={}", x);
            assert_eq!(buffer[7], 1, "z (DMX channel 8) mismatch for x={}", x);
        } else {
            return Err(anyhow!("DMX port not initialized for x={}", x));
        }

        // Check notifications
        if !laser.blu_rec_content.is_empty() {
            notification_count += 1;
            let hex = laser.blu_rec_content.back().cloned().unwrap_or_default();
            debug!("Notification for x={}: {}", x, hex);
            laser.parse_response(&hex).await.map_err(|e| anyhow!(e))?;
        }

        sleep(Duration::from_millis(20)).await; // Respect device timing
    }

    debug!("Total notifications received: {}", notification_count);
    if notification_count == 0 {
        debug!("Warning: No notifications received during DMX channel 4 scan");
    }

    laser.cleanup().await.map_err(|e| anyhow!(e))?;
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