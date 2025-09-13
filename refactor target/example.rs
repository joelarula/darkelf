use std::error::Error;

use std::future::Future;
use std::pin::Pin;


pub const LASER_DEVICE_PREFIX: &str = "TD5322A";

pub const GENERIC_ACCESS_SERVICE_UUID: &str = "00001800-0000-1000-8000-00805F9B34FB";

pub const DEVICE_INFORMATION_SERVICE_UUID: &str = "0000180A-0000-1000-8000-00805F9B34FB";

pub const LASER_SERVICE_UUID: [&str; 2] = [
    "0000FF00-0000-1000-8000-00805F9B34FB",
    "0000FFE0-0000-1000-8000-00805F9B34FB1"
];

// UUIDs from JavaScript example
pub const WRITE_UUIDS: [&str; 2] = [
    "0000FFE2-0000-1000-8000-00805F9B34FB",
    "0000FF02-0000-1000-8000-00805F9B34FB"
];
pub const NOTIFY_UUIDS: [&str; 2] = [
    "0000FFE1-0000-1000-8000-00805F9B34FB",
    "0000FF01-0000-1000-8000-00805F9B34FB"
];


pub trait BlueController: Send + Sync {
    
    fn connect<'a>(&'a mut self) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn Error>>> + Send + 'a>>;
        
    fn send<'a>(&'a mut self, bytes: &'a [u8]) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send + 'a>>;
    
    fn get_content(&self) -> String;
    
    fn is_connected(&self) -> bool;
}


use std::sync::{Arc, Mutex};
use std::time::{Instant, Duration};
use log::{debug, error};
use hex::decode;
use btleplug::api::Characteristic; // Still needed for type references
use tokio;
use tokio::time::sleep; // For async handling
use crate::blue::BlueController;
use std::error::Error;
use std::collections::VecDeque; // For buffer
use regex::Regex;
use std::fmt::Write;

/// Represents a single laser point with coordinates and control data.
#[derive(Debug, Clone)]
pub struct LaserPoint {
    x: i16,          // X coordinate (e.g., -399 to 399, normalized to 16-bit)
    y: i16,          // Y coordinate (e.g., -399 to 399, normalized to 16-bit)
    z: u8,           // Control flag or z-depth (e.g., 0, 1, 2, 3)
    color: u8,       // Color or intensity (derived from control byte)
}

impl LaserPoint {
    pub fn new(x: i16, y: i16, z: u8, color: u8) -> Self {
        LaserPoint {
            x,
            y,
            z,
            color, // Allow explicit color setting, overriding z & 0x0F if needed
        }
    }
}


/// Represents a laser command with point and segment data.
#[derive(Debug)]
pub struct LaserCommand {
    point_count: u16,         // Total points (cnt)
    char_count: u16,          // Character segments (charCount)
    command_data: String,     // Main hex command string (cmd)
    char_width_cmd: String,   // Width command (charWidthCmd)
    char_point_cmd: String,   // Point command (charPointCmd)
    segment1: String,         // First segment command (se1)
    segment2: String,         // Second segment command (se2)
    version: String,          // Version (ver)
    time: String,             // Time value (time)
    points: Vec<LaserPoint>,  // Original points
    points_right: Option<Vec<LaserPoint>>, // For direction 255
    points_up: Option<Vec<LaserPoint>>,    // For direction 127
    points_down: Option<Vec<LaserPoint>>,  // For direction 128
}

impl LaserCommand {
    pub fn new() -> Self {
        LaserCommand {
            point_count: 0,
            char_count: 0,
            command_data: String::new(),
            char_width_cmd: String::new(),
            char_point_cmd: String::new(),
            segment1: String::new(),
            segment2: String::new(),
            version: "00".to_string(),
            time: "00".to_string(),
            points: Vec::new(),
            points_right: None,
            points_up: None,
            points_down: None,
        }

        
    }

    pub fn to_hex_string(&self) -> String {
        format!(
            "A0A1A2A3{}{}{}{}{}{}{}A4A5A6A7",
            format!("{:04x}", self.point_count),
            format!("{:04x}", self.char_count),
            self.command_data,
            format!("{:04x}", self.char_count), // Placeholder for u
            self.char_width_cmd,
            self.char_point_cmd,
            self.segment1.clone() + &self.segment2 + &self.version + &self.time
        ).to_uppercase()
    }

}

impl LaserCommand {
    pub fn to_ble_command(&self) -> Vec<Vec<u8>> {
        let mut result = Vec::new();
        // Convert the hex string to bytes first
        let cmd_bytes = match hex::decode(&self.command_data) {
            Ok(bytes) => bytes,
            Err(_) => Vec::new(),
        };
        for chunk in cmd_bytes.chunks(16) { // 16 bytes + 4-byte header = 20 bytes
            let mut packet = vec![0xE0, 0xE1, 0xE2, 0xE3];
            packet.extend_from_slice(chunk);
            if packet.len() < 20 {
                packet.extend(vec![0x00; 20 - packet.len()]); // Pad to 20 bytes
            }
            result.push(packet);
        }
        result
    }
}
/// Configuration options for laser control.
#[derive(Debug, Clone)]
pub struct LaserOptions {
    text_decimal_time: bool,  // Use decimal time
    text_stop_time: bool,     // Stop time flag
    tx_color: u8,             // Text color
    tx_size: u8,              // Text size (scaled)
    run_speed: u8,            // Run speed (scaled)
    tx_dist: u8,              // Text distance (scaled)
    rd_mode: u8,              // Read mode
    sound_val: u8,            // Sound value (scaled)
}

impl LaserOptions {
    pub fn new() -> Self {
        LaserOptions {
            text_decimal_time: false,
            text_stop_time: false,
            tx_color: 0,
            tx_size: 0,
            run_speed: 0,
            tx_dist: 0,
            rd_mode: 0,
            sound_val: 0,
        }
    }
}

/// Project data structure.
#[derive(Debug)]
pub struct ProjectData {
    public: PublicSettings,
    prj_item: Vec<PrjItem>,
}

/// Public settings within project data.
#[derive(Debug)]
pub struct PublicSettings {
    rd_mode: u8,      // Read mode
    sound_val: u8,    // Sound value
}

/// Project item configuration.
#[derive(Debug, Clone)]
pub struct PrjItem {
    py_mode: u8,      // Play mode
    prj_selected: Vec<u8>, // Selected project indices
}

/// Text data for command generation.
#[derive(Debug)]
pub struct TextData {
    tx_color: u8,
    tx_size: u8,
    run_speed: u8,
    tx_dist: u8,
    tx_point_time: u8,
    run_dir: u8,
}

/// Features for command customization.
#[derive(Debug, Default)]
pub struct Features {
    group_list: Option<Vec<Group>>,
    text_stop_time: bool,
    animation_fix: bool,
    arb_play: bool,
    pics_play: bool, // Added to match JavaScript logic
    sel_index: Option<u8>,
}

/// Group configuration for features.
#[derive(Debug)]
pub struct Group {
    color: u8,
}

/// XY value for shake command.
#[derive(Debug)]
pub struct XYValue {
    value: u8,
}

/// XY configuration for shake command.
#[derive(Debug)]
pub struct XYConfig {
    auto: bool,
    auto_value: u8,
    phase: u8,
    xy: Vec<XYValue>,
}

/// Subset data for shake command.
#[derive(Debug)]
pub struct SubsetData {
    xy_cnf: XYConfig,
}

