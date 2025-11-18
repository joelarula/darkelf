use crate::{draw::DrawUtils, ilda, model::{EncodedCommandData, Playback, PlaybackMode, Point, PolyPoint}};
use log::{debug, info};
use ilda::model::ILDA_BLANK;
use crate::model::{DeviceInfo, DeviceState, DeviceFeatures,DrawConfig, MainCommandData, DrawCommandData, DeviceSettings, PlaybackData, DeviceMode, DisplayColor};

pub const HEADER: &str = "E0E1E2E3";
pub const FOOTER: &str = "E4E5E6E7";

pub const POWER_ON_CMD: &str = "B0B1B2B3FFB4B5B6B7";
pub const POWER_OFF_CMD: &str = "B0B1B2B300B4B5B6B7";

const MAIN_CMD_HEADER: &str = "C0C1C2C3";
const MAIN_CMD_FOOTER: &str = "C4C5C6C7";
const SETTINGS_CMD_HEADER: &str = "00010203";
const SETTINGS_CMD_FOOTER: &str = "000000000004050607";
const DRAWCONFIG_CMD_HEADER: &str = "D0D1D2D3";
const DRAWCONFIG_CMD_FOOTER: &str = "D4D5D6D7";
const DRAW_CMD_HEADER: &str = "F0F1F2F3";
const DRAW_CMD_FOOTER: &str = "F4F5F6F7";
pub const XYS_CMD_HEADER: &str = "A0A1A2A3";
pub const XYS_CMD_FOOTER: &str = "A4A5A6A7";


pub struct BlueProtocol;

impl BlueProtocol {

   pub fn to_fixed_width_hex<T: std::fmt::UpperHex>(value: T, width: usize) -> String {
        format!("{:0width$X}", value, width = width)
    }

    pub fn combine_nibbles(high: u8, low: u8) -> u8 {
        (high & 0x0F) << 4 | (low & 0x0F)
    }

    pub fn to_fixed_width_hex_float(value: f64, width: usize) -> String {
        let mut rounded_value = value.round() as i16;
        
        if rounded_value < 0 {
            rounded_value = ILDA_BLANK as i16 | -rounded_value;
        }
        
        format!("{:0width$x}", rounded_value, width = width)
    }

    fn clamp_value<T: PartialOrd + Copy>(value: T, min: T, max: T, default: T) -> T {
        if value < min || value > max {
            default
        } else {
            value
        }
    }
   
    fn extract_hex_value(pos: usize, len: usize, data: &str) -> u16 {
        let start = if pos > 0 { 2 * (pos - 1) } else { 0 };
        let end = start + 2 * len;
        if end <= data.len() {
            u16::from_str_radix(&data[start..end], 16).unwrap_or(0)
        } else {
            0
        }
    }

    pub fn get_cmd_value(start: &str, end: &str, input: &str) -> Option<String> {
        
        if let Some(start_idx) = input.find(start) {
            if let Some(end_idx) = input[start_idx..].find(end) {
                let start_pos = start_idx + start.len();
                let content = &input[start_pos..start_idx + end_idx];
                return Some(content.to_string());
            }
        }
        debug!("No matching string found that meets the requirements");
        None
    }

    pub fn extract_settings_command(cmd_data: &str) -> Option<DeviceSettings> {

        let cmd = Self::get_cmd_value(SETTINGS_CMD_HEADER, SETTINGS_CMD_FOOTER, cmd_data)?;
        if cmd.len() == 22 {

            let proto = Self::clamp_value(Self::extract_hex_value(1, 2, &cmd), 1, 512, 1) as u8;
            let channel = Self::extract_hex_value(3, 1, &cmd) as u8;
            let display_range = Self::clamp_value(Self::extract_hex_value(4, 1, &cmd), 10, 100, 10) as u8;
            let xy = Self::clamp_value(Self::extract_hex_value(5, 1, &cmd), 0, 7, 0) as u8;
            let r_val = Self::clamp_value(Self::extract_hex_value(6, 1, &cmd), 0, 255, 255) as u8;
            let g_val = Self::clamp_value(Self::extract_hex_value(7, 1, &cmd), 0, 255, 255) as u8;
            let b_val = Self::clamp_value(Self::extract_hex_value(8, 1, &cmd), 0, 255, 255) as u8;
            let beams = Self::clamp_value(Self::extract_hex_value(9, 1, &cmd), 1, 3, 3) as u8;
            let ttl_analog = Self::clamp_value(Self::extract_hex_value(10, 1, &cmd), 0, 255, 0) as u8;

            Some(DeviceSettings {
                    proto,
                    display_range,
                    red_beam : r_val,
                    green_beam : g_val,
                    blue_beam : b_val,
                    dmx_channel: channel,   
                    xy,
                    beams,
                    ttl_or_analog: ttl_analog
            })
        
        } else {
           None
        }

    }



