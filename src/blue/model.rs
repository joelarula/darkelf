

use std::{convert::TryFrom};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

pub const MAX_DRAW_POINT_COUNT: usize = 800;

/// Represents the available show/playback modes for the device.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash,Default)]
pub enum DeviceMode {
    Dmx = 0,
    #[default]
    RandomPlayback = 1,
    LineGeometryPlayback = 2,
    AnimationPlayback = 3,
    TextPlayback = 4,
    ChristmasPlayback = 5,
    OutdoorPlayback = 6,
    Program = 7,
    Draw = 8,
    Playlist = 9,
}


impl TryFrom<u8> for DeviceMode {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(DeviceMode::Dmx),
            1 => Ok(DeviceMode::RandomPlayback),
            2 => Ok(DeviceMode::LineGeometryPlayback),
            3 => Ok(DeviceMode::AnimationPlayback),
            4 => Ok(DeviceMode::TextPlayback),
            5 => Ok(DeviceMode::ChristmasPlayback),
            6 => Ok(DeviceMode::OutdoorPlayback),
            7 => Ok(DeviceMode::Program),
            8 => Ok(DeviceMode::Draw),
            _ => Ok(DeviceMode::RandomPlayback), // fallback to default
        }
    }
}

#[derive(Debug, Clone,Default,PartialEq)]
pub struct DeviceInfo {
    pub device_on: bool,
    pub device_type: String,
    pub version: String,
    pub user_type: String,
}



#[derive(Debug, Clone,Default)]
pub struct DeviceFeatures{
    pub text_stop_time: bool,
    pub text_decimal_time: bool,
    pub display_type: u8,
    pub show_outdoor_tips: bool,
    pub xy_cnf: bool,
    pub arb_play: bool,
    pub ilda: bool,
    pub ttl_an: bool,
    pub pics_play: bool,
    pub text_up_down: bool,
    pub animation_fix: bool
}




#[derive(Debug, Clone, Default,PartialEq)]
pub struct MainCommandData {
    pub device_mode: DeviceMode,
    pub audio_trigger_mode: u8,
    pub color: BeamColor, // 0: Black 1: Red 2: Green 3: Blue 4: Yellow 5: Magenta 6: Cyan 7: White 8: Orange 9: RGB
    pub text_size_x: u8, // text size x 10 - 100
    pub text_size_y: u8, // text size y 10 - 100
    pub run_speed: u8, // speed 0 - 100
    pub filler: u8, // extra byte filler
    pub text_distance: u8, // text distance 10 - 100
    pub audio_mode: u8,
    pub sound_value: u8,
    pub text_point_time: u8,
    pub draw_point_time: u8,
    pub run_direction: u8,
    pub playback: PlaybackData,
}

#[derive(Debug, Clone,PartialEq)]
pub struct DeviceSettings {
    pub proto : u8,        // Protocol value
    pub display_range : u8,  // Projection angle 10 - 100. 6.86 - 61.92 aproxim
    pub red_beam : u8,     // Red beam 0 ~ 255
    pub green_beam : u8,   // Green beam 0 ~ 255
    pub blue_beam : u8,    // Blue beam 0 ~ 255
    pub dmx_channel: u8,   // DMX  channel
    pub xy: u8,            // Normal: X+Y+ X+Y- X-Y- X-Y+ Interchange: X+Y+ X+Y- X-Y- X-Y+ (0-7)
    pub beams: u8,         // 1=single, 2=dual, 3=full rgb
    pub ttl_or_analog: u8, // 0=ttl, 255=analog
}


#[derive(Debug, Clone,Default)]
pub struct DeviceState {
    pub settings: DeviceSettings,
    pub main_data: MainCommandData,
    pub device_info: DeviceInfo, 
    pub features: DeviceFeatures,
    pub features_config: Vec<DrawConfig>,
    pub draw_data: Option<DrawConfig>,
}


impl Default for DeviceSettings {
    fn default() -> Self {
        Self {
            dmx_channel: 1,  
            xy: 0,     
            beams: 3,   
            ttl_or_analog: 0,
            proto: 0,
            display_range: 45,
            red_beam: 255,
            green_beam: 255,
            blue_beam: 255,     
        }
    }
}

// Data structures needed by the trait methods
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub color: BeamColor,
    pub pen_state: u8,  // 
}


#[derive(Debug, Clone, serde::Serialize)]
pub struct PolylineData {
    pub lines: Vec<Vec<Point>>, 
    pub w: f32,
    pub h: f32,
}