/// Shake data structure.
#[derive(Debug)]
pub struct ShakeData {
    subset_data: SubsetData,
}

/// Shake options structure.
#[derive(Debug)]
pub struct ShakeOptions {
    xy_cnf_save: Option<bool>,
}

impl ShakeOptions {
    pub fn new(xy_cnf_save: Option<bool>) -> Self {
        ShakeOptions { xy_cnf_save }
    }
}

/// Setting parameters for configuration commands.
#[derive(Debug)]
pub struct SettingParams {
    val_arr: [u8; 5], // Array of 5 values
    ch: u8,           // Channel
    xy: u8,           // X-Y setting
    light: u8,        // Light intensity
    cfg: u8,          // Configuration mode
    lang: u8,         // Language
}

impl SettingParams {
    pub fn new(val_arr: [u8; 5], ch: u8, xy: u8, light: u8, cfg: u8, lang: u8) -> Self {
        SettingParams {
            val_arr,
            ch,
            xy,
            light,
            cfg,
            lang,
        }
    }
}

/// Command generator for creating laser commands.
pub struct CommandGenerator {
    scale_factor: f32,
    cur_mode: u8,
    text_data: TextData,
    prj_data: ProjectData,
}

impl CommandGenerator {
    pub fn new() -> Self {
        CommandGenerator {
            scale_factor: 0.5,
            cur_mode: 0,
            text_data: TextData {
                tx_color: 0,
                tx_size: 0,
                run_speed: 0,
                tx_dist: 0,
                tx_point_time: 0,
                run_dir: 0,
            },
            prj_data: ProjectData {
                public: PublicSettings { rd_mode: 0, sound_val: 0 },
                prj_item: Vec::new(),
            },
        }
    }

    fn to_hex(&self, value: u16, length: usize) -> String {
        format!("{:0width$x}", value, width = length).to_uppercase()
    }

    fn combine_control(&self, frame: u8, flag: u8) -> u8 {
        (frame << 4) | (0x0F & flag)
    }

    pub fn generate_command_simple(
        &self,
        points: Vec<LaserPoint>,
        time: f32,
        options: LaserOptions,
        direction: u8,
        version: u8,
    ) -> Option<LaserCommand> {
        let mut cmd = LaserCommand::new();
        if points.is_empty() { return None; }

        let (new_points, se1, se2, offset) = self.generate_grid(points, direction);
        cmd.point_count = new_points.len() as u16;
        cmd.char_count = (new_points.len() / 2) as u16; // Example segmentation
        cmd.version = self.to_hex(version as u16, 2);
        cmd.time = if options.text_decimal_time {
            self.to_hex((time * 10.0) as u16, 2)
        } else {
            self.to_hex(time as u16, 2)
        };
        cmd.points = new_points.clone();

        for (i, point) in new_points.iter().enumerate() {
            let frame = (i / 8) as u8;
            let flag = if i == 0 || i == new_points.len() - 1 { 1 } else { 0 };
            let ctrl = self.combine_control(frame, flag);
            cmd.command_data += &self.to_hex((point.x as f32 * self.scale_factor).round() as u16 + offset as u16, 4);
            cmd.command_data += &self.to_hex((point.y as f32 * self.scale_factor).round() as u16, 4);
            cmd.command_data += &self.to_hex(ctrl as u16, 2);
        }

        cmd.char_width_cmd = se1.clone();
        cmd.char_point_cmd = se2.clone();
        cmd.segment1 = se1;
        cmd.segment2 = se2;
        Some(cmd)
    }

    pub fn get_xts_cmd(&self, input: &str) -> String {
        let mut point_count = 0;
        let mut cmd_data = String::new();

        for line in input.lines() {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                point_count += 1;
                let parts: Vec<&str> = trimmed.split(',').collect();
                if parts.len() == 4 {
                    let x = (parts[0].parse::<i16>().unwrap_or(0) + 400) as u16;
                    let y = (parts[1].parse::<i16>().unwrap_or(0) + 400) as u16;
                    let color = parts[2].parse::<u8>().unwrap_or(0);
                    let flag = parts[3].parse::<u8>().unwrap_or(0);
                    cmd_data.push_str(&self.to_hex(x, 4));
                    cmd_data.push_str(&self.to_hex(y, 4));
                    cmd_data.push_str(&self.to_hex(color as u16, 2));
                    cmd_data.push_str(&self.to_hex(flag as u16, 2));
                }
            }
        }

