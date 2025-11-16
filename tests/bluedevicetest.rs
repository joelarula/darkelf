
use std::env;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;


use darkelf::draw::DrawUtils;
use darkelf::model::{DeviceMode, DeviceState, DrawConfig, EncodedCommandData, LegacyDrawData, Playback, PlaybackMode};
use darkelf::winblue::{ self, WinBlueController};
use darkelf::util;
use darkelf::bluedevice::BlueLaserDevice;
use darkelf::blueprotocol::BlueProtocol;
use anyhow::{anyhow, Ok};
use ttf_parser::Face;
use windows::Devices::Enumeration::DeviceInformation;
use log::{error, info};
use darkelf::model::{ Point, DrawCommandData};
use std::fs;


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

    //sleep(Duration::from_millis(500));
    //test_on_off(device).await;
    //sleep(Duration::from_millis(500));
    //test_settings(device).await;
    
    ///sleep(Duration::from_millis(500));
   // test_playback_command(device).await;

    //sleep(Duration::from_millis(500));
    //test_show_playback(device).await;
    //sleep(Duration::from_millis(500));
    //test_tick_playback_command(device).await;

   // test_shapes(device).await;

    //sleep(Duration::from_millis(500));
    
    //test_show_text(device).await;

    //sleep(Duration::from_millis(500));
    
   // test_show_drawings(device).await;

    //sleep(Duration::from_millis(500));
    
    //test_show_drawing_protocol_b(device).await;

    //sleep(Duration::from_millis(500));

    test_show_drawing_protocol_b(device).await;

    test_pis_command(device).await;

    Ok(())
}


async fn test_pis_command(device: &mut BlueLaserDevice) {



   let mode = DeviceMode::Program;
    info!("Set playback mode: {:?}", mode);
    if let Some(mut cmd) = device.get_command_data() {
        cmd.device_mode = mode;
        device.set_main_command(cmd).await;
        sleep(Duration::from_secs(3));
    }
    
    sleep(Duration::from_millis(500));


    let pis = convert_pic_idx_to_255(4,72,false);

    let draw_config = DrawConfig {
        config_values: [pis.group as u32, pis.idx as u32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3],
        play_time: 5,
    };
    
    device.draw_builtin_shape(1, draw_config.clone()).await;
    sleep(Duration::from_millis(500));


}


async fn test_show_drawing_protocol(device: &mut BlueLaserDevice) {


   let mode = DeviceMode::Draw;
    info!("Set playback mode: {:?}", mode);
    if let Some(mut cmd) = device.get_command_data() {
        cmd.device_mode = mode;
        device.set_main_command(cmd).await;
        sleep(Duration::from_secs(3));
    }
    
    let draw_config = DrawCommandData {
        cnf_valus: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3],
        tx_point_time: 5,
    };
    
    let mut draw_points: Vec<Point> = Vec::new();
    //let mut values: Vec<i32> = (-400..400).step_by(40).collect();
 
    //for (idx, i) in values.iter().enumerate() {
        
     //   let c = match *i {
     //       -400..=-361 => 0,
     //       -360..=-241 => 1,
     //       -240..=-121 => 4,
     //       -120..=-41  => 2,
     //       -40..=39    => 5,
     //       40..=159    => 6,
     //       160..=279   => 3,
     //       280..=400   => 7,
     //       _           => 1,
    //    };

     //   let mut b = if *i == -400 { 1 } else { 0 };
     //   let mut d = if *i == 400 { 1 } else { 0 };
     //   let mut pen_state = b + d;


    //    if *i > 0 {
    //        draw_points.push(Point::new(0 as f64, *i as f64, c, pen_state));
    //    } else {
    //        draw_points.push(Point::new(*i as f64, 0 as f64, c, pen_state));
    //    }
    //}
    






    // Add hardcoded points to draw_points
    draw_points.push(Point::new(-403.0, 0.0, 0, 1));

    draw_points.push(Point::new(-336.0, 0.0, 1, 0));
    draw_points.push(Point::new(-269.0, 0.0, 1, 0));
    draw_points.push(Point::new(-202.0, 0.0, 1, 0));
    draw_points.push(Point::new(-134.0, 0.0, 0, 1));
    draw_points.push(Point::new(-67.0, 0.0, 0, 1));
    
    draw_points.push(Point::new(0.0, 0.0, 0, 1));

    draw_points.push(Point::new(67.0, 0.0, 0, 1));
    draw_points.push(Point::new(134.0, 0.0, 0, 1));
    draw_points.push(Point::new(202.0, 0.0, 2, 0));
    draw_points.push(Point::new(269.0, 0.0, 2, 0));
    draw_points.push(Point::new(336.0, 0.0, 2, 1));

    draw_points.push(Point::new(403.0, 0.0, 2, 1));
    
    draw_points.push(Point::new(0.0, 403.0, 0, 1));
    
    draw_points.push(Point::new(0.0, 336.0, 6, 1));
    draw_points.push(Point::new(0.0, 269.0, 6, 0));
    draw_points.push(Point::new(0.0, 202.0, 6, 0));
    draw_points.push(Point::new(0.0, 134.0, 0, 1));
    draw_points.push(Point::new(0.0, 67.0, 0, 1));
    
    draw_points.push(Point::new(0.0, 0.0, 0, 1));
    
    draw_points.push(Point::new(0.0, -67.0, 0, 1));
    draw_points.push(Point::new(0.0, -134.0, 0, 1));
    draw_points.push(Point::new(0.0, -202.0, 7, 0));
    draw_points.push(Point::new(0.0, -269.0, 7, 0));
    draw_points.push(Point::new(0.0, -336.0, 7, 0));
    
    draw_points.push(Point::new(0.0, -403.0, 7, 1));

    device.draw(draw_points, draw_config.clone()).await;
    sleep(Duration::from_millis(500));


}


