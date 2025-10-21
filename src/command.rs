use crate::{dmxliterals::{OFF, ON}, model::{DrawData, DrawItem, DrawMode, DrawPoint, Point, ProjectItem}};
use log::{debug, info};

use crate::model::{CommandConfig, DeviceInfo, DeviceResponse, DmxLaserState, FeatureConfig, MainCommandData, PisObject, SettingsData};

pub const HEADER: &str = "E0E1E2E3";
pub const FOOTER: &str = "E4E5E6E7";

pub const POWER_ON_CMD: &str = "B0B1B2B3FFB4B5B6B7";
pub const POWER_OFF_CMD: &str = "B0B1B2B300B4B5B6B7";

const MAIN_CMD_HEADER: &str = "C0C1C2C3";
const MAIN_CMD_FOOTER: &str = "C4C5C6C7";
const SETTINGS_CMD_HEADER: &str = "00010203";
const SETTINGS_CMD_FOOTER: &str = "04050607";
const FEATURES_CMD_HEADER: &str = "D0D1D2D3";
const FEATURES_CMD_FOOTER: &str = "D4D5D6D7";
const DRAW_CMD_HEADER: &str = "F0F1F2F3";
const DRAW_CMD_FOOTER: &str = "F4F5F6F7";

const REFERENCE_COORDINATE_SIZE: f64 = 800.0;


pub struct CommandGenerator;

impl CommandGenerator {

    // Core conversion utilities
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

    /// Get command value between patterns, matching JavaScript implementation
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

    pub fn get_setting_cmd(settings: &SettingsData) -> String {
        // Convert values to hex strings matching JavaScript's toFixedWidthHex
        let channel_hex = Self::to_fixed_width_hex(settings.values[0], 4); // valArr[0] - 2 bytes
        let ch_hex = Self::to_fixed_width_hex(settings.channel as u16, 2);
        let display_hex = Self::to_fixed_width_hex(settings.values[1], 2); // valArr[1]
        let xy_hex = Self::to_fixed_width_hex(settings.xy as u16, 2);
        
        // RGB values - handle special case when cfg is 0
        let (r_hex, g_hex, b_hex) = if settings.cfg == 0 {
            ("FF".to_string(), "FF".to_string(), "FF".to_string())
        } else {
            (
                Self::to_fixed_width_hex(settings.values[2], 2),
                Self::to_fixed_width_hex(settings.values[3], 2),
                Self::to_fixed_width_hex(settings.values[4], 2)
            )
        };

        let light_hex = Self::to_fixed_width_hex(settings.light as u16, 2);
        let cfg_hex = Self::to_fixed_width_hex(settings.cfg as u16, 2);
        let lang_hex = Self::to_fixed_width_hex(0, 2); // Default to "00" for now

        // Construct the final command string
        format!(
            "{}{}{}{}{}{}{}{}{}{}{}{}", 
            SETTINGS_CMD_HEADER,
            channel_hex,   // valArr[0]
            ch_hex,       // ch
            display_hex,  // valArr[1]
            xy_hex,      // xy
            r_hex,       // valArr[2]
            g_hex,       // valArr[3]
            b_hex,       // valArr[4]
            light_hex,   // light
            cfg_hex,     // cfg
            lang_hex,    // lang
            SETTINGS_CMD_FOOTER
        ).to_uppercase()
    }
    
    // Layout and segmentation functions
    pub fn split_into_segments_by_sum_limit(values: &[f64], limit: f64) -> Vec<(usize, usize)> {
        debug!("split_into_segments_by_sum_limit called with values: {:?}, limit: {}", values, limit);
        Vec::new()
    }

    pub fn generate_segmented_layout_data(layout: &[Vec<f64>], scale: f64, direction: i32) -> (Vec<Vec<f64>>, String, String, f64) {
        debug!("generate_segmented_layout_data called with layout: {:?}, scale: {}, direction: {}", layout, scale, direction);
        (Vec::new(), String::new(), String::new(), 0.0)
    }

    /// Helper function to extract and clamp numeric values
    fn clamp_value<T: PartialOrd + Copy>(value: T, min: T, max: T, default: T) -> T {
        if value < min || value > max {
            default
        } else {
            value
        }
    }

    /// Extract hex value from a position in command data, matching JavaScript behavior
    fn extract_hex_value(pos: usize, len: usize, data: &str) -> u16 {
        let start = if pos > 0 { 2 * (pos - 1) } else { 0 };
        let end = start + 2 * len;
        if end <= data.len() {
            u16::from_str_radix(&data[start..end], 16).unwrap_or(0)
        } else {
            0
        }
    }