        if point_count == 0 {
            String::new()
        } else {
            format!(
                "55667788{}{}88776655",
                self.to_hex(point_count as u16, 2),
                cmd_data
            )
        }
    }

    pub fn get_xys_cmd(&self, points: Vec<LaserPoint>, version: u8) -> String {
        let mut point_count = 0;
        let mut char_count = 0;
        let mut cmd_data = String::new();
        let mut width_cmd = String::new();
        let mut point_cmd = String::new();

        let (new_points, se1, se2, x_offset) = self.generate_grid(points, 0);
        for (frame_idx, point_group) in new_points.chunks(8).enumerate() {
            char_count += 1;
            if !point_group.is_empty() {
                width_cmd.push_str(&self.to_hex(
                    (point_group[0].x as f32 * self.scale_factor).round() as u16,
                    2,
                ));
            }
            let mut frame_points = 0;
            for (idx, point) in point_group.iter().enumerate() {
                point_count += 1;
                frame_points += 1;
                let x = (point.x as f32 * self.scale_factor + x_offset as f32).round() as u16;
                let y = (point.y as f32 * self.scale_factor).round() as u16;
                let flag = if idx == 0 || idx == point_group.len() - 1 { 1 } else { point.z };
                let ctrl = self.combine_control(frame_idx as u8, flag);
                cmd_data.push_str(&self.to_hex(x, 4));
                cmd_data.push_str(&self.to_hex(y, 4));
                cmd_data.push_str(&self.to_hex(ctrl as u16, 2));
            }
            point_cmd.push_str(&self.to_hex(frame_points as u16, 2));
        }

        if point_count == 0 {
            String::new()
        } else {
            format!(
                "A0A1A2A3{}{}{}{}{}{}{}A4A5A6A7",
                self.to_hex(point_count as u16, 4),
                self.to_hex(char_count as u16, 4),
                cmd_data,
                width_cmd,
                point_cmd,
                se1,
                se2
            )
            .to_uppercase()
        }
    }

    pub fn get_xys_cmd_arr(
        &self,
        commands: Vec<LaserCommand>,
        options: LaserOptions,
        direction: u8,
    ) -> String {
        let mut all_commands = Vec::new();
        for cmd in commands {
            let mut points = cmd.points;
            match direction {
                255 => if let Some(right) = cmd.points_right { points = right; },
                127 => if let Some(up) = cmd.points_up { points = up; },
                128 => if let Some(down) = cmd.points_down { points = down; },
                _ => {},
            }
            let frame_points = vec![(0u8, points)];
            if let Some(new_cmd) = self.generate_command(frame_points, 0.0, options.clone(), direction, 0) {
                all_commands.push(new_cmd);
            }
        }

        if all_commands.is_empty() {
            String::new()
        } else {
            let mut total_points = 0;
            let mut total_chars = 0;
            let mut cmd_data = String::new();
            let mut width_cmd = String::new();
            let mut point_cmd = String::new();
            let mut seg1 = String::new();
            let mut seg2 = String::new();
            let mut ver = String::new();
            let mut time = String::new();

            for cmd in &all_commands {
                total_points += cmd.point_count;
                total_chars += cmd.char_count;
                cmd_data.push_str(&cmd.command_data);
                width_cmd.push_str(&cmd.char_width_cmd);
                point_cmd.push_str(&cmd.char_point_cmd);
                seg1.push_str(&cmd.segment1);
                seg2.push_str(&cmd.segment2);
                ver.push_str(&cmd.version);
                time.push_str(&cmd.time);
            }

            format!(
                "A0A1A2A3{}{}{}{}{}{}{}{}A4A5A6A7",
                self.to_hex(total_points, 4),
                self.to_hex(total_chars, 4),
                cmd_data,
                self.to_hex(all_commands.len() as u16, 2),
                self.to_hex(total_chars, 4), // Placeholder for g
                width_cmd,
                point_cmd,
                seg1
            )
            .to_uppercase()
        }
    }

    pub fn get_cmd_str(&self, features: Option<Features>) -> String {
        let mut cmd = String::new();

        let r = self.to_hex(self.cur_mode as u16, 2);
        let h = self.to_hex(0, 2);
        let a = self.to_hex(self.text_data.tx_color as u16, 2);
        let c = self.to_hex((self.text_data.tx_size as f32 / 100.0 * 255.0) as u16, 2);
        let o = self.to_hex((self.text_data.tx_size as f32 / 100.0 * 255.0) as u16, 2);
        let s = self.to_hex((self.text_data.run_speed as f32 / 100.0 * 255.0) as u16, 2);
        let l = "00".to_string();
        let p = self.to_hex((self.text_data.tx_dist as f32 / 100.0 * 255.0) as u16, 2);
        let d = self.to_hex(self.prj_data.public.rd_mode as u16, 2);
        let j = self.to_hex((self.prj_data.public.sound_val as f32 / 100.0 * 255.0) as u16, 2);

        let mut x = "FFFFFFFF0000".to_string();
        if let Some(ref f) = features {
            x = String::new();
            if let Some(groups) = &f.group_list {
                for group in groups {
                    x.push_str(&self.to_hex(group.color as u16, 2));
                }
            }
            x.push_str("FFFFFFFF");
            x.truncate(8);
            if f.text_stop_time {
                x.push_str(&self.to_hex(self.text_data.tx_point_time as u16, 2));
            }
            x.push_str("0000");
            x.truncate(12);
        }

        let mut f_str = String::new();
        for (idx, item) in self.prj_data.prj_item.iter().enumerate() {
            let mut p = if item.py_mode == 0 { 0 } else { 128 };
            if let Some(f) = features.as_ref() {
                if idx == 3 && f.animation_fix {
                    p |= 50 - f.sel_index.unwrap_or(0); // Placeholder
                } else if p != 0 {
                    p |= f.sel_index.unwrap_or(0);
                }
            }
            let u = self.to_hex(p as u16, 2);
            let mut x_item = String::new();
            let mut n = process_selected(&item.prj_selected);
            if idx == 3 && features.as_ref().map_or(false, |f| f.animation_fix) {
                n = adjust_selected(&[2, 4, 11, 13, 19], n);
            }
            for val in n.iter().rev() {
                x_item.push_str(&self.to_hex(*val as u16, 2));
            }
            f_str.push_str(&u);
            f_str.push_str(&x_item);
        }

        let mut z = String::new();
        if features.as_ref().map_or(false, |f| f.arb_play) {
            z.push_str(&self.to_hex(self.text_data.run_dir as u16, 2));
        }

        let mut q = String::new();
        let r_len = z.len() / 2;
        for _ in r_len..44 {
            q.push_str("00");
        }

        cmd.push_str("C0C1C2C3");
        cmd.push_str(&r);
        cmd.push_str(&h);
        cmd.push_str(&a);
        cmd.push_str(&c);
        cmd.push_str(&o);
        cmd.push_str(&s);
        cmd.push_str(&l);
        cmd.push_str(&p);
        cmd.push_str(&d);
        cmd.push_str(&j);
        cmd.push_str(&x);
        cmd.push_str(&f_str);
        cmd.push_str(&z);
        cmd.push_str(&q);
        cmd.push_str("C4C5C6C7");

        cmd.to_uppercase()
    }

    pub fn get_draw_point_str(
        &self,
        points: Vec<Vec<f32>>,
        config: Vec<u8>,
        features: &Features,
        time: i16,
        version: &str,
    ) -> String {
        let mut config_data = String::new();
        let mut point_data = String::new();

        for s in 0..15 {
            match s {
                0..=11 => config_data.push_str(&self.to_hex(config[s] as u16, 2)),
                13 => {
                    if features.pics_play && time == -1 {
                        config_data.push_str(&self.to_hex((10 * config[12] as u16), 2));
                    } else {
                        config_data.push_str("00");
                    }
                }
                14 => {
                    if features.text_stop_time {
                        config_data.push_str(&self.to_hex(self.text_data.tx_point_time as u16, 2));
                    } else {
                        config_data.push_str("00");
                    }
                }
                _ => config_data.push_str("00"),
            }
        }

        if version == "00" {
            config_data.push_str(version);
            for (l, point) in points.iter().enumerate() {
                if point.len() >= 4 {
                    let x = point[0];
                    let y = point[1];
                    let z = point[2] as u8;
                    let mut flag = point[3] as u8;
                    if features.text_stop_time {
                        if z == 0 {
                            flag = 2;
                        } else if (l < points.len() - 1 && points[l + 1][2] == 0.0) || l == points.len() - 1 {
                            flag = 3;
                        }
                    }
                    point_data.push_str(&self.to_hex(x.round() as u16, 4));
                    point_data.push_str(&self.to_hex(y.round() as u16, 4));
                    point_data.push_str(&self.to_hex(self.combine_control(z, flag) as u16, 2));
                }
            }
            config_data.push_str(&self.to_hex(points.len() as u16, 4));
            config_data.push_str(&point_data);
        } else {
            config_data.push_str(version);
        }

        config_data
    }

    pub fn get_setting_cmd(&self, settings: SettingParams) -> String {
        let t = self.to_hex(settings.val_arr[0] as u16, 2);
        let r = self.to_hex(settings.ch as u16, 2);
        let n = self.to_hex(settings.val_arr[1] as u16, 2);
        let h = self.to_hex(settings.xy as u16, 2);
        let mut a = self.to_hex(settings.val_arr[2] as u16, 2);
        let mut c = self.to_hex(settings.val_arr[3] as u16, 2);
        let mut o = self.to_hex(settings.val_arr[4] as u16, 2);
        let s = self.to_hex(settings.light as u16, 2);
        let l = self.to_hex(settings.cfg as u16, 2);

        if settings.cfg == 0 {
            a = "FF".to_string();
            c = "FF".to_string();
            o = "FF".to_string();
        }

        let p = self.to_hex(settings.lang as u16, 2);

        let mut cmd = String::new();
        write!(
            cmd,
            "E0E1E2E3{}{}{}{}{}{}{}{}{}{}000000000000",  // Need 6 bytes (12 chars) of padding to make 20 bytes total
            t, r, n, h, a, c, o, s, l, p
        )
        .expect("Failed to write to string");

        cmd.to_uppercase()
    }

