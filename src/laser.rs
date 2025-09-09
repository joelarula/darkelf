use std::sync::{Arc, Mutex};
use std::time::{Instant, Duration};
use hex::decode;
use btleplug::api::Characteristic; // Still needed for type references
use tokio; // For async handling
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
            "00010203{}{}{}{}{}{}{}{}{}{}000000000004050607",
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
    blu_rec_content: VecDeque<String>,
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
    pub async fn new<T: BlueController + 'static>(is_ble: bool, mock_mode: bool, ble_controller: T) -> Result<Self, Box<dyn Error>> {
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


    pub async fn send(&mut self, cmd_hex: &str, show_loading: bool, callback: Option<&mut dyn FnMut(i8, u8)>) -> Result<(), String> {
        if cmd_hex.is_empty() { return Err("Empty command".to_string()); }

        let mut guard = self.sending.lock().unwrap();
        if *guard { return Err("Previous send in progress".to_string()); }
        *guard = true;

        if !self.is_connected() { return Err("Not connected".to_string()); }
        drop(guard); // Drop the lock before mutable borrow

        let mut cb_result: Option<(i8, u8)> = None;
        let mut cb = callback;
        if let Some(cb_ref) = cb.as_deref_mut() {
            cb_ref(0, 0);
        }
        if show_loading {
            self.last_send_time = Some(Instant::now());
        }

        let bytes = decode(cmd_hex).map_err(|e| e.to_string())?;
        if bytes.is_empty() { 
            let mut guard = self.sending.lock().unwrap();
            *guard = false; 
            return Err("Invalid hex".to_string()); 
        }

        let result = self.send_ble(&bytes).await;
        
        let mut guard = self.sending.lock().unwrap();
        *guard = false;
        if let Some(cb_ref) = cb.as_deref_mut() {
            if result.is_ok() { cb_ref(1, 100); } else { cb_ref(-1, 0); }
        }
        result
    }

    async fn send_ble(&mut self, bytes: &[u8]) -> Result<(), String> {
        if let Some(controller) = &mut self.ble_controller {
            controller.send(bytes).await
        } else {
            Err("BLE controller not initialized".to_string())
        }
    }


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