    pub fn pack_setting_cmd(settings: &DeviceSettings) -> String {

        format!(
            "{}{}{}{}{}{}{}{}{}{}{}{}", 
            SETTINGS_CMD_HEADER,
            Self::to_fixed_width_hex(settings.proto, 4),  
            Self::to_fixed_width_hex(settings.dmx_channel as u16, 2),       
            Self::to_fixed_width_hex(settings.display_range as u16, 2),  
            Self::to_fixed_width_hex(settings.xy as u16, 2),     
            Self::to_fixed_width_hex(settings.red_beam, 2),      
            Self::to_fixed_width_hex(settings.green_beam, 2),       
            Self::to_fixed_width_hex(settings.blue_beam, 2),       
            Self::to_fixed_width_hex(settings.beams as u16, 2),   
            Self::to_fixed_width_hex(settings.ttl_or_analog as u16, 2),     
            Self::to_fixed_width_hex(0, 2),    
            SETTINGS_CMD_FOOTER
        ).to_uppercase()
    }
    
    pub fn extract_device_response(data: &str) -> Option<DeviceState> {
        
        let response = DeviceState {
            main_data: Self::extract_main_command(&data)?,
            settings: Self::extract_settings_command(&data)?,
            device_info: Self::extract_device_info(&data)?,
            features: Self::extract_features(&data)?,
            features_config: Self::extract_features_config(&data)?,
            draw_data:  Self::extract_draw_command(&data),
        };


        Some(response)
    }


    pub fn extract_main_command(cmd_data: &str) -> Option<MainCommandData> {

        let cmd = Self::get_cmd_value(MAIN_CMD_HEADER, MAIN_CMD_FOOTER, cmd_data)?;
        info!("Parsing main command: {} {}", cmd, cmd.len());

        info!("Main command: {} {} {}", MAIN_CMD_HEADER, cmd, MAIN_CMD_FOOTER);

        let value = Self::clamp_value(Self::extract_hex_value(1, 1, &cmd) as u8, 0, 12, 0);
        let device_mode: DeviceMode = DeviceMode::try_from(value).unwrap();

        Some(MainCommandData {
            device_mode: device_mode ,
            audio_trigger_mode: Self::clamp_value(Self::extract_hex_value(2, 1, &cmd) as u8, 0, 9, 0),
            color: DisplayColor::try_from(Self::clamp_value(Self::extract_hex_value(3, 1, &cmd) as u8, 0, 9, 0)).unwrap(),
            text_size_x: Self::clamp_value(Self::extract_hex_value(4, 1, &cmd) as u8 , 0, 255, 125),
            text_size_y: Self::clamp_value(Self::extract_hex_value(5, 1, &cmd) as u8 , 0, 255, 125),
            run_speed:  Self::clamp_value(Self::extract_hex_value(6, 1, &cmd) as u8 , 0, 255, 128),
            filler:  Self::clamp_value(Self::extract_hex_value(7, 1, &cmd) as u8 , 0, 255, 128),
            text_distance:  Self::clamp_value(Self::extract_hex_value(8, 1, &cmd) as u8, 0, 255, 125),
            audio_mode: Self::clamp_value(Self::extract_hex_value(9, 1, &cmd) as u8, 0, 255, 0),
            sound_value:  Self::clamp_value(Self::extract_hex_value(10, 1, &cmd) as u8, 0, 255, 0),         
            text_point_time: Self::clamp_value(Self::extract_hex_value(15, 1, &cmd) as u8, 0, 100, 50),
            draw_point_time: Self::clamp_value(Self::extract_hex_value(16, 1, &cmd) as u8, 0, 100, 50),
            run_direction: Self::clamp_value(Self::extract_hex_value(17, 1, &cmd) as u8, 0, 255, 0),
            playback: Self::extract_playback_command(&cmd)?,
        })
    }