pub fn generate_command(
    &self,
    points: Vec<(u8, Vec<LaserPoint>)>, // [frame, [points]]
    time: f32,
    options: LaserOptions,
    direction: u8,
    version: u8,
) -> Option<LaserCommand> {
    if points.is_empty() {
        return None;
    }

    let mut point_count = 0;
    let mut char_count = 0;
    let mut cmd_data = String::new();
    let mut char_width_cmd = String::new();
    let mut char_point_cmd = String::new();
    let max_points_per_frame = 8;
    let mut frame_index = max_points_per_frame;
    let mut total_points_in_segment = 0;
    let mut prev_frame = -1;

    let time_str = if options.text_decimal_time {
        self.to_hex((time * 10.0).round() as u16, 2)
    } else {
        self.to_hex(time.round() as u16, 2)
    };

    let (processed_points, se1, se2, x_offset) = self.generate_grid(
        points.iter().flat_map(|(_, p)| p.clone()).collect(),
        direction,
    );

    let xyss: Vec<(u8, Vec<LaserPoint>)> = points
        .iter()
        .map(|(frame, pts)| {
            let new_pts = processed_points
                .iter()
                .enumerate()
                .filter(|(_, p)| p.x == pts[0].x && p.y == pts[0].y) // Match by coordinates
                .map(|(_, p)| p.clone())
                .collect();
            (*frame, new_pts)
        })
        .collect();

    for (frame, point_group) in &xyss {
        if *frame as i32 != prev_frame {
            if prev_frame != -1 && total_points_in_segment > 0 {
                char_point_cmd.push_str(&self.to_hex(total_points_in_segment as u16, 2));
                total_points_in_segment = 0;
            }
            char_count += 1;
            char_width_cmd.push_str(&self.to_hex(
                (point_group[0].x as f32 * self.scale_factor).round() as u16,
                2,
            ));
            prev_frame = *frame as i32;
        }
        if max_points_per_frame >= 8 && point_group.len() > 1 {
            frame_index += 1;
        }
        if frame_index >= 8 {
            frame_index = 1;
        }

        total_points_in_segment += point_group.len();
        for (idx, point) in point_group.iter().enumerate() {
            point_count += 1;
            let x = (point.x as f32 * self.scale_factor + x_offset as f32).round() as u16;
            let y = (point.y as f32 * self.scale_factor).round() as u16;
            let mut flag = point.z;
            let ctrl_frame = frame_index;
            if idx == 0 {
                flag = 1;
            } else if idx == point_group.len() - 1 {
                flag = 1;
            } else if point_group.len() == 1 {
                flag = point.z;
            }
            if options.text_stop_time && point_group.len() > 1 {
                if ctrl_frame == 0 {
                    flag = 2;
                } else if (idx < point_group.len() - 1 && point_group[idx + 1].z == 0)
                    || idx == point_group.len() - 1
                {
                    flag = 3;
                }
            }
            let ctrl = self.combine_control(ctrl_frame as u8, flag);
            cmd_data.push_str(&self.to_hex(x, 4));
            cmd_data.push_str(&self.to_hex(y, 4));
            cmd_data.push_str(&self.to_hex(ctrl as u16, 2));
            cmd_data.push_str(&self.to_hex(point.color as u16, 2)); // Add color data
        }
    }

    if total_points_in_segment > 0 {
        char_point_cmd.push_str(&self.to_hex(total_points_in_segment as u16, 2));
    }

    if point_count == 0 {
        None
    } else {
        let mut cmd = LaserCommand::new();
        cmd.point_count = point_count as u16;
        cmd.char_count = char_count as u16;
        cmd.command_data = cmd_data;
        cmd.char_width_cmd = char_width_cmd;
        cmd.char_point_cmd = char_point_cmd;
        cmd.segment1 = se1;
        cmd.segment2 = se2;
        cmd.version = self.to_hex(version as u16, 2);
        cmd.time = time_str;
        cmd.points = points.iter().flat_map(|(_, p)| p.clone()).collect();
        Some(cmd)
    }
}


    fn generate_grid(
        &self,
        points: Vec<LaserPoint>,
        direction: u8,
    ) -> (Vec<LaserPoint>, String, String, i32) {
        let mut prev_frame = -1;
        let mut x_sums = Vec::new();
        let mut y_sums = Vec::new();
        let grid_step = 200;
        let mut x_total = 0;
        let mut y_total = 0;

        for point in &points {
            let frame = point.x as i32 / 100; // Approximate frame from x
            if frame != prev_frame {
                if prev_frame != -1 {
                    x_sums.push(x_total as f32 * self.scale_factor);
                    y_sums.push(y_total as f32 * self.scale_factor);
                }
                x_total = 0;
                y_total = 0;
                prev_frame = frame;
            }
            x_total += point.x as i32;
            y_total += point.y as i32;
        }
        if !x_sums.is_empty() {
            x_sums.push(x_total as f32 * self.scale_factor);
            y_sums.push(y_total as f32 * self.scale_factor);
        }

        let mut new_points = points;
        let mut se1 = String::new();
        let mut se2 = String::new();
        let mut offset = 0;

        match direction {
            127 => { // Up
                let mut y_offset = 0;
                let mut grid_points = Vec::new();
                for _ in 0..9 {
                    let y = (y_sums.iter().sum::<f32>() / 2.0 + grid_step as f32 / 2.0 + y_offset as f32) as i16;
                    grid_points.push(LaserPoint::new(0, y, 0, 0));
                    y_offset += grid_step;
                }
                new_points.extend(grid_points);
                let normalized = normalize_sums(&y_sums, 800);
                for (idx, cnt) in normalized {
                    se1.push_str(&self.to_hex(idx as u16, 2));
                    se2.push_str(&self.to_hex(cnt as u16, 2));
                }
                offset = -(y_offset as f32 * self.scale_factor / 2.0) as i32;
            }
            128 | _ => { // Down or default
                let mut x_offset = 0;
                let mut grid_points = Vec::new();
                for _ in 0..9 {
                    let x = (x_sums.iter().sum::<f32>() / 2.0 + (grid_step as f32) / 2.0 + x_offset as f32) as i16;
                    grid_points.push(LaserPoint::new(x, 0, 0, 0));
                    x_offset += grid_step;
                }
                new_points.extend(grid_points);
                let normalized = normalize_sums(&x_sums, 800);
                for (idx, cnt) in normalized {
                    se1.push_str(&self.to_hex(idx as u16, 2));
                    se2.push_str(&self.to_hex(cnt as u16, 2));
                }
                offset = -(x_offset as f32 * self.scale_factor / 2.0) as i32;
            }
        }

        (new_points, se1, se2, offset)
    }

    pub fn get_query_cmd(&self, params: &[u8]) -> String {
        let mut param_data = String::new();
        for &param in params {
            param_data.push_str(&self.to_hex(param as u16, 2));
        }

        let mut cmd = String::new();
        write!(
            cmd,
            "E0E1E2E3{}E4E5E6E7",
            param_data,
        )
        .expect("Failed to write to string");

        cmd.to_uppercase()
    }

    pub fn get_shake_cmd_str(
        &self,
        data: &ShakeData,
        options: Option<&ShakeOptions>,
    ) -> String {
        let mut cmd_data = String::new();

        if let Some(opts) = options {
            cmd_data.push_str(if opts.xy_cnf_save.unwrap_or(true) { "00" } else { "ff" });
        } else {
            cmd_data.push_str("00");
        }

        let xy_cnf = &data.subset_data.xy_cnf;
        if xy_cnf.auto {
            cmd_data.push_str(&self.to_hex(xy_cnf.auto_value as u16, 2));
        } else {
            cmd_data.push_str(&self.to_hex(255 - xy_cnf.auto_value as u16, 2));
        }
        cmd_data.push_str(&self.to_hex(xy_cnf.phase as u16, 2));

        for xy_value in &xy_cnf.xy {
            cmd_data.push_str(&self.to_hex(xy_value.value as u16, 2));
        }

        while cmd_data.len() / 2 < 16 {
            cmd_data.push_str("00");
        }

        let mut cmd = String::new();
        write!(
            cmd,
            "10111213{}14151617",
            cmd_data,
        )
        .expect("Failed to write to string");

        cmd.to_uppercase()
    }
}