async fn test_show_drawing_protocol_b(device: &mut BlueLaserDevice) {


   let mode = DeviceMode::Draw;
    info!("Set playback mode: {:?}", mode);
    if let Some(mut cmd) = device.get_command_data() {
        cmd.device_mode = mode;
        device.set_main_command(cmd).await;
        sleep(Duration::from_secs(3));
    }
    
    let draw_config = DrawCommandData {
        cnf_valus: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3],
        tx_point_time: 5,
    };
    
    let arr = [
                    [-394, 0, 0, 1],
                    [-394, 0, 7, 4],
                    [-341, 0, 0, 1],
                    [-341, 0, 7, 4],
                    [-289, 0, 0, 1],
                    [-289, 0, 1, 4],
                    [-236, 0, 0, 1],
                    [-236, 0, 1, 4],
                    [-184, 0, 0, 1],
                    [-184, 0, 3, 4],
                    [-131, 0, 0, 1],
                    [-131, 0, 3, 4],
                    [-79, 0, 0, 1],
                    [-79, 0, 6, 4],
                    [-26, 0, 0, 1],
                    [-26, 0, 6, 4],
                    [26, 0, 0, 1],
                    [26, 0, 5, 4],
                    [79, 0, 0, 1],
                    [79, 0, 5, 4],
                    [131, 0, 0, 1],
                    [131, 0, 4, 4],
                    [184, 0, 0, 1],
                    [184, 0, 4, 4],
                    [236, 0, 0, 1],
                    [236, 0, 2, 4],
                    [289, 0, 0, 1],
                    [289, 0, 2, 4],
                    [341, 0, 0, 1],
                    [341, 0, 7, 4],
                    [394, 0, 0, 1],
                    [394, 0, 7, 4]
    ];
    let draw_points: Vec<Point> = arr.iter().map(|&[x, y, color, pen]| Point::new(x as f64, y as f64, color as u8, pen as u8)).collect();


    device.draw(draw_points, draw_config.clone()).await;
    sleep(Duration::from_millis(500));


}