    pub fn extract_features(data: &str) -> Option<DeviceFeatures> {
        if let Some(device_info) = Self::extract_device_info(data) {
            let device_type = device_info.device_type.parse::<u8>().unwrap_or(0);
            let version = device_info.version.parse::<u8>().unwrap_or(0);

            let mut features = DeviceFeatures {
                text_stop_time: false,
                text_decimal_time: false,
                display_type: device_type,
                show_outdoor_tips: false,
                xy_cnf: false,
                arb_play: false,
                ilda: false,
                ttl_an: false,
                pics_play: false,
                text_up_down: false,
                animation_fix: false
            };

            if (device_type == 1 && version >= 1)
                || (device_type == 0 && version >= 2)
                || (device_type >= 2)
            {
                features.text_stop_time = true;
                features.text_decimal_time = true;
            }

            if (device_type == 1 && version >= 2) || (device_type > 1) {
                features.show_outdoor_tips = true;
            }

            if device_type == 2 {
                features.xy_cnf = true;
            }

            if device_type == 1 || device_type == 2 {
                features.ilda = true;
                features.ttl_an = true;
            }

            if device_type >= 2 || version >= 3 {
                features.arb_play = true;
            }

            if device_type >= 3 || version >= 4 {
                features.text_up_down = true;
            }

            if device_type >= 3 || version >= 5 {
                features.pics_play = true;
            }

            if device_type == 1 {
                features.animation_fix = true;
            }

            return Some(features);
        }
        None
    }



    pub fn extract_features_config(data: &str) -> Option<Vec<DrawConfig>> {
        let mut features = Vec::new();


        if let Some(features_cmd) = Self::get_cmd_value(DRAWCONFIG_CMD_HEADER, DRAWCONFIG_CMD_FOOTER, data) {
            
            info!("Parsing features command: {} {}", features_cmd, features_cmd.len());
            
            let feature_count = Self::clamp_value(Self::extract_hex_value(1, 1, &features_cmd), 0, 127, 0);
            let values_per_feature = 16; // or 22 if xy_config is enabled

            for i in 0..feature_count {
                
                let mut config_array = [0u8; 14];

                for j in 0..values_per_feature {
                    let value = Self::clamp_value(
                        Self::extract_hex_value(3 + i as usize * values_per_feature + j, 1, &features_cmd).try_into().unwrap(),
                        0,
                        255,
                        0
                    );
                    if j < 13 {
                        config_array[j] = value.try_into().unwrap();
                    }
                    if j == 13 {
                          config_array[j] = value.try_into().unwrap();
                    }
                }

                let mut config = DrawConfig::from_config_values( &config_array);
                features.push(config);
            }
            Some(features)
        } else {
            None
        }



    }


    pub fn extract_playback_command(playback_cmd: &str) -> Option<PlaybackData> {
  
        let mut prj_item = std::collections::HashMap::new();
        // Use keys [2, 3, 5, 6] for project items
        let prj_keys = [2, 3, 5, 6];
        let mut project_item_start_index = 17;
        for &key in prj_keys.iter() {
            info!("Extract playback mode {}: {}", key, Self::extract_hex_value(project_item_start_index, 1, &playback_cmd));
            let py_mode = PlaybackMode::try_from(Self::clamp_value(Self::extract_hex_value(project_item_start_index, 1, &playback_cmd), 0, 255, 0) as u8).unwrap();
            let mut prj_selected = vec![0u16; 4];
            prj_selected[3] = Self::extract_hex_value(project_item_start_index + 1, 2, &playback_cmd);
            prj_selected[2] = Self::extract_hex_value(project_item_start_index + 3, 2, &playback_cmd);
            prj_selected[1] = Self::extract_hex_value(project_item_start_index + 5, 2, &playback_cmd);
            prj_selected[0] = Self::extract_hex_value(project_item_start_index + 7, 2, &playback_cmd);
            prj_item.insert(key, Playback { playback_mode: py_mode, selected_plays: prj_selected, selected_play: 1 });
            project_item_start_index += 9;
        }

        let prj_data = PlaybackData { playback_items: prj_item };

        Some(prj_data)

    }

