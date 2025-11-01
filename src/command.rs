use crate::{command, draw::DrawUtils, model::{EncodedCommandData, Point, PolyPoint, Playback}};
use log::{debug, info};
use tokio::time::Timeout;

use crate::model::{CommandConfig, DeviceInfo, DeviceResponse, FeatureConfig, MainCommandData, PisObject, DeviceSettings, PlaybackData, DeviceMode, PlaybackCommand};

pub const HEADER: &str = "E0E1E2E3";
pub const FOOTER: &str = "E4E5E6E7";

pub const POWER_ON_CMD: &str = "B0B1B2B3FFB4B5B6B7";
pub const POWER_OFF_CMD: &str = "B0B1B2B300B4B5B6B7";

const MAIN_CMD_HEADER: &str = "C0C1C2C3";
const MAIN_CMD_FOOTER: &str = "C4C5C6C7";
const SETTINGS_CMD_HEADER: &str = "00010203";
const SETTINGS_CMD_FOOTER: &str = "000000000004050607";
const FEATURES_CMD_HEADER: &str = "D0D1D2D3";
const FEATURES_CMD_FOOTER: &str = "D4D5D6D7";
const DRAW_CMD_HEADER: &str = "F0F1F2F3";
const DRAW_CMD_FOOTER: &str = "F4F5F6F7";
pub const XYS_CMD_HEADER: &str = "A0A1A2A3";
pub const XYS_CMD_FOOTER: &str = "A4A5A6A7";


pub struct CommandGenerator;

impl CommandGenerator {

    pub fn ab2hex(bytes: &[u8]) -> String {
        debug!("ab2hex called with bytes: {:?}", bytes);
        String::new()
    }

    pub fn ab2str(bytes: &[u8]) -> String {
        debug!("ab2str called with bytes: {:?}", bytes);
        String::new()
    }

    pub fn string_to_bytes(s: &str) -> Vec<u8> {
        debug!("string_to_bytes called with s: {}", s);
        Vec::new()
    }
   pub fn to_fixed_width_hex<T: std::fmt::UpperHex>(value: T, width: usize) -> String {
        debug!("to_fixed_width_hex called with width: {}", width);
        format!("{:0width$X}", value, width = width)
    }

    pub fn combine_nibbles(high: u8, low: u8) -> u8 {
        debug!("combine_nibbles called with high: {}, low: {}", high, low);
        // Combine two 4-bit values into one 8-bit value
        // High nibble (upper 4 bits) and low nibble (lower 4 bits)
        (high & 0x0F) << 4 | (low & 0x0F)
    }

    pub fn pad_hex_string_to_byte_length(hex: &str, byte_len: usize, pad: &str) -> String {
        debug!("pad_hex_string_to_byte_length called with hex: {}, byte_len: {}, pad: {}", hex, byte_len, pad);
        String::new()
    }

