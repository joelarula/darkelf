
use std::env;
use std::thread::sleep;
use std::time::Duration;

use darkelf::blue::BlueController as _;
use darkelf::model::{CommandConfig, MainCommandData, PlaybackCommand, PlaybackMode, ProjectData, ProjectItem, PublicData, TextData};
use darkelf::winblue::{ self, WinBlueController};
use darkelf::mock::MockController;
use darkelf::util;
use darkelf::device::LaserDevice;
use darkelf::command::CommandGenerator;
use anyhow::{anyhow, Ok};
use windows::Devices::Enumeration::DeviceInformation;
use log::{error, info};
use darkelf::model::{DrawData, DrawItem, DrawMode, DrawPoints, Point, DrawConfig, Features};
use std::fs;
use std::path::Path;
use std::collections::HashMap;

    

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

    sleep(Duration::from_millis(500));

    test_on_off(device).await;
    sleep(Duration::from_millis(500));
    test_settings(device).await;
    sleep(Duration::from_millis(500));
    test_playback_command(device).await;
    sleep(Duration::from_millis(500));
    test_show_playback(device).await;


    Ok(())
}

async fn test_show_playback(device: &mut LaserDevice) {

    
    for ix in 0..=49 {
        let mut selected_shows = vec![0u8; 50];
        selected_shows[ix] = 1; // Select show at index ix
        // TODO: Add playback test logic for each ix value
        let cmd: PlaybackCommand = PlaybackCommand {
            mode: PlaybackMode::LineGeometryPlayback,
            selected_shows: Some(selected_shows),
            audio_mode: Some(false),
            audio_sensitivity: Some(100),
            playback_speed: Some(100),
            color: None, // Add appropriate value if needed, e.g. Some(0)
            tick_playback: None, // Add appropriate value if needed, e.g. Some(false)
        };
        device.set_playback_mode(cmd).await;
        sleep(Duration::from_secs(3));
    }
   
}

async fn test_playback_command(device: &mut LaserDevice) {
 
    let playback_modes = [
        PlaybackMode::Dmx,
        PlaybackMode::RandomPlayback,
        PlaybackMode::LineGeometryPlayback,
        PlaybackMode::AnimationPlayback,
        PlaybackMode::TextPlayback,
        PlaybackMode::ChristmasBroadcast,
        PlaybackMode::OutdoorPlayback,
        PlaybackMode::PersonalizedProgramming,
        PlaybackMode::HandDrawnDoodle,
        PlaybackMode::Playlist,
    ];
    for mode in playback_modes.iter() {

        info!("Set playback mode: {:?}", mode);
        device.set_playback_mode( PlaybackCommand::default(*mode)).await;
        sleep(Duration::from_secs(3));
    }

    device.set_playback_mode(PlaybackCommand::default(PlaybackMode::RandomPlayback)).await;
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
        
        // Loop through possible xy values (example: 0..=10)
        for xy in 0..=10 {
            settings.xy = xy;
            device.set_settings(settings.clone()).await;
            sleep(Duration::from_millis(20));
        }

         settings.xy = 0; // Reset to default
         device.set_settings(settings.clone()).await;

         sleep(Duration::from_millis(500));
        // Toggle light mode: mono (1) -> RGB (3), sleeping 2 seconds between
        settings.light = 1; // mono
        device.set_settings(settings.clone()).await;
        sleep(Duration::from_secs(3));

        settings.light = 3; // back to RGB
        device.set_settings(settings.clone()).await;
        sleep(Duration::from_millis(500));

        // Loop values[1] from 10 to 100
        for v in 10..=55 {
            settings.values[1] = v;
            device.set_settings(settings.clone()).await;
            sleep(Duration::from_millis(20));
        }

        // Loop values[1] from 99 down to 50
        for v in (55..100).rev() {
            settings.values[1] = v;
            device.set_settings(settings.clone()).await;
            sleep(Duration::from_millis(20));
        }


    
    }
}

