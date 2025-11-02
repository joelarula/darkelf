
use std::env;
use std::thread::sleep;
use std::time::Duration;


use darkelf::draw::DrawUtils;
use darkelf::model::{DeviceMode, DeviceState, Playback, PlaybackMode};
use darkelf::winblue::{ self, WinBlueController};
use darkelf::util;
use darkelf::bluedevice::BlueLaserDevice;
use darkelf::command::CommandGenerator;
use anyhow::{anyhow, Ok};
use windows::Devices::Enumeration::DeviceInformation;
use log::{error, info};
use darkelf::model::{DrawData, DrawMode, DrawPoints, Point, PisObject};
use std::fs;
use std::path::Path;




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
    let mut device: BlueLaserDevice = BlueLaserDevice::new(controller);
    test_laser_device_functionality(&mut device).await?;
    
    Ok(())
}

async fn test_laser_device_functionality(device: &mut BlueLaserDevice) -> Result<(), anyhow::Error> {
    
    device.setup().await;
    sleep(Duration::from_millis(500));
    device.on().await;

    let response_b: DeviceState = device.get_device_response().unwrap();
    info!("Device response after on: {:?}", response_b);

    sleep(Duration::from_millis(500));

     test_tick_playback_command(device).await;

    //test_on_off(device).await;
    //sleep(Duration::from_millis(500));
    //test_settings(device).await;
    
    ///sleep(Duration::from_millis(500));
    //test_playback_command(device).await;

    //sleep(Duration::from_millis(500));
    //test_show_playback(device).await;

    //sleep(Duration::from_millis(500));
    //test_shapes(device).await;

    //sleep(Duration::from_millis(500));
    //test_show_drawings(device).await;





    Ok(())
}



async fn test_tick_playback_command(device: &mut BlueLaserDevice) { 

      if let Some(mut cmd) = device.get_command_data() {

        cmd.device_mode = DeviceMode::LineGeometryPlayback;
        device.set_main_command(cmd.clone()).await; 

        let mut selected_shows = vec![0u8; 50];
        selected_shows[8] = 1; // Select show at index 45

        cmd.run_speed = 255;
           // cmd.playback.audio_config.audio_trigger_mode = 0; 
           // cmd.playback.audio_config.sound_sensitivity = 125;

        cmd.playback.playback_items.insert(
                DeviceMode::LineGeometryPlayback as u8,
                Playback {
                    playback_mode: PlaybackMode::TickPlay,
                    selected_plays: CommandGenerator::pack_bits_to_prj_selected(&selected_shows),
                },
        );

        device.set_main_command(cmd.clone()).await;

        selected_shows[44] = 1; // Select show at index 45

           cmd.run_speed = 125;
           // cmd.playback.audio_config.audio_trigger_mode = 0; 
           // cmd.playback.audio_config.sound_sensitivity = 125;

        cmd.playback.playback_items.insert(
                DeviceMode::LineGeometryPlayback as u8,
                Playback {
                    playback_mode: PlaybackMode::TickPlay,
                    selected_plays: CommandGenerator::pack_bits_to_prj_selected(&selected_shows),
                },
        );

        device.set_main_command(cmd.clone()).await;


        selected_shows[42] = 1; // Select show at index 45

           // cmd.run_speed = 255;
           // cmd.playback.audio_config.audio_trigger_mode = 0; 
           // cmd.playback.audio_config.sound_sensitivity = 125;

        cmd.playback.playback_items.insert(
                DeviceMode::LineGeometryPlayback as u8,
                Playback {
                    playback_mode: PlaybackMode::TickPlay,
                    selected_plays: CommandGenerator::pack_bits_to_prj_selected(&selected_shows),
                },
        );

        device.set_main_command(cmd.clone()).await;

        

    }
}   

async fn test_peed(device: &mut BlueLaserDevice) {
    if let Some(mut cmd) = device.get_command_data() {
         // Ramp run_speed up
         for speed in (1..=255).step_by(10) {
             cmd.run_speed = speed;
             device.set_main_command(cmd.clone()).await;
             info!("Ramping up run_speed: {}", speed);
             sleep(Duration::from_millis(200));
         }

         // Ramp run_speed down
         for speed in (1..=255).rev().step_by(10) {
             cmd.run_speed = speed;
             device.set_main_command(cmd.clone()).await;
             info!("Ramping down run_speed: {}", speed);
             sleep(Duration::from_millis(200));
         }

         sleep(Duration::from_millis(5000));
    }

}

