use std::sync::{Arc, Mutex};
use std::time::{Instant, Duration};
use log::{debug, error};
use hex::decode;
use tokio::time::sleep;
use crate::blue::BlueController;
use std::error::Error;
use std::collections::VecDeque;
use regex::Regex;
use std::fmt::Write;
use serialport::{SerialPort, SerialPortType};

#[derive(Debug, Clone)]
pub struct LaserPoint {
    x: i16,
    y: i16,
    z: u8,
    rgb: [u8; 3], // Changed from color to rgb for explicit RGB control
}

impl LaserPoint {
    pub fn new(x: i16, y: i16, z: u8, rgb: [u8; 3]) -> Self {
        LaserPoint { x, y, z, rgb }
    }
}

#[derive(Debug)]
pub struct LaserCommand {
    point_count: u16,
    char_count: u16,
    command_data: String,
    char_width_cmd: String,
    char_point_cmd: String,
    segment1: String,
    segment2: String,
    version: String,
    time: String,
    points: Vec<LaserPoint>,
    points_right: Option<Vec<LaserPoint>>,
    points_up: Option<Vec<LaserPoint>>,
    points_down: Option<Vec<LaserPoint>>,
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
            format!("{:04x}", self.char_count),
            self.char_width_cmd,
            self.char_point_cmd,
            self.segment1.clone() + &self.segment2 + &self.version + &self.time
        ).to_uppercase()
    }

    pub fn to_ble_command(&self) -> Vec<u8> {
        // Generate a 20-byte E0E1E2E3 command for each point
        let mut commands = Vec::new();
        for point in &self.points {
            let mut cmd = vec![
                0xE0, 0xE1, 0xE2, 0xE3, // Start marker
                0xC0, 0xC1, 0xC2, 0xC3, // Command type
                (point.x & 0xFF) as u8, (point.x >> 8) as u8, // x
                (point.y & 0xFF) as u8, (point.y >> 8) as u8, // y
                point.rgb[0], point.rgb[1], point.rgb[2], // RGB
                point.z, // z
                0x00, 0x00, 0x00, 0x00, // Padding
            ];
            commands.push(cmd);
        }
        commands.into_iter().flatten().collect()
    }
}

#[derive(Debug, Clone)]
pub struct LaserOptions {
    text_decimal_time: bool,
    text_stop_time: bool,
    tx_color: u8,
    tx_size: u8,
    run_speed: u8,
    tx_dist: u8,
    rd_mode: u8,
    sound_val: u8,
}

impl LaserOptions {
    pub fn new() -> Self {
        LaserOptions {
            text_decimal_time: false,
            text_stop_time: false,
            tx_color: 0,
            tx_size: 60,
            run_speed: 128,
            tx_dist: 60,
            rd_mode: 0,
            sound_val: 0,
        }
    }
}

#[derive(Debug)]
pub struct ProjectData {
    public: PublicSettings,
    prj_item: Vec<PrjItem>,
}

#[derive(Debug)]
pub struct PublicSettings {
    rd_mode: u8,
    sound_val: u8,
}

#[derive(Debug, Clone)]
pub struct PrjItem {
    py_mode: u8,
    prj_selected: Vec<u8>,
}

#[derive(Debug, Default)]
pub struct Features {
    group_list: Option<Vec<Group>>,
    text_stop_time: bool,
    animation_fix: bool,
    arb_play: bool,
    pics_play: bool,
    sel_index: Option<u8>,
}

#[derive(Debug)]
pub struct Group {
    color: u8,
}

#[derive(Debug)]
pub struct XYValue {
    value: u8,
}

#[derive(Debug)]
pub struct XYConfig {
    auto: bool,
    auto_value: u8,
    phase: u8,
    xy: Vec<XYValue>,
}

#[derive(Debug)]
pub struct SubsetData {
    xy_cnf: XYConfig,
}

#[derive(Debug)]
pub struct ShakeData {
    subset_data: SubsetData,
}

#[derive(Debug)]
pub struct ShakeOptions {
    xy_cnf_save: Option<bool>,
}

impl ShakeOptions {
    pub fn new(xy_cnf_save: Option<bool>) -> Self {
        ShakeOptions { xy_cnf_save }
    }
}

#[derive(Debug)]
pub struct SettingParams {
    val_arr: [u8; 5],
    ch: u8,
    xy: u8,
    light: u8,
    cfg: u8,
    lang: u8,
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
                tx_size: 60,
                run_speed: 128,
                tx_dist: 60,
                tx_point_time: 50,
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