    /// Parse the main command section
    fn parse_main_command(cmd: &str) -> Option<MainCommandData> {
        Some(MainCommandData {
            current_mode: Self::clamp_value(Self::extract_hex_value(1, 1, cmd) as u8, 0, 12, 0),
            text_color: Self::clamp_value(Self::extract_hex_value(3, 1, cmd) as u8, 0, 9, 0),
            text_size: {
                let raw = Self::extract_hex_value(4, 1, cmd);
                Self::clamp_value((raw as f32 / 255.0 * 100.0) as u8, 10, 100, 60)
            },
            run_speed: {
                let raw = Self::extract_hex_value(6, 1, cmd);
                Self::clamp_value((raw as f32 / 255.0 * 100.0) as u8, 0, 255, 128)
            },
            text_distance: {
                let raw = Self::extract_hex_value(8, 1, cmd);
                Self::clamp_value((raw as f32 / 255.0 * 100.0) as u8, 10, 100, 60)
            },
            audio_mode: Self::clamp_value(Self::extract_hex_value(9, 1, cmd) as u8, 0, 255, 0),
            sound_value: {
                let raw = Self::extract_hex_value(10, 1, cmd);
                Self::clamp_value((raw as f32 / 255.0 * 100.0) as u8, 0, 255, 0)
            },
            text_point_time: Self::clamp_value(Self::extract_hex_value(15, 1, cmd) as u8, 0, 100, 50),
            draw_point_time: Self::clamp_value(Self::extract_hex_value(16, 1, cmd) as u8, 0, 100, 50),
            run_direction: Self::clamp_value(Self::extract_hex_value(17, 1, cmd) as u8, 0, 255, 0),
        })
    }

    /// Parse the settings command section
    fn parse_settings_command(cmd: &str) -> SettingsData {
        
        // Extract and clamp values as per JavaScript logic
        let channel_val = Self::clamp_value(Self::extract_hex_value(1, 2, cmd), 1, 512, 1) as u16;
        let channel = Self::extract_hex_value(3, 1, cmd) as u8;
        let display_val = Self::clamp_value(Self::extract_hex_value(4, 1, cmd), 10, 100, 10) as u8;
        let xy = Self::clamp_value(Self::extract_hex_value(5, 1, cmd), 0, 7, 0) as u8;
        let mut r_val = Self::clamp_value(Self::extract_hex_value(6, 1, cmd), 0, 255, 255) as u8;
        let mut g_val = Self::clamp_value(Self::extract_hex_value(7, 1, cmd), 0, 255, 255) as u8;
        let mut b_val = Self::clamp_value(Self::extract_hex_value(8, 1, cmd), 0, 255, 255) as u8;
        let light = Self::clamp_value(Self::extract_hex_value(9, 1, cmd), 1, 3, 3) as u8;
        let cfg = Self::clamp_value(Self::extract_hex_value(10, 1, cmd), 0, 255, 0) as u8;

        // If cfg == 0, force RGB to 255 (as in JS logic)
        if cfg == 0 {
            r_val = 255;
            g_val = 255;
            b_val = 255;
        }

        SettingsData {
            values: [channel_val, display_val as u16, r_val as u16, g_val as u16, b_val as u16],
            channel,
            dmx: 0, // Default 0
            xy,
            light,
            cfg,
            lang: String::from("en"),
        }
    }

