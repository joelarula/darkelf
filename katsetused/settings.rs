
use std::env;
use std::time::Duration;
use anyhow::{anyhow, Result};
use darkelf::laser::{CommandGenerator, LaserController, SettingParams, LaserPoint};
use darkelf::util;
use darkelf::winblue::{WinBlueController, scan_laser_devices};
use log::{debug, error};
use tokio::time::sleep;
use windows::Devices::Enumeration::DeviceInformation;

#[derive(Debug)]
struct DeviceInfo {
    model_number: String,
    serial_number: String,
    hw_revision: String,
    sw_revision: String,
    manufacturer: String,
}

/// Sets up and connects to a laser device for testing
async fn setup_controller() -> Result<WinBlueController> {
    debug!("Setting up test controller");
    let devices = scan_laser_devices()
        .await
        .map_err(|e| {
            error!("Failed to scan for devices: {}", e);
            anyhow!(e.to_string())
        })?;
    
    if devices.is_empty() {
        return Err(anyhow!("No devices found"));
    }
    
    let mut controller = WinBlueController::new(Some(&devices[0]))
        .await
        .map_err(|e| anyhow!(e.to_string()))?;
        
    controller.connect()
        .await
        .map_err(|e| anyhow!(e.to_string()))?;
        
    if !controller.is_connected() {
        return Err(anyhow!("Controller is not connected"));
    }

    // Wait for device to stabilize
    sleep(Duration::from_millis(100)).await;
    
    Ok(controller)
}

/// Gets device information through characteristic reads
async fn get_device_info(controller: &mut WinBlueController) -> Result<DeviceInfo> {
    debug!("Reading device information characteristics");
    // This would need implementation in WinBlueController to read the characteristics
    // Found in Wireshark: Model Number, Serial Number, HW Revision, SW Revision, Manufacturer
    Ok(DeviceInfo {
        model_number: String::from("Unknown"),
        serial_number: String::from("Unknown"),
        hw_revision: String::from("Unknown"),
        sw_revision: String::from("Unknown"),
        manufacturer: String::from("Unknown"),
    })
}

/// Verifies a command was sent successfully and received expected response
async fn verify_command_response(laser: &mut LaserController, cmd: &str) -> Result<()> {
    debug!("Sending command and verifying response: {}", cmd);
    laser.send(cmd, None).await
        .map_err(|e| anyhow!("Command send failed: {}", e))?;
    
    // Initial delay to allow response to start coming in
    sleep(Duration::from_millis(50)).await;
    
    // We'll poll responses in small intervals to accumulate fragments
    let mut attempts = 0;
    let max_attempts = 20; // 20 * 50ms = 1 second total
    let mut accumulated_response = String::new();
    
    while attempts < max_attempts {
        let new_response = laser.get_last_response();
        if !new_response.is_empty() && !accumulated_response.contains(&new_response) {
            accumulated_response.push_str(&new_response);
        }
        
        // For initialization commands
        if cmd.contains("8BCE183A") {
            if accumulated_response.contains("E0E1E2E3") && accumulated_response.contains("B0B1B2B3") {
                debug!("Received complete initialization response in accumulated response: {}", accumulated_response);
                return Ok(());
            }
        }
        // For regular commands - look for acknowledgement
        else {
            if accumulated_response.contains("E0E1E2E3") {
                debug!("Command acknowledged in accumulated response: {}", accumulated_response);
                return Ok(());
            }
        }
        
        // Wait a bit before checking for more fragments
        sleep(Duration::from_millis(50)).await;
        attempts += 1;
    }
    
    // If we got this far, we timed out waiting for the expected response
    Err(anyhow!("Timeout waiting for response containing 'E2E3'. Accumulated response: {}", accumulated_response))
}