async fn test_settings(device: &mut BlueLaserDevice) {

    
    let mut settings = device.get_setting();
    info!("Initial settings: {:?}", settings);
    
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
        settings.beams = 1; // mono
        device.set_settings(settings.clone()).await;
        sleep(Duration::from_secs(3));

        settings.beams = 3; // back to RGB
        device.set_settings(settings.clone()).await;
        sleep(Duration::from_millis(500));

        // Loop display_range from 10 to 100
        for v in 10..=55 {
            settings.display_range = v;
            device.set_settings(settings.clone()).await;
            sleep(Duration::from_millis(20));
            info!("Command data: {:?}", device.get_setting());
        }

        // Loop values[1] from 99 down to 50
        for v in (55..100).rev() {
            settings.display_range = v;
            device.set_settings(settings.clone()).await;
            sleep(Duration::from_millis(20));
            info!("Command data: {:?}", device.get_setting());
        }

        let response: DeviceState = device.get_device_response().unwrap();
        info!("Device response after settings: {:?}", response);

    
    }
}

async fn test_show_playback(device: &mut BlueLaserDevice) {

    if let Some(mut cmd) = device.get_command_data() {

        cmd.device_mode = DeviceMode::LineGeometryPlayback;
        device.set_main_command(cmd.clone()).await; 
    
        sleep(Duration::from_secs(5));
   
        for ix in 0..=49 {
            let mut selected_shows = vec![0u8; 50];
            selected_shows[ix] = 1; // Select show at index ix
            
            cmd.run_speed = 255;

            cmd.playback.playback_items.insert(
                DeviceMode::LineGeometryPlayback as u8,
                Playback {
                    playback_mode: PlaybackMode::LoopPlay,
                    selected_plays: CommandGenerator::pack_bits_to_prj_selected(&selected_shows),
                },
            );

            device.set_main_command(cmd.clone()).await;
            sleep(Duration::from_secs(5));
        }   
    }   
   
}

async fn test_playback_command(device: &mut BlueLaserDevice) {
 
    let playback_modes = [
        DeviceMode::Dmx,
        DeviceMode::RandomPlayback,
        DeviceMode::LineGeometryPlayback,
        DeviceMode::AnimationPlayback,
        DeviceMode::TextPlayback,
        DeviceMode::ChristmasPlayback,
        DeviceMode::OutdoorPlayback,
        DeviceMode::Program,
        DeviceMode::Draw
    ];
    
    for mode in playback_modes.iter() {

        info!("Set playback mode: {:?}", mode);
        if let Some(mut cmd) = device.get_command_data() {
            cmd.device_mode = *mode;
            device.set_main_command(cmd).await;
            sleep(Duration::from_secs(3));
        }
        
    }



}


async fn test_shapes(device: &mut BlueLaserDevice) {

     // Load the point arrays from picArrayShapes.json
    let json_content = fs::read_to_string("scripts/picArrayShapes.json")
        .expect("Failed to read picArrayShapes.json");
    
    let point_arrays: Vec<Vec<Vec<f64>>> = serde_json::from_str(&json_content)
        .expect("Failed to parse JSON");
    
    info!("Loaded {} shape arrays from JSON", point_arrays.len());
    
    // Create default PisObject for command generation
    let draw_config = PisObject {
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
                        point_data[2] as u8,     // color
                        point_data[3] as u8,     // pen_state
                    )
                } else {
                    Point::new(0.0, 0.0, 1, 0)  // Default fallback
                }
            })
            .collect();
        
        
        device.draw(draw_points, draw_config.clone()).await;
        sleep(Duration::from_millis(2500));
    }

}

async fn test_show_drawings(device: &mut BlueLaserDevice) {

       // let cmd: PlaybackCommand = PlaybackCommand::default(PlaybackMode::HandDrawnDoodle);
       // device.set_playback_mode(cmd).await;

        // Load the ruut.json file using utility function
        let draw_data = load_draw_data("./scripts/ruut.json")
            .expect("Should be able to load ruut.json DrawData");
    
      
        // Use PisObject directly from the loaded data
        let draw_config = draw_data.pis_obj.clone();

        let points = DrawUtils::prepare_draw_data(&draw_data, 300.0);
        device.draw(points, draw_config).await;

       //  Load the lill.json file using utility function
        let draw_data2 = load_draw_data("./scripts/lill.json")
            .expect("Should be able to load lill.json DrawData");

        // Use PisObject directly from the loaded data
        let draw_config2 = draw_data2.pis_obj.clone();
        
        let points2 = DrawUtils::prepare_draw_data(&draw_data2, 300.0);
        device.draw(points2, draw_config2).await;

    }