    pub fn extract_draw_command(data: &str) -> Option<DrawCommandData> {

        let main_cmd = Self::get_cmd_value(MAIN_CMD_HEADER, MAIN_CMD_FOOTER, data)?;
   
        let tx_point_time = BlueProtocol::clamp_value(BlueProtocol::extract_hex_value(15, 1, &main_cmd) as u8, 0, 100, 50);
        let mut cnf_valus_vec = Vec::new();
        for i in 0..13 {
            cnf_valus_vec.push(BlueProtocol::clamp_value(BlueProtocol::extract_hex_value(18 + i, 1, &main_cmd) as u8, 0, 255, 0) as u32);
        }
        let mut cnf_valus_arr = [0u32; 13];
        for (i, val) in cnf_valus_vec.iter().enumerate().take(13) {
            cnf_valus_arr[i] = *val;
        }

        let mut pis_obj = DrawCommandData {
            tx_point_time: tx_point_time as u32,
            cnf_valus: cnf_valus_arr,
        };

        // If draw config section exists, update PisObject fields from draw config data
        if let Some(draw_cmd) = Self::get_cmd_value(DRAW_CMD_HEADER, DRAW_CMD_FOOTER, data) {
            for i in 0..15 {
                let value = Self::clamp_value(Self::extract_hex_value(i + 1, 1, &draw_cmd) as u32, 0, 255, 0);
                if i < pis_obj.cnf_valus.len() {
                    pis_obj.cnf_valus[i] = value;
                }
                if i == 14 {
                    pis_obj.tx_point_time = value;
                }
            }
        }

        Some(pis_obj)
    }

    pub fn extract_device_info(data: &str) -> Option<DeviceInfo> {
 
        if let Some(footer_idx) = data.rfind(FOOTER) {
            if footer_idx >= 8 {
                let info_start = footer_idx - 8;
                let device_info_str = &data[info_start..footer_idx];

                let device_status = &device_info_str[..2];
                let version = &device_info_str[2..4];
                let device_type = &device_info_str[4..6];

                return Some(DeviceInfo {
                    device_on: true,               
                    device_type: device_type.to_string(),
                    version: version.to_string(),
                    user_type: device_status.to_string(), 
                });
            }
        }
        None
    }


    pub fn pack_query_cmd(random_verify: &[u8]) -> String {
        let middle = if random_verify.len() >= 4 {
            format!("{:02X}{:02X}{:02X}{:02X}", 
                random_verify[0], random_verify[1], 
                random_verify[2], random_verify[3])
        } else {
            "00000000".to_string() 
        };      
        let cmd = format!("{}{}{}", HEADER, middle, FOOTER).to_uppercase();
        cmd
    }

    pub fn pack_main_command(command: &MainCommandData) -> String {
      
        let cur_mode_hex = Self::to_fixed_width_hex(command.device_mode as u8, 2);
        let reserved_hex = Self::to_fixed_width_hex(0, 2);
        let audio_trigger_mode_hex = Self::to_fixed_width_hex(command.audio_trigger_mode as u8, 2);
        let color_hex = Self::to_fixed_width_hex(command.color as u8, 2);
        let tx_size_scaled_x = Self::to_fixed_width_hex(command.text_size_x as u8, 2);
        let tx_size_scaled_y = Self::to_fixed_width_hex(command.text_size_y as u8, 2);
        let run_speed_hex = Self::to_fixed_width_hex(command.run_speed, 2);
        let filler_hex = Self::to_fixed_width_hex(command.filler, 2);
        let tx_dist_scaled_hex = Self::to_fixed_width_hex(command.text_distance, 2);
        let sound_sensitivity_hex = Self::to_fixed_width_hex(command.sound_value  as u8, 2);
        let text_point_time_hex: String = Self::to_fixed_width_hex(command.text_point_time, 2);
        let draw_point_time_hex: String = Self::to_fixed_width_hex(command.draw_point_time, 2);
        let run_direction = Self::to_fixed_width_hex(command.run_direction, 2);

        let mut playback_selection_hex = String::new();
        let show_keys = [2, 3, 5, 6];
        for &key in show_keys.iter() {
            let playback = command.playback.playback_items.get(&key).cloned().unwrap_or_else(|| Playback {
                playback_mode: PlaybackMode::TickPlay,
                selected_play: 1,
                selected_plays: vec![0; 4],
            });
            let mut play_back_mode = if playback.playback_mode == PlaybackMode::LoopPlay { 0 } else { 128 };
            if play_back_mode == 128 {
                let _play_back_mode_shifted = play_back_mode | playback.selected_play as u8;
            }

            let play_back_mode_hex = Self::to_fixed_width_hex(play_back_mode, 2);
            let mut show_selected_hex = String::new();
            for &val in playback.selected_plays.iter().rev() {
                show_selected_hex.push_str(&Self::to_fixed_width_hex(val, 4));
            }
            playback_selection_hex.push_str(&format!("{}{}", play_back_mode_hex, show_selected_hex));
        }

        let mut padding = String::new();
        let run_direction_bytes = run_direction.len() / 2;
        if run_direction_bytes < 44 {
            padding = "00".repeat(44 - run_direction_bytes);
        }

        //let filler = "00000000".to_string();

        let mut color_group_hex = String::from("ffffffff0000");
        color_group_hex.clear();
    //if let Some(group_list) = feature_params.group_list.as_ref() {
        //for group_color in group_color_list {
        color_group_hex.push_str(&Self::to_fixed_width_hex(2.0 as u32, 2));
      //  }
    //}
    color_group_hex.push_str("ffffffff");
    color_group_hex.truncate(8); // Only keep first 8 hex chars

    //if get_feature_value(feature_params, "textStopTime") {
    //    color_group_hex.push_str(&to_fixed_width_hex(command_config.text_data.tx_point_time as u32, 2));
    //}
    //color_group_hex.push_str("0000");
    //color_group_hex.truncate(12); // Only keep first 12 hex chars




        let command = format!(
            "{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            MAIN_CMD_HEADER,         
            cur_mode_hex,            
            reserved_hex,            
            color_hex,               
            tx_size_scaled_x,    
            tx_size_scaled_y,   
            run_speed_hex,    
            filler_hex,                   
            tx_dist_scaled_hex,      
            audio_trigger_mode_hex,  
            sound_sensitivity_hex,   
            color_group_hex, //filler  
            text_point_time_hex,        
            draw_point_time_hex,               
            playback_selection_hex,                      
            run_direction,           
            padding,               
            MAIN_CMD_FOOTER         
        );
        command.to_uppercase()

    }