/// Normalizes sums to create segments within a threshold.
fn normalize_sums(values: &[f32], threshold: i32) -> Vec<(i32, i32)> {
    let mut running_sum = 0.0;
    let mut result = Vec::new();
    let mut start_idx = 0;
    let mut count = 0;

    for (i, &value) in values.iter().enumerate() {
        if running_sum + value <= threshold as f32 {
            count += 1;
            running_sum += value;
            result.push((start_idx as i32, count as i32));
        } else {
            let mut temp_width = running_sum;
            loop {
                if temp_width <= threshold as f32 {
                    count += 1;
                    result.push((start_idx as i32, count as i32));
                    running_sum = temp_width + value;
                    break;
                } else if temp_width > threshold as f32 && temp_width - values[start_idx] < threshold as f32 {
                    count += 1;
                    result.push((start_idx as i32, count as i32));
                    running_sum += value;
                    break;
                } else {
                    temp_width -= values[start_idx];
                    running_sum -= values[start_idx];
                    start_idx += 1;
                    count -= 1;
                }
            }
        }
    }

    result
}

/// Pads a string to a specified length with a given padding.
fn pad_string(s: String, length: usize, pad: &str) -> String {
    let mut result = s;
    while result.len() / 2 < length {
        result += pad;
    }
    result
}

/// Processes selected project indices.
fn process_selected(selected: &[u8]) -> Vec<u8> {
    let mut vec = selected.to_vec();
    vec.reverse();
    vec
}

/// Adjusts selected indices based on a reference array (placeholder).
fn adjust_selected(e: &[u8], t: Vec<u8>) -> Vec<u8> {
    t // Placeholder, implement bit logic as needed
}


/// Main controller for laser device communication.
pub struct LaserController {
    sending: Arc<Mutex<bool>>,
    ble_controller: Option<Box<dyn BlueController>>, // Unified BLE controller using trait
    last_send_time: Option<Instant>,
    options: LaserOptions,
    project_data: ProjectData,
    connected: bool,
    pub blu_rec_content: VecDeque<String>,
    rec_device_msg_timer: Option<tokio::time::Instant>,
    discovery_started: bool,
    testing_idx: usize,
    not_pass: i32,
    pass_count: i32,
}

impl Clone for LaserController {
    fn clone(&self) -> Self {
        LaserController {
            sending: Arc::clone(&self.sending),
            ble_controller: None, // BLE controller can't be cloned directly
            last_send_time: self.last_send_time,
            options: self.options.clone(),
            project_data: ProjectData {
                public: PublicSettings {
                    rd_mode: self.project_data.public.rd_mode,
                    sound_val: self.project_data.public.sound_val,
                },
                prj_item: self.project_data.prj_item.clone(),
            },
            connected: self.connected,
            blu_rec_content: self.blu_rec_content.clone(),
            rec_device_msg_timer: self.rec_device_msg_timer,
            discovery_started: self.discovery_started,
            testing_idx: self.testing_idx,
            not_pass: self.not_pass,
            pass_count: self.pass_count,
        }
    }
}

impl LaserController {
    /// Creates a new LaserController with a BLE controller passed directly
    pub async fn new<T: BlueController + 'static>(ble_controller: T) -> Result<Self, Box<dyn Error>> {
        let sending = Arc::new(Mutex::new(false));
 
        let options = LaserOptions {
            text_decimal_time: false,
            text_stop_time: false,
            tx_color: 0,
            tx_size: 60,
            run_speed: 128,
            tx_dist: 60,
            rd_mode: 0,
            sound_val: 0,
        };

        let project_data = ProjectData {
            public: PublicSettings { rd_mode: 0, sound_val: 0 },
            prj_item: Vec::new(),
        };

        let mut controller = LaserController {
            sending,
            ble_controller: Some(Box::new(ble_controller)),
            last_send_time: None,
            options,
            project_data,
            connected: false,
            blu_rec_content: VecDeque::new(),
            rec_device_msg_timer: None,
            discovery_started: false,
            testing_idx: 0,
            not_pass: 0,
            pass_count: 0,
        };
        controller.connected = controller.is_connected();
        Ok(controller)
    }


    pub async fn send(&mut self, cmd_hex: &str, callback: Option<&mut dyn FnMut(i8, u8)>) -> Result<(), String> {
        debug!("[LaserController] send() called with cmd_hex: {}", cmd_hex);
        if cmd_hex.is_empty() {
            debug!("[LaserController] send() failed: Empty command");
            return Err("Empty command".to_string());
        }

        let mut guard = self.sending.lock().unwrap();
        if *guard {
            debug!("[LaserController] send() failed: Previous send in progress");
            return Err("Previous send in progress".to_string());
        }
        *guard = true;

        if !self.is_connected() {
            debug!("[LaserController] send() failed: Not connected");
            return Err("Not connected".to_string());
        }
        drop(guard); // Drop the lock before mutable borrow

        let mut cb_result: Option<(i8, u8)> = None;
        let mut cb = callback;
        if let Some(cb_ref) = cb.as_deref_mut() {
            cb_ref(0, 0);
        }

        let bytes = decode(cmd_hex).map_err(|e| {
            debug!("[LaserController] send() failed: Invalid hex: {}", e);
            e.to_string()
        })?;
        if bytes.is_empty() {
            debug!("[LaserController] send() failed: Decoded bytes empty");
            let mut guard = self.sending.lock().unwrap();
            *guard = false;
            return Err("Invalid hex".to_string());
        }

        debug!("[LaserController] send() sending BLE bytes: {:02X?}", bytes);
        let result = self.send_ble(&bytes).await;

        let mut guard = self.sending.lock().unwrap();
        *guard = false;
        if let Some(cb_ref) = cb.as_deref_mut() {
            if result.is_ok() {
                cb_ref(1, 100);
            } else {
                cb_ref(-1, 0);
            }
        }
        if let Err(ref e) = result {
            debug!("[LaserController] send() BLE send failed: {}", e);
        } else {
            debug!("[LaserController] send() BLE send succeeded");
        }
        result
    }

    async fn send_ble(&mut self, bytes: &[u8]) -> Result<(), String> {
        debug!("[LaserController] send_ble() called with bytes: {:02X?}", bytes);
        if let Some(controller) = &mut self.ble_controller {
            let result = controller.send(bytes).await;
            if let Err(ref e) = result {
                debug!("[LaserController] send_ble() failed: {}", e);
            } else {
                debug!("[LaserController] send_ble() succeeded");
            }
            result
        } else {
            debug!("[LaserController] send_ble() failed: BLE controller not initialized");
            Err("BLE controller not initialized".to_string())
        }
    }

 //   pub async fn send_animation(&mut self, points: &[LaserPoint]) -> Result<(), String> {
 //   if points.is_empty() {
 //       return Err("No points provided".to_string());
 //   }