    pub fn generate_command(
        &self,
        points: Vec<(u8, Vec<LaserPoint>)>,
        time: f32,
        options: LaserOptions,
        direction: u8,
        version: u8,
    ) -> Option<LaserCommand> {
        if points.is_empty() {
            return None;
        }

        let mut cmd = LaserCommand::new();
        let (new_points, se1, se2, offset) = self.generate_grid(
            points.iter().flat_map(|(_, p)| p.clone()).collect(),
            direction,
        );
        cmd.point_count = new_points.len() as u16;
        cmd.char_count = (new_points.len() / 2) as u16;
        cmd.version = self.to_hex(version as u16, 2);
        cmd.time = if options.text_decimal_time {
            self.to_hex((time * 10.0) as u16, 2)
        } else {
            self.to_hex(time as u16, 2)
        };
        cmd.points = new_points.clone();

        for (i, point) in new_points.iter().enumerate() {
            let frame = (i / 8) as u8;
            let flag = if i == 0 || i == new_points.len() - 1 { 1 } else { point.z };
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

    pub fn generate_ble_command(&self, points: &[LaserPoint]) -> Vec<Vec<u8>> {
        points.iter().map(|point| {
            vec![
                0xE0, 0xE1, 0xE2, 0xE3,
                0xC0, 0xC1, 0xC2, 0xC3,
                (point.x & 0xFF) as u8, (point.x >> 8) as u8,
                (point.y & 0xFF) as u8, (point.y >> 8) as u8,
                point.rgb[0], point.rgb[1], point.rgb[2],
                point.z,
                0x00, 0x00, 0x00, 0x00,
            ]
        }).collect()
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
                    p |= 50 - f.sel_index.unwrap_or(0);
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
            let frame = point.x as i32 / 100;
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
            127 => {
                let mut y_offset = 0;
                let mut grid_points = Vec::new();
                for _ in 0..9 {
                    let y = (y_sums.iter().sum::<f32>() / 2.0 + grid_step as f32 / 2.0 + y_offset as f32) as i16;
                    grid_points.push(LaserPoint::new(0, y, 0, [0, 0, 0]));
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
            128 | _ => {
                let mut x_offset = 0;
                let mut grid_points = Vec::new();
                for _ in 0..9 {
                    let x = (x_sums.iter().sum::<f32>() / 2.0 + (grid_step as f32) / 2.0 + x_offset as f32) as i16;
                    grid_points.push(LaserPoint::new(x, 0, 0, [0, 0, 0]));
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
}

pub struct LaserController {
    sending: Arc<Mutex<bool>>,
    ble_controller: Option<Box<dyn BlueController>>,
    dmx_port: Option<Box<dyn SerialPort>>,
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
            ble_controller: self.ble_controller.as_ref().map(|c| c.clone()), // Requires BlueController to implement Clone
            dmx_port: self.dmx_port.as_ref().map(|port| Box::new(port.try_clone().unwrap()) as Box<dyn SerialPort>),
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
    pub async fn new<T: BlueController + Clone + 'static>(ble_controller: T, dmx_port_path: &str) -> Result<Self, Box<dyn Error>> {
        let sending = Arc::new(Mutex::new(false));
        let dmx_port = serialport::new(dmx_port_path, 250_000)
            .data_bits(serialport::DataBits::Eight)
            .parity(serialport::Parity::None)
            .stop_bits(serialport::StopBits::Two)
            .open()?;
        let mut controller = LaserController {
            sending,
            ble_controller: Some(Box::new(ble_controller)),
            dmx_port: Some(dmx_port),
            last_send_time: None,
            options: LaserOptions::new(),
            project_data: ProjectData {
                public: PublicSettings { rd_mode: 0, sound_val: 0 },
                prj_item: Vec::new(),
            },
            connected: false,
            blu_rec_content: VecDeque::new(),
            rec_device_msg_timer: None,
            discovery_started: false,
            testing_idx: 0,
            not_pass: 0,
            pass_count: 0,
        };
        controller.connect().await?;
        controller.send_init_command().await?;
        Ok(controller)
    }

    async fn send_init_command(&mut self) -> Result<(), String> {
        let init_command = vec![
            0xE0, 0xE1, 0xE2, 0xE3,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ];
        self.send_ble(&init_command).await
    }

    pub async fn connect(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(controller) = &mut self.ble_controller {
            controller.connect().await?;
            self.connected = controller.is_connected();
        }
        Ok(())
    }

    pub async fn send(&mut self, cmd_hex: &str, callback: Option<&mut dyn FnMut(i8, u8)>) -> Result<(), String> {
        let mut guard = self.sending.lock().unwrap();
        if *guard {
            return Err("Previous send in progress".to_string());
        }
        *guard = true;

        if !self.is_connected() {
            *guard = false;
            return Err("Not connected".to_string());
        }

        let bytes = decode(cmd_hex).map_err(|e| e.to_string())?;
        if bytes.len() % 20 != 0 || !bytes.chunks(20).all(|chunk| chunk.starts_with(&[0xE0, 0xE1, 0xE2, 0xE3])) {
            *guard = false;
            return Err("Invalid command: must be 20-byte chunks starting with E0E1E2E3".to_string());
        }

        let mut cb_result: Option<(i8, u8)> = None;
        let mut cb = callback;
        if let Some(cb_ref) = cb.as_deref_mut() {
            cb_ref(0, 0);
        }

        let result = self.send_ble(&bytes).await;
        *guard = false;

        if let Some(cb_ref) = cb.as_deref_mut() {
            if result.is_ok() {
                cb_ref(1, 100);
            } else {
                cb_ref(-1, 0);
            }
        }
        result
    }

    pub async fn send_animation(&mut self, points: &[LaserPoint]) -> Result<(), String> {
        let generator = CommandGenerator::new();
        let commands = generator.generate_ble_command(points);
        for cmd in commands {
            if let Some(last_send) = self.last_send_time {
                let elapsed = Instant::now().duration_since(last_send);
                if elapsed < Duration::from_millis(20) {
                    sleep(Duration::from_millis(20) - elapsed).await;
                }
            }
            self.send_ble(&cmd).await?;
            self.last_send_time = Some(Instant::now());

            // Send to DMX
            if let Some(ref mut dmx_port) = self.dmx_port {
                let mut dmx_frame = [0u8; 512];
                dmx_frame[0] = cmd[12]; // Red (channel 1)
                dmx_frame[1] = cmd[13]; // Green (channel 2)
                dmx_frame[2] = cmd[14]; // Blue (channel 3)
                dmx_frame[3] = cmd[8]; // x low byte (channel 4)
                dmx_frame[4] = cmd[9]; // x high byte (channel 5)
                dmx_frame[5] = cmd[10]; // y low byte (channel 6)
                dmx_frame[6] = cmd[11]; // y high byte (channel 7)
                dmx_frame[7] = cmd[15]; // z (laser on/off, channel 8)
                dmx_port.write(&[0x00]).map_err(|e| e.to_string())?;
                dmx_port.write(&dmx_frame).map_err(|e| e.to_string())?;
            }
        }
        Ok(())
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

            if last_end > 0 && last_end == content.len() - 8 {
                let packet = content[last_start..last_end + 8].to_string();
                self.data_received(packet);
                processed = String::new();
            } else if last_start > 0 {
                processed = content[last_start..].to_string();
            }
            self.blu_rec_content.clear();
            self.blu_rec_content.push_back(processed);
        }
    }

    fn data_received(&mut self, packet: String) {
        if let Ok(bytes) = hex::decode(&packet) {
            if bytes.len() >= 16 && bytes.starts_with(&[0xE0, 0xE1, 0xE2, 0xE3]) {
                if let Some(ref mut dmx_port) = self.dmx_port {
                    let mut dmx_frame = [0u8; 512];
                    dmx_frame[0] = bytes[12]; // Red (channel 1)
                    dmx_frame[1] = bytes[13]; // Green (channel 2)
                    dmx_frame[2] = bytes[14]; // Blue (channel 3)
                    dmx_frame[3] = bytes[8]; // x low byte (channel 4)
                    dmx_frame[4] = bytes[9]; // x high byte (channel 5)
                    dmx_frame[5] = bytes[10]; // y low byte (channel 6)
                    dmx_frame[6] = bytes[11]; // y high byte (channel 7)
                    dmx_frame[7] = bytes[15]; // z (laser on/off, channel 8)
                    dmx_port.write(&[0x00]).unwrap_or_else(|e| error!("DMX break error: {}", e));
                    dmx_port.write(&dmx_frame).unwrap_or_else(|e| error!("DMX write error: {}", e));
                }
            }
        }
    }

    pub async fn cleanup(&mut self) -> Result<(), String> {
        if let Some(controller) = &mut self.ble_controller {
            controller.disconnect().await.map_err(|e| e.to_string())?;
        }
        self.ble_controller = None;
        self.dmx_port = None;
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
        .ok_or_else(|| "No matching string found".to_string())
}

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

fn process_selected(selected: &[u8]) -> Vec<u8> {
    let mut vec = selected.to_vec();
    vec.reverse();
    vec
}

fn adjust_selected(e: &[u8], t: Vec<u8>) -> Vec<u8> {
    t
}