use std::error::Error;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Assume a mock BlueController implementation for demonstration
    // In real use, replace with actual BlueController (e.g., btleplug-based)
    struct MockBlueController;
    impl BlueController for MockBlueController {
        fn is_connected(&self) -> bool { true }
        async fn send(&mut self, bytes: &[u8]) -> Result<(), String> {
            println!("Mock sending bytes: {:?}", bytes);
            Ok(())
        }
    }

    // Initialize LaserController with mock BLE controller
    let mut laser_controller = LaserController::new(true, false, MockBlueController).await?;

    // Create CommandGenerator
    let generator = CommandGenerator::new();

    // Example 1: Change color (e.g., set tx_color via settings; assume val_arr[2..4] for RGB components)
    // Here, set to full RGB (0xFF, 0xFF, 0xFF) with cfg=0x03 for RGB mode
    let color_settings = SettingParams::new(
        [0x00, 0x00, 0xFF, 0xFF, 0xFF], // val_arr: set a,c,o to FF for full RGB
        0x01,                           // ch (type)
        0x00,                           // xy (orientation/angle)
        0x80,                           // light (medium intensity)
        0x03,                           // cfg (RGB mode)
        0x00,                           // lang
    );
    let color_cmd = generator.get_setting_cmd(color_settings);
    println!("Color change command: {}", color_cmd);
    laser_controller.send(&color_cmd, true, None).await?;

    // Example 2: Adjust laser angle (e.g., set xy to a value on 100-point scale, scaled to 0x32 ~50%)
    // Assume xy controls angle/position
    let angle_settings = SettingParams::new(
        [0x00, 0x00, 0xFF, 0xFF, 0xFF], // Keep RGB
        0x01,                           // ch
        0x32,                           // xy: angle ~50 on 100-point scale
        0x80,                           // light
        0x03,                           // cfg (RGB)
        0x00,                           // lang
    );
    let angle_cmd = generator.get_setting_cmd(angle_settings);
    println!("Angle adjust command: {}", angle_cmd);
    laser_controller.send(&angle_cmd, true, None).await?;

    // Example 3: Set DMX channel (e.g., map to ch for channel, val_arr[0] for value)
    // Assume ch selects DMX channel, val_arr[0] sets value (e.g., channel 1, value 0x64 ~100)
    let dmx_settings = SettingParams::new(
        [0x64, 0x00, 0xFF, 0xFF, 0xFF], // val_arr[0]: DMX value 100
        0x01,                           // ch: DMX channel 1
        0x32,                           // xy
        0x80,                           // light
        0x03,                           // cfg
        0x00,                           // lang
    );
    let dmx_cmd = generator.get_setting_cmd(dmx_settings);
    println!("DMX channel set command: {}", dmx_cmd);
    laser_controller.send(&dmx_cmd, true, None).await?;

    // Example 4: Change orientation (e.g., set xy to different values for X/Y flip, assume 0x00=X, 0x80=Y)
    let orientation_settings = SettingParams::new(
        [0x00, 0x00, 0xFF, 0xFF, 0xFF], // Keep defaults
        0x01,                           // ch
        0x80,                           // xy: orientation Y (e.g., flip)
        0x80,                           // light
        0x03,                           // cfg
        0x00,                           // lang
    );
    let orientation_cmd = generator.get_setting_cmd(orientation_settings);
    println!("Orientation change command: {}", orientation_cmd);
    laser_controller.send(&orientation_cmd, true, None).await?;

    // Cleanup
    laser_controller.cleanup()?;

    Ok(())
}

use std::error::Error;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Mock BlueController for demonstration
    struct MockBlueController;
    impl BlueController for MockBlueController {
        fn is_connected(&self) -> bool { true }
        async fn send(&mut self, bytes: &[u8]) -> Result<(), String> {
            println!("Mock sending bytes: {:?}", bytes);
            Ok(())
        }
    }

    // Initialize LaserController
    let mut laser_controller = LaserController::new(true, false, MockBlueController).await?;
    let generator = CommandGenerator::new();

    // Combined settings: monochrome (red), angle, DMX channel, orientation
    let combined_settings = SettingParams::new(
        [0x64, 0x00, 0xFF, 0x00, 0x00], // val_arr: DMX value=100, red only
        0x01,                           // ch: DMX channel 1 or type
        0x32,                           // xy: angle ~50 on 100-point scale
        0x80,                           // light: medium intensity
        0x01,                           // cfg: monochrome mode
        0x00,                           // lang
    );
    let combined_cmd = generator.get_setting_cmd(combined_settings);
    println!("Combined command (monochrome, angle, DMX, orientation): {}", combined_cmd);
    laser_controller.send(&combined_cmd, true, None).await?;

    // Cleanup
    laser_controller.cleanup()?;

    Ok(())
}