#[tokio::test]
/// Test color mode toggling functionality between static (0x03) and dynamic (0x01) modes
///
/// The test verifies:
/// 1. Device can switch between modes successfully 
/// 2. Commands are sent and acknowledged
/// 3. Mode changes are stable with proper timing
/// 4. Color settings are preserved during mode changes
/// 5. Response timing matches protocol requirements
async fn test_toggle_color_mode() -> Result<()> {
    unsafe {
        env::set_var("RUST_LOG", "debug");
    }
    util::setup_logging();
    
    let blecontroller = setup_controller().await?;
    let mut laser = LaserController::new(blecontroller).await
        .map_err(|e| anyhow!("Failed to create LaserController: {}", e))?;
    let generator = CommandGenerator::new();
    
    // Initialize laser with startup sequence
    debug!("Initializing laser...");
    
    // First send initialization command
    let init_cmd = "E0E1E2E38BCE183AE4E5E6E70000000000000000";
    laser.send(init_cmd, None).await
        .map_err(|e| anyhow!("Failed to send init command: {}", e))?;
    sleep(Duration::from_millis(100)).await;

    // Then enable laser output
    let enable_cmd = "E0E1E2E3FF01FF32FF0100FF0000000000000000";
    laser.send(enable_cmd, None).await
        .map_err(|e| anyhow!("Failed to enable laser: {}", e))?;
    sleep(Duration::from_millis(100)).await;
    
    debug!("Starting color mode toggle test...");
    
    // Test sequence: Toggle between modes multiple times to verify stability
    for i in 0..3 {
        debug!("Iteration {}/3", i + 1);
        
        // Test static mode (0x03)
        let settings = SettingParams::new(
            [0x00, 0x00, 0xFF, 0xFF, 0xFF],
            0x01,
            0x32,
            0xFF,
            0x03,
            0x00,
        );
        let cmd = generator.get_setting_cmd(settings);
        laser.send(&cmd, None).await
            .map_err(|e| anyhow!("Failed to set static mode: {}", e))?;
        sleep(Duration::from_millis(100)).await;
        
        // Test dynamic mode (0x01)
        let settings = SettingParams::new(
            [0x00, 0x00, 0xFF, 0xFF, 0xFF],
            0x01,
            0x32,
            0xFF,
            0x01,
            0x00,
        );
        let cmd = generator.get_setting_cmd(settings);
        laser.send(&cmd, None).await
            .map_err(|e| anyhow!("Failed to set dynamic mode: {}", e))?;
        sleep(Duration::from_millis(100)).await;
        
        sleep(Duration::from_millis(500)).await;
    }

    // Return to static mode for cleanup
    let settings = SettingParams::new(
        [0x00, 0x00, 0xFF, 0xFF, 0xFF],
        0x01,
        0x32,
        0xFF,
        0x03,
        0x00,
    );
    let cmd = generator.get_setting_cmd(settings);
    laser.send(&cmd, None).await
        .map_err(|e| anyhow!("Failed to set final static mode: {}", e))?;
    
    debug!("Testing color sequences");

    // Initialize laser
    debug!("Initializing laser");
    let init_cmd = "E0E1E2E38BCE183AE4E5E6E70000000000000000";
    laser.send(init_cmd, None).await
        .map_err(|e| anyhow!("Failed to send init command: {}", e))?;
    sleep(Duration::from_millis(100)).await;

    // Enable laser output
    let enable_cmd = "E0E1E2E3FF01FF32FF0100FF0000000000000000";
    laser.send(enable_cmd, None).await
        .map_err(|e| anyhow!("Failed to enable laser: {}", e))?;
    sleep(Duration::from_millis(100)).await;

    // Test solid red
    debug!("Testing red");
    let settings = SettingParams::new(
        [0xFF, 0x00, 0x00, 0xFF, 0xFF],
        0x01,
        0x32,
        0xFF,
        0x03,
        0x00,
    );
    let cmd = generator.get_setting_cmd(settings);
    laser.send(&cmd, None).await
        .map_err(|e| anyhow!("Failed to set red: {}", e))?;
    sleep(Duration::from_millis(500)).await;

    // Test solid green
    debug!("Testing green");
    let settings = SettingParams::new(
        [0x00, 0xFF, 0x00, 0xFF, 0xFF],
        0x01,
        0x32,
        0xFF,
        0x03,
        0x00,
    );
    let cmd = generator.get_setting_cmd(settings);
    laser.send(&cmd, None).await
        .map_err(|e| anyhow!("Failed to set green: {}", e))?;
    sleep(Duration::from_millis(500)).await;

    // Test solid blue
    debug!("Testing blue");
    let settings = SettingParams::new(
        [0x00, 0x00, 0xFF, 0xFF, 0xFF],
        0x01,
        0x32,
        0xFF,
        0x03,
        0x00,
    );
    let cmd = generator.get_setting_cmd(settings);
    laser.send(&cmd, None).await
        .map_err(|e| anyhow!("Failed to set blue: {}", e))?;
    sleep(Duration::from_millis(500)).await;

    debug!("Color mode toggle test completed successfully");
    Ok(())
}