    /// Parses a complete device response into structured data
    pub fn parse_device_response(data: &str) -> Option<DeviceResponse> {
        // Parse main and settings command sections
        let main_cmd = Self::get_cmd_value(MAIN_CMD_HEADER, MAIN_CMD_FOOTER, data)?;
        let settings_cmd = Self::get_cmd_value(SETTINGS_CMD_HEADER, SETTINGS_CMD_FOOTER, data)?;

        // Parse ProjectData from main_cmd (example logic, adjust as needed for your model)
        // This assumes project item info is encoded in main_cmd or another section
        // You may need to adjust parsing logic to match your protocol
        let mut prj_item = std::collections::HashMap::new();
        // Use keys [2, 3, 5, 6] for project items
        let prj_keys = [2, 3, 5, 6];
        let mut project_item_start_index = 17;
        for &key in prj_keys.iter() {
            let py_mode = Self::clamp_value(Self::extract_hex_value(project_item_start_index, 1, &main_cmd), 0, 255, 0) as u8;
            let mut prj_selected = vec![0u16; 4];
            prj_selected[3] = Self::extract_hex_value(project_item_start_index + 1, 2, &main_cmd);
            prj_selected[2] = Self::extract_hex_value(project_item_start_index + 3, 2, &main_cmd);
            prj_selected[1] = Self::extract_hex_value(project_item_start_index + 5, 2, &main_cmd);
            prj_selected[0] = Self::extract_hex_value(project_item_start_index + 7, 2, &main_cmd);
            prj_item.insert(key, ProjectItem { py_mode, prj_selected });
            project_item_start_index += 9;
        }

        // Example: parse PublicData from main_cmd
        let rd_mode = Self::extract_hex_value(2, 1, &main_cmd) as u8;
        let sound_val = Self::extract_hex_value(3, 1, &main_cmd) as u8;
        let public = crate::model::PublicData { rd_mode, sound_val };

        let prj_data = crate::model::ProjectData { public, prj_item };

        // PisObject extraction from main_cmd
        let tx_point_time = Self::clamp_value(Self::extract_hex_value(15, 1, &main_cmd) as u8, 0, 100, 50);
        let mut cnf_valus_vec = Vec::new();
       
        for i in 0..13 {
            cnf_valus_vec.push(Self::clamp_value(Self::extract_hex_value(18 + i, 1, &main_cmd) as u8, 0, 255, 0) as u32);
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

        let mut response = DeviceResponse {
            main_data: Self::parse_main_command(&main_cmd)?,
            settings: Self::parse_settings_command(&settings_cmd),
            features: Vec::new(),

            device_info: None,
            prj_data: Some(prj_data),
            pis_obj: Some(pis_obj),
        };

        // Parse features section
        if let Some(features_cmd) = Self::get_cmd_value(FEATURES_CMD_HEADER, FEATURES_CMD_FOOTER, data) {
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

                response.features.push(config);
            }
        }

        // Parse draw config section
        if let Some(draw_cmd) = Self::get_cmd_value(DRAW_CMD_HEADER, DRAW_CMD_FOOTER, data) {
            for i in 0..15 {
                let value = Self::clamp_value(Self::extract_hex_value(i + 1, 1, &draw_cmd).try_into().unwrap(), 0, 255, 0);
                if i < 13 {
                    if let Some(ref mut pis_obj) = response.pis_obj {
                        pis_obj.cnf_valus[i] = value;
                    }
                } else if i == 14 {
                    if let Some(ref mut pis_obj) = response.pis_obj {
                        pis_obj.tx_point_time = value;
                    }
                }
            }
        }

        // Extract device info from end of data (FF000200E4E5E6E7)
        // Find footer pattern and extract 8 bytes before it
        if let Some(footer_idx) = data.rfind("E4E5E6E7") {
            let info_start = footer_idx - 8;
            let device_info_str = &data[info_start..footer_idx];
            info!("Device info extracted: {}", device_info_str);

            // Expected format: FF000200
            let _device_status = &device_info_str[..2];
            let _version = &device_info_str[2..4];
            let _device_type = &device_info_str[4..6];

            // Create device info with known values
            response.device_info = Some(DeviceInfo {
                device_on: true,               // Status FF = device on
                device_type: "02".to_string(), // Type = 02
                version: "00".to_string(),     // Version = 00
                user_type: "FF".to_string()    // User type must be FF
            });
        }

        Some(response)
    }