//    let generator = CommandGenerator::new();
//    let options = self.options.clone();
//    let mut cmd = generator.generate_command(
//        vec![(0, points.to_vec())],
//        1.0,
//        options,
//        self.project_data.public.prj_selected,
 //       self.project_data.public.prj_item,
//    );
//    let cmd = cmd.unwrap();
//    debug!("Sending animation with {} points", cmd.point_count);
//    for ble_cmd in cmd.to_ble_command() {
//        let hex_cmd = hex::encode(&ble_cmd);
//        debug!("[LaserController] Sending BLE command: {}", hex_cmd);
//        let result = self.send(&hex_cmd, None).await;
//        if let Err(e) = result {
//            error!("Failed to send BLE command: {}", e);
//            return Err(e);
//        }
//        sleep(Duration::from_millis(20)).await;
//    }
//    Ok(())
//}

    pub fn is_connected(&self) -> bool {
        self.ble_controller.is_some() && self.ble_controller.as_ref().unwrap().is_connected()
    }

    pub fn parse_response(&mut self, hex: &str) -> Result<(), String> {
        let c_section = get_cmd_value("C0C1C2C3", "C4C5C6C7", hex)?;
        let bytes = hex::decode(&c_section).map_err(|e| e.to_string())?;

        self.project_data.public.rd_mode = clamp(bytes.get(9).cloned().unwrap_or(0), 0, 255);
        self.options.tx_color = clamp(bytes.get(3).cloned().unwrap_or(0), 0, 9);
        self.options.tx_size = clamp(((bytes.get(4).cloned().unwrap_or(0) as f32 / 255.0) * 100.0).round() as u8, 10, 100);
        self.options.run_speed = clamp(((bytes.get(6).cloned().unwrap_or(0) as f32 / 255.0) * 100.0).round() as u8, 0, 255);
        self.options.tx_dist = clamp(((bytes.get(8).cloned().unwrap_or(0) as f32 / 255.0) * 100.0).round() as u8, 10, 100);
        self.project_data.public.sound_val = clamp(((bytes.get(10).cloned().unwrap_or(0) as f32 / 255.0) * 100.0).round() as u8, 0, 255);

        let mut offset = 17;
        if let Some(item) = self.project_data.prj_item.get_mut(0) {
            item.py_mode = clamp(bytes.get(offset).cloned().unwrap_or(0), 0, 255);
            item.prj_selected[3] = clamp(bytes.get(offset + 1).cloned().unwrap_or(0) as u16, 0, 65535) as u8;
            item.prj_selected[2] = clamp(bytes.get(offset + 3).cloned().unwrap_or(0) as u16, 0, 65535) as u8;
            item.prj_selected[1] = clamp(bytes.get(offset + 5).cloned().unwrap_or(0) as u16, 0, 65535) as u8;
            item.prj_selected[0] = clamp(bytes.get(offset + 7).cloned().unwrap_or(0) as u16, 0, 65535) as u8;
            offset += 9;
        }

        let xy_offset = offset;
        self.options.tx_dist = clamp(bytes.get(xy_offset).cloned().unwrap_or(0), 0, 255); // Placeholder
        // Expand XY config as needed

        let b_section = get_cmd_value("00010203", "04050607", hex)?;
        let b_bytes = hex::decode(&b_section).map_err(|e| e.to_string())?;
        // Expand settings parsing

        let d_section = get_cmd_value("D0D1D2D3", "D4D5D6D7", hex)?;
        if !d_section.is_empty() {
            let d_bytes = hex::decode(&d_section).map_err(|e| e.to_string())?;
            let count = clamp(d_bytes.get(1).cloned().unwrap_or(0) & 0x7F, 0, 127);
            for i in 0..count as usize {
                let item_offset = 3 + i * 22;
                if item_offset + 22 <= d_bytes.len() {
                    let mut item = PrjItem {
                        py_mode: clamp(d_bytes[item_offset], 0, 255),
                        prj_selected: vec![0; 4],
                    };
                    for j in 0..4 {
                        item.prj_selected[j] = clamp(d_bytes[item_offset + 1 + j * 2] as u16, 0, 65535) as u8;
                    }
                    self.project_data.prj_item.push(item);
                }
            }
        }
        Ok(())
    }

    fn add_content(&mut self, hex: String) {
        println!("addContent: {}", hex);
        let mut content = self.blu_rec_content.back().cloned().unwrap_or_else(String::new);
        if content.is_empty() && hex.starts_with("E0E1E2E3") {
            content = hex;
        } else {
            content.push_str(&hex);
        }

        if !content.is_empty() {
            let last_start = content.rfind("E0E1E2E3").unwrap_or(0);
            let last_end = content.rfind("E4E5E6E7").unwrap_or(0);
            let mut processed = content.clone();

            if last_end > 0 {
                if last_end == content.len() - 8 {
                    let packet = content[last_start..last_end + 8].to_string();
                    self.data_received(packet);
                    processed = String::new();
                } else {
                    processed = content[last_start..].to_string();
                }
            }
            self.blu_rec_content.clear();
            self.blu_rec_content.push_back(processed);
        }
    }

    fn data_received(&mut self, packet: String) {
        println!("Data received: {}", packet);
        // Placeholder: implement actual packet handling logic here.
    }



    pub fn cleanup(&mut self) -> Result<(), String> {
        // Close any BLE connections
        self.ble_controller = None;
        self.connected = false;
        Ok(())
    }
}

fn clamp<T: Ord>(val: T, min: T, max: T) -> T {
    val.max(min).min(max)
}

fn get_cmd_value(start: &str, end: &str, input: &str) -> Result<String, String> {
    let re = format!("{}(.+?){}", regex::escape(start), regex::escape(end));
    let regex = Regex::new(&re).map_err(|e| format!("Regex error: {}", e))?;
    regex.captures(input)
        .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()))
        .ok_or_else(|| {
            println!("No matching string found at getCmdValue");
            "No matching string found at getCmdValue".to_string()
        })
}

use windows::{
    core::{HSTRING, Result as WindowsResult, GUID},
    Devices::Bluetooth::BluetoothLEDevice,
    Devices::Enumeration::DeviceInformation,
    Devices::Bluetooth::GenericAttributeProfile::{
        GattDeviceService, GattCharacteristic, GattCharacteristicProperties, 
        GattCommunicationStatus, GattWriteOption,
        GattClientCharacteristicConfigurationDescriptorValue
    },
    Storage::Streams::{DataWriter, DataReader},
    Foundation::TypedEventHandler,
};

// Wrapper type for notification token to avoid direct EventRegistrationToken dependency
#[derive(Clone, Copy)]
struct NotificationToken(i64);
use std::sync::{Arc, Mutex};
use tokio::sync::Mutex as TokioMutex;
use std::error::Error;
use std::time::Duration;
use hex::decode;
use tokio::time::{Instant, sleep};
use log::{info, warn, error, debug};
use serialport::{SerialPort, SerialPortType};

use crate::blue::{self, BlueController};



pub type Characteristic = GattCharacteristic;

pub struct WinBlueController {
    device_info: Option<DeviceInformation>,
    device: Option<BluetoothLEDevice>,
    write_char: Option<GattCharacteristic>,
    notify_char: Option<GattCharacteristic>,
    service_uuid: Option<GUID>,
    connected: bool,
    buffer: Arc<Mutex<String>>,
    last_send_time: Option<Instant>,
    sending: Arc<TokioMutex<()>>,
    notification_token: Option<NotificationToken>,
}