#[derive(Debug, Clone)]
pub struct MirroredPolylines {
    pub new_lines_up: Vec<Vec<Point>>,
    pub new_lines_down: Vec<Vec<Point>>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct EncodedCommandData {
    pub cnt: usize,
    pub char_count: usize,
    pub cmd: String,
    pub char_width_cmd: String,
    pub char_point_cmd: String,
    pub se1: String,
    pub se2: String,
    pub ver: String,
    pub time: String,
}

#[derive(Debug, Clone)]
pub struct PathCommand {
    pub cmd_type: char, // 'M', 'L', 'Q', 'Z'
    pub x: f32,
    pub y: f32,
    pub x1: Option<f32>, // for 'Q'
    pub y1: Option<f32>, // for 'Q'
}


impl Point {
    pub fn new(x: f64, y: f64, color: BeamColor, pen_state: u8) -> Self {
        Self { x, y, color, pen_state }
    }
    
    /// Create from drawPs2 output format: [x, y, color, pen_state]
    pub fn from_draw_ps2(arr: [f64; 4]) -> Self {
        Self {
            x: arr[0],
            y: arr[1], 
            color: BeamColor::try_from(arr[2] as u8).unwrap_or(BeamColor::Blank),
            pen_state: arr[3] as u8,
        }
    }
    
    /// Create from JavaScript array format: [x, y, color, pen_state]
    pub fn from_js_array(x: f64, y: f64, color: f64, pen_state: f64) -> Self {
        Self {
            x,
            y,
            color: BeamColor::try_from(color as u8).unwrap_or(BeamColor::Blank),
            pen_state: pen_state as u8,
        }
    }
    
    /// Convert to array format: [x, y, color, pen_state]
    pub fn to_array(&self) -> [f64; 4] {
        [self.x, self.y, self.color as u8 as f64, self.pen_state as f64]
    }
    

}

#[derive(Debug, Clone,Default,PartialEq)]
pub struct PlaybackData {
    pub playback_items: HashMap<u8, Playback>,
}


#[derive(Debug, Clone,PartialEq)]
pub struct Playback {
    pub playback_mode: PlaybackMode,
    pub selected_play: u16,
    pub selected_plays: Vec<u16>,
}


pub struct TextPlayback {
    pub text: String, // text content
    pub time: u16, // seconds
    pub color: BeamColor, // color
}


// cnf_valus[12] playback time configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrawCommandData {
    #[serde(rename = "txPointTime")]
    pub tx_point_time: u32,
    #[serde(rename = "cnfValus")]
    pub cnf_valus: [u32; 13],
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DrawConfig {
    pub group_index: u8, // Graphics Group Selection" 1
    pub pattern_index: u8, //Pattern Selection 
    pub color: u8, // Color
    pub color_flow_speed: u8, // Color Change Speed"  
    pub pattern_size: u8, // Pattern Size 
    pub pattern_scale: u8, // Size scale 
    pub pattern_rotation: u8, // Rotation 
    pub pattern_vertical_flip: u8, // Vertical Flip 
    pub pattern_horizontal_flip: u8, // Horizontal Flip  
    pub pattern_horizontal_position: u8, // Horizontal Position 
    pub pattern_vertical_position: u8, //Pattern Position  
    pub pattern_wave: u8, // Wave     
    pub gradient_draw: u8, // Wave    
    pub play_time: u8, // Time Control
}

impl Default for DrawConfig {
    fn default() -> Self {
        Self {
            group_index: 0,
            pattern_index: 0,
            color: 0,
            color_flow_speed: 0,
            pattern_size: 0,
            pattern_scale: 0,
            pattern_rotation: 0,
            pattern_vertical_flip: 0,
            pattern_horizontal_flip: 0,
            pattern_horizontal_position: 0,
            pattern_vertical_position: 0,
            pattern_wave: 0,
            gradient_draw: 0,
            play_time: 0,
        }
    }
}

impl DrawConfig {
    /// Converts a DrawCommandData to a DrawConfig by mapping cnf_valus and tx_point_time.
    pub fn from_draw_command_data(cmd: &DrawCommandData) -> Self {
        // Build a [u8; 14] array from the 13 cnf_valus and tx_point_time
        let mut config_values = [0u8; 14];
        for i in 0..13 {
            config_values[i] = cmd.cnf_valus[i] as u8;
        }
        config_values[13] = cmd.tx_point_time as u8;
        Self::from_config_values(&config_values)
    }
    /// Converts this DrawConfig to a DrawCommandData (fills cnf_valus and tx_point_time)
    pub fn to_draw_command_data(&self) -> DrawCommandData {
        let config_values = self.to_config_values();
        let mut cnf_valus = [0u32; 13];
        for i in 0..13 {
            cnf_valus[i] = config_values[i] as u32;
        }
        DrawCommandData {
            tx_point_time: self.play_time as u32,
            cnf_valus,
        }
    }
    /// Converts the DrawConfig fields back to a [u8; 14] array, matching the order used in from_config_values.
    pub fn to_config_values(&self) -> [u8; 14] {
        [
            self.group_index,
            self.pattern_index,
            self.color,
            self.color_flow_speed,
            self.pattern_size,
            self.pattern_scale,
            self.pattern_rotation,
            self.pattern_vertical_flip,
            self.pattern_horizontal_flip,
            self.pattern_horizontal_position,
            self.pattern_vertical_position,
            self.pattern_wave,
            self.gradient_draw,
            self.play_time,
        ]
    }
    pub fn from_config_values( config_values: &[u8; 14]) -> Self {
        Self { 
            group_index: config_values[0],
            pattern_index: config_values[1],
            color: config_values[2],
            color_flow_speed: config_values[3],
            pattern_size: config_values[4],
            pattern_scale: config_values[5],
            pattern_rotation: config_values[6],
            pattern_vertical_flip: config_values[7],
            pattern_horizontal_flip: config_values[8],
            pattern_horizontal_position: config_values[9],
            pattern_vertical_position: config_values[10],
            pattern_wave: config_values[11],
            gradient_draw: config_values[12],
            play_time:  config_values[13]
        }
    }
}


impl Default for DrawCommandData {
    fn default() -> Self {
        Self {
            tx_point_time: 55,
            cnf_valus: [0; 13],
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum BeamColor {
    Blank = 0,
    Red = 1,
    Yellow = 4,
    Green = 2,
    Cyan = 5,
    Blue = 3,
    Purple = 6,
    White = 7,
    Jump = 8,
    #[default]
    RGB = 9,
}

impl TryFrom<u8> for BeamColor {
    type Error = ();
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(BeamColor::Blank),
            1 => Ok(BeamColor::Red),
            2 => Ok(BeamColor::Green),
            3 => Ok(BeamColor::Blue),
            4 => Ok(BeamColor::Yellow),
            5 => Ok(BeamColor::Cyan),
            6 => Ok(BeamColor::Purple),
            7 => Ok(BeamColor::White),
            8 => Ok(BeamColor::Jump),
            9 => Ok(BeamColor::RGB),
            _ => Err(()),
        }
    }
}


impl BeamColor {
    pub fn from_u8(val: u8) -> Option<Self> {
        match val {
            0 => Some(BeamColor::Blank),
            1 => Some(BeamColor::Red),
            2 => Some(BeamColor::Green),
            3 => Some(BeamColor::Blue),
            4 => Some(BeamColor::Yellow),
            5 => Some(BeamColor::Cyan),
            6 => Some(BeamColor::Purple),
            7 => Some(BeamColor::White),
            8 => Some(BeamColor::Jump),
            9 => Some(BeamColor::RGB),
            _ => None,
        }
    }

