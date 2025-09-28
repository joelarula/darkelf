
use std::env;
use std::thread::sleep;
use std::time::Duration;

use darkelf::blue::BlueController as _;
use darkelf::model::{CommandConfig, MainCommandData, ProjectData, ProjectItem, PublicData, TextData};
use darkelf::ui::console::DeviceCommand;
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
    test_laser_device_functionality(&mut device).await?;

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
    test_laser_device_functionality(&mut device).await?;
    
    Ok(())
}

async fn test_laser_device_functionality(device: &mut LaserDevice) -> Result<(), anyhow::Error> {
    
    device.setup().await;
    device.on().await;


    test_playback_command(device).await;

    //test_on_off(device).await;
    //test_settings(device).await;

    Ok(())
}

async fn test_playback_command(device: &mut LaserDevice) {
    use std::thread::sleep;
    use std::time::Duration;

    use std::time::Instant;
    let start = Instant::now();
    let mut cmd_opt = None;
    while start.elapsed() < Duration::from_secs(5) {
        cmd_opt = device.get_command_data();
        if cmd_opt.is_some() {
            break;
        }
        sleep(Duration::from_millis(100));
    }
    let mut cmd: MainCommandData = cmd_opt.expect("Device should return command data after waiting");
    use darkelf::model::PlaybackMode;
    use std::collections::HashMap;
    let playback_modes = [
        PlaybackMode::Dmx,
        PlaybackMode::RandomPlayback,
        PlaybackMode::TimelinePlayback,
        PlaybackMode::AnimationPlayback,
        PlaybackMode::TextPlayback,
        PlaybackMode::ChristmasBroadcast,
        PlaybackMode::OutdoorPlayback,
        PlaybackMode::PersonalizedProgramming,
        PlaybackMode::HandDrawnDoodle,
        PlaybackMode::Playlist,
    ];
    for mode in playback_modes.iter() {
        cmd.current_mode = *mode as u8;
        // Manual conversion from MainCommandData to CommandConfig
        let config = CommandConfig {
            cur_mode: cmd.current_mode,
            text_data: TextData {
                tx_color: cmd.text_color,
                tx_size: cmd.text_size,
                run_speed: cmd.run_speed,
                tx_dist: cmd.text_distance,
                run_dir: cmd.run_direction,
                tx_point_time: cmd.text_point_time,
            },
            prj_data: ProjectData {
                public: PublicData {
                    rd_mode: cmd.read_mode,
                    sound_val: cmd.sound_value,
                },
                    prj_item: {
            let mut map = std::collections::HashMap::new();
            map.insert(0, ProjectItem { py_mode: 128, prj_selected: vec![255, 255, 255, 255] });
            map.insert(1, ProjectItem { py_mode: 128, prj_selected: vec![255, 255, 255, 255] });
            map.insert(2, ProjectItem { py_mode: 128, prj_selected: vec![255, 255, 255, 255] });
            map.insert(3, ProjectItem { py_mode: 128, prj_selected: vec![255, 255, 255, 255] });
            map
        },
            },
        };
        //let cmd_str = CommandGenerator::get_cmd_str(&config, None);
        println!("Set playback mode: {:?} | Command: {:?}", mode, config);
        device.set_command_data(config).await;
        sleep(Duration::from_secs(3));
    }
}

async fn test_on_off(device: &mut LaserDevice) {
    for _ in 0..3 {
        info!("Turning device off");
        device.off().await;
        sleep(Duration::from_millis(500));
        info!("Turning device on");
        device.on().await;
        sleep(Duration::from_millis(500));
    }
}