impl WinBlueController {
    pub async fn new(device_info: Option<&DeviceInformation>) -> Result<Self, Box<dyn Error>> {

        Ok(Self {
            device_info: device_info.cloned(),
            device: None,
            write_char: None,
            notify_char: None,
            service_uuid: None,            
            connected: false,
            buffer: Arc::new(Mutex::new(String::new())),
            last_send_time: None,
            sending: Arc::new(TokioMutex::new(())),
            notification_token: None,
        })
    }

    pub async fn connect(&mut self) -> Result<(), Box<dyn Error>> {
        debug!("WinBlueController::connect called");
        if let Some(device_info) = self.device_info.as_ref() {
            let device_id = device_info.Id()?;
            debug!("Connecting to BLE device with id: {}", device_id);
            self.device = Some(BluetoothLEDevice::FromIdAsync(&device_id)?.get()?);
                self.discover_characteristics().await?;
            self.connected = true;
            debug!("Device connected");
            // Send initialization command (if required)
            //self.send_init_command().await?;
        }
        Ok(())
    }

    //async fn send_init_command(&mut self) -> Result<(), String> {
    //    let init_cmd = "E0E1E2E38BCE183AE4E5E6E70000000000000000"; // Padded to 20 bytes
    //    let bytes = decode(init_cmd).map_err(|e| format!("Invalid init command: {}", e))?;
    //    let result = self.send_ble(&bytes).await;
    //    if result.is_ok() {
    //        // Initialize DMX
    //        //if let Some(ref mut dmx_port) = self.dmx_port {
    //        //    let dmx_frame = [0u8; 512]; // Reset all channels
    //        //    dmx_port.write(&[0x00]).map_err(|e| e.to_string())?;
    //       //     dmx_port.write(&dmx_frame).map_err(|e| e.to_string())?;
     //       //}
     //       debug!("Initialization command sent successfully");
    //    } else {
    //        error!("Failed to send initialization command");
    //    }
    //    result
    //'}

    pub async fn discover_characteristics(&mut self) -> Result<(), Box<dyn Error>> {
        debug!("WinBlueController::discover_characteristics called");
        if let Some(device) = &self.device {
            let services_result = device.GetGattServicesAsync()?.get()?;
            debug!("Enumerating GATT services");
            for j in 0..services_result.Services()?.Size()? {
                let service: GattDeviceService = services_result.Services()?.GetAt(j)?;
                let service_uuid = service.Uuid()?;
                let service_uuid_str = format!("{:?}", service_uuid).to_uppercase();
                debug!("Found service UUID: {}", service_uuid_str);
                if blue::LASER_SERVICE_UUID.contains(&service_uuid_str.as_str()) {
                    self.service_uuid = Some(service_uuid);
                    info!("Service UUID: {:?} Found Laser Service uuid", service_uuid);
                    let characteristics_result = service.GetCharacteristicsAsync()?.get()?;
                    let characteristics = characteristics_result.Characteristics()?;
                    debug!("Enumerating characteristics for service: {}", service_uuid_str);
                    for k in 0..characteristics.Size()? {
                        let characteristic: GattCharacteristic = characteristics.GetAt(k)?;
                        let props = characteristic.CharacteristicProperties()?;
                        let char_uuid: GUID = characteristic.Uuid()?;
                        let char_uuid_str = format!("{:?}", char_uuid).to_uppercase();
                        debug!("Characteristic UUID: {} Properties: {:?}", char_uuid_str, props);
                        if (props & GattCharacteristicProperties::Write == GattCharacteristicProperties::Write ||
                            props & GattCharacteristicProperties::WriteWithoutResponse == GattCharacteristicProperties::WriteWithoutResponse) &&
                            blue::WRITE_UUIDS.contains(&char_uuid_str.as_str()) {
                            info!("Write UUID: {:?} Found Laser Service write uuid", char_uuid);
                                self.write_char = Some(characteristic.clone());
                            }
                        if (props & GattCharacteristicProperties::Notify == GattCharacteristicProperties::Notify ||
                            props & GattCharacteristicProperties::Indicate == GattCharacteristicProperties::Indicate) &&
                            blue::NOTIFY_UUIDS.contains(&char_uuid_str.as_str()) {
                            info!("Notify UUID: {:?} Found Laser Service notification uuid", char_uuid);
                                self.notify_char = Some(characteristic.clone());
                        }
                    }
                }
            }
        } else {
            return Err("Device not connected".into());
        }
        if self.write_char.is_none() || self.notify_char.is_none() {
            return Err("Required characteristics not found".into());
        }
        self.setup_all_notifications().await?;
        Ok(())
    }

    pub async fn setup_all_notifications(&mut self) -> Result<(), Box<dyn Error>> {
        debug!("WinBlueController::setup_all_notifications called");
        if let Some(ref notify_char) = self.notify_char.clone() {
            debug!("Setting up notifications for characteristic");
            self.setup_notifications(notify_char).await?;
        }
        Ok(())
    }

    async fn setup_notifications(&mut self, characteristic: &GattCharacteristic) -> Result<(), Box<dyn Error>> {
        debug!("WinBlueController::setup_notifications called for characteristic");
        let buffer_clone = self.buffer.clone();
        //let dmx_port_clone = self.dmx_port.as_mut().map(|port| Box::new(port.try_clone().unwrap()) as Box<dyn SerialPort>);

        let handler = TypedEventHandler::<
            GattCharacteristic,
            windows::Devices::Bluetooth::GenericAttributeProfile::GattValueChangedEventArgs,
        >::new(move |_sender, args| {
            if let Some(args) = args.as_ref() {
                if let Ok(value_buffer) = args.CharacteristicValue() {
                    if let Ok(len) = value_buffer.Length() {
                        let mut value = vec![0u8; len as usize];
                        let data_reader = DataReader::FromBuffer(&value_buffer)?;
                        data_reader.ReadBytes(&mut value)?;
                        let hex = value.iter().map(|b| format!("{:02X}", b)).collect::<String>();
                        debug!("Notification received: {}", hex);
                        
                        let mut buffer = buffer_clone.lock().unwrap();
                        *buffer = hex;

                        // Map to DMX
                        //if let Some(ref mut dmx_port) = dmx_port_clone.as_ref() {
                        //    let mut dmx_frame = [0u8; 512];
                        //    if value.len() >= 16 && value.starts_with(&[0xE0, 0xE1, 0xE2, 0xE3]) {
                        //        dmx_frame[0] = value[12]; // Red (channel 1)
                        //        dmx_frame[1] = value[13]; // Green (channel 2)
                        //        dmx_frame[2] = value[14]; // Blue (channel 3)
                        //        dmx_frame[3] = value[8]; // x low byte (channel 4)
                        //        dmx_frame[4] = value[9]; // x high byte (channel 5)
                        //        dmx_frame[5] = value[10]; // y low byte (channel 6)
                        //        dmx_frame[6] = value[11]; // y high byte (channel 7)
                        //        dmx_frame[7] = value[15]; // z (laser on/off, channel 8)
                        //    }
                        //    dmx_port.write(&[0x00]).unwrap_or_else(|e| error!("DMX break error: {}", e));
                        //    dmx_port.write(&dmx_frame).unwrap_or_else(|e| error!("DMX write error: {}", e));
                        //}
                    }
                }
            }
            Ok(())
        });

        match characteristic.WriteClientCharacteristicConfigurationDescriptorAsync(
            GattClientCharacteristicConfigurationDescriptorValue::Notify
        )?.get() {
            Ok(GattCommunicationStatus::Success) => {
                debug!("Successfully enabled notifications");
                let windows_token = characteristic.ValueChanged(&handler)?;
                // Convert the Windows token value to our wrapper
                self.notification_token = Some(NotificationToken(unsafe { std::mem::transmute(windows_token) }));
                Ok(())
            },
            Ok(status) => Err(format!("Failed to enable notifications: {:?}", status).into()),
            Err(e) => Err(e.into()),
        }
    }
    