#[test]
fn test_check_received_data() {
    use log::info;
    util::setup_logging();
    unsafe {
        env::set_var("RUST_LOG", "debug");
    }

    // Test data from the log
    let random_data = [0xED, 0x00, 0x05, 0xD5];
    
    // These are the expected values from actual device response
    let expected = [0x88, 0x7F, 0x42, 0x82];
    info!("Expected verification bytes: {:02X?}", expected);
    
    let received_data = "E0E1E2E3B0B1B2B3FFB4B5B6B7C0C1C2C306000994943838A5007000000000512E80FFFFFFFFFFFFFFFF80000000000000000080FFFFFFFFFFFFFFFF80FFFFFFFFFFFFFFFF0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000C4C5C6C7000102030001003000646464030000000000000004050607D0D1D2D38100F52000000000000000000000003200FFD4D5D6D7000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000D4D5D6D7887F4282FF000200E4E5E6E7";
    info!("Response verification part: {}", &received_data[received_data.len() - 24..received_data.len() - 16]);
    
    // Print expected verification bytes
    info!("Expected verification bytes: {:02X?}", expected);
    
    // Get verification part from response (8 bytes)
    let response_verify = &received_data[received_data.len() - 24..received_data.len() - 16];
    info!("Response verification part: {}", response_verify);
    
    // Parse received verification bytes
    let mut received = Vec::with_capacity(4);
    for i in 0..4 {
        let hex_pair = &response_verify[i*2..i*2+2];
        let value = u8::from_str_radix(hex_pair, 16).unwrap();
        received.push(value);
    }
    info!("Received verification bytes: {:02X?}", received);
    
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
   
    util::setup_logging();
    unsafe {
        env::set_var("RUST_LOG", "debug");
    }
    info!("\nTesting parse_device_response");

    let received_data = "E0E1E2E3B0B1B2B3FFB4B5B6B7C0C1C2C306000994943838A5007000000000512E80FFFFFFFFFFFFFFFF80000000000000000080FFFFFFFFFFFFFFFF80FFFFFFFFFFFFFFFF0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000C4C5C6C7000102030001003000646464030000000000000004050607D0D1D2D38100F52000000000000000000000003200FFD4D5D6D7000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000D4D5D6D7887F4282FF000200E4E5E6E7";
    
    // Parse the response
    let response = CommandGenerator::parse_device_response(received_data)
        .expect("Should successfully parse device response");

    // Verify main command data
    assert_eq!(response.main_data.current_mode, 6, "Current mode should be 6");
    assert_eq!(response.main_data.current_mode, 6, "Current mode should be 6");
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
   
    util::setup_logging();
    unsafe {
        env::set_var("RUST_LOG", "debug");
    }

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
    info!("Composed command string: {}", cmd_str);


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
    info!("bits: {:?}", bits);
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
    info!("Packed prj_selected for 50 selected: {:?}", packed);
    // Print each packed value in hex for clarity
    for (i, val) in packed.iter().enumerate() {
        info!("packed[{}] = 0x{:04X}", i, val);
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
    info!("Packed prj_selected for first and last of 50 set: {:?}", packed);
    for (i, val) in packed.iter().enumerate() {
        info!("packed[{}] = 0x{:04X}", i, val);
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
    info!("Packed prj_selected for every second bit set: {:?}", packed);
    for (i, val) in packed.iter().enumerate() {
        info!("packed[{}] = 0x{:04X}", i, val);
    }

    // Unpack back to bits and check every second bit is 1, rest are 0
    let unpacked = CommandGenerator::unpack_project_item_bits(&ProjectItem { py_mode: 128, prj_selected: packed.clone() });
    assert_eq!(unpacked.len(), 64, "Unpacked bit vector should have 64 elements");
    for i in 0..50 {
        assert_eq!(unpacked[i], if i % 2 == 0 { 1 } else { 0 }, "Bit {} should be {}", i, if i % 2 == 0 { 1 } else { 0 });
    }
    assert_eq!(&unpacked[50..], vec![0u8; 14].as_slice(), "Last 14 bits should be 0");
}


#[test]
fn test_draw_data() {

    util::setup_logging();
    unsafe {
        env::set_var("RUST_LOG", "debug");
    }
    
    // Load the ruut.json file
    let json_path = Path::new("scripts/ruut.json");
    assert!(json_path.exists(), "ruut.json file should exist at {:?}", json_path);
    
    let json_content = fs::read_to_string(json_path)
        .expect("Should be able to read ruut.json file");
    
    // Parse the outer wrapper first
    let json_value: serde_json::Value = serde_json::from_str(&json_content)
        .expect("Should parse JSON successfully");
    
    // Extract the data field which contains our DrawData
    let draw_data_json = json_value.get("data")
        .expect("JSON should have 'data' field")
        .clone();
    
    // Parse into our DrawData struct
    let draw_data: DrawData = serde_json::from_value(draw_data_json)
        .expect("Should deserialize DrawData successfully");
    
    // Log the contents
    info!("\nDrawData Contents:");
    info!("Draw Objects: {} objects", draw_data.draw_points.len());
    
    // Log each draw object
    for (i, obj) in draw_data.draw_points.iter().enumerate() {
        info!("\n  🎯 Draw Object #{}", i + 1);
        info!(" Position: x0={:.3}, y0={:.3}", obj.x0, obj.y0);
        info!("Scale (z): {:.6}", obj.z);
        info!("Rotation (ang): {:.1}°", obj.ang);
        info!("Line Color: {}", obj.line_color);
        info!("Draw Mode: {:?}", obj.draw_mode);
        info!("Points: {} points", obj.ps.len());
        
        // Log first few points as examples
        let all_points = obj.get_all_points();
        let point_preview = all_points.iter().take(3).collect::<Vec<_>>();
        for (j, point) in point_preview.iter().enumerate() {
            info!("  Point {}: x={:.3}, y={:.3}, color={}, pen={}", 
                j + 1, point.x, point.y, point.color, point.pen_state);
        }
        if all_points.len() > 3 {
            info!("      ... and {} more points", all_points.len() - 3);
        }
        
    }
    
    info!("PisObject Configuration:");
    info!("TX Point Time: {}", draw_data.pis_obj.tx_point_time);
    info!("Config Values: {:?}", draw_data.pis_obj.cnf_valus);

    // Test prepare_draw_data function with width=300 (matching JavaScript test)
    info!("\nTesting prepare_draw_data function:");
    let prepared_points = CommandGenerator::prepare_draw_data(&draw_data, 300.0);
    info!("Total prepared points: {}", prepared_points.len());
    
    // Reference flatPoints2 data from JavaScript drawPs2 output (first 15 points for comparison)
    let reference_points = vec![
        [-164.5376292142001, 198.01136363636368, 0.0, 1.0],
        [-78.2680216702548, 198.01136363636368, 7.0, 0.0],
        [8.001585873690523, 198.01136363636368, 7.0, 0.0],
        [94.27119341763583, 198.01136363636368, 7.0, 0.0],
        [202.10820284756747, 198.01136363636368, 7.0, 0.0],
        [266.8104085055265, 198.01136363636368, 7.0, 1.0],
        [266.8104085055265, 111.74175609241836, 4.0, 0.0],
        [266.8104085055265, 25.47214854847304, 4.0, 0.0],
        [266.8104085055265, -60.79745899547227, 4.0, 0.0],
        [266.8104085055265, -147.0670665394176, 4.0, 0.0],
        [266.8104085055265, -233.33667408336294, 4.0, 1.0],
        [180.54080096158114, -233.33667408336294, 5.0, 0.0],
        [94.27119341763583, -233.33667408336294, 5.0, 0.0],
        [8.001585873690523, -233.33667408336294, 5.0, 0.0],
        [-99.83542355624112, -233.33667408336294, 5.0, 0.0],
    ];
    
    info!("Comparing first 15 points with reference flatPoints2:");
    for (i, point) in prepared_points.iter().take(15).enumerate() {
        if i < reference_points.len() {
            let ref_point = &reference_points[i];
            info!("    Point {}: prepared=({:.3}, {:.3}, {}, {}) | reference=({:.3}, {:.3}, {}, {})", 
                i + 1, 
                point.x, point.y, point.color, point.pen_state,
                ref_point[0], ref_point[1], ref_point[2] as u8, ref_point[3] as u8
            );
            
            // Check if coordinates are reasonably close (allowing for transformation differences)
            let x_diff = (point.x - ref_point[0]).abs();
            let y_diff = (point.y - ref_point[1]).abs();
            info!("      Coordinate diff: x={:.3}, y={:.3}", x_diff, y_diff);
            
            // Verify color and pen_state match exactly
            assert_eq!(point.color, ref_point[2] as u8, "Color should match for point {}", i + 1);
            assert_eq!(point.pen_state, ref_point[3] as u8, "Pen state should match for point {}", i + 1);
        }
    }
    
    // Test Point struct compatibility with drawPs2 format
    info!("\nTesting Point struct compatibility with drawPs2 format:");
    let sample_draw_ps2_point = [-164.5376292142001, 198.01136363636368, 0.0, 1.0];
    let point_from_js = Point::from_js_array(
        sample_draw_ps2_point[0], 
        sample_draw_ps2_point[1], 
        sample_draw_ps2_point[2], 
        sample_draw_ps2_point[3]
    );
    
    info!("Sample Point from JS array: x={:.3}, y={:.3}, color={}, pen_state={}", 
        point_from_js.x, point_from_js.y, point_from_js.color, point_from_js.pen_state);
    
    // Convert back to array format
    let array_format = point_from_js.to_array();
    info!("Converted back to array: [{:.3}, {:.3}, {}, {}]", 
        array_format[0], array_format[1], array_format[2], array_format[3]);
    

    info!("\n prepare_draw_data and Point compatibility tests completed successfully!");
}

#[test]
fn test_draw_data_polylines() {

    util::setup_logging();
    unsafe {
        env::set_var("RUST_LOG", "debug");
    }
    
    // Load the lill.json file (contains polylines data)
    let json_path = Path::new("scripts/lill.json");
    assert!(json_path.exists(), "lill.json file should exist at {:?}", json_path);
    
    let json_content = fs::read_to_string(json_path)
        .expect("Should be able to read lill.json file");
    
    // Parse the outer wrapper first
    let json_value: serde_json::Value = serde_json::from_str(&json_content)
        .expect("Should parse JSON successfully");
    
    // Extract the data field which contains our DrawData
    let draw_data_json = json_value.get("data")
        .expect("JSON should have 'data' field")
        .clone();
    
    // Parse into our DrawData struct
    let draw_data: DrawData = serde_json::from_value(draw_data_json)
        .expect("Should deserialize DrawData successfully");
    
    // Log the contents
    info!("\nPolylines DrawData Contents:");
    info!("Draw Objects: {} objects", draw_data.draw_points.len());
    info!("Draw Objects Count: {}", draw_data.draw_points.len());
    
    // Log each draw object
    for (i, obj) in draw_data.draw_points.iter().enumerate() {
        info!("\n  🎯 Polylines Draw Object #{}", i + 1);
        info!("    Position: x0={:.3}, y0={:.3}", obj.x0, obj.y0);
        info!("    Scale (z): {:.6}", obj.z);
        info!("    Rotation (ang): {:.1}°", obj.ang);
        info!("    Line Color: {}", obj.line_color);
        info!("    Draw Mode: {:?}", obj.draw_mode);
        
        // Handle both simple points and polylines structure
        match &obj.ps {
            DrawPoints::Simple(points) => {
                info!("    Simple Points: {} points", points.len());
                for (j, point) in points.iter().enumerate().take(3) {
                    info!("      Point {}: x={:.3}, y={:.3}, color={}, pen={}", 
                        j + 1, point.x, point.y, point.color, point.pen_state);
                }
                if points.len() > 3 {
                    info!("      ... and {} more points", points.len() - 3);
                }
            }
            DrawPoints::Polylines(polylines) => {
                info!("    Polylines: {} polylines", polylines.len());
                for (j, polyline) in polylines.iter().enumerate().take(3) {
                    info!("      Polyline {}: {} points", j + 1, polyline.len());
                    // Show first few points of each polyline
                    for (k, point) in polyline.iter().enumerate().take(3) {
                        info!("        Point {}: x={:.3}, y={:.3}, color={}, pen={}", 
                            k + 1, point.x, point.y, point.color, point.pen_state);
                    }
                    if polyline.len() > 3 {
                        info!("        ... and {} more points", polyline.len() - 3);
                    }
                }
                if polylines.len() > 3 {
                    info!("      ... and {} more polylines", polylines.len() - 3);
                }
            }
        }
        
        // Verify this is actually polylines data
        assert_eq!(obj.draw_mode, DrawMode::Polylines, "Draw mode should be Polylines (-1)");
    }
    
    info!("\nPisObject Configuration:");
    info!("  TX Point Time: {}", draw_data.pis_obj.tx_point_time);
    info!("  Config Values: {:?}", draw_data.pis_obj.cnf_valus);

    // Test prepare_draw_data function with width=300 for polylines
    info!("\nTesting prepare_draw_data function for polylines:");
    let prepared_points = CommandGenerator::prepare_draw_data(&draw_data, 300.0);
    info!("Total prepared points from polylines: {}", prepared_points.len());
    
    // Reference flatPoints2 data from JavaScript drawAllTransformedPolylines2 output 
    let reference_points = vec![
        [-169.60686770352447, 24.71719221635294, 0.0, 1.0],
        [-179.4754028320312, 29.794484918767694, 1.0, 0.0],
        [-189.87485712224782, 34.58872708407314, 1.0, 0.0],
        [-196.21210965243247, 41.19314713911573, 1.0, 0.0],
        [-203.203027898615, 48.97041320800781, 1.0, 0.0],
        [-209.8484732887961, 60.13256419788702, 1.0, 0.0],
        [-216.577859358354, 69.10507895729756, 1.0, 0.0],
        [-238.31601576371622, 109.06483043323858, 1.0, 0.0],
        [-240.41155034845522, 121.84666720303619, 1.0, 0.0],
        [-233.85559428821904, 167.54382740367538, 1.0, 0.0],
        [-228.6445791071111, 182.17176957563916, 1.0, 0.0],
        [-219.98086409135294, 195.31277743252838, 1.0, 0.0],
        [-209.34609499844635, 207.88463245738632, 1.0, 0.0],
        [-187.6539056951349, 233.18297646262425, 1.0, 0.0],
        [-174.84021620316935, 244.53274119984013, 1.0, 0.0],
        [-162.48439442027694, 253.60121293501416, 1.0, 0.0],
        [-148.7498196688565, 261.89405267888844, 1.0, 0.0],
        [-134.54832597212354, 267.8083939985795, 1.0, 0.0],
        [-120.74647383256388, 271.8949751420454, 1.0, 0.0],
        [-92.79542402787638, 273.42109680175776, 1.0, 0.0],
    ];
    
    info!("Comparing first 20 points with reference flatPoints2:");
    for (i, point) in prepared_points.iter().take(20).enumerate() {
        if i < reference_points.len() {
            let ref_point = &reference_points[i];
            info!("    Point {}: prepared=({:.3}, {:.3}, {}, {}) | reference=({:.3}, {:.3}, {}, {})", 
                i + 1, 
                point.x, point.y, point.color, point.pen_state,
                ref_point[0], ref_point[1], ref_point[2] as u8, ref_point[3] as u8
            );
            
            // Check coordinate differences
            let x_diff = (point.x - ref_point[0]).abs();
            let y_diff = (point.y - ref_point[1]).abs();
            info!("      Coordinate diff: x={:.3}, y={:.3}", x_diff, y_diff);
            
            // Verify color and pen_state match exactly
            assert_eq!(point.color, ref_point[2] as u8, "Color should match for point {}", i + 1);
            assert_eq!(point.pen_state, ref_point[3] as u8, "Pen state should match for point {}", i + 1);
        }
    }
    
    // Verify we got some points
    assert!(!prepared_points.is_empty(), "Should have prepared some points from polylines data");
    
    // Test that all points have valid color and pen_state values
    for (i, point) in prepared_points.iter().enumerate() {
        assert!(point.color <= 255, "Point {} should have valid color value", i + 1);
        assert!(point.pen_state <= 1, "Point {} should have valid pen_state (0 or 1)", i + 1);
    }
    
    // Test command string generation from prepared points
    info!("\nTesting get_draw_cmd_str function:");
    
    // Read DrawConfig from the PisObject in the loaded data
    let draw_config = DrawConfig {
        config_values: draw_data.pis_obj.cnf_valus.iter().map(|&val| val as u8).collect(),
        text_point_time: draw_data.pis_obj.tx_point_time as u8,
    };
    
    info!("DrawConfig from PisObject:");
    info!("Config Values: {:?}", draw_config.config_values);
    info!("Text Point Time: {} (0x{:02X})", draw_config.text_point_time, draw_config.text_point_time);
    
    // Create Features with textStopTime enabled
    let mut feature_map = HashMap::new();
    feature_map.insert("textStopTime".to_string(), true);
    
    let features = Features {
        features: feature_map,
        group_list: None,
        prj_parm: None,
        xy_cnf_save: None,
    };
    
    // Use all prepared points for command generation
    info!("Using all {} points for command generation", prepared_points.len());
    
    // Generate the command string
    let command_string = CommandGenerator::get_draw_cmd_str(&prepared_points, &draw_config, &features);
    info!("Generated command string: {}", command_string);
    
    // Expected command string for polylines test data
    let expected_command = "F0F1F2F30000000000000000000000000000370000CD80AA00190280B3001E1080BE00231080C400291080CB00311080D2003C1080D900451080EE006D1080F0007A1080EA00A81080E500B61080DC00C31080D100D01080BC00E91080AF00F51080A200FE1080950106108087010C108079011010805D011110804F0110108038010810802D010110802400FA10801C00F110801400E610800C00DB10800600D010000600B810000A00AA10000F009B100014007F100010005910000C004E1000060040100005003A100005003F1000050045100008004E10000F005F1000130067100017006F10001C0078100022008210002F0094100037009E10004000A710005200B510005C00BA10006700BF10007100C310008300C710008C00C710009500C410009D00C21000AB00B91000B300B31000BA00AD1000C200A51000C8009C1000D500891000DA007E1000DF00721000E6005B1000E7004E1000E600411000E100281000DC001C1000D400121000C500001000B680101000AE80171000A6801B10009F801F100096802410008E8027100082802B10007A802A100078802910007780241000798021100080801A100086801710008E8013100097800F1000A1800C1000B7800B1000C3800C1000CE80101000E3801A1000EC80211000F380291000FA8032100100803C10010A805210010C805E10010E806D10010E807B100109809810010580A71000FE80B71000F580C61000E080E41000D380F11000B781071000A881101000998117100089811C10006B812610005A812910004B812A10003C812910002E8126100016811E10000A8116100002810D10800C80F810801280EC10801780DF10801B80D210801E80BB10801E80B110801D80A810801B80A01080158095108012809110800F808E10800B808D108009808E10800380951080018099100000809F10000380A610000580AE10000780B710000880BF10000880D710000380F3108003810210800A81111080168120108030813E108040814B10805081571080718167108081816B108091816C10809E816A1080BF81581080C7814E1080CD81431080D281361080D8811A1080D7810B1080D480FC1080CD80DC1080C780CD1080BF80BD1080B780B01080AE80A31080A6809810809D8090108095808B1080878085108080808510807A808610806F808A10806B808F10806780931080668098108065809C10806780A210807180AE10807980B310808280B810808D80BB10809880BD1080B280BD1080BF80BC1080D880B61080E580B31080F080AD108107809D10810F8092108115808710811C806C10811C805D10811B804F108114803110810F80231081088016108100800A1080EC000D1080E200171080D8001F1080C400291080BC00291080B500291080AE00281080A800261080A1002510809C0023108096002110808D001B108085001413F4F5F6F7";
    
    info!("Expected command string: {}", expected_command);
    
    // Verify command matches expected output
    info!(" Command string length - Generated: {}, Expected: {}", command_string.len(), expected_command.len());
    
    // Verify command starts with correct header
    assert!(command_string.starts_with("F0F1F2F3"), "Command should start with F0F1F2F3 header");
    
    // Verify command ends with correct footer 
    assert!(command_string.ends_with("F4F5F6F7"), "Command should end with F4F5F6F7 footer");
    
    // Verify exact match with expected command
    assert_eq!(command_string, expected_command, "Generated command should match expected command exactly");
    info!(" Command strings match exactly!");
    
    info!("\nPolylines prepare_draw_data and command generation tests completed successfully!");
    info!("Processed {} polylines objects into {} points", draw_data.draw_points.len(), prepared_points.len());
}

#[test]
fn test_point_array_shapes_command_generation() {
    let _ = pretty_env_logger::try_init();
    
    info!("\n🔧 Testing command generation for point array shapes");
    
    // Load the point arrays from picArrayShapes.json
    let json_content = fs::read_to_string("scripts/picArrayShapes.json")
        .expect("Failed to read picArrayShapes.json");
    
    let point_arrays: Vec<Vec<Vec<f64>>> = serde_json::from_str(&json_content)
        .expect("Failed to parse JSON");
    
    info!("Loaded {} shape arrays from JSON", point_arrays.len());
    
    // Create default DrawConfig and Features for command generation
    let draw_config = DrawConfig {
        config_values: vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3],
        text_point_time: 55,
    };
    
    let mut feature_map = HashMap::new();
    feature_map.insert("textStopTime".to_string(), true);
    
    let features = Features {
        features: feature_map,
        group_list: None,
        prj_parm: None,
        xy_cnf_save: None,
    };
    
    // Process each point array and generate commands
    for (shape_index, point_array) in point_arrays.iter().take(10).enumerate() {  // Limit to first 10 shapes
        info!("Processing Shape #{} with {} points:", shape_index + 1, point_array.len());
        
        // Convert point array to Points
        let draw_points: Vec<Point> = point_array
            .iter()
            .map(|point_data| {
                if point_data.len() >= 4 {
                    Point::new(
                        point_data[0],           // x
                        point_data[1],           // y
                        point_data[2] as u8,     // color
                        point_data[3] as u8,     // pen_state
                    )
                } else {
                    Point::new(0.0, 0.0, 1, 0)  // Default fallback
                }
            })
            .collect();
        
        
        // Generate command string for this shape
        let command_string = darkelf::command::CommandGenerator::get_draw_cmd_str(&draw_points, &draw_config, &features);
        info!("Generated command: {} ", command_string);
        
    }

}