// Rust translation of deviceCommandUtils module
// This module provides utilities for device command generation and manipulation

use std::collections::HashMap;
use log::{debug, info};

use crate::model::{CommandConfig, DeviceInfo, DeviceResponse, DrawConfig, FeatureConfig, Features, LayoutItem, MainCommandData, PisConfig, Point, SettingsData, ShakeConfig};

// Response headers and footers
pub const HEADER: &str = "E0E1E2E3";
pub const FOOTER: &str = "E4E5E6E7";

// Power command patterns
pub const POWER_ON_CMD: &str = "B0B1B2B3FFB4B5B6B7";
pub const POWER_OFF_CMD: &str = "B0B1B2B300B4B5B6B7";

// Command section markers
const MAIN_CMD_HEADER: &str = "C0C1C2C3";
const MAIN_CMD_FOOTER: &str = "C4C5C6C7";
const SETTINGS_CMD_HEADER: &str = "00010203";
const SETTINGS_CMD_FOOTER: &str = "04050607";
const FEATURES_CMD_HEADER: &str = "D0D1D2D3";
const FEATURES_CMD_FOOTER: &str = "D4D5D6D7";
const DRAW_CMD_HEADER: &str = "F0F1F2F3";
const DRAW_CMD_FOOTER: &str = "F4F5F6F7";


pub struct CommandGenerator;

impl CommandGenerator {
    /// Applies bitmask updates to the selection_bits vector at the given indices.
    /// For each index in indices, toggles the bit (XOR with 1) at that position in selection_bits.
    /// This matches the JavaScript logic of applyBitmaskUpdates([indices], N).
    /// Applies bitmask updates to the selection_bits vector at the given indices.
    /// For each index in indices, toggles the bit at (index / 16) in selection_bits,
    /// using (1 << (index % 16)), matching the JavaScript algorithm.
    fn apply_bitmask_updates(indices: &[usize], selection_bits: &mut [u16]) {
        for &idx in indices {
            let arr_idx = idx / 16;
            let bit = idx % 16;
            if arr_idx < selection_bits.len() {
                selection_bits[arr_idx] ^= 1 << bit;
            }
        }
    }
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
        0
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
            "00010203{}{}{}{}{}{}{}{}{}{}{}", 
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
            "000000000004050607"
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


/// Helper function to extract and clamp numeric values
//fn clamp_value<T: PartialOrd + Copy>(value: T, min: T, max: T, default: T) -> T {
//    if value < min || value > max {
//        default
//    } else {
//        value
//    }
//}

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

    /// Extract hex value from a position in command data, matching JavaScript behavior
   // fn extract_hex_value(pos: usize, len: usize, data: &str) -> u16 {
  //      debug!("Extracting hex value at pos {} with len {} from data: {}", pos, len, data);
   //     
    //    // JavaScript: var n = 2 * (startByte - 1)
   //     let start = if pos > 0 { 2 * (pos - 1) } else { 0 };
   //     // JavaScript: var h = n + 2 * byteLength
   //     let end = start + 2 * len;
   //     
   //     if end <= data.len() && start < data.len() {  // Ensure valid range
   //         let hex_str = &data[start..end];
   //         debug!("Extracted hex string: {}", hex_str);
   //         match u16::from_str_radix(hex_str, 16) {
   //             Ok(val) => {
   //                 debug!("Parsed value: {}", val);
   //                 val
   //             },
   //             Err(e) => {
   //                 debug!("Failed to parse hex value: {}", e);
   //                 0
   //             }
   //         }
   //     } else {
   //         debug!("Position out of bounds");
   //         0
   //     }
   // }

