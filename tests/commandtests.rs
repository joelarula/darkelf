use std::{env, fs, path::Path};
use log::info;
use darkelf::{blueprotocol::BlueProtocol, draw::DrawUtils, model::{DeviceMode, DeviceSettings, DisplayColor, DrawCommandData, DrawMode, DrawPoints, LegacyDrawData, Playback, PlaybackMode, Point}, util};
use anyhow::{anyhow, Ok};

use std::sync::Once;
static INIT: Once = Once::new();

fn init_logger() {
    INIT.call_once(|| {
        util::setup_logging();
    });
}

#[test]
fn test_check_received_data() {
   
    init_logger();

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
    let (success, device_info) = BlueProtocol::check_received_data(received_data, &random_data);
    
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
   
    init_logger();

    info!("Testing parse_device_response");

    let received_data = "E0E1E2E3B0B1B2B3FFB4B5B6B7C0C1C2C306000994943838A5007000000000512E80FFFFFFFFFFFFFFFF80000000000000000080FFFFFFFFFFFFFFFF80FFFFFFFFFFFFFFFF0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000C4C5C6C7000102030001003000646464030000000000000004050607D0D1D2D38100F52000000000000000000000003200FFD4D5D6D7000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000D4D5D6D7887F4282FF000200E4E5E6E7";
    
    // Parse the response
    let response = BlueProtocol::extract_device_response(received_data)
        .expect("Should successfully parse device response");

    // Verify main command data
    assert_eq!(response.main_data.device_mode, DeviceMode::try_from(6).unwrap(), "Current mode should be 6");
    assert_eq!(response.main_data.color, DisplayColor::try_from(9).unwrap(), "Text color should be 9");
    assert_eq!(response.main_data.text_size_x, 148, "Text size should be 148");
    assert_eq!(response.main_data.run_speed, 56, "Run speed should be 56");
    assert_eq!(response.main_data.text_distance, 165, "Text distance should be 165");

    // Verify settings data
    assert_eq!(response.settings.dmx_channel, 0, "Channel value (dmx_channel) should be 0");  // Channel starts at 1 (range 1-512)
    assert_eq!(response.settings.display_range, 48, "Display range (display_range) should be 48 (hex 0x30)");  // Display range from command
    assert_eq!(response.settings.red_beam, 100, "R value (red_beam) should be 100");  // Position 6, Red
    assert_eq!(response.settings.green_beam, 100, "G value (green_beam) should be 100");  // Position 7, Green
    assert_eq!(response.settings.blue_beam, 100, "B value (blue_beam) should be 100");  // Position 8, Blue
    assert_eq!(response.settings.xy, 0, "XY config should be 0");
    assert_eq!(response.settings.beams, 3, "Light mode should be 3");
    assert_eq!(response.settings.ttl_or_analog, 0, "Config should be 0");


    // Verify device info
    assert!(response.device_info.device_on, "Device should be on");
    assert_eq!(response.device_info.device_type, "02", "Device type should be '02'");
    assert_eq!(response.device_info.version, "00", "Version should be '00'");
    assert_eq!(response.device_info.user_type, "FF", "User type should be 'FF'");

}




#[test]
fn test_parse_device_info() {
    init_logger();

   let test1 =   "E0E1E2E3B0B1B2B3FFB4B5B6B7C0C1C2C306000994943838A5007000000000512E80FFFFFFFFFFFFFFFF80000000000000000080FFFFFFFFFFFFFFFF80FFFFFFFFFFFFFFFF0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000C4C5C6C7000102030001003000646464030000000000000004050607D0D1D2D38100F52000000000000000000000003200FFD4D5D6D7000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000D4D5D6D7887F4282FF000200E4E5E6E7";
   let main_command = BlueProtocol::extract_device_info(&test1);
   let main_command = main_command.unwrap();
   info!("Parsed device info   : {:?}", main_command);

}



