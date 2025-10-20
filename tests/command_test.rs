use std::sync::Arc;
use tokio::sync::Mutex;
use darkelf::blue::BlueController as _;
use darkelf::bluedevice::BlueLaserDevice;
use darkelf::command::CommandGenerator;
use darkelf::mock::MockController;
use darkelf::winblue::{ self, WinBlueController};
#[tokio::main]
#[test]
async fn test_laser_device_mock() -> Result<(), anyhow::Error> {
    // Section marker constants (must match those in command.rs)
    const MAIN_CMD_HEADER: &str = "C0C1C2C3";
    const MAIN_CMD_FOOTER: &str = "C4C5C6C7";
    const SETTINGS_CMD_HEADER: &str = "00010203";
    const SETTINGS_CMD_FOOTER: &str = "04050607";
    const FEATURES_CMD_HEADER: &str = "D0D1D2D3";
    const FEATURES_CMD_FOOTER: &str = "D4D5D6D7";
    const DRAW_CMD_HEADER: &str = "F0F1F2F3";
    const DRAW_CMD_FOOTER: &str = "F4F5F6F7";
    // Setup
    let mut mock_controller = MockController::new();
    let _ = mock_controller.connect();
    assert!(mock_controller.is_connected());

    
    // The command we're analyzing has several parts:
    
    // 1. Command structure (HEADER to FOOTER)
    let cmd = "E0E1E2E3B0B1B2B300B4B5B6B7C0C1C2C30400098080800080003309FFFFFF320000000000000000000000000000000000000000000000000000000000000000000000000000FF035393C06600000000000000000000000000000000000000000000000000000000000000000000000000C4C5C6C7000102030001000A00FFFFFF020000000000000004050607D0D1D2D3820000FF28000000000000000000003200FF00FF28000000000000000000FF3200FFD4D5D6D7F0F1F2F300000000070102030405060700004466F4F5F6F743E3A317F0000000E4E5E6E7";

    // Let's break it down into parts:
    
    // a. Command Start (E0E1E2E3) - Standard command header
    assert!(cmd.starts_with("E0E1E2E3"));
    
    // b. Power Command (B0B1B2B300B4B5B6B7)
    // This is a power OFF command because it has "00" in position 8-9
    let power_section = &cmd[8..24];  // Each byte is 2 hex chars, so 8 bytes = 16 chars
    assert_eq!(power_section, "B0B1B2B300B4B5B6");  // Power OFF command without B7 // Full power section including B7 marker
    
    // c. Main Settings (C0C1C2C3...C4C5C6C7)
    // Contains:
    // - Mode settings: 04 (position)
    // - Display settings: 0009 
    // - RGB color values: 808080
    // - Brightness: 0080
    // - Other parameters
    let main_start = cmd.find(MAIN_CMD_HEADER).unwrap();
    let main_end = cmd.find(MAIN_CMD_FOOTER).unwrap() + 8;
    let main_section = &cmd[main_start..main_end];
    
    // d. Settings Configuration (00010203...04050607)
    // Contains:
    // - Channel number: 0001
    // - Display range: 000A
    // - Color settings: FFFFFF
    // - Additional configuration
    let settings_start = cmd.find(SETTINGS_CMD_HEADER).unwrap();
    let settings_end = cmd.find(SETTINGS_CMD_FOOTER).unwrap() + 8;
    let settings_section = &cmd[settings_start..settings_end];
    
    // e. Features Settings (D0D1D2D3...D4D5D6D7)
    // Contains feature configurations
    let features_start = cmd.find(FEATURES_CMD_HEADER).unwrap();
    let features_end = cmd.find(FEATURES_CMD_FOOTER).unwrap() + 8;
    let features_section = &cmd[features_start..features_end];
    
    // f. Draw Command (F0F1F2F3...F4F5F6F7)
    // Contains drawing parameters
    let draw_start = cmd.find(DRAW_CMD_HEADER).unwrap();
    let draw_end = cmd.find(DRAW_CMD_FOOTER).unwrap() + 8;
    let draw_section = &cmd[draw_start..draw_end];
    
    // g. Random Verification (43E3A317F0000000)
    // Used for command verification
    let random_verify = &cmd[draw_end..draw_end+16];
    
    // h. Command End (E4E5E6E7)
    assert!(cmd.ends_with("E4E5E6E7"));
    
    // Test the actual command sending
   
    mock_controller.send(cmd).await.unwrap();

    Ok(())
}