    pub fn add_content(&mut self, content: String) {
        let mut buffer = self.buffer.lock().unwrap();
        *buffer = content;
    }

    pub fn get_content(&self) -> String {
        let buffer = self.buffer.lock().unwrap();
        buffer.clone()
    }

    pub fn is_connected(&self) -> bool {
        self.connected
    }

    pub async fn send(&mut self, bytes: &[u8]) -> Result<(), String> {
        debug!("WinBlueController::send called with {} bytes", bytes.len());
        if bytes.len() != 20 || !bytes.starts_with(&[0xE0, 0xE1, 0xE2, 0xE3]) {
            return Err("Invalid command: must be 20 bytes starting with E0E1E2E3".to_string());
        }

        let _guard = self.sending.lock().await;
        if !self.is_connected() {
            return Err("Not connected".to_string());
        }

        if let Some(last_send) = self.last_send_time {
            let elapsed = Instant::now().duration_since(last_send);
            if elapsed < Duration::from_millis(20) {
                sleep(Duration::from_millis(20) - elapsed).await;
            }
        }
        self.last_send_time = Some(Instant::now());
        self.send_data(bytes).await
    }

    async fn send_data(&self, bytes: &[u8]) -> Result<(), String> {
        if let Some(write_char) = &self.write_char {
            let hex_str = bytes.iter().map(|b| format!("{:02X}", b)).collect::<Vec<_>>().join("");
            debug!("Writing command to BLE: {}", hex_str);

            let writer = DataWriter::new().map_err(|e| e.to_string())?;
            writer.WriteBytes(bytes).map_err(|e| e.to_string())?;
            let buffer = writer.DetachBuffer().map_err(|e| e.to_string())?;
            
            let write_result = write_char.WriteValueAsync(&buffer).map_err(|e| e.to_string())?.get();
            match write_result {
                Ok(GattCommunicationStatus::Success) => {
                    debug!("Write to BLE characteristic succeeded");
                    // Send to DMX
                    //if let Some(ref mut dmx_port) = self.dmx_port.as_ref() {
                    //    let mut dmx_frame = [0u8; 512];
                    //    dmx_frame[0] = bytes[12]; // Red (channel 1)
                    //    dmx_frame[1] = bytes[13]; // Green (channel 2)
                    //     dmx_frame[2] = bytes[14]; // Blue (channel 3)
                    //    dmx_frame[3] = bytes[8]; // x low byte (channel 4)
                    //    dmx_frame[4] = bytes[9]; // x high byte (channel 5)
                    //    dmx_frame[5] = bytes[10]; // y low byte (channel 6)
                    //    dmx_frame[6] = bytes[11]; // y high byte (channel 7)
                    //    dmx_frame[7] = bytes[15]; // z (laser on/off, channel 8)
                    //    dmx_port.write(&[0x00]).map_err(|e| e.to_string())?;
                    //    dmx_port.write(&dmx_frame).map_err(|e| e.to_string())?;
                    //}
                    Ok(())
                },
                Ok(status) => Err(format!("Write failed with status: {:?}", status)),
                Err(e) => Err(format!("Write failed with error: {:?}", e)),
            }
        } else {
            Err("Write characteristic not found".to_string())
        }
    }

    pub async fn send_animation(&mut self, coordinates: &[[u16; 2]], color: [u8; 3]) -> Result<(), String> {
        for &[x, y] in coordinates {
            let command = vec![
                0xE0, 0xE1, 0xE2, 0xE3,
                0xC0, 0xC1, 0xC2, 0xC3,
                (x & 0xFF) as u8, (x >> 8) as u8,
                (y & 0xFF) as u8, (y >> 8) as u8,
                color[0], color[1], color[2],
                1, // z=1 (laser on)
                0x00, 0x00, 0x00, 0x00,
            ];
            self.send(&command).await?;
        }
        // Send laser off command
        let off_command = vec![
            0xE0, 0xE1, 0xE2, 0xE3,
            0xC0, 0xC1, 0xC2, 0xC3,
            0x00, 0x00, 0x00, 0x00,
            color[0], color[1], color[2],
            0, // z=0 (laser off)
            0x00, 0x00, 0x00, 0x00,
        ];
        self.send(&off_command).await
    }

    pub async fn disconnect(&mut self) -> Result<(), Box<dyn Error>> {
        debug!("WinBlueController::disconnect called");
        if let (Some(characteristic), Some(token)) = (&self.notify_char, self.notification_token) {
            // Convert our wrapper back to Windows token type
            let windows_token = unsafe { std::mem::transmute(token.0) };
            characteristic.RemoveValueChanged(windows_token)?;
        }
        self.device = None;
        self.write_char = None;
        self.notify_char = None;
        self.connected = false;
        self.notification_token = None;
        debug!("Device disconnected and resources cleaned up");
        Ok(())
    }
}

impl BlueController for WinBlueController {
    fn connect<'a>(&'a mut self) -> std::pin::Pin<Box<dyn Future<Output = Result<(), Box<dyn Error>>> + Send + 'a>> {
        Box::pin(async move { self.connect().await })
    }
    
    fn send<'a>(&'a mut self, bytes: &'a [u8]) -> std::pin::Pin<Box<dyn Future<Output = Result<(), String>> + Send + 'a>> {
        Box::pin(async move { self.send(bytes).await })
    }
    
    fn get_content(&self) -> String {
        self.get_content()
    }
    
    fn is_connected(&self) -> bool {
        self.is_connected()
    }
}

pub async fn scan_laser_devices() -> Result<Vec<DeviceInformation>, Box<dyn Error>> {
    let selector = BluetoothLEDevice::GetDeviceSelector()?;
    let devices = DeviceInformation::FindAllAsyncAqsFilter(&selector)?.get()?;
    let mut device_list = Vec::new();

    for i in 0..devices.Size()? {
        let device_info: DeviceInformation = devices.GetAt(i)?;
        let device_name = device_info.Name()?;
        let device_name_str = device_name.to_string_lossy();
        if !device_name_str.starts_with(blue::LASER_DEVICE_PREFIX) {
            continue;
        }

        let device_id = device_info.Id()?;
        info!("Found laser device: {} ({})", device_name, device_id);
        let ble_device = BluetoothLEDevice::FromIdAsync(&device_id)?.get()?;
        let services_result = ble_device.GetGattServicesAsync()?.get()?;
        for j in 0..services_result.Services()?.Size()? {
            let service: GattDeviceService = services_result.Services()?.GetAt(j)?;
            let service_uuid = service.Uuid()?;
            let str = format!("{:?}", service_uuid).to_uppercase();
            if blue::LASER_SERVICE_UUID.contains(&str.as_str()) {
                info!("Found laser service: ({:?})", service_uuid);
                device_list.push(device_info.clone());
            }
        }
    }
    Ok(device_list)
}