    pub fn value(&self) -> u8 {
        *self as u8
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq,Default)]
pub enum PlaybackMode {
    #[default]
    LoopPlay = 0,
    TickPlay = 128
}

impl std::convert::TryFrom<u8> for PlaybackMode {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PlaybackMode::LoopPlay),
            128 => Ok(PlaybackMode::TickPlay),
            _ => Ok(PlaybackMode::TickPlay),
        }
    }
}


impl BeamColor {
    /// Returns the display name for the color.
    pub fn name(&self) -> &'static str {
        match self {
            BeamColor::Blank => "Blank",
            BeamColor::Red => "Red",
            BeamColor::Yellow => "yellow",
            BeamColor::Green => "green",
            BeamColor::Cyan => "Cyan",
            BeamColor::Blue => "blue",
            BeamColor::Purple => "purple",
            BeamColor::White => "white",
            BeamColor::Jump => "Jump",
            BeamColor::RGB => "RGB",
        }
    }

    /// Returns the color value as a string (CSS color or hex code).
    pub fn color(&self) -> &'static str {
        match self {
            BeamColor::Blank => "black",
            BeamColor::Red => "red",
            BeamColor::Yellow => "yellow",
            BeamColor::Green => "green",
            BeamColor::Cyan => "#00FFFF",
            BeamColor::Blue => "blue",
            BeamColor::Purple => "purple",
            BeamColor::White => "white",
            BeamColor::Jump => "Jump",
            BeamColor::RGB => "RGB",
        }
    }

    /// Returns the idx value for the color.
    pub fn idx(&self) -> u8 {
        match self {
            BeamColor::Blank => 0,
            BeamColor::Red => 1,
            BeamColor::Yellow => 4,
            BeamColor::Green => 2,
            BeamColor::Cyan => 5,
            BeamColor::Blue => 3,
            BeamColor::Purple => 6,
            BeamColor::White => 7,
            BeamColor::Jump => 8,
            BeamColor::RGB => 9,
        }
    }
}





impl DrawCommandData{

    /// Converts this DrawCommandData to a DrawConfig by mapping cnf_valus and tx_point_time.
    pub fn to_draw_config(&self) -> DrawConfig {
        let mut config_values = [0u8; 14];
        for i in 0..13 {
            config_values[i] = self.cnf_valus[i] as u8;
        }
        config_values[13] = self.tx_point_time as u8;
        DrawConfig::from_config_values(&config_values)
    }

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrawData {
    pub points: Vec<Point>,
    pub config: DrawConfig,
}




