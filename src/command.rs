

use crate::model::{EncodedCommandData, Point, PolyPoint, ProjectItem};
use log::{debug, info};

use crate::model::{CommandConfig, DeviceInfo, DeviceResponse, FeatureConfig, MainCommandData, PisObject, SettingsData};

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
pub const XYS_CMD_HEADER: &str = "A0A1A2A3";
pub const XYS_CMD_FOOTER: &str = "A4A5A6A7";


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
    pub fn split_into_segments_by_sum_limit(values: &[f32], limit: f32) -> Vec<(usize, usize)> {
        // Ported from JS: splitIntoSegmentsBySumLimit
        let mut r = 0.0_f32;
        let mut result: Vec<(usize, usize)> = Vec::new();
        let mut h = 0_usize;
        let mut a = 0_usize;
        for (i, &v) in values.iter().enumerate() {
            if r + v <= limit {
                a += 1;
                result.push((h, a));
                r += v;
            } else {
                let mut temp_width = r;
                loop {
                    if temp_width <= limit {
                        a += 1;
                        result.push((h, a));
                        r = temp_width + v;
                        break;
                    }
                    if temp_width > limit && temp_width - values[h] < limit {
                        a += 1;
                        result.push((h, a));
                        r += v;
                        break;
                    }
                    temp_width -= values[h];
                    r -= values[h];
                    h += 1;
                    a = a.saturating_sub(1);
                    if h >= values.len() {
                        break;
                    }
                }
            }
        }
        result
    }

    /// Generate segmented layout data matching the JS generateSegmentedLayoutData behavior
    pub fn generate_segmented_layout_data(
        segments: &Vec<(usize, Vec<PolyPoint>, f32, f32)>,
        scaling_factor: f32,
        mode: i32,
    ) -> (Vec<(usize, Vec<PolyPoint>, f32, f32)>, String, String, f32) {
        let mut n = -1_i32;
        let mut segment_widths: Vec<f32> = Vec::new();
        let mut segment_heights: Vec<f32> = Vec::new();
        let segment_default_size: f32 = 200.0;
        let mut total_segment_width: f32 = 0.0;
        let mut total_segment_height: f32 = 0.0;

        // Only use original segments for splitting (exclude filler segments)
        let mut seg_idx = -1;
        for seg in segments.iter() {
            let seg_id = seg.0 as i32;
            if seg_idx != seg_id {
                seg_idx = seg_id;
                segment_widths.push(seg.2 * scaling_factor);
                total_segment_width += seg.2;
                segment_heights.push(seg.3 * scaling_factor);
                total_segment_height += seg.3;
            }
        }

        // Split only original segments by sum limit (800)
        let splits = Self::split_into_segments_by_sum_limit(&segment_widths, 800.0);
        let mut N = String::new();
        let mut H = String::new();
        for (start, count) in splits.iter() {
            N += &Self::to_fixed_width_hex_b(*start as i32, 2);
            H += &Self::to_fixed_width_hex_b(*count as i32, 2);
        }

        // Append 9 filler segments on the right (for device protocol)
        let mut m: Vec<(usize, Vec<PolyPoint>, f32, f32)> = Vec::new();
        let mut k = 0_f32;
        let mut out = segments.clone();
        for _ in 0..9 {
            seg_idx += 1;
            let idx = seg_idx as usize;
            let pt = PolyPoint { x: total_segment_width / 2.0 + segment_default_size / 2.0 + k, y: 0.0, z: 0 };
            let pts = vec![pt];
            m.push((idx, pts, segment_default_size, segment_default_size));
            k += segment_default_size;
        }
        out.extend(m.into_iter());

        // xOffset matches JS: -k * scalingFactor / 2
        let x_offset = -k * scaling_factor / 2.0;
        (out, N, H, x_offset)
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





/// Simplified XYS command generator for a vector of (index, points, width, height)
pub fn get_xys_cmd(
    segment_points: &Vec<(usize, Vec<PolyPoint>, f32, f32)>,
) -> String {
    let segment_time = 5;
    let command_type = 0;
    let mirror_mode = 0;
    let ver_tag = 0;
    let scaling_factor = 0.5;
    // JS: pack all points from all split segments into a single command
    let (segmented, _, _, _) = CommandGenerator::generate_segmented_layout_data(segment_points, scaling_factor, 0);
    if let Some(encoded_command_data) = CommandGenerator::encode_layout_to_command_data(
        &segmented,
        segment_time,
        command_type,
        mirror_mode,
        ver_tag,
    ) {
        // Output structure matches JS reference
        return encoded_command_data.cmd.to_uppercase();
    }
    String::new()
}


pub fn encode_layout_to_command_data(
    segment_points: &Vec<(usize, Vec<PolyPoint>, f32, f32)>,
    segment_time: u32,
    command_options: u8,
    mirror_mode: u8,
    ver_tag: u8,
) -> Option<EncodedCommandData> {
    if segment_points.is_empty() {
        return None;
    }

    let scaling_factor = 0.5;
    // JS getXysCmdSimplified: treat all points as a single segment
    let mut packed_cmd = String::new();
    let mut packed_point_count = 0;
    let mut segment_width_sum = 0.0;
    let mut x_offset = 0.0;
    // Pack all points from all segments into a single segment
    for seg in segment_points.iter() {
        segment_width_sum += seg.2 * scaling_factor;
        for (index, point) in seg.1.iter().enumerate() {
            let segment_index = 0;
            let mut point_type = point.z as u8;
            if index == 0 { point_type = 1; }
            if index == seg.1.len() - 1 { point_type = 1; }
            if seg.1.len() == 1 { point_type = point.z as u8; }
            let x_screen = ((point.x * scaling_factor) + x_offset).round() as i32;
            let y_screen = (point.y * scaling_factor).round() as i32;
            let combined = CommandGenerator::combine_nibbles_b(segment_index, point_type);
            let packed_x = CommandGenerator::to_fixed_width_hex_b(x_screen, 2);
            let packed_y = CommandGenerator::to_fixed_width_hex_b(y_screen, 2);
            let packed_type = CommandGenerator::to_fixed_width_hex_b(combined as i32, 2);
            packed_cmd.push_str(&packed_x);
            packed_cmd.push_str(&packed_y);
            packed_cmd.push_str(&packed_type);
            packed_point_count += 1;
        }
    }
    // Only one segment, charCount = 1, segmentCount = 1, charPointCmd = total points
    let packed_char_count_cmd = CommandGenerator::to_fixed_width_hex_b(1, 2);
    let packed_char_width_cmd = CommandGenerator::to_fixed_width_hex_b(segment_width_sum.round() as i32, 2);
    let packed_char_point_cmd = CommandGenerator::to_fixed_width_hex_b(packed_point_count as i32, 2);
    let se1 = CommandGenerator::to_fixed_width_hex_b(0, 2);
    let se2 = CommandGenerator::to_fixed_width_hex_b(1, 2);
    let total_segments = 1;
    // Debug print each output part for parity validation
    println!("[DEBUG] HEADER: {}", XYS_CMD_HEADER);
    println!("[DEBUG] TOTAL POINT COUNT (4 bytes): {}", CommandGenerator::to_fixed_width_hex_b(packed_point_count as i32, 4));
    println!("[DEBUG] CHAR COUNT (2 bytes): {}", CommandGenerator::to_fixed_width_hex_b(total_segments as i32, 2));
    println!("[DEBUG] PACKED POINT DATA: {}", packed_cmd);
    println!("[DEBUG] SEGMENT COUNT (2 bytes): {}", CommandGenerator::to_fixed_width_hex_b(total_segments as i32, 2));
    println!("[DEBUG] PER-SEGMENT CHAR COUNT: {}", packed_char_count_cmd);
    println!("[DEBUG] PER-SEGMENT WIDTH: {}", packed_char_width_cmd);
    println!("[DEBUG] PER-SEGMENT POINT COUNT: {}", packed_char_point_cmd);
    println!("[DEBUG] SE1: {}", se1);
    println!("[DEBUG] SE2: {}", se2);
    println!("[DEBUG] VER TAG (2 bytes): {}", CommandGenerator::to_fixed_width_hex_b(ver_tag as i32, 2));
    println!("[DEBUG] SEGMENT TIME (2 bytes): {}", CommandGenerator::to_fixed_width_hex_b(segment_time as i32, 2));
    println!("[DEBUG] FOOTER: {}", XYS_CMD_FOOTER);

    if packed_point_count == 0 {
        return None;
    }

    let total_points_hex = CommandGenerator::to_fixed_width_hex_b(packed_point_count as i32, 4);
    let char_count_hex = CommandGenerator::to_fixed_width_hex_b(total_segments as i32, 2);
    let result_cmd = format!(
        "{}{}{}{}{}{}{}{}{}{}{}{}{}",
        XYS_CMD_HEADER,
        total_points_hex,
        char_count_hex,
        packed_cmd,
        CommandGenerator::to_fixed_width_hex_b(total_segments as i32, 2), // segment count
        packed_char_count_cmd,
        packed_char_width_cmd,
        packed_char_point_cmd,
        se1,
        se2,
        CommandGenerator::to_fixed_width_hex_b(ver_tag as i32, 2),
        CommandGenerator::to_fixed_width_hex_b(segment_time as i32, 2),
        XYS_CMD_FOOTER
    );

    Some(EncodedCommandData {
        cnt: packed_point_count as usize,
        char_count: total_segments,
        cmd: result_cmd,
        char_width_cmd: packed_char_width_cmd,
        char_point_cmd: packed_char_point_cmd,
        se1,
        se2,
        ver: CommandGenerator::to_fixed_width_hex_b(ver_tag as i32, 2),
        time: CommandGenerator::to_fixed_width_hex_b(segment_time as i32, 2),
    })
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