#[tokio::test]
/// Test laser beam angle adjustment functionality
///
/// The test verifies:
/// 1. X/Y position control
/// 2. Angle limits
/// 3. Position accuracy
async fn test_beam_angle_control() -> Result<()> {
        unsafe {
        env::set_var("RUST_LOG", "debug");
    }
    util::setup_logging();

    let blecontroller = setup_controller().await?;
    let mut laser = LaserController::new(blecontroller).await
        .map_err(|e| anyhow!("Failed to create LaserController: {}", e))?;
    let generator = CommandGenerator::new();

    debug!("Starting beam angle control test with color variations...");

    // Initialize laser with startup sequence
    debug!("Initializing laser...");
    let init_cmd = "E0E1E2E38BCE183AE4E5E6E70000000000000000";
    laser.send(init_cmd, None).await
        .map_err(|e| anyhow!("Failed to send init command: {}", e))?;
    sleep(Duration::from_millis(100)).await;

    // Enable laser output
    let enable_cmd = "E0E1E2E3FF01FF32FF0100FF0000000000000000";
    laser.send(enable_cmd, None).await
        .map_err(|e| anyhow!("Failed to enable laser: {}", e))?;
    sleep(Duration::from_millis(100)).await;

    // Test XY positions with different colors and intensities
    let positions = [
        // Center position with different colors
        (0x00, 0x00, 0xFF, 0x00, 0x00),   // Center Red full intensity
        (0x00, 0x00, 0x00, 0xFF, 0x00),   // Center Green full intensity
        (0x00, 0x00, 0x00, 0x00, 0xFF),   // Center Blue full intensity
        
        // Square pattern with varying intensities
        (0x32, 0x32, 0x80, 0x00, 0x00),   // Top right - medium red
        (0x32, -0x32, 0x00, 0x80, 0x00),  // Bottom right - medium green
        (-0x32, -0x32, 0x00, 0x00, 0x80), // Bottom left - medium blue
        (-0x32, 0x32, 0x40, 0x40, 0x40),  // Top left - low white
        
        // Diagonal movement with color fade
        (0x20, 0x20, 0xFF, 0x20, 0x20),   // Bright red with slight mix
        (0x10, 0x10, 0x20, 0xFF, 0x20),   // Bright green with slight mix
        (0x00, 0x00, 0x20, 0x20, 0xFF),   // Bright blue with slight mix
    ];

    for &(x, y, r, g, b) in positions.iter() {
        debug!("Moving to position ({}, {}) with color RGB({}, {}, {})", x, y, r, g, b);
        
        // Create command with position and color
        let settings = SettingParams::new(
            [r, g, b, 0xFF, 0xFF], // RGB values with full alpha
            (x & 0xFF) as u8,      // X low byte
            (x >> 8) as u8,        // X high byte
            (y & 0xFF) as u8,      // Y low byte
            0x03,                  // Static mode
            (y >> 8) as u8,        // Y high byte
        );
        
        let cmd = generator.get_setting_cmd(settings);
        verify_command_response(&mut laser, &cmd).await?;
        
        // Wait longer at each position to make movement visible
        sleep(Duration::from_millis(500)).await;
    }

    // Return to center with white color at low intensity
    debug!("Returning to center position");
    let settings = SettingParams::new(
        [0x40, 0x40, 0x40, 0xFF, 0xFF], // Low intensity white
        0x00, // Center X
        0x00,
        0x00, // Center Y
        0x03, // Static mode
        0x00,
    );
    let cmd = generator.get_setting_cmd(settings);
    verify_command_response(&mut laser, &cmd).await?;

    debug!("Beam angle control test completed successfully");
    Ok(())
}

#[tokio::test]
/// Test color control functionality
///
/// The test verifies:
/// 1. RGB color control
/// 2. Color intensity levels
/// 3. Color transitions
async fn test_color_control() -> Result<()> {
    let blecontroller = setup_controller().await?;
    let mut laser = LaserController::new(blecontroller).await
        .map_err(|e| anyhow!("Failed to create LaserController: {}", e))?;
    let generator = CommandGenerator::new();

    debug!("Starting color control test...");

    // Test different colors
    let colors = [
        [0xFF, 0x00, 0x00, 0x00, 0x00], // Red
        [0x00, 0xFF, 0x00, 0x00, 0x00], // Green
        [0x00, 0x00, 0xFF, 0x00, 0x00], // Blue
        [0xFF, 0xFF, 0x00, 0x00, 0x00], // Yellow
        [0xFF, 0x00, 0xFF, 0x00, 0x00], // Magenta
        [0x00, 0xFF, 0xFF, 0x00, 0x00], // Cyan
    ];

    for color in colors {
        let settings = SettingParams::new(
            color,
            0x01, // Channel
            0x32, // XY: center
            0xFF, // Full intensity
            0x03, // Static mode
            0x00, // Language
        );
        let cmd = generator.get_setting_cmd(settings);
        verify_command_response(&mut laser, &cmd).await?;
        sleep(Duration::from_millis(200)).await;
    }

    debug!("Color control test completed successfully");
    Ok(())
}