#[test]
fn test_parse_main_command() {

    init_logger();

   //let test1 = "E0E1E2E3B0B1B2B3FFB4B5B6B7C0C1C2C306000994943838A5007000000000512E80FFFFFFFFFFFFFFFF80000000000000000080FFFFFFFFFFFFFFFF80FFFFFFFFFFFFFFFF0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000C4C5C6C7000102030001003000646464030000000000000004050607D0D1D2D38100F52000000000000000000000003200FFD4D5D6D7000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000D4D5D6D7887F4282FF000200E4E5E6E7";
   let test2 = "E0E1E2E3B0B1B2B3FFB4B5B6B7C0C1C2C3060002BABA5454C100A800000000003700FFFFFFFFFFFFFFFF00FFFFFFFFFFFFFFFF00FFFFFFFFFFFFFFFF00FFFFFFFFFFFFFFFF0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000C4C5C6C7000102030011002300646464030000000000000004050607D0D1D2D38300B5190000000000002959005E003200FFD23250000000000000000000003200FF000050000000000000000000003200FF0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000D4D5D6D78049B92700000200E4E5E6E7";
   info!("Test command   : {:?}", test2);
   let main_command = BlueProtocol::extract_main_command(&test2);
   let main_command = main_command.unwrap();
   info!("Example main command   : {:?}", main_command);

   // Test packing and unpacking
   let example1 ="C0C1C2C3060002BABA5454C100A800000000003700FFFFFFFFFFFFFFFF00FFFFFFFFFFFFFFFF00FFFFFFFFFFFFFFFF00FFFFFFFFFFFFFFFF0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000C4C5C6C7";
   //let example = "C0C1C2C306000994943838A5007000000000512E80FFFFFFFFFFFFFFFF80000000000000000080FFFFFFFFFFFFFFFF80FFFFFFFFFFFFFFFF0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000C4C5C6C7";
   info!("Example main command   : {:?}", example1);
   let packed = BlueProtocol::pack_main_command(&main_command);
   let test_command = BlueProtocol::extract_main_command(&packed);
   info!("Parsed main command   : {:?}", test_command);
   info!("Packed main command   : {:?}", packed);
   assert_eq!(packed, example1, "Packed command should match example");
 

}


#[test]
fn test_parse_pis_command() {

    init_logger();

   let test1 =   "E0E1E2E3B0B1B2B3FFB4B5B6B7C0C1C2C306000994943838A5007000000000512E80FFFFFFFFFFFFFFFF80000000000000000080FFFFFFFFFFFFFFFF80FFFFFFFFFFFFFFFF0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000C4C5C6C7000102030001003000646464030000000000000004050607D0D1D2D38100F52000000000000000000000003200FFD4D5D6D7000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000D4D5D6D7887F4282FF000200E4E5E6E7";
   let pis_command = BlueProtocol::extract_draw_command(&test1);
   let pis_command = pis_command.unwrap();
   info!("Parsed pis  command   : {:?}", pis_command);


}

#[test]
fn test_parse_playback_command() {

   init_logger();

   let test1 =   "E0E1E2E3B0B1B2B3FFB4B5B6B7C0C1C2C306000994943838A5007000000000512E80FFFFFFFFFFFFFFFF80000000000000000080FFFFFFFFFFFFFFFF80FFFFFFFFFFFFFFFF0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000C4C5C6C7000102030001003000646464030000000000000004050607D0D1D2D38100F52000000000000000000000003200FFD4D5D6D7000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000D4D5D6D7887F4282FF000200E4E5E6E7";
   let prj_command = BlueProtocol::extract_playback_command(&test1);
   let prj_command = prj_command.unwrap();
   info!("Parsed prj  command   : {:?}", prj_command);


}

#[test]
fn test_parse_device_features() {
    init_logger();
   let test1 =   "E0E1E2E3B0B1B2B3FFB4B5B6B7C0C1C2C306000994943838A5007000000000512E80FFFFFFFFFFFFFFFF80000000000000000080FFFFFFFFFFFFFFFF80FFFFFFFFFFFFFFFF0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000C4C5C6C7000102030001003000646464030000000000000004050607D0D1D2D38100F52000000000000000000000003200FFD4D5D6D7000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000D4D5D6D7887F4282FF000200E4E5E6E7";
   let features = BlueProtocol::extract_features(&test1);
   let features = features.unwrap();
   info!("Parsed device features   : {:?}", features);

}