async fn test_show_text(device: &mut BlueLaserDevice) { 



    if let Some(mut cmd) = device.get_command_data() {


        //let fontName = "Roboto-Bold.ttf";
        let font_name = "laser.regular.ttf";
        let ttf_bytes = std::fs::read(format!("assets/fonts/{}", font_name)).unwrap();
        let face = Face::from_slice(&ttf_bytes, 0).unwrap();

        cmd.device_mode = DeviceMode::TextPlayback;
        cmd.text_distance = 50;
        cmd.run_speed = 50;
        cmd.text_point_time = 10;
        cmd.draw_point_time = 10;

        device.set_main_command(cmd.clone()).await; 

        sleep(Duration::from_secs(2));

        info!("Sending text draw command");
        device.text("AIAS SADAS SAIA".to_string(),face).await;

        sleep(Duration::from_millis(5000));
    }
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
                    selected_play: 8,
                    selected_plays: BlueProtocol::pack_bits_to_prj_selected(&selected_shows),
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
                    selected_play: 45,
                    selected_plays: BlueProtocol::pack_bits_to_prj_selected(&selected_shows),
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
                    selected_play: 43,
                    selected_plays: BlueProtocol::pack_bits_to_prj_selected(&selected_shows),
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
                    selected_play: (ix + 1) as u16,
                    selected_plays: BlueProtocol::pack_bits_to_prj_selected(&selected_shows),
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

    let mode = DeviceMode::Draw;
    info!("Set playback mode: {:?}", mode);
    if let Some(mut cmd) = device.get_command_data() {
        cmd.device_mode = mode;
        device.set_main_command(cmd).await;
        sleep(Duration::from_secs(3));
    }

    let file_name = "scripts/picArrayShapes.json";
    let file_name2 = "scripts/lineShapes.json";
    let file_name3 = "scripts/shapePatternTemplates.json";
     // Load the point arrays from picArrayShapes.json
    let json_content = fs::read_to_string(file_name3)
        .expect("Failed to read picArrayShapes.json");


    
    let point_arrays: Vec<Vec<Vec<f64>>> = serde_json::from_str(&json_content)
        .expect("Failed to parse JSON");
    
    info!("Loaded {} shape arrays from JSON", point_arrays.len());
    
    // Create default PisObject for command generation
    let draw_config = DrawCommandData {
        cnf_valus: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3],
        tx_point_time: 5,
    };
    
    // Process each point array and generate commands
    for (shape_index, point_array) in point_arrays.iter().enumerate() {  // Limit to first 10 shapes
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

    let mode = DeviceMode::Draw;
    info!("Set playback mode: {:?}", mode);
    if let Some(mut cmd) = device.get_command_data() {
        cmd.device_mode = mode;
        device.set_main_command(cmd).await;
        sleep(Duration::from_secs(3));
    }

    let filenames: Vec<String>   = vec![
        "scripts/kaksjoont.json".to_string(),
        "scripts/ufo.json".to_string(),
        "scripts/lill.json".to_string(),
        "scripts/ruut.json".to_string(),
        
    ];

    for filename in filenames.iter() {
        let draw_data: LegacyDrawData = load_draw_data(filename)
            .expect(&format!("Should be able to load {} DrawData", filename));
        
        let draw_config = DrawCommandData {
            cnf_valus: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3],
            tx_point_time: 5,
        };

        let points = DrawUtils::prepare_draw_data(&draw_data, 300.0);
        device.draw(points, draw_config).await;

        sleep(Duration::from_millis(1500));
    }


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


fn load_draw_data<P: AsRef<Path>>(path: P) -> Result<LegacyDrawData, anyhow::Error> {
    let json_content = fs::read_to_string(path)
        .map_err(|e| anyhow!("Failed to read draw data file: {}", e))?;
    
    let draw_data: LegacyDrawData = serde_json::from_str(&json_content)
        .map_err(|e| anyhow!("Failed to parse draw data JSON: {}", e))?;
    
    Ok(draw_data)
}



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PicIdxResult {
    pub group: i32,
    pub idx: i32,
}

/// Converts a (group_index, item_index) pair into device protocol values, matching the JS convertPicIdxTo255 logic.
/// Pass `ilda_mode` as true for ILDA mode, false for normal mode.
pub fn convert_pic_idx_to_255(group_index: i32, item_index: i32, ilda_mode: bool) -> PicIdxResult {
    let mut group_value = 0;
    let mut item_value = 0;
    if ilda_mode {
        group_value = 25 * group_index;
        item_value = 0;
        if group_index == 0 || group_index == 1 {
            item_value = 4 * item_index;
        }
        if group_index == 2 || group_index == 3 {
            item_value = 6 * item_index;
        }
        if group_index == 5 || group_index == 6 {
            item_value = 5 * item_index;
        }
    } else {
        group_value = 25 * group_index;
        item_value = 0;
        if group_index == 0 || group_index == 1 || group_index == 5 || group_index == 6 {
            item_value = 5 * item_index;
        }
        if group_index == 2 || group_index == 3 {
            item_value = 10 * item_index;
        }
        if group_index == 4 {
            if item_index <= 19 {
                item_value = 5 * item_index;
            } else if item_index >= 70 && item_index <= 77 {
                item_value = 5 * (item_index - 70) + 120;
            } else if item_index == 20 {
                item_value = 100;
            } else if item_index == 22 {
                item_value = 105;
            } else if item_index == 24 {
                item_value = 110;
            } else if item_index == 30 {
                item_value = 115;
            } else {
                item_value = 160;
            }
        }
    }
    PicIdxResult {
        group: group_value,
        idx: item_value,
    }
}