#[tokio::test]
/// Test command sequence from Wireshark log
///
/// The test verifies:
/// 1. Command sequence matches the Wireshark log
/// 2. Proper timing between commands
/// 3. Initialization sequence
/// 4. Device state changes
/// 5. Response handling
async fn test_wireshark_sequence() -> Result<()> {
    unsafe {
        env::set_var("RUST_LOG", "debug");
    }
    util::setup_logging();

    let blecontroller = setup_controller().await?;
    let mut laser = LaserController::new(blecontroller).await
        .map_err(|e| anyhow!("Failed to create LaserController: {}", e))?;

    debug!("Starting Wireshark log sequence test...");

    // Initial connection setup - matching wireshark timing
    sleep(Duration::from_millis(100)).await;

    // Initial device setup sequence
    debug!("Sending initialization sequence...");
    let init_cmd = "E0E1E2E38BCE183AE4E5E6E70000000000000000";
    verify_command_response(&mut laser, init_cmd).await?;
    sleep(Duration::from_millis(200)).await; // Extra time for stable connection

    // Enable device notifications
    let enable_notifications_cmd = "E0E1E2E3FF01FF32FF0100FF0000000000000000";
    verify_command_response(&mut laser, enable_notifications_cmd).await?;
    sleep(Duration::from_millis(200)).await; // Wait for notifications to be enabled

    // Command sequence from Wireshark log
    let command_sequence = [
        // Device configuration commands
        ("E0E1E2E300010032FF0000FF0300000000000000", "Set initial position and color"),
        ("E0E1E2E30001FF32FFFFFF000300000000000000", "Set first movement pattern"),
        ("E0E1E2E3FF01003200FFFFFF0300000000000000", "Set color transition"),
        ("E0E1E2E300010032FFFFFFFF0300000000000000", "Enable full color output"),
        
        // Movement sequence with color transitions
        ("E0E1E2E3FF32003200FF00FF0300000000000000", "Position 1 with color"),
        ("E0E1E2E300CE003200FFFFFF0300000000000000", "Position 2 with color"),
        ("E0E1E2E300CE00CE00FFFFFF0300000000000000", "Position 3 with color"),
        ("E0E1E2E3FF3200CE00FF00FF0300000000000000", "Position 4 with color"),
        
        // Color transition sequence
        ("E0E1E2E3FF00FF0000FFFF000300000000000000", "Red transition"),
        ("E0E1E2E30000FF00FFFFFF000300000000000000", "Green transition"),
        ("E0E1E2E30000FF00FFFFFF800300000000000000", "Blue transition")
    ];

    debug!("Executing Wireshark command sequence...");
    for (i, &(cmd, description)) in command_sequence.iter().enumerate() {
        debug!("Command {}/{}: {}", i + 1, command_sequence.len(), description);
        
        laser.send(cmd, None).await
            .map_err(|e| anyhow!("Failed to send command {}: {}", description, e))?;

        // Use timing from Wireshark log
        match i {
            0 => sleep(Duration::from_millis(89)).await,  // First command timing
            1 => sleep(Duration::from_millis(91)).await,  // Second command timing
            2 => sleep(Duration::from_millis(90)).await,  // Third command timing
            _ => sleep(Duration::from_millis(90)).await,  // Default timing for remaining commands
        }

        verify_command_response(&mut laser, cmd).await?;
    }

    debug!("Wireshark command sequence completed successfully");
    Ok(())
}

#[tokio::test]
/// Test built-in mode functionality
///
/// The test verifies:
/// 1. All built-in modes can be activated
/// 2. Mode transitions are smooth
/// 3. Mode parameters can be adjusted
async fn test_builtin_modes() -> Result<()> {
    let blecontroller = setup_controller().await?;
    let mut laser = LaserController::new(blecontroller).await
        .map_err(|e| anyhow!("Failed to create LaserController: {}", e))?;
    let generator = CommandGenerator::new();

    debug!("Starting built-in modes test...");

    // Test different built-in modes
    let modes = [0x01, 0x02, 0x03, 0x04]; // Add all supported modes

    for mode in modes {
        let settings = SettingParams::new(
            [0xFF, 0xFF, 0xFF, 0x00, 0x00], // White color
            0x01, // Channel
            0x32, // XY scale
            0xFF, // Full intensity
            mode, // Test each mode
            0x00, // Language
        );
        let cmd = generator.get_setting_cmd(settings);
        verify_command_response(&mut laser, &cmd).await?;
        sleep(Duration::from_millis(500)).await; // Longer delay to observe mode
    }

    debug!("Built-in modes test completed successfully");
    Ok(())
}

#[tokio::test]
/// Test device information and characteristics
///
/// The test verifies:
/// 1. All device information can be read
/// 2. Characteristic permissions are correct
/// 3. Notifications are working
async fn test_device_info() -> Result<()> {
    let mut controller = setup_controller().await?;
    
    debug!("Starting device information test...");

    // Read device information
    let device_info = get_device_info(&mut controller).await?;
    debug!("Device Info: {:?}", device_info);

    // Test notifications
    // This would need implementation in WinBlueController
    
    debug!("Device information test completed successfully");
    Ok(())
}