    pub fn pack_draw_points_cmd(points: &[Point], config: &DrawCommandData) -> String {
        let encoded_draw_cmd = Self::encode_draw_point_command(points, config);
        let command_str = format!("{}{}{}", DRAW_CMD_HEADER, encoded_draw_cmd, DRAW_CMD_FOOTER);
        command_str.to_uppercase()
    }

    pub fn encode_draw_point_command(points: &[Point], config: &DrawCommandData, ) -> String {
        let point_time = "00";  
        let mut config_str = String::new();
        let mut points_str = String::new();
        

        for index in 0..15 {
            if index <= 11 {
                let value = if index < config.cnf_valus.len() {
                    config.cnf_valus[index] as u8
                } else {
                    0
                };
                config_str.push_str(&Self::to_fixed_width_hex(value, 2));
            } else if index == 13 {
                // Handle picsPlay feature
                config_str.push_str(point_time);
            } else if index == 14 {
                // Use tx_point_time for textStopTime feature
                config_str.push_str(&Self::to_fixed_width_hex(config.tx_point_time as u8, 2));
            } else {
                config_str.push_str(point_time);
            }
        }

        config_str.push_str(point_time);

        for (ix, point) in points.iter().enumerate() {
            let mut pen_state = point.pen_state;
                
            // Handle textStopTime feature logic - always enabled for polylines mode
            if point.color == 0 {
                pen_state = 2;
            } else if (ix < points.len() - 1 && points[ix + 1].color == 0) || ix == points.len() - 1 {
                pen_state = 3;
            }
                
            let combined = Self::combine_nibbles(point.color, pen_state);
                
            points_str.push_str(&Self::to_fixed_width_hex_float(point.x, 4));
            points_str.push_str(&Self::to_fixed_width_hex_float(point.y, 4));
            points_str.push_str(&Self::to_fixed_width_hex(combined, 2));
        }
            
        format!("{}{}{}", config_str, Self::to_fixed_width_hex(points.len(), 4), points_str)

    }