async fn test_on_off(device: &mut BlueLaserDevice) -> Result<(), anyhow::Error> {
    for _ in 0..3 {
        info!("Turning device off");
        device.off().await;

        let response: DeviceState = device.get_device_response().unwrap();
        info!("Device response after off: {:?}", response);

        sleep(Duration::from_millis(500));
        info!("Turning device on");
        device.on().await;
        sleep(Duration::from_millis(500));
        
        let response_b: DeviceState = device.get_device_response().unwrap();
        info!("Device response after on: {:?}", response_b);
    }
    Ok(())
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
    let unpacked = CommandGenerator::unpack_project_item_bits(&Playback { playback_mode: PlaybackMode::TickPlay, selected_plays: packed.clone() });
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
    let unpacked = CommandGenerator::unpack_project_item_bits(&Playback { playback_mode: PlaybackMode::TickPlay, selected_plays: packed.clone() });
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
    
    // Load the ruut.json file using utility function
    let draw_data = load_draw_data("./scripts/ruut.json")
        .expect("Should be able to load ruut.json DrawData");
    
    // Log the contents
    info!("DrawData Contents:");
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
        info!("Points type: {:?}", match &obj.ps {
            darkelf::model::DrawPoints::Simple(_) => "Simple",
            darkelf::model::DrawPoints::Polylines(_) => "Polylines",
        });
        
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
    let prepared_points = DrawUtils::prepare_draw_data(&draw_data, 300.0);
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
    

    // Use PisObject directly from the loaded data
    let draw_config = draw_data.pis_obj.clone();

     // Expected command string for polylines test data
    let expected_command = "F0F1F2F300000000000000000000000000003700002A80A500C602804E00C670000800C670005E00C67000CA00C670010B00C671010B007040010B001940010B803D40010B809340010B80E94100B580E950005E80E950000880E950806480E95080A580E95180A580936080A5803D6080A500196080A500706080A500C663004A80E702005180E770005880E770005F80E770006780E770006C80E771006C80EE40006C80F440006C80FB40006C810240006C8109410066810950005F8109500058810950004F810950004A810951004A810260004A80FB60004A80F460004A80EE60004A80E763F4F5F6F7";
    
    // Generate the command string
    let command_string = CommandGenerator::get_draw_cmd_str(&prepared_points, &draw_config);
    info!("Generated command string: {}", command_string);

    // Verify exact match with expected command
    assert_eq!(command_string, expected_command, "Generated command should match expected command exactly");
    info!(" Command strings match exactly!");

}

#[test]
fn test_draw_data_polylines() {

    util::setup_logging();
    unsafe {
        env::set_var("RUST_LOG", "debug");
    }
    
    // Load the lill.json file (contains polylines data) using utility function
    let draw_data = load_draw_data("./scripts/lill.json")
        .expect("Should be able to load lill.json DrawData");
    
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
    let prepared_points = DrawUtils::prepare_draw_data(&draw_data, 300.0);
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
    let draw_config = draw_data.pis_obj.clone();
    
    info!("PisObject:");
    info!("Config Values: {:?}", draw_config.cnf_valus);
    info!("Text Point Time: {} (0x{:02X})", draw_config.tx_point_time, draw_config.tx_point_time);
    
    // Use all prepared points for command generation
    info!("Using all {} points for command generation", prepared_points.len());
    
    // Generate the command string
    let command_string = CommandGenerator::get_draw_cmd_str(&prepared_points, &draw_config);
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
    
    // Create default PisObject for command generation
    let draw_config = PisObject {
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
                        point_data[2] as u8,     // color
                        point_data[3] as u8,     // pen_state
                    )
                } else {
                    Point::new(0.0, 0.0, 1, 0)  // Default fallback
                }
            })
            .collect();
        
        
        // Generate command string for this shape
        let command_string = darkelf::command::CommandGenerator::get_draw_cmd_str(&draw_points, &draw_config);
        info!("Generated command: {} ", command_string);
        
    }

}


fn load_draw_data(filename: &str) -> Result<DrawData, anyhow::Error> {

    let json_filename =  filename.to_string();
    let json_path = Path::new(&json_filename);
    let json_content = fs::read_to_string(&json_path)?;
    let json_value: serde_json::Value = serde_json::from_str(&json_content)?;
    let draw_data_json = json_value.get("data")
        .ok_or_else(|| anyhow!("JSON should have 'data' field"))?
        .clone();
    
    let draw_data: DrawData = serde_json::from_value(draw_data_json)?;

    Ok(draw_data)
}