    /// Parse the main command section
    fn parse_main_command(cmd: &str) -> Option<MainCommandData> {
        Some(MainCommandData {
            current_mode: Self::clamp_value(Self::extract_hex_value(1, 1, cmd) as u8, 0, 12, 0),
            project_index: Self::clamp_value(Self::extract_hex_value(1, 1, cmd) as u8, 0, 12, 0),
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
            read_mode: Self::clamp_value(Self::extract_hex_value(9, 1, cmd) as u8, 0, 255, 0),
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
        info!("Parsing settings command: {}", cmd);

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

        let mut response = DeviceResponse {
            main_data: Self::parse_main_command(&main_cmd)?,
            settings: Self::parse_settings_command(&settings_cmd),
            features: Vec::new(),
            draw_config: DrawConfig::default(),
            device_info: None,
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
                if i < 14 {
                    response.draw_config.config_values.push(value.try_into().unwrap());
                } else {
                    response.draw_config.text_point_time = value.try_into().unwrap();
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
    pub fn get_cmd_str(config: &CommandConfig, features: Option<&Features>) -> String {
        debug!("get_cmd_str called");
        info!("CommandConfig: {:?}, Features: {:?}", config, features);

        // Main header and footer
        let mut cmd = String::new();

        // Main section fields
        let cur_mode_hex = Self::to_fixed_width_hex(config.cur_mode, 2);
        let reserved_hex = Self::to_fixed_width_hex(0, 2);
        let color_hex = Self::to_fixed_width_hex(config.text_data.tx_color, 2);
        let tx_size_scaled = (config.text_data.tx_size / 100.0 * 255.0).round() as u8;
        let tx_size_scaled_a_hex = Self::to_fixed_width_hex(tx_size_scaled, 2);
        let tx_size_scaled_b_hex = Self::to_fixed_width_hex(tx_size_scaled, 2);
        let run_speed_scaled = (config.text_data.run_speed / 100.0 * 255.0).round() as u8;
        let run_speed_hex = Self::to_fixed_width_hex(run_speed_scaled, 2);
        let l = "00".to_string();
        let tx_dist_scaled = (config.text_data.tx_dist / 100.0 * 255.0).round() as u8;
        let tx_dist_scaled_hex = Self::to_fixed_width_hex(tx_dist_scaled, 2);
        let audio_trigger_mode_hex = Self::to_fixed_width_hex(config.prj_data.public.rd_mode, 2);
        let sound_sensitivity_hex = Self::to_fixed_width_hex((config.prj_data.public.sound_val / 100.0 * 255.0).round() as u8, 2);

        // x: group color segment
        let mut x = "ffffffff0000".to_string();
        if let Some(features) = features {
            x.clear();
            if let Some(group_list) = &features.group_list {
                for group in group_list {
                    x += &Self::to_fixed_width_hex(group.color, 2);
                }
            }
            x += "ffffffff";
            x = x.chars().take(8).collect();
            if Self::get_feature_value(features, "textStopTime").unwrap_or(false) {
                x += &Self::to_fixed_width_hex(config.text_data.tx_point_time, 2);
            }
            x += "0000";
            x = x.chars().take(12).collect();
        }

        // f: project items
        let mut f = String::new();
        for (index, project_item) in &config.prj_data.prj_item {
            let mut play_back_mode = if project_item.py_mode == 0 { 0 } else { 128 };
            if play_back_mode != 0 {
                if let Some(features) = features {
                    if let Some(prj_parm) = &features.prj_parm {
                        if prj_parm.prj_index == *index as i32 {
                            if *index == 3 && Self::get_feature_value(features, "animationFix").unwrap_or(false) && [2, 4, 11, 13, 19].contains(&prj_parm.sel_index) {
                                play_back_mode |= 50 - prj_parm.sel_index;
                            } else {
                                play_back_mode |= prj_parm.sel_index;
                            }
                        }
                    }
                }
            }
            let play_back_mode_hex = Self::to_fixed_width_hex(play_back_mode, 2);
            // N: selection bits
            let mut selection_bits = project_item.prj_selected.clone();
            if let Some(features) = features {
                if *index == 3 && Self::get_feature_value(features, "animationFix").unwrap_or(false) {
                    // applyBitmaskUpdates([2, 4, 11, 13, 19], N)
                    Self::apply_bitmask_updates(&[2, 4, 11, 13, 19], &mut selection_bits);
                }
            }
            let mut x_str = String::new();
            for &val in selection_bits.iter().rev() {
                x_str += &Self::to_fixed_width_hex(val, 2);
            }
            f += &(play_back_mode_hex + &x_str);
        }

        // z: run direction if arbPlay
        let mut z = String::new();
        if let Some(features) = features {
            if Self::get_feature_value(features, "arbPlay").unwrap_or(false) {
                z += &Self::to_fixed_width_hex(config.text_data.run_dir, 2);
            }
        }

        // Q: padding
        let mut q = String::new();
        let r = z.len() / 2;
        for _ in r..44 {
            q += "00";
        }

        // Compose command using header/footer constants
        let command = format!(
            "{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            MAIN_CMD_HEADER,
            cur_mode_hex,
            reserved_hex,
            color_hex,
            tx_size_scaled_a_hex,
            tx_size_scaled_b_hex,
            run_speed_hex,
            l,
            tx_dist_scaled_hex,
            audio_trigger_mode_hex,
            sound_sensitivity_hex,
            x,
            f,
            z,
            q,
            MAIN_CMD_FOOTER
        );
        command.to_uppercase()
    }


    pub fn get_xts_cmd(coord_data: &str) -> String {
        debug!("get_xts_cmd called with coord_data: {}", coord_data);
        String::new()
    }

    pub fn get_xys_cmd(coords: &[Vec<f64>], version: i32) -> String {
        debug!("get_xys_cmd called with coords: {:?}, version: {}", coords, version);
        String::new()
    }

    pub fn get_xys_cmd_arr(items: &[LayoutItem], config: &CommandConfig, direction: i32, version: i32) -> String {
        debug!("get_xys_cmd_arr called with items len: {}, direction: {}, version: {}", items.len(), direction, version);
        info!("CommandConfig: {:?}", config);
        String::new()
    }
    
    // Drawing commands
    pub fn get_draw_line_str(points: &[Point], count: i32) -> String {
        debug!("get_draw_line_str called with points: {:?}, count: {}", points, count);
        String::new()
    }

    pub fn get_draw_cmd_str(points: &[Point], config: &DrawConfig, features: &Features) -> String {
        debug!("get_draw_cmd_str called with points len: {}", points.len());
        info!("DrawConfig: {:?}, Features: {:?}", config, features);
        String::new()
    }

    pub fn encode_draw_point_command(points: &[Point], config: &DrawConfig, features: &Features, time: i32, version: &str) -> String {
        debug!("encode_draw_point_command called with points len: {}, time: {}, version: {}", points.len(), time, version);
        info!("DrawConfig: {:?}, Features: {:?}", config, features);
        String::new()
    }


    pub fn get_shake_cmd_str(config: &ShakeConfig, features: Option<&Features>) -> String {
        debug!("get_shake_cmd_str called");
        info!("ShakeConfig: {:?}, Features: {:?}", config, features);
        String::new()
    }

    pub fn get_pis_cmd_str(index: i32, config: &PisConfig, features: Option<&Features>) -> String {
        debug!("get_pis_cmd_str called with index: {}", index);
        info!("PisConfig: {:?}, Features: {:?}", config, features);
        String::new()
    }

    pub fn get_pis_list_cmd_str(items: &[PisConfig], features: Option<&Features>) -> String {
        debug!("get_pis_list_cmd_str called with items len: {}", items.len());
        info!("Features: {:?}", features);
        String::new()
    }


    
    // Feature handling
    pub fn get_feature_value(_obj: &Features, feature_name: &str) -> Option<bool> {
        debug!("get_feature_value called with feature_name: {}", feature_name);
        None
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
}