    pub fn check_received_data(data: &str, random_verify: &[u8]) -> (bool, Option<DeviceInfo>) {
        info!("Checking received data - data: {}, random_verify: {:02X?}", data, random_verify);
        
        if random_verify.len() != 4 || data.len() < 24 { 
            debug!("Invalid input lengths - random_verify: {}, data: {}", random_verify.len(), data.len());
            return (false, None);
        }


        let footer_idx = match data.rfind(FOOTER) {
            Some(idx) => idx,
            None => {
                debug!("Response data does not contain footer.");
                return (false, None);
            }
        };

        if footer_idx < 16 {
            debug!("Response data is too short to contain verification info.");
            return (false, None);
        }
        let info_and_verify_start = footer_idx - 16;
        let response_verify_hex = &data[info_and_verify_start..info_and_verify_start + 8];
        debug!("Response verification part: {}", response_verify_hex);
        
        let mut expected = Vec::with_capacity(4);
        for (i, &c) in random_verify.iter().enumerate() {

            let c = c as i32;
            let val = match i {
                0 => ((c + 55) >> 1) - 10,
                1 => 7 + ((c - 68) << 1),
                2 => 15 + ((c + 97) >> 1),
                3 => 87 + ((c - 127) >> 1),
                _ => unreachable!(), 
            };

            expected.push(val as u8);
        }

        let mut received = Vec::with_capacity(4);
        for i in 0..4 {
            let hex_pair = &response_verify_hex[i*2..i*2+2];
            match u8::from_str_radix(hex_pair, 16) {
                Ok(value) => received.push(value),
                Err(_) => {
                    debug!("Failed to parse verification hex value: {}", hex_pair);
                    return (false, None);
                }
            }
        }

        debug!("Comparing expected values {:02X?} with received values {:02X?}", expected, received);
        if expected != received {
            info!("Verification mismatch - expected: {:02X?}, received: {:02X?}", expected, received);
            return (false, None);
        }

        debug!("Verification passed successfully");
        
        let device_info_hex = &data[info_and_verify_start + 8..footer_idx];
        
        let device_status = &device_info_hex[..2];
        let version = &device_info_hex[2..4];
        let device_type = &device_info_hex[4..6];
        
        debug!("Parsing device info - status: {}, type: {}, version: {}", 
            device_status, device_type, version);

        let device_on = u8::from_str_radix(device_status, 16).map(|v| v != 0).unwrap_or(false);

        let device_info = DeviceInfo {
            device_on,
            device_type: device_type.to_string(),
            version: version.to_string(),
            user_type: "FF".to_string(),  // Fixed user type as required by test
        };

        info!("Device info parsed successfully: {:?}", device_info);
        (true, Some(device_info))
    }

    pub fn extract_project_item_bits(project_item: &crate::model::Playback) -> Vec<u8> {
        let mut bits = Vec::with_capacity(project_item.selected_plays.len() * 16);
        for &n in &project_item.selected_plays {
            for h in 0..16 {
                let a = ((n >> h) & 1) as u8;
                bits.push(a);
            }
        }
        bits
    }

    pub fn pack_bits_to_prj_selected(bits: &[u8]) -> Vec<u16> {
        let mut prj_selected = Vec::new();
        for chunk in bits.chunks(16) {
            let mut val = 0u16;
            for (h, &bit) in chunk.iter().enumerate() {
                if bit != 0 {
                    val |= 1 << h;
                }
            }
            prj_selected.push(val);
        }
        prj_selected
    }



pub fn pack_xys_cmd(
    segment_points: &Vec<(usize, Vec<PolyPoint>, f32, f32)>,
    time: f32,
) -> String {
    if let Some(encoded_command_data) = Self::encode_layout_to_command_data(
        segment_points,
        time, 
    ) {
        let result_cmd = format!(
            "{}{}{}{}{}{}{}{}{}{}{}{}{}",
            XYS_CMD_HEADER,
            BlueProtocol::to_fixed_width_hex(encoded_command_data.cnt, 4), 
            BlueProtocol::to_fixed_width_hex(encoded_command_data.char_count, 2), 
            encoded_command_data.cmd,
            BlueProtocol::to_fixed_width_hex(1, 2), 
            BlueProtocol::to_fixed_width_hex(encoded_command_data.char_count, 2), 
            encoded_command_data.char_width_cmd,
            encoded_command_data.char_point_cmd,
            encoded_command_data.se1,
            encoded_command_data.se2,
            encoded_command_data.ver,
            encoded_command_data.time,
            XYS_CMD_FOOTER
        );
        return result_cmd.to_uppercase();
    }
    String::new()
}