#[test]
fn test_parse_settings_data() {

    init_logger();
   
   let test1 =   "E0E1E2E3B0B1B2B3FFB4B5B6B7C0C1C2C306000994943838A5007000000000512E80FFFFFFFFFFFFFFFF80000000000000000080FFFFFFFFFFFFFFFF80FFFFFFFFFFFFFFFF0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000C4C5C6C7000102030001003000646464030000000000000004050607D0D1D2D38100F52000000000000000000000003200FFD4D5D6D7000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000D4D5D6D7887F4282FF000200E4E5E6E7";
   let settings1 = BlueProtocol::extract_settings_command(&test1);
   let settings1 = settings1.unwrap();
   info!("Parsed settings1   : {:?}", settings1);


    let test2 = "000102030001003707FFFFFF010000000000000004050607";

    let settings = BlueProtocol::extract_settings_command(test2);
    let settings = settings.unwrap();

    let test3 = BlueProtocol::pack_setting_cmd(&settings);

    let expected_settings = DeviceSettings {
        dmx_channel: 0,
        xy: 7,
        beams: 1,
        ttl_or_analog: 0,
        proto: 1,
        display_range: 55,
        red_beam: 255,
        green_beam: 255,
        blue_beam: 255,
    };

    let test4 = BlueProtocol::pack_setting_cmd(&expected_settings);

   
    info!("Example settings  : {:?}", expected_settings);
    info!("Example command   : {}", test4);
    info!("Parsed settings   : {:?}", settings);
    info!("Parse command     : {}", test2);
    info!("Generated command : {}", test3);
   

    assert_eq!(test2,test4,"Generated command should match original");
    assert_eq!(test2,test3,"Generated command should match original");

}

#[test]
fn test_prj_selected_bit_conversion() {
    
    // Create a ProjectItem with prj_selected = vec![255, 255, 255, 255]
    let item = Playback { playback_mode: PlaybackMode::TickPlay, selected_plays: vec![255, 255, 255, 255], selected_play: 1 };

    // Unpack to bits
    let bits = BlueProtocol::extract_project_item_bits(&item);
    // Should be 64 bits, each 255 is 8 ones then 8 zeros
    assert_eq!(bits.len(), 64, "Bit vector should have 64 elements");
    info!("bits: {:?}", bits);
    for chunk in bits.chunks(16) {
        let expected: Vec<u8> = vec![1; 8].into_iter().chain(vec![0; 8].into_iter()).collect();
        assert_eq!(chunk, expected.as_slice(), "Each chunk should be 8 ones then 8 zeros for 255");
    }

    // Pack back to prj_selected
    let packed = BlueProtocol::pack_bits_to_prj_selected(&bits);
    assert_eq!(packed, vec![255, 255, 255, 255], "Packed prj_selected should match original");
}

#[test]
fn test_pack_bits_to_prj_selected_50_selected() {
    // Create a bit vector with 50 selected buttons (first 50 bits set to 1, rest 14 bits set to 0)
    let mut bits = vec![1u8; 50];
    bits.extend(vec![0u8; 14]); // total 64 bits
    assert_eq!(bits.len(), 64, "Bit vector should have 64 elements");

    // Pack bits into prj_selected Vec<u8>
    let packed = BlueProtocol::pack_bits_to_prj_selected(&bits);
    info!("Packed prj_selected for 50 selected: {:?}", packed);
    // Print each packed value in hex for clarity
    for (i, val) in packed.iter().enumerate() {
        info!("packed[{}] = 0x{:04X}", i, val);
    }

    // Unpack back to bits and check first 50 are 1, rest are 0
    let unpacked = BlueProtocol::extract_project_item_bits(&Playback { playback_mode: PlaybackMode::TickPlay, selected_plays: packed.clone(), selected_play: 1 });
    assert_eq!(unpacked.len(), 64, "Unpacked bit vector should have 64 elements");
    assert_eq!(&unpacked[..50], vec![1u8; 50].as_slice(), "First 50 bits should be 1");
    assert_eq!(&unpacked[50..], vec![0u8; 14].as_slice(), "Last 14 bits should be 0");
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
    let packed = BlueProtocol::pack_bits_to_prj_selected(&bits);
    info!("Packed prj_selected for every second bit set: {:?}", packed);
    for (i, val) in packed.iter().enumerate() {
        info!("packed[{}] = 0x{:04X}", i, val);
    }

    // Unpack back to bits and check every second bit is 1, rest are 0
    let unpacked = BlueProtocol::extract_project_item_bits(&Playback { playback_mode: PlaybackMode::TickPlay, selected_plays: packed.clone(), selected_play: 1 });
    assert_eq!(unpacked.len(), 64, "Unpacked bit vector should have 64 elements");
    for i in 0..50 {
        assert_eq!(unpacked[i], if i % 2 == 0 { 1 } else { 0 }, "Bit {} should be {}", i, if i % 2 == 0 { 1 } else { 0 });
    }
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
    let packed = BlueProtocol::pack_bits_to_prj_selected(&bits);
    info!("Packed prj_selected for first and last of 50 set: {:?}", packed);
    for (i, val) in packed.iter().enumerate() {
        info!("packed[{}] = 0x{:04X}", i, val);
    }

    // Unpack back to bits and check
    let unpacked = BlueProtocol::extract_project_item_bits(&Playback { playback_mode: PlaybackMode::TickPlay, selected_plays: packed.clone(), selected_play: 1 });
    assert_eq!(unpacked.len(), 64, "Unpacked bit vector should have 64 elements");
    assert_eq!(unpacked[0], 1, "First bit should be 1");
    assert_eq!(unpacked[49], 1, "Last bit of 50 should be 1");
    assert_eq!(&unpacked[1..49], vec![0u8; 48].as_slice(), "Bits 1-48 should be 0");
    assert_eq!(&unpacked[50..], vec![0u8; 14].as_slice(), "Bits 50-63 should be 0");
}