    pub fn get_query_cmd(random_verify: &[u8]) -> String {
        // Create the middle section using the random bytes
        let middle = if random_verify.len() >= 4 {
            format!("{:02X}{:02X}{:02X}{:02X}", 
                random_verify[0], random_verify[1], 
                random_verify[2], random_verify[3])
        } else {
            "00000000".to_string() // Default if not enough random bytes
        };
        
        // Construct the full command: header + middle + footer (12 bytes total)
        info!("Generating query command with random bytes: {:02X?}", random_verify);
        let cmd = format!("{}{}{}", HEADER, middle, FOOTER).to_uppercase();
        info!("Generated command: {}", cmd);
        cmd
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
        let audio_trigger_mode_hex = Self::to_fixed_width_hex(config.prj_data.public.rd_mode, 2);
        // let sound_sensitivity_scaled = ((config.prj_data.public.sound_val as f64) / 100.0 * 255.0).round() as u8;
        // let sound_sensitivity_hex = Self::to_fixed_width_hex(sound_sensitivity_scaled, 2);

        // x: group color segment
        let mut x = "ffffffff0000".to_string();
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
            let project_item = config.prj_data.prj_item.get(&key).cloned().unwrap_or_else(|| ProjectItem {
                py_mode: 128,
                prj_selected: vec![0; 4],
            });
            let play_back_mode = if project_item.py_mode == 0 { 0 } else { 128 };
            let play_back_mode_hex = Self::to_fixed_width_hex(play_back_mode, 2);
            let mut prj_selected_hex = String::new();
            for &val in project_item.prj_selected.iter().rev() {
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
        let sound_sensitivity_hex = Self::to_fixed_width_hex(((config.prj_data.public.sound_val as f64) / 100.0 * 255.0).round() as u8, 2);
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
    pub fn unpack_project_item_bits(project_item: &crate::model::ProjectItem) -> Vec<u8> {
        let mut bits = Vec::with_capacity(project_item.prj_selected.len() * 16);
        for &n in &project_item.prj_selected {
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


    
    pub fn prepare_draw_data(draw_data: &DrawData, width: f64) -> Vec<Point> {
        let mut points = Vec::new();
        

        for draw_object in &draw_data.draw_points {
            let object_points = match draw_object.draw_mode {
                DrawMode::Polylines => {
                    Self::draw_all_transformed_polylines(draw_object, width)
                }
                DrawMode::Text => {
                    Self::draw_transformed_text(draw_object, width)
                }
                _ => {
                    Self::draw_transformed_object(draw_object, width)
                }
            };
            
            // Concatenate results (points = points.concat(currentDrawResult))
            points.extend(object_points);
        }
        
        points
    }
    

    fn draw_transformed_object(draw_object: &DrawItem, width: f64) -> Vec<Point> {
        let rotated_points = Self::rotate_points_around_bounding_box_center(&draw_object.get_all_points(), draw_object.ang);
        
        let mut result_points = Vec::new();
        let scaling_factor = REFERENCE_COORDINATE_SIZE / width;  // scalingFactor = REFERENCE_COORDINATE_SIZE / width
        let center_offset_x = width / 2.0;   // centerOffsetX = width / 2
        let position_x = draw_object.x0;     // positionX = drawObject.x0
        let position_y = draw_object.y0;     // positionY = drawObject.y0
        let scale_z = draw_object.z;         // scaleZ = drawObject.z
        
        // Color calculation logic
        let base_line_color = draw_object.line_color as i32;  // baseLineColor = drawObject.lineColor
        let color_segment_index = base_line_color - 9;        // colorSegmentIndex = baseLineColor - 9
        let mut current_color_index = if base_line_color >= 8 { -1 } else { base_line_color }; // currentColorIndex = baseLineColor >= 8 ? -1 : baseLineColor
        
 
        for (point_index, rotated_point) in rotated_points.iter().enumerate() {
 
            if color_segment_index < 0 {
                current_color_index = if base_line_color >= 8 { 
                    current_color_index + 1 
                } else { 
                    current_color_index 
                };
                current_color_index = if current_color_index >= 8 { 1 } else { current_color_index };
            } else {
                current_color_index = 1;
            }
            
            let final_color = if rotated_point.color != 0 {
                if color_segment_index < 0 {
                    current_color_index as u8
                } else if color_segment_index == 0 {
                    rotated_point.color  // Keep original color
                } else {
                    current_color_index as u8
                }
            } else {
                rotated_point.color
            };
            
            let result_x = rotated_point.x * scaling_factor * scale_z + (position_x - center_offset_x) * scaling_factor;
            let result_y = rotated_point.y * scale_z * scaling_factor + (-position_y + center_offset_x) * scaling_factor;
            let result_color = if point_index == 0 { 0 } else { final_color };
            let result_pen_state = rotated_point.pen_state;
            
            result_points.push(Point::new(result_x, result_y, result_color, result_pen_state));
        }
        
        result_points
    }
    

    fn draw_all_transformed_polylines(draw_object: &DrawItem, width: f64) -> Vec<Point> {
        let mut accumulated_results = Vec::new();
        
        if let crate::model::DrawPoints::Polylines(polylines) = &draw_object.ps {
            for (index, _polyline) in polylines.iter().enumerate() {
                let current_polyline_result = Self::draw_transformed_polyline(draw_object, index, width);
                accumulated_results.extend(current_polyline_result);
            }
        }
        
        accumulated_results
    }

    fn draw_transformed_polyline(draw_object: &DrawItem, index: usize, width: f64) -> Vec<Point> {
        // Get the specific polyline at the given index
        if let crate::model::DrawPoints::Polylines(polylines) = &draw_object.ps {
            if let Some(polyline) = polylines.get(index) {
                // Rotate points around bounding box center (passing true for polyline mode)
                let rotated_points = Self::rotate_points_around_bounding_box_center_polyline(polyline, draw_object.ang);
                
                let base_line_color = draw_object.line_color;
                let mut current_color_index = if base_line_color >= 8 { 1 } else { base_line_color };
                let color_segment_index = base_line_color.wrapping_sub(9); // This will wrap around for values < 9
                
                let position_x = draw_object.x0;
                let position_y = draw_object.y0;
                let scale_z = draw_object.z;
                
                let scaling_factor = REFERENCE_COORDINATE_SIZE / width;
                let center_offset = width / 2.0;
                let mut result_points = Vec::new();
                
                for (k, rotated_point) in rotated_points.iter().enumerate() {
                    // Transform the point coordinates
                    let transformed_x = rotated_point.x * scale_z + position_x;
                    let transformed_y = rotated_point.y * scale_z + position_y;
                    
                    // Handle color logic
                    if base_line_color >= 8 {
                        if color_segment_index < 0 { // This handles the wrapping case
                            current_color_index += 1;
                            if current_color_index >= 8 {
                                current_color_index = 1;
                            }
                        }
                        // TODO: Handle color segment array logic when we have color segments
                    }
                    
                    // Create result point with coordinate transformation
                    let final_x = (transformed_x - center_offset) * scaling_factor;
                    let final_y = (center_offset - transformed_y) * scaling_factor;
                    let color = if k == 0 { 0 } else { current_color_index };
                    let pen_state = rotated_point.pen_state;
                    
                    result_points.push(Point::from_js_array(final_x, final_y, color as f64, pen_state as f64));
                }
                
                return result_points;
            }
        }
        
        vec![]
    }

    /// Rotate points around bounding box center for a single polyline
    fn rotate_points_around_bounding_box_center_polyline(points: &[DrawPoint], angle_degrees: f64) -> Vec<DrawPoint> {
        // This is similar to the regular rotation but for a single polyline
        if points.is_empty() || angle_degrees == 0.0 {
            return points.to_vec();
        }

        // Calculate bounding box
        let min_x = points.iter().map(|p| p.x).fold(f64::INFINITY, f64::min);
        let max_x = points.iter().map(|p| p.x).fold(f64::NEG_INFINITY, f64::max);
        let min_y = points.iter().map(|p| p.y).fold(f64::INFINITY, f64::min);
        let max_y = points.iter().map(|p| p.y).fold(f64::NEG_INFINITY, f64::max);

        let center_x = (min_x + max_x) / 2.0;
        let center_y = (min_y + max_y) / 2.0;

        // Rotate each point around the bounding box center
        points.iter()
            .map(|point| {
                let (rotated_x, rotated_y) = Self::rotate_point_around_center(angle_degrees, center_x, center_y, point.x, point.y);
                DrawPoint {
                    x: rotated_x,
                    y: rotated_y,
                    color: point.color,
                    pen_state: point.pen_state,
                }
            })
            .collect()
    }
    
    /// Placeholder for drawTransformedText2 (drawMode == 9999)  
    fn draw_transformed_text(_draw_object: &DrawItem, _width: f64) -> Vec<Point> {
        // TODO: Implement text transformation
        Vec::new()
    }
    
    /// Rust implementation of JavaScript rotatePointsAroundBoundingBoxCenter
    fn rotate_points_around_bounding_box_center(points: &[DrawPoint], angle: f64) -> Vec<DrawPoint> {
        if points.is_empty() {
            return Vec::new();
        }
        
        let mut rotated_points = Vec::new();
        
        // Calculate bounding box
        let mut left = f64::MAX;
        let mut top = f64::MAX; 
        let mut right = f64::MIN;
        let mut bottom = f64::MIN;
        
        for point in points {
            let x = point.x;
            let y = -point.y;  // JavaScript uses -point.y for bounding box calculation
            left = left.min(x);
            top = top.min(y);
            right = right.max(x);
            bottom = bottom.max(y);
        }
        
        // Calculate center of bounding box
        let center_x = (right - left) / 2.0 + left;   // (a.right - a.left) / 2 + a.left
        let center_y = (bottom - top) / 2.0 + top;    // (a.bottom - a.top) / 2 + a.top
        
        // Rotate each point around the center
        for point in points {
            let x = point.x;
            let y = -point.y;  // JavaScript uses -point.y
            
            let rotated = Self::rotate_point_around_center(angle, center_x, center_y, x, y);
            
            rotated_points.push(DrawPoint::new(
                rotated.0,      // rotated x 
                -rotated.1,     // -rotated y (flip back)
                point.color,
                point.pen_state
            ));
        }
        
        rotated_points
    }
    
    /// Rust implementation of JavaScript rotatePointAroundCenter
    fn rotate_point_around_center(angle: f64, center_x: f64, center_y: f64, point_x: f64, point_y: f64) -> (f64, f64) {
        // JavaScript: function rotatePointAroundCenter(e, t, r, n, h)
        // var a = n - t, i = h - r
        let a = point_x - center_x;
        let i = point_y - center_y;
        
        // c = t + (a * Math.cos(e) - i * Math.sin(e))
        // o = r + (a * Math.sin(e) + i * Math.cos(e))
        let c = center_x + (a * angle.cos() - i * angle.sin());
        let o = center_y + (a * angle.sin() + i * angle.cos());
        
        (c, o)
    }

    /// Parse device response data directly into DMX laser state
    /// Direct hex-to-DMX mapping following standard DMX-512 protocol
    /// Based on the device's actual command structure analysis
    pub fn parse_device_response_to_dmx_state(data: &str) -> Option<DmxLaserState> {
        let mut dmx_state = DmxLaserState::default();
        
        // Parse Settings Command (00010203...04050607)
        if let Some(settings_cmd) = Self::get_cmd_value(SETTINGS_CMD_HEADER, SETTINGS_CMD_FOOTER, data) {
            Self::parse_settings_to_dmx(&settings_cmd, &mut dmx_state);
        }
        
        // Parse Main Command (C0C1C2C3...C4C5C6C7) 
        if let Some(main_cmd) = Self::get_cmd_value(MAIN_CMD_HEADER, MAIN_CMD_FOOTER, data) {
            Self::parse_main_to_dmx(&main_cmd, &mut dmx_state);
        }
        
        // Parse Draw Command (F0F1F2F3...F4F5F6F7)
        if let Some(draw_cmd) = Self::get_cmd_value(DRAW_CMD_HEADER, DRAW_CMD_FOOTER, data) {
            Self::parse_draw_to_dmx(&draw_cmd, &mut dmx_state);
        }
        
        Some(dmx_state)
    }

    /// Parse Settings Command directly to DMX channels 1, 2, 10
    /// Settings structure: 00010203 + [channel][field][range][xy][R][G][B][light][cfg] + 04050607
    fn parse_settings_to_dmx(settings_cmd: &str, dmx_state: &mut DmxLaserState) {
        if settings_cmd.len() < 18 { return; } // Need at least 9 bytes of data
        
        // Byte 4: Display Range (32 = 50) -> CH1 Master Dimmer
        let display_range = Self::extract_hex_value(3, 1, settings_cmd) as u8;
        dmx_state.master_dimmer = if display_range > 9 { ON } else { OFF };
        
        // Byte 5: XY Config -> CH10 Rotation Control  
        let xy_config = Self::extract_hex_value(4, 1, settings_cmd) as u8;
        dmx_state.rotation = xy_config.saturating_mul(2); // Scale 0-7 to 0-14 range
        
        // Bytes 6-8: RGB Values -> CH2 Color Control
        let r = Self::extract_hex_value(5, 1, settings_cmd) as u8;
        let g = Self::extract_hex_value(6, 1, settings_cmd) as u8; 
        let b = Self::extract_hex_value(7, 1, settings_cmd) as u8;
        dmx_state.color_control = Self::rgb_to_dmx_color_direct(r, g, b);
        
        // Byte 9: Light Mode (03 = RGB) -> affects CH1 behavior
        let light_mode = Self::extract_hex_value(8, 1, settings_cmd) as u8;
        if light_mode > 1 && dmx_state.master_dimmer == 0 {
            dmx_state.master_dimmer = 255; // Force on if light mode is active
        }
    }

    /// Parse Main Command directly to DMX channels 3-8
    /// Main structure: C0C1C2C3 + [mode][00][color][size1][size2][speed][...] + C4C5C6C7
    fn parse_main_to_dmx(main_cmd: &str, dmx_state: &mut DmxLaserState) {
        if main_cmd.len() < 12 { return; } // Need at least 6 bytes of data
        
        // Byte 1: Mode (00-06) -> CH4 Pattern Group + CH6 Dynamic Effects
        let mode = Self::extract_hex_value(0, 1, main_cmd) as u8;
        Self::map_mode_to_dmx_channels(mode, dmx_state);
        
        // Byte 3: Color (05) -> affects CH2 Color Control
        let color = Self::extract_hex_value(2, 1, main_cmd) as u8;
        if dmx_state.color_control == 0 { // Only override if not set by RGB
            dmx_state.color_control = Self::map_color_index_to_dmx(color);
        }
        
        // Bytes 4-5: TX Size (80/80) -> CH8 Pattern Size
        let size1 = Self::extract_hex_value(3, 1, main_cmd) as u8;
        let size2 = Self::extract_hex_value(4, 1, main_cmd) as u8;
        dmx_state.pattern_size = ((size1 as u16 + size2 as u16) / 2) as u8; // Average both size values
        
        // Byte 6: Run Speed (80) -> CH7 Effect Speed + CH3 Color Speed
        let run_speed = Self::extract_hex_value(5, 1, main_cmd) as u8;
        dmx_state.effect_speed = run_speed;
        
        // Parse additional bytes for audio mode -> affects CH6
        if main_cmd.len() > 14 {
            let audio_mode = Self::extract_hex_value(7, 1, main_cmd) as u8;
            if audio_mode > 0 {
                Self::apply_audio_mode_to_dmx(mode, dmx_state);
            }
        }
        
        // Direction affects CH3 Color Speed
        if main_cmd.len() > 16 {
            let direction = Self::extract_hex_value(8, 1, main_cmd) as u8;
            dmx_state.color_speed = if direction > 0 { run_speed } else { 0 };
        }
    }

    /// Parse Draw Command directly to DMX channels 9, 11-16
    /// Draw structure: F0F1F2F3 + [config_15_bytes] + [point_count] + [point_data...] + F4F5F6F7
    fn parse_draw_to_dmx(draw_cmd: &str, dmx_state: &mut DmxLaserState) {
        if draw_cmd.len() < 32 { return; } // Need config section + point count
        
        // Extract 15 configuration bytes directly to DMX channels 9-16
        // These map to advanced DMX control channels
        
        // Config[0] -> CH9 Size Control
        dmx_state.size_control = Self::extract_hex_value(0, 1, draw_cmd) as u8;
        
        // Config[1] -> CH10 Rotation (additional control, combined with settings)
        let rotation_config = Self::extract_hex_value(1, 1, draw_cmd) as u8;
        dmx_state.rotation = dmx_state.rotation.saturating_add(rotation_config);
        
        // Config[2] -> CH11 Vertical Flip
        dmx_state.vertical_flip = Self::extract_hex_value(2, 1, draw_cmd) as u8;
        
        // Config[3] -> CH12 Horizontal Flip  
        dmx_state.horizontal_flip = Self::extract_hex_value(3, 1, draw_cmd) as u8;
        
        // Config[4] -> CH13 Horizontal Position
        dmx_state.horizontal_pos = Self::extract_hex_value(4, 1, draw_cmd) as u8;
        
        // Config[5] -> CH14 Vertical Position
        dmx_state.vertical_pos = Self::extract_hex_value(5, 1, draw_cmd) as u8;
        
        // Config[6] -> CH15 Wave Effect
        dmx_state.wave_effect = Self::extract_hex_value(6, 1, draw_cmd) as u8;
        
        // Config[7] -> CH16 Manual Drawing
        dmx_state.manual_drawing = Self::extract_hex_value(7, 1, draw_cmd) as u8;
        
        // If manual drawing is 0 but we have points, enable basic drawing mode
        if dmx_state.manual_drawing == 0 {
            let point_count = Self::extract_hex_value(15, 1, draw_cmd) as u8;
            if point_count > 0 {
                dmx_state.manual_drawing = 32; // Manual gradual drawing mode 1
            }
        }
        
        // Parse first point for position fine-tuning (if available)
        if draw_cmd.len() > 40 { // Has point data
            let point_x = Self::extract_hex_value(16, 2, draw_cmd) as u16;
            let point_y = Self::extract_hex_value(18, 2, draw_cmd) as u16;
            
            // Map coordinates to DMX position range (0-127)
            if dmx_state.horizontal_pos == 0 {
                dmx_state.horizontal_pos = ((point_x * 127) / 65535) as u8;
            }
            if dmx_state.vertical_pos == 0 {
                dmx_state.vertical_pos = ((point_y * 127) / 65535) as u8;
            }
        }
    }

    /// Map device mode directly to DMX Pattern Group (CH4) and Dynamic Effects (CH6)
    fn map_mode_to_dmx_channels(mode: u8, dmx_state: &mut DmxLaserState) {
        match mode {
            0 => { // DMX Mode
                dmx_state.pattern_group = 0;
                dmx_state.dynamic_effects = 0;
            },
            1 => { // Random Play
                dmx_state.pattern_group = 0;
                dmx_state.dynamic_effects = 251; // All effects random play
            },
            2 => { // Line Geometry
                dmx_state.pattern_group = 12; // Static graphics group 1
                dmx_state.dynamic_effects = 0;
            },
            3 => { // Animation
                dmx_state.pattern_group = 137; // Animation group 1
                dmx_state.dynamic_effects = 0;
            },
            5 => { // Christmas
                dmx_state.pattern_group = 112; // Christmas patterns
                dmx_state.dynamic_effects = 0;
            },
            6 => { // Outdoor
                dmx_state.pattern_group = 25; // Static graphics group 2
                dmx_state.dynamic_effects = 0;
            },
            9 => { // Draw Mode
                dmx_state.pattern_group = 12; // Static for drawing
                dmx_state.dynamic_effects = 0;
                dmx_state.manual_drawing = 32; // Enable manual drawing
            },
            _ => { // Default
                dmx_state.pattern_group = 12;
                dmx_state.dynamic_effects = 0;
            }
        }
    }

    /// Apply audio mode effects to Dynamic Effects channel (CH6)
    fn apply_audio_mode_to_dmx(base_mode: u8, dmx_state: &mut DmxLaserState) {
        dmx_state.dynamic_effects = match base_mode {
            2 => 211, // Line effect random play
            3 => 221, // Animation effect random play
            5 => 231, // Christmas effect random play
            6 => 241, // Outdoor effect random play
            _ => 251, // All effects random play
        };
    }

    /// Map color index directly to DMX Color Control values (CH2)
    fn map_color_index_to_dmx(color: u8) -> u8 {
        match color {
            0 => 0,  // White (0-9)
            1 => 15, // Red (10-19)  
            2 => 65, // Green (60-69)
            3 => 25, // Blue (20-29)
            4 => 55, // Yellow (50-59)
            5 => 45, // Cyan (40-49)
            6 => 35, // Pink (30-39)
            7 => 0,  // White
            _ => 254, // Color gradient for unknown colors
        }
    }

    /// Direct RGB to DMX color mapping (more precise than the previous method)
    fn rgb_to_dmx_color_direct(r: u8, g: u8, b: u8) -> u8 {
        // Direct mapping to DMX CH2 Color Control ranges
        // Based on actual DMX specification: 0-69 fixed colors, 70+ dynamic
        
        // Check for exact primary/secondary colors first
        if r > 200 && g > 200 && b > 200 { return 0; }  // White (0-9)
        if r > 200 && g < 100 && b < 100 { return 15; } // Red (10-19)
        if r < 100 && g < 100 && b > 200 { return 25; } // Blue (20-29)
        if r > 200 && g < 100 && b > 200 { return 35; } // Pink (30-39)
        if r < 100 && g > 200 && b > 200 { return 45; } // Cyan (40-49)
        if r > 200 && g > 200 && b < 100 { return 55; } // Yellow (50-59)
        if r < 100 && g > 200 && b < 100 { return 65; } // Green (60-69)
        
        // For mixed colors, use dynamic color ranges
        let color_intensity = ((r as u16 + g as u16 + b as u16) / 3) as u8;
        
        // Map to appropriate dynamic color range based on complexity
        if color_intensity < 64 {
            90 // Colorful rainbow (90-92)
        } else if color_intensity < 128 {
            150 // 8-segment color (150-182)
        } else if color_intensity < 192 {
            200 // 16-segment color (183-218)
        } else {
            254 // Color gradient (254-255)
        }
    }


}