    /// Encodes layout to command data, matching JS encodeLayoutToCommandData logic.
pub fn encode_layout_to_command_data(
        polyline_segments: &Vec<(usize, Vec<PolyPoint>, f32, f32)>,
        segment_time: f32,
    ) -> Option<EncodedCommandData> {
        let a = 0;
        if polyline_segments.is_empty() {
            return None;
        }
        let mut counter = 0;
        let mut counter2 = 0;
        let mut prev_index = -1;
        let mut command = String::new();
        let ver = Self::to_fixed_width_hex(a, 2);
        let mut char_point_cmd = String::new();
        let mut char_width_cmd = String::new();
        let v = 8;
        let scaling_factor = 0.5;
        let mut f = v;
        let mut segment_point_count = 0;
        let mut time = Self::to_fixed_width_hex(segment_time.floor() as i32, 2);
      
        if v >= 8 {
            f = 0;
        }

        let (xyss, grouped_segments, se1, se2, x_offset, _group_point_counts, _segment_widths) = {
            let seg_data = DrawUtils::generate_segmented_layout_data(
                polyline_segments,
                scaling_factor,
                0,
            );
            (
                seg_data.0,
                seg_data.1,
                seg_data.2,
                seg_data.3,
                seg_data.4,
                seg_data.5,
                seg_data.6,
            )
        };


        let mut segment_boundaries = Vec::new();
        let mut segment_point_counts = Vec::new();
        let mut last_index: Option<usize> = None;
        let mut current_count = 0;
        for seg in &xyss {
            if Some(seg.0) != last_index {
                if let Some(idx) = last_index {
                    segment_point_counts.push(current_count);
                }
                segment_boundaries.push(seg.0);
                last_index = Some(seg.0);
                current_count = 0;
            }
            current_count += seg.1.len();
        }
        if last_index.is_some() {
            segment_point_counts.push(current_count);
        }

        for (ix, seg) in xyss.iter().enumerate() {
            if prev_index != seg.0 as i32 {
                prev_index = seg.0 as i32;
                if counter2 > 0 {
                    char_point_cmd += &Self::to_fixed_width_hex(segment_point_count as i32, 2);
                    segment_point_count = 0;
                }
                counter2 += 1;
                let width = (seg.2 * scaling_factor).round() as i32;
                char_width_cmd += &Self::to_fixed_width_hex(width, 2);

                if v >= 8 && seg.1.len() > 1 {
                    f += 1;
                }
                if f >= 8 {
                    f = 1;
                }
            }
            let segment_points = &seg.1;
            segment_point_count += segment_points.len();
            for (index, point) in segment_points.iter().enumerate() {
                counter += 1;
                let x_screen = (point.x * scaling_factor) + x_offset;
                let y_screen = point.y * scaling_factor;
                let mut point_type = point.z as u8;
                let mut segment_index = f as u8;
                if index == 0 {
                    segment_index = 0;
                    point_type = 1;
                }
                if index == segment_points.len() - 1 {
                    point_type = 1;
                }
                if segment_points.len() == 1 {
                    point_type = point.z as u8;
                }
                let combined = BlueProtocol::combine_nibbles(segment_index, point_type);
                let x_hex = BlueProtocol::to_fixed_width_hex_float(x_screen as f64, 4);
                let y_hex = BlueProtocol::to_fixed_width_hex_float(y_screen as f64, 4);
                let combined_hex = BlueProtocol::to_fixed_width_hex(combined as i32, 2);
                
                command += &x_hex;
                command += &y_hex;
                command += &combined_hex;
            }
        }
        char_point_cmd += &Self::to_fixed_width_hex(segment_point_count as i32, 2);


        if counter == 0 {
            None
        } else {
            Some(EncodedCommandData {
                cnt: counter,
                char_count: counter2,
                cmd: command,
                char_width_cmd,
                char_point_cmd,
                se1,
                se2,
                ver,
                time,
            })
        }
    }