async fn test_settings(device: &mut LaserDevice) {

    let mut settings = device.get_setting();
    if let Some(ref mut settings) = settings {
        // Loop values[1] from 10 to 100
        for v in 10..=100 {
            settings.values[1] = v;
            device.set_settings(settings.clone()).await;
            sleep(Duration::from_millis(50));
        }
        // Loop values[1] from 99 down to 50
        for v in (50..100).rev() {
            settings.values[1] = v;
            device.set_settings(settings.clone()).await;
            sleep(Duration::from_millis(50));
        }
    }
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

#[test]
fn test_parse_device_response() {
    use std::println;
    util::setup_logging();
    unsafe {
        env::set_var("RUST_LOG", "debug");
    }
    println!("\nTesting parse_device_response");

    let received_data = "E0E1E2E3B0B1B2B3FFB4B5B6B7C0C1C2C306000994943838A5007000000000512E80FFFFFFFFFFFFFFFF80000000000000000080FFFFFFFFFFFFFFFF80FFFFFFFFFFFFFFFF0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000C4C5C6C7000102030001003000646464030000000000000004050607D0D1D2D38100F52000000000000000000000003200FFD4D5D6D7000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000D4D5D6D7887F4282FF000200E4E5E6E7";
    
    // Parse the response
    let response = CommandGenerator::parse_device_response(received_data)
        .expect("Should successfully parse device response");

    // Verify main command data
    assert_eq!(response.main_data.current_mode, 6, "Current mode should be 6");
    assert_eq!(response.main_data.project_index, 6, "Project index should be 6");
    assert_eq!(response.main_data.text_color, 9, "Text color should be 9");
    assert_eq!(response.main_data.text_size, 58, "Text size should be 58");
    assert_eq!(response.main_data.run_speed, 21, "Run speed should be 21");
    assert_eq!(response.main_data.text_distance, 64, "Text distance should be 64");

    // Verify settings data
    assert_eq!(response.settings.values[0], 1, "Channel value (values[0]) should be 1");  // Channel starts at 1 (range 1-512)
    assert_eq!(response.settings.values[1], 48, "Display range (values[1]) should be 48 (hex 0x30)");  // Display range from command
    assert_eq!(response.settings.values[2], 255, "R value (values[2]) should be 255");  // Position 6, Red
    assert_eq!(response.settings.values[3], 255, "G value (values[3]) should be 255");  // Position 7, Green
    assert_eq!(response.settings.values[4], 255, "B value (values[4]) should be 255");  // Position 8, Blue
    assert_eq!(response.settings.xy, 0, "XY config should be 0");
    assert_eq!(response.settings.light, 3, "Light mode should be 3");
    assert_eq!(response.settings.cfg, 0, "Config should be 0");


    // Verify device info
    let device_info = response.device_info.expect("Device info should be present");
    assert!(device_info.device_on, "Device should be on");
    assert_eq!(device_info.device_type, "02", "Device type should be '02'");
    assert_eq!(device_info.version, "00", "Version should be '00'");
    assert_eq!(device_info.user_type, "FF", "User type should be 'FF'");
}



#[test]
fn test_compose_main_command() {
    use std::println;
    util::setup_logging();
    unsafe {
        env::set_var("RUST_LOG", "debug");
    }
    println!("\nTesting compose_main_command");

// Example object:
let command_config = CommandConfig {
    cur_mode: 6,
    text_data: TextData {
        tx_color: 5, // color
        tx_size: 50,
        run_speed: 50, // speed
        tx_dist: 50,
        run_dir: 1,
        tx_point_time: 10,
    },
    prj_data: ProjectData {
        public: PublicData {
            rd_mode: 1, // audio trigger mode
            sound_val: 77, // sound sensitivity
        },
        prj_item: {
            let mut map = std::collections::HashMap::new();
            map.insert(2, ProjectItem { py_mode: 128, prj_selected: vec![21845, 21845, 21845, 1] });
            map.insert(3, ProjectItem { py_mode: 128, prj_selected: vec![1, 0, 0, 2] });
            map.insert(5, ProjectItem { py_mode: 128, prj_selected: vec![0, 0, 0, 0] });
            map.insert(6, ProjectItem { py_mode: 128, prj_selected: vec![65535, 65535, 65535, 3] });
            map
        },
    },
};

    let cmd_str = CommandGenerator::get_cmd_str(&command_config, None);
    println!("Composed command string: {}", cmd_str);


    let test_str2 = "C0C1C2C3060005808080008001C4FFFFFFFF0000800001555555555555800002000000000001800000000000000000800003FFFFFFFFFFFF0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000C4C5C6C7";
    assert_eq!(cmd_str, test_str2, "Composed command string should match expected");
}

#[test]
fn test_prj_selected_bit_conversion() {
        // Create a ProjectItem with prj_selected = vec![255, 255, 255, 255]
        let item = ProjectItem { py_mode: 128, prj_selected: vec![255, 255, 255, 255] };

    // Unpack to bits
    let bits = CommandGenerator::unpack_project_item_bits(&item);
    // Should be 64 bits, each 255 is 8 ones then 8 zeros
    assert_eq!(bits.len(), 64, "Bit vector should have 64 elements");
    println!("bits: {:?}", bits);
    for chunk in bits.chunks(16) {
        let expected: Vec<u8> = vec![1; 8].into_iter().chain(vec![0; 8].into_iter()).collect();
        assert_eq!(chunk, expected.as_slice(), "Each chunk should be 8 ones then 8 zeros for 255");
    }

    // Pack back to prj_selected
    let packed = CommandGenerator::pack_bits_to_prj_selected(&bits);
    assert_eq!(packed, vec![255, 255, 255, 255], "Packed prj_selected should match original");
}

#[test]
fn test_pack_bits_to_prj_selected_50_selected() {
    // Create a bit vector with 50 selected buttons (first 50 bits set to 1, rest 14 bits set to 0)
    let mut bits = vec![1u8; 50];
    bits.extend(vec![0u8; 14]); // total 64 bits
    assert_eq!(bits.len(), 64, "Bit vector should have 64 elements");

    // Pack bits into prj_selected Vec<u8>
    let packed = CommandGenerator::pack_bits_to_prj_selected(&bits);
    println!("Packed prj_selected for 50 selected: {:?}", packed);
    // Print each packed value in hex for clarity
    for (i, val) in packed.iter().enumerate() {
        println!("packed[{}] = 0x{:04X}", i, val);
    }

    // Unpack back to bits and check first 50 are 1, rest are 0
    let unpacked = CommandGenerator::unpack_project_item_bits(&ProjectItem { py_mode: 128, prj_selected: packed.clone() });
    assert_eq!(unpacked.len(), 64, "Unpacked bit vector should have 64 elements");
    assert_eq!(&unpacked[..50], vec![1u8; 50].as_slice(), "First 50 bits should be 1");
    assert_eq!(&unpacked[50..], vec![0u8; 14].as_slice(), "Last 14 bits should be 0");
}

#[test]
fn test_pack_bits_to_prj_selected_first_last_50() {
    // Create a bit vector with only the first and last bit set to 1, rest are 0 (50 bits)
    let mut bits = vec![0u8; 50];
    bits[0] = 1;
    bits[49] = 1;
    // Pad to 64 bits
    bits.extend(vec![0u8; 14]);
    assert_eq!(bits.len(), 64, "Bit vector should have 64 elements");

    // Pack bits into prj_selected Vec<u8>
    let packed = CommandGenerator::pack_bits_to_prj_selected(&bits);
    println!("Packed prj_selected for first and last of 50 set: {:?}", packed);
    for (i, val) in packed.iter().enumerate() {
        println!("packed[{}] = 0x{:04X}", i, val);
    }

    // Unpack back to bits and check
    let unpacked = CommandGenerator::unpack_project_item_bits(&ProjectItem { py_mode: 128, prj_selected: packed.clone() });
    assert_eq!(unpacked.len(), 64, "Unpacked bit vector should have 64 elements");
    assert_eq!(unpacked[0], 1, "First bit should be 1");
    assert_eq!(unpacked[49], 1, "Last bit of 50 should be 1");
    assert_eq!(&unpacked[1..49], vec![0u8; 48].as_slice(), "Bits 1-48 should be 0");
    assert_eq!(&unpacked[50..], vec![0u8; 14].as_slice(), "Bits 50-63 should be 0");
}

#[test]
fn test_pack_bits_to_prj_selected_every_second_1() {
    // Create a bit vector with every second bit set to 1, rest are 0 (50 bits)
    let mut bits = Vec::with_capacity(50);
    for i in 0..50 {
        bits.push(if i % 2 == 0 { 1 } else { 0 });
    }
    // Pad to 64 bits
    bits.extend(vec![0u8; 14]);
    assert_eq!(bits.len(), 64, "Bit vector should have 64 elements");

    // Pack bits into prj_selected Vec<u8>
    let packed = CommandGenerator::pack_bits_to_prj_selected(&bits);
    println!("Packed prj_selected for every second bit set: {:?}", packed);
    for (i, val) in packed.iter().enumerate() {
        println!("packed[{}] = 0x{:04X}", i, val);
    }

    // Unpack back to bits and check every second bit is 1, rest are 0
    let unpacked = CommandGenerator::unpack_project_item_bits(&ProjectItem { py_mode: 128, prj_selected: packed.clone() });
    assert_eq!(unpacked.len(), 64, "Unpacked bit vector should have 64 elements");
    for i in 0..50 {
        assert_eq!(unpacked[i], if i % 2 == 0 { 1 } else { 0 }, "Bit {} should be {}", i, if i % 2 == 0 { 1 } else { 0 });
    }
    assert_eq!(&unpacked[50..], vec![0u8; 14].as_slice(), "Last 14 bits should be 0");
}