#[test]
fn test_draw_data() {

    init_logger();
    
    // Load the ruut.json file using utility function
    let draw_data = load_draw_data("./scripts/ruut.json")
        .expect("Should be able to load ruut.json DrawData");
    
    let prepared_points = DrawUtils::prepare_draw_data(&draw_data, 300.0);

    let draw_config = draw_data.pis_obj.clone();

     // Expected command string for polylines test data
    let expected_command = "F0F1F2F300000000000000000000000000003700002A80A500C602804E00C670000800C670005E00C67000CA00C670010B00C671010B007040010B001940010B803D40010B809340010B80E94100B580E950005E80E950000880E950806480E95080A580E95180A580936080A5803D6080A500196080A500706080A500C663004A80E702005180E770005880E770005F80E770006780E770006C80E771006C80EE40006C80F440006C80FB40006C810240006C8109410066810950005F8109500058810950004F810950004A810951004A810260004A80FB60004A80F460004A80EE60004A80E763F4F5F6F7";
    
    // Generate the command string
    let command_string = BlueProtocol::pack_draw_points_cmd(&prepared_points, &draw_config.to_draw_config());
    info!("Generated command string: {}", command_string);

    // Verify exact match with expected command
    assert_eq!(command_string, expected_command, "Generated command should match expected command exactly");
    info!(" Command strings match exactly!");

}

#[test]
fn test_draw_data_polylines() {

    init_logger();
    // Load the lill.json file (contains polylines data) using utility function
    let draw_data = load_draw_data("./scripts/lill.json")
        .expect("Should be able to load lill.json DrawData");
    
    // Test prepare_draw_data function with width=300 for polylines
    info!("\nTesting prepare_draw_data function for polylines:");
    let prepared_points = DrawUtils::prepare_draw_data(&draw_data, 300.0);
    info!("Total prepared points from polylines: {}", prepared_points.len());
    
   
    // Verify we got some points
    assert!(!prepared_points.is_empty(), "Should have prepared some points from polylines data");
    
    // Read DrawConfig from the PisObject in the loaded data
    let draw_config = draw_data.pis_obj.clone();
    

    // Generate the command string
    let command_string = BlueProtocol::pack_draw_points_cmd(&prepared_points, &draw_config.to_draw_config());
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
    
    init_logger();
    
    info!("\n🔧 Testing command generation for point array shapes");
    
    // Load the point arrays from picArrayShapes.json
    let json_content = fs::read_to_string("scripts/picArrayShapes.json")
        .expect("Failed to read picArrayShapes.json");
    
    let point_arrays: Vec<Vec<Vec<f64>>> = serde_json::from_str(&json_content)
        .expect("Failed to parse JSON");
    
    info!("Loaded {} shape arrays from JSON", point_arrays.len());
    
    // Create default PisObject for command generation
    let draw_config = DrawCommandData {
        cnf_valus: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3],
        tx_point_time: 55,
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
                        DisplayColor::try_from(point_data[2] as u8).unwrap(),     // color
                        point_data[3] as u8,     // pen_state
                    )
                } else {
                    Point::new(0.0, 0.0, DisplayColor::Red, 0)  // Default fallback
                }
            })
            .collect();
        
        
        // Generate command string for this shape
        let command_string = darkelf::blueprotocol::BlueProtocol::pack_draw_points_cmd(&draw_points, &draw_config.to_draw_config());
        info!("Generated command: {} ", command_string);
        
    }

}



fn load_draw_data<P: AsRef<Path>>(path: P) -> Result<LegacyDrawData, anyhow::Error> {
    let json_content = fs::read_to_string(path)
        .map_err(|e| anyhow!("Failed to read draw data file: {}", e))?;
    
    let draw_data: LegacyDrawData = serde_json::from_str(&json_content)
        .map_err(|e| anyhow!("Failed to parse draw data JSON: {}", e))?;
    
    Ok(draw_data)
}