    pub fn pack_draw_shape_command(segment_index: &u8,  config: &DrawConfig) -> String {
    
        let start_marker = "01";
        let segment_index_hex = Self::to_fixed_width_hex(*segment_index as u8, 2);
        let mut packed_hex = format!("{}{}", start_marker, segment_index_hex);

        packed_hex.push_str(&Self::to_fixed_width_hex(config.group_index as u8, 2));
        packed_hex.push_str(&Self::to_fixed_width_hex(config.pattern_index as u8, 2));
        packed_hex.push_str(&Self::to_fixed_width_hex(config.color as u8, 2));
        packed_hex.push_str(&Self::to_fixed_width_hex(config.color_flow_speed as u8, 2));
        packed_hex.push_str(&Self::to_fixed_width_hex(config.pattern_size as u8, 2));
        packed_hex.push_str(&Self::to_fixed_width_hex(config.pattern_scale as u8, 2));
        packed_hex.push_str(&Self::to_fixed_width_hex(config.pattern_rotation as u8, 2));
        packed_hex.push_str(&Self::to_fixed_width_hex(config.pattern_vertical_flip as u8, 2));
        packed_hex.push_str(&Self::to_fixed_width_hex(config.pattern_horizontal_flip as u8, 2));
        packed_hex.push_str(&Self::to_fixed_width_hex(config.pattern_horizontal_position as u8, 2));
        packed_hex.push_str(&Self::to_fixed_width_hex(config.pattern_vertical_position as u8, 2));
        packed_hex.push_str(&Self::to_fixed_width_hex(config.pattern_wave as u8, 2));
        packed_hex.push_str(&Self::to_fixed_width_hex(config.gradient_draw as u8, 2));

        let play_time_hex = Self::to_fixed_width_hex(config.play_time as u8, 2);
        packed_hex.push_str(&play_time_hex);

        // For now, we do not have featureParams/xyCnf logic, so always pad to 18 bytes (36 hex chars)
        // If you want to support xyCnf, add extra config values 14..=18 and pad to 24 bytes (48 hex chars)
        while packed_hex.len() < 36 {
            packed_hex.push_str("00");
        }

    let full_command = format!("{}{}{}", DRAWCONFIG_CMD_HEADER, packed_hex, DRAWCONFIG_CMD_FOOTER);
    full_command.to_uppercase()
    }


    pub fn pack_play_shapes_command(config: &Vec<DrawConfig>) -> String {
        // Segment count: 128 | len (JS: toFixedWidthHex(128 | e.length, 2))
        let segment_count = 128 | (config.len() as u8);
        let segment_count_hex = Self::to_fixed_width_hex(segment_count, 2);
        let segment_end_marker = "FF";
        let mut encoded_segments = String::new();

        for segment in config.iter() {
            let mut segment_hex = String::new();
            
            segment_hex.push_str(&Self::to_fixed_width_hex(segment.group_index as u8, 2));
            segment_hex.push_str(&Self::to_fixed_width_hex(segment.pattern_index as u8, 2));
            segment_hex.push_str(&Self::to_fixed_width_hex(segment.color as u8, 2));
            segment_hex.push_str(&Self::to_fixed_width_hex(segment.color_flow_speed as u8, 2));
            segment_hex.push_str(&Self::to_fixed_width_hex(segment.pattern_size as u8, 2));
            segment_hex.push_str(&Self::to_fixed_width_hex(segment.pattern_scale as u8, 2));
            segment_hex.push_str(&Self::to_fixed_width_hex(segment.pattern_rotation as u8, 2));
            segment_hex.push_str(&Self::to_fixed_width_hex(segment.pattern_vertical_flip as u8, 2));
            segment_hex.push_str(&Self::to_fixed_width_hex(segment.pattern_horizontal_flip as u8, 2));
            segment_hex.push_str(&Self::to_fixed_width_hex(segment.pattern_horizontal_position as u8, 2));
            segment_hex.push_str(&Self::to_fixed_width_hex(segment.pattern_vertical_position as u8, 2));
            segment_hex.push_str(&Self::to_fixed_width_hex(segment.pattern_wave as u8, 2));
            segment_hex.push_str(&Self::to_fixed_width_hex(segment.gradient_draw as u8, 2));

            // Play time: JS multiplies by 10
            let play_time_hex = Self::to_fixed_width_hex((segment.play_time as u16 * 10) as u16, 2);
            segment_hex.push_str(&play_time_hex);




            // No featureParams/xyCnf support for now, so pad to 15 bytes (30 hex chars)
            while segment_hex.len() < 30 {
                segment_hex.push_str("00");
            }
            encoded_segments.push_str(&segment_hex);
            encoded_segments.push_str(segment_end_marker);
        }

        // JS: "d0d1d2d3" + segmentCountHex + "00" + encodedSegments + "d4d5d6d7"
        let full_command = format!(
            "{}{}{}{}{}",
            DRAWCONFIG_CMD_HEADER,
            segment_count_hex,
            "00",
            encoded_segments,
            DRAWCONFIG_CMD_FOOTER
        );
        
        full_command.to_uppercase()

    }


} // end impl BlueProtocol