    pub fn to_fixed_width_hex_float(value: f64, width: usize) -> String {
        // Round the value to nearest integer
        let mut rounded_value = value.round() as i32;
        
        // Handle negative values by setting bit 15 and using absolute value
        if rounded_value < 0 {
            rounded_value = 32768 | -rounded_value;
        }
        
        // Convert to hex string and pad with zeros
        // format! with width specifier handles padding automatically
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
            //info!("Extracted hex value from {}..{}: {}", start, end, &data[start..end]);
            u16::from_str_radix(&data[start..end], 16).unwrap_or(0)
        } else {
            0
        }
    }

    pub fn get_cmd_value(start: &str, end: &str, input: &str) -> Option<String> {
        debug!("get_cmd_value called with start: {}, end: {}", start, end);
        
        if let Some(start_idx) = input.find(start) {
            if let Some(end_idx) = input[start_idx..].find(end) {
                let start_pos = start_idx + start.len();
                let content = &input[start_pos..start_idx + end_idx];
                debug!("Found command value: {}", content);
                return Some(content.to_string());
            }
        }
        debug!("No matching string found that meets the requirements");
        None
    }

    pub fn parse_settings_command(cmd_data: &str) -> Option<DeviceSettings> {

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



    pub fn get_setting_cmd(settings: &DeviceSettings) -> String {

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
    
    /// Parses a complete device response into structured data
    pub fn parse_device_response(data: &str) -> Option<DeviceResponse> {
        
        let response = DeviceResponse {
            main_data: Self::parse_main_command(&data)?,
            settings: Self::parse_settings_command(&data)?,
            device_info: Self::parse_device_info(&data)?,
            features: Self::parse_features(&data)?,
            pis_obj:  Self::parse_pis_command(&data),
        };


        Some(response)
    }

    /// Parse the main command section
    pub fn parse_main_command(cmd_data: &str) -> Option<MainCommandData> {

        let cmd = Self::get_cmd_value(MAIN_CMD_HEADER, MAIN_CMD_FOOTER, cmd_data)?;
        info!("Parsing main command: {} {}", cmd, cmd.len());

        info!("Main command: {} {} {}", MAIN_CMD_HEADER, cmd, MAIN_CMD_FOOTER);

        let value = Self::clamp_value(Self::extract_hex_value(1, 1, &cmd) as u8, 0, 12, 0);
        let device_mode: DeviceMode = DeviceMode::try_from(value).unwrap();

        Some(MainCommandData {
            device_mode: device_mode ,
            audio_trigger_mode: Self::clamp_value(Self::extract_hex_value(2, 1, &cmd) as u8, 0, 9, 0),
            color: Self::clamp_value(Self::extract_hex_value(3, 1, &cmd) as u8, 0, 9, 0),
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
            playback: Self::parse_playback_command(&cmd)?,
        })
    }




    pub fn parse_features(data: &str) -> Option<Vec<FeatureConfig>> {
        let mut features = Vec::new();


        if let Some(features_cmd) = Self::get_cmd_value(FEATURES_CMD_HEADER, FEATURES_CMD_FOOTER, data) {
            
            info!("Parsing features command: {} {}", features_cmd, features_cmd.len());
            
            let feature_count = Self::clamp_value(Self::extract_hex_value(1, 1, &features_cmd), 0, 127, 0);
            let values_per_feature = 16; // or 22 if xy_config is enabled

            for i in 0..feature_count {
                let mut config = FeatureConfig {
                    play_time: 0.0,
                    config_values: Vec::new(),
                };

                for j in 0..values_per_feature {
                    let value = Self::clamp_value(
                        Self::extract_hex_value(3 + i as usize * values_per_feature + j, 1, &features_cmd).try_into().unwrap(),
                        0,
                        255,
                        0
                    );
                    config.config_values.push(value.try_into().unwrap());
                    if j == 13 {
                        config.play_time = value as f32 / 10.0;
                    }
                }

                features.push(config);
            }
            Some(features)
        } else {
            None
        }



    }


    pub fn parse_playback_command(playback_cmd: &str) -> Option<PlaybackData> {
 
        info!("Parsing playback command: {} {}", playback_cmd, playback_cmd.len());

        let audio_trigger_mode = Self::extract_hex_value(9, 1, &playback_cmd) as u8;
        let sound_sensitivity =  Self::clamp_value(Self::extract_hex_value(10, 1, &playback_cmd) as u8, 0, 255, 0);        
            
        let public = crate::model::AudioConfig { audio_trigger_mode, sound_sensitivity };
        
        // Parse ProjectData from main_cmd (example logic, adjust as needed for your model)
        // This assumes project item info is encoded in main_cmd or another section
        // You may need to adjust parsing logic to match your protocol
        let mut prj_item = std::collections::HashMap::new();
        // Use keys [2, 3, 5, 6] for project items
        let prj_keys = [2, 3, 5, 6];
        let mut project_item_start_index = 17;
        for &key in prj_keys.iter() {
            let py_mode = Self::clamp_value(Self::extract_hex_value(project_item_start_index, 1, &playback_cmd), 0, 255, 0) as u8;
            let mut prj_selected = vec![0u16; 4];
            prj_selected[3] = Self::extract_hex_value(project_item_start_index + 1, 2, &playback_cmd);
            prj_selected[2] = Self::extract_hex_value(project_item_start_index + 3, 2, &playback_cmd);
            prj_selected[1] = Self::extract_hex_value(project_item_start_index + 5, 2, &playback_cmd);
            prj_selected[0] = Self::extract_hex_value(project_item_start_index + 7, 2, &playback_cmd);
            prj_item.insert(key, Playback { playback_mode: py_mode, selected_plays: prj_selected });
            project_item_start_index += 9;
        }

        let prj_data = PlaybackData { audio_config: public, playback_items: prj_item };

        Some(prj_data)

    }

    pub fn parse_pis_command(data: &str) -> Option<PisObject> {

        let main_cmd = Self::get_cmd_value(MAIN_CMD_HEADER, MAIN_CMD_FOOTER, data)?;
        info!("Parsing main command: {} {}", main_cmd, main_cmd.len());

        let tx_point_time = CommandGenerator::clamp_value(CommandGenerator::extract_hex_value(15, 1, &main_cmd) as u8, 0, 100, 50);
        let mut cnf_valus_vec = Vec::new();
        for i in 0..13 {
            cnf_valus_vec.push(CommandGenerator::clamp_value(CommandGenerator::extract_hex_value(18 + i, 1, &main_cmd) as u8, 0, 255, 0) as u32);
        }
        let mut cnf_valus_arr = [0u32; 13];
        for (i, val) in cnf_valus_vec.iter().enumerate().take(13) {
            cnf_valus_arr[i] = *val;
        }

        let mut pis_obj = PisObject {
            tx_point_time: tx_point_time as u32,
            cnf_valus: cnf_valus_arr,
        };

        // If draw config section exists, update PisObject fields from draw config data
        if let Some(draw_cmd) = Self::get_cmd_value(DRAW_CMD_HEADER, DRAW_CMD_FOOTER, data) {
            info!("Parsing draw command: {} {}", draw_cmd, draw_cmd.len());
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

    pub fn parse_device_info(data: &str) -> Option<DeviceInfo> {
        // Find footer pattern and extract 8 bytes before it
        if let Some(footer_idx) = data.rfind(FOOTER) {
            if footer_idx >= 8 {
                let info_start = footer_idx - 8;
                let device_info_str = &data[info_start..footer_idx];
                info!("Device info extracted: {}", device_info_str);

                // Expected format: FF000200
                let device_status = &device_info_str[..2];
                let version = &device_info_str[2..4];
                let device_type = &device_info_str[4..6];

                // Create device info with known values
                return Some(DeviceInfo {
                    device_on: true,               // Status FF = device on
                    device_type: device_type.to_string(),
                    version: version.to_string(),
                    user_type: device_status.to_string(), // User type from status
                });
            }
        }
        None
    }


    pub fn get_query_cmd(random_verify: &[u8]) -> String {
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
                playback_mode: 128,
                selected_plays: vec![0; 4],
            });
            let play_back_mode = if playback.playback_mode == 0 { 0 } else { 128 };
            let play_back_mode_hex = Self::to_fixed_width_hex(play_back_mode, 2);
            let mut show_selected_hex = String::new();
            for &val in playback.selected_plays.iter().rev() {
                show_selected_hex.push_str(&Self::to_fixed_width_hex(val, 4));
            }
            playback_selection_hex.push_str(&format!("{}{}", play_back_mode_hex, show_selected_hex));
        }

        // Q: padding (JS logic: run_direction + padding = 44 bytes)
        let mut padding = String::new();
        let run_direction_bytes = run_direction.len() / 2;
        if run_direction_bytes < 44 {
            padding = "00".repeat(44 - run_direction_bytes);
        }

        let filler = "00000000".to_string();

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
            filler,  
            text_point_time_hex,        
            draw_point_time_hex,               
            playback_selection_hex,                      
            run_direction,           
            padding,               
            MAIN_CMD_FOOTER         
        );
        command.to_uppercase()

    }

    // Configuration commands
    pub fn get_cmd_str(config: &CommandConfig) -> String {
    
        let cur_mode_hex = Self::to_fixed_width_hex(config.cur_mode, 2);
        let reserved_hex = Self::to_fixed_width_hex(0, 2);
        let color_hex = Self::to_fixed_width_hex(config.text_data.tx_color, 2);
        let tx_size_scaled = ((config.text_data.tx_size as f64) / 100.0 * 255.0).round() as u8;
        let tx_size_scaled_a_hex = Self::to_fixed_width_hex(tx_size_scaled, 2);
        let tx_size_scaled_b_hex = Self::to_fixed_width_hex(tx_size_scaled, 2);
        let run_speed_scaled = ((config.text_data.run_speed as f64) / 100.0 * 255.0).round() as u8;
        let run_speed_hex = Self::to_fixed_width_hex(run_speed_scaled, 2);
        let l = "00".to_string();
        let tx_dist_scaled = ((config.text_data.tx_dist as f64) / 100.0 * 255.0).round() as u8;
        let tx_dist_scaled_hex = Self::to_fixed_width_hex(tx_dist_scaled, 2);
        let audio_trigger_mode_hex = Self::to_fixed_width_hex(config.prj_data.audio_config.audio_trigger_mode, 2);
        // let sound_sensitivity_scaled = ((config.prj_data.public.sound_val as f64) / 100.0 * 255.0).round() as u8;
        // let sound_sensitivity_hex = Self::to_fixed_width_hex(sound_sensitivity_scaled, 2);

        // x: group color segment
    let x = "ffffffff0000".to_string();
        //if let Some(features) = features {
        //    x.clear();
        //    if let Some(group_list) = &features.group_list {
         //       for group in group_list {
        //            x += &Self::to_fixed_width_hex(group.color, 2);
        //        }
        //    }
        //    x += "ffffffff";
        //    x = x.chars().take(8).collect();
            //if Self::get_feature_value(features, "textStopTime").unwrap_or(false) {
            //    x += &Self::to_fixed_width_hex(config.text_data.tx_point_time, 2);
            //}
        //    x += "0000";
        //    x = x.chars().take(12).collect();
        //}

        // f: project items (ordered by protocol: TimelinePlayback, AnimationPlayback, ChristmasBroadcast, OutdoorPlayback)
        let mut f = String::new();
        let prj_keys = [2, 3, 5, 6];
        for &key in prj_keys.iter() {
            let project_item = config.prj_data.playback_items.get(&key).cloned().unwrap_or_else(|| Playback {
                playback_mode: 128,
                selected_plays: vec![0; 4],
            });
            let play_back_mode = if project_item.playback_mode == 0 { 0 } else { 128 };
            let play_back_mode_hex = Self::to_fixed_width_hex(play_back_mode, 2);
            let mut prj_selected_hex = String::new();
            for &val in project_item.selected_plays.iter().rev() {
                prj_selected_hex.push_str(&Self::to_fixed_width_hex(val, 4));
            }
            f.push_str(&format!("{}{}", play_back_mode_hex, prj_selected_hex));
        }

        // z: run direction if arbPlay
        let run_direction = String::new();
        //if let Some(features) = features {
        //    if Self::get_feature_value(features, "arbPlay").unwrap_or(false) {
        //        run_direction += &Self::to_fixed_width_hex(config.text_data.run_dir, 2);
        //    }
        //}

        // Q: padding (JS logic: run_direction + padding = 44 bytes)
        let mut padding = String::new();
        let run_direction_bytes = run_direction.len() / 2;
        if run_direction_bytes < 44 {
            padding = "00".repeat(44 - run_direction_bytes);
        }

        // Compose command using header/footer constants, matching JS order
        let sound_sensitivity_hex = Self::to_fixed_width_hex(((config.prj_data.audio_config.sound_sensitivity as f64) / 100.0 * 255.0).round() as u8, 2);
        let command = format!(
            "{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            MAIN_CMD_HEADER,         // header
            cur_mode_hex,            // curMode
            reserved_hex,            // reserved
            color_hex,               // color
            tx_size_scaled_a_hex,    // txSizeA
            tx_size_scaled_b_hex,    // txSizeB
            run_speed_hex,           // runSpeed
            l,                      // l
            tx_dist_scaled_hex,      // txDist
            audio_trigger_mode_hex,  // audioTriggerMode
            sound_sensitivity_hex,   // soundSensitivity
            x,                      // x (group color segment)
            f,                      // f (project items)
            run_direction,           // runDirection
            padding,                 // padding
            MAIN_CMD_FOOTER         // footer
        );
        command.to_uppercase()
    }


    pub fn get_draw_cmd_str(points: &[Point], config: &PisObject) -> String {
        let encoded_draw_cmd = Self::encode_draw_point_command(points, config);
        Self::draw_point_str_to_cmd(&encoded_draw_cmd)
    }

    pub fn encode_draw_point_command(points: &[Point], config: &PisObject, ) -> String {
        let point_time = "00";  // Default point time
        let mut config_str = String::new();
        let mut points_str = String::new();
        
        // Build config values (15 iterations as in JS)
        for index in 0..15 {
            if index <= 11 {
                // Use cnf_valus for indices 0-11
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


    /// Convert draw point string to command format with headers and footers
    pub fn draw_point_str_to_cmd(point_string: &str) -> String {
        let command_str = format!("{}{}{}", DRAW_CMD_HEADER, point_string, DRAW_CMD_FOOTER);
       
        command_str.to_uppercase()
    }


    /// Verifies the received data against the random verification bytes sent in the query.
    /// Returns a tuple of (bool, DeviceInfo) where bool indicates if verification passed.
    pub fn check_received_data(data: &str, random_verify: &[u8]) -> (bool, Option<DeviceInfo>) {
        info!("Checking received data - data: {}, random_verify: {:02X?}", data, random_verify);
        
        // Validate input lengths. The full response needs to be at least long enough
        // to contain the header, footer, verification, and device info.
        if random_verify.len() != 4 || data.len() < 24 { // 12-byte query + 12-byte response
            debug!("Invalid input lengths - random_verify: {}, data: {}", random_verify.len(), data.len());
            return (false, None);
        }

        // Find the footer to locate the verification and device info bytes.
        let footer_idx = match data.rfind(FOOTER) {
            Some(idx) => idx,
            None => {
                debug!("Response data does not contain footer.");
                return (false, None);
            }
        };

        // The 16 bytes before the footer contain verification (8 hex chars) and device info (8 hex chars).
        // Check if the string is long enough to prevent a panic.
        if footer_idx < 16 {
            debug!("Response data is too short to contain verification info.");
            return (false, None);
        }
        let info_and_verify_start = footer_idx - 16;
        let response_verify_hex = &data[info_and_verify_start..info_and_verify_start + 8];
        debug!("Response verification part: {}", response_verify_hex);
        
        // Calculate the expected response by applying a transformation to the random bytes.
        // This logic was reverse-engineered from the device's behavior. The device
        // uses a specific arithmetic formula for each byte of the random challenge.
        let mut expected = Vec::with_capacity(4);
        for (i, &c) in random_verify.iter().enumerate() {
            // Re-implementing the verification formula from the original JavaScript code.
            // We cast to i32 to correctly handle potential negative intermediate results,
            // mimicking JavaScript's number handling.
            let c = c as i32;
            let val = match i {
                0 => ((c + 55) >> 1) - 10,
                1 => 7 + ((c - 68) << 1),
                2 => 15 + ((c + 97) >> 1),
                3 => 87 + ((c - 127) >> 1),
                _ => unreachable!(), // Should not happen with a 4-byte array
            };
            // The `& 255` in JS is equivalent to casting back to u8 in Rust,
            // wrapping the value to a single byte.
            expected.push(val as u8);
        }

        // Parse received verification bytes from hex string
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

        // Compare expected with received
        debug!("Comparing expected values {:02X?} with received values {:02X?}", expected, received);
        if expected != received {
            info!("Verification mismatch - expected: {:02X?}, received: {:02X?}", expected, received);
            return (false, None);
        }

        debug!("Verification passed successfully");
        
        // Extract and parse device information. It's in the 8 hex chars after verification.
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


    /// Unpacks prj_selected from ProjectItem into a flat Vec<u8> of bits (0/1), matching JS getCkValues
    pub fn unpack_project_item_bits(project_item: &crate::model::Playback) -> Vec<u8> {
        let mut bits = Vec::with_capacity(project_item.selected_plays.len() * 16);
        for &n in &project_item.selected_plays {
            for h in 0..16 {
                let a = ((n >> h) & 1) as u8;
                bits.push(a);
            }
        }
        bits
    }

    /// Packs a flat Vec<u8> of bits (0/1) into a Vec<u16>, inverse of unpack_project_item_bits
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



pub fn get_xys_cmd(
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
            CommandGenerator::to_fixed_width_hex(encoded_command_data.cnt, 4), 
            CommandGenerator::to_fixed_width_hex(encoded_command_data.char_count, 2), 
            encoded_command_data.cmd,
            CommandGenerator::to_fixed_width_hex(1, 2), 
            CommandGenerator::to_fixed_width_hex(encoded_command_data.char_count, 2), 
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
        let mut b = String::new();
        let ver = Self::to_fixed_width_hex_b(a, 2);
        let mut char_point_cmd = String::new();
        let mut char_width_cmd = String::new();
        let v = 8;
        let scaling_factor = 0.5;
        let mut f = v;
        let mut segment_point_count = 0;
        let mut time = Self::to_fixed_width_hex_b(segment_time.floor() as i32, 2);
      
        if v >= 8 {
            f = 0;
        }
        let test = false;
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
        // Debug: print segment grouping and metadata
        println!("[Rust] generate_segmented_layout_data output:");
        println!("  xyss.len(): {}", xyss.len());
        println!("  se1: {:?}", se1);
        println!("  se2: {:?}", se2);
        println!("  x_offset: {}", x_offset);
        // Print segment indices and point counts
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
        println!("  segment_boundaries: {:?}", segment_boundaries);
        println!("  segment_point_counts: {:?}", segment_point_counts);
        // Print xyss segment array for parity comparison
        println!("[Rust] xyss (segment array):");
        for (seg_idx, seg) in xyss.iter().enumerate() {
            println!("  seg {}: idx={}, width={:.2}, points={}", seg_idx, seg.0, seg.2, seg.1.len());
            for (pt_idx, pt) in seg.1.iter().enumerate() {
                println!("    pt {}: x={:.2} y={:.2} z={}", pt_idx, pt.x, pt.y, pt.z);
            }
        }
        // Packing loop
        for (ix, seg) in xyss.iter().enumerate() {
            if prev_index != seg.0 as i32 {
                prev_index = seg.0 as i32;
                if counter2 > 0 {
                    char_point_cmd += &Self::to_fixed_width_hex_b(segment_point_count as i32, 2);
                    println!(
                        "[Rust] char_point_cmd append: seg {} count {} -> {}",
                        counter2 - 1,
                        segment_point_count,
                        Self::to_fixed_width_hex_b(segment_point_count as i32, 2)
                    );
                    segment_point_count = 0;
                }
                counter2 += 1;
                let width = (seg.2 * scaling_factor).round() as i32;
                char_width_cmd += &Self::to_fixed_width_hex_b(width, 2);
                println!(
                    "[Rust] char_width_cmd append: seg {} width {} -> {}",
                    counter2 - 1,
                    width,
                    Self::to_fixed_width_hex_b(width, 2)
                );
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
                let combined = CommandGenerator::combine_nibbles_b(segment_index, point_type);
                let x_hex = CommandGenerator::to_fixed_width_hex_float(x_screen as f64, 4);
                let y_hex = CommandGenerator::to_fixed_width_hex_float(y_screen as f64, 4);
                let combined_hex = CommandGenerator::to_fixed_width_hex_b(combined as i32, 2);
                // Print packed hex for each point for parity comparison
                println!("[Rust] Packed point: seg={} idx={} x={:.3} y={:.3} z={} segIdx={} type={} -> {}{}{}", seg.0, index, x_screen, y_screen, point.z, segment_index, point_type, x_hex, y_hex, combined_hex);
                command += &x_hex;
                command += &y_hex;
                command += &combined_hex;
            }
        }
        char_point_cmd += &Self::to_fixed_width_hex_b(segment_point_count as i32, 2);
        println!(
            "[Rust] char_point_cmd final append: seg {} count {} -> {}",
            counter2 - 1,
            segment_point_count,
            Self::to_fixed_width_hex_b(segment_point_count as i32, 2)
        );
        // Print all packed fields for parity analysis
        println!("[Rust] encode_layout_to_command_data packed fields:");
        println!("  cnt: {}", counter);
        println!("  charCount: {}", counter2);
        println!("  cmd: {}", command);
        println!("  charWidthCmd: {}", char_width_cmd);
        println!("  charPointCmd: {}", char_point_cmd);
        println!("  se1: {}", se1);
        println!("  se2: {}", se2);
        println!("  ver: {}", ver);
        println!("  time: {}", time);
        if test {
            println!("Text coordinates (drawing software format): {}", b);
        }
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



pub fn combine_nibbles_b(a: u8, b: u8) -> u8 {
    ((a & 0x0F) << 4) | (b & 0x0F)
}

pub fn to_fixed_width_hex_b(val: i32, width: usize) -> String {
    let clamped = if width == 2 {
        val.max(0).min(255) as u32
    } else {
        val as u32
    };
    format!("{:0width$X}", clamped, width = width)
}



}