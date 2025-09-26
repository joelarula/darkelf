use std::collections::HashMap;

/// Represents the available show/playback modes for the device.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlaybackMode {
    Dmx = 0,
    RandomPlayback = 1,
    TimelinePlayback = 2,
    AnimationPlayback = 3,
    TextPlayback = 4,
    ChristmasBroadcast = 5,
    //Ilda = 5,
    OutdoorPlayback = 6,
    PersonalizedProgramming = 7,
    HandDrawnDoodle = 8,
    Playlist = 9,
}


#[derive(Debug, Clone)]
pub struct DeviceInfo {
    pub device_on: bool,
    pub device_type: String,
    pub version: String,
    pub user_type: String,
}

#[derive(Debug, Clone)]
pub struct FeatureConfig {
    pub play_time: f32,
    pub config_values: Vec<u8>,
}

#[derive(Debug, Clone, Default)]
pub struct DrawConfig {
    pub config_values: Vec<u8>,
    pub text_point_time: u8,
}


#[derive(Debug, Clone, Default)]
pub struct MainCommandData {
    pub current_mode: u8,
    pub project_index: u8,
    pub text_color: u8,
    pub text_size: u8, // text size 10 - 100
    pub run_speed: u8, // speed 0 - 100
    pub text_distance: u8, // text distance 10 - 100
    pub read_mode: u8,
    pub sound_value: u8,
    pub text_point_time: u8,
    pub draw_point_time: u8,
    pub run_direction: u8,
}

#[derive(Debug, Clone)]
pub struct SettingsData {
    pub values: [u16; 5],  // [channel, display_range, r, g, b]
    pub channel: u8,       // DMX 
    pub dmx: u8,           // 0 or 1
    pub xy: u8,            // Normal: X+Y+ X+Y- X-Y- X-Y+ Interchange: X+Y+ X+Y- X-Y- X-Y+ (0-7)
    pub light: u8,         // 1=single, 2=dual, 3=full
    pub cfg: u8,           // 0=ttl, 255=analog
    pub lang: String,      // Language setting
}


#[derive(Debug, Clone)]
pub struct DeviceResponse {
    pub main_data: MainCommandData,
    pub settings: SettingsData,
    pub features: Vec<FeatureConfig>,
    pub draw_config: DrawConfig,
    pub device_info: Option<DeviceInfo>,
}


impl Default for SettingsData {
    fn default() -> Self {
        Self {
            values: [1, 10, 255, 255, 255], // [channel, display_range, r, g, b]
            channel: 0,  // DMX channel
            dmx: 0,     // Default to TTL mode
            xy: 0,      // Default to normal X+Y+
            light: 3,   // Default to full mode
            cfg: 0,     // Default to TTL
            lang: "en".to_string(), // Default to English
        }
    }
}



// Data structures needed by the trait methods
#[derive(Debug, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: i32,
    pub color: u8,
}

#[derive(Debug)]
pub struct LayoutItem {
    pub xys: Vec<Vec<f64>>,
    pub time: f64,
    pub xys_right: Option<Vec<Vec<f64>>>,
    pub xys_up: Option<Vec<Vec<f64>>>,
    pub xys_down: Option<Vec<Vec<f64>>>,
}

#[derive(Debug)]
pub struct Features {
    pub features: HashMap<String, bool>,
    pub group_list: Option<Vec<ColorGroup>>,
    pub prj_parm: Option<ProjectParams>,
    pub xy_cnf_save: Option<bool>,
}

#[derive(Debug)]
pub struct ColorGroup {
    pub color: u8,
}

#[derive(Debug)]
pub struct ProjectParams {
    pub prj_index: i32,
    pub sel_index: i32,
}

#[derive(Debug)]
pub struct CommandConfig {
    pub cur_mode: i32,
    pub text_data: TextData,
    pub prj_data: ProjectData,
}

#[derive(Debug)]
pub struct TextData {
    pub tx_color: u8,
    pub tx_size: f64,
    pub run_speed: f64,
    pub tx_dist: f64,
    pub tx_point_time: u8,
    pub run_dir: u8,
}

#[derive(Debug)]
pub struct ProjectData {
    pub public: PublicData,
    pub prj_item: HashMap<i32, ProjectItem>,
}

#[derive(Debug)]
pub struct PublicData {
    pub rd_mode: u8, // audio trigger mode
    pub sound_val: f64, // sound sensitivity
}

#[derive(Debug)]
pub struct ProjectItem {
    pub py_mode: i32, //  playBackMode  0 : 128;
    pub prj_selected: Vec<u16>, // selected show
}

// Struct moved to top-level definition

#[derive(Debug)]
pub struct ShakeConfig {
    pub subset_data: SubsetData,
}

#[derive(Debug)]
pub struct SubsetData {
    pub xy_cnf: XYConfig,
}

#[derive(Debug)]
pub struct XYConfig {
    pub auto: bool,
    pub auto_value: u8,
    pub phase: u8,
    pub xy: Vec<XYValue>,
}

#[derive(Debug)]
pub struct XYValue {
    pub value: u8,
}

#[derive(Debug)]
pub struct PisConfig {
    pub cnf_valus: Vec<u8>,
    pub play_time: f64,
}



/// Represents a color option from the colorDisplayOrder array.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DisplayColor {
    Red,
    Yellow,
    Green,
    Cyan,
    Blue,
    Purple,
    White,
    Jump,
    RGB,
}

impl DisplayColor {
    /// Returns the display name for the color.
    pub fn name(&self) -> &'static str {
        match self {
            DisplayColor::Red => "Red",
            DisplayColor::Yellow => "yellow",
            DisplayColor::Green => "green",
            DisplayColor::Cyan => "Cyan",
            DisplayColor::Blue => "blue",
            DisplayColor::Purple => "purple",
            DisplayColor::White => "white",
            DisplayColor::Jump => "Jump",
            DisplayColor::RGB => "RGB",
        }
    }

    /// Returns the color value as a string (CSS color or hex code).
    pub fn color(&self) -> &'static str {
        match self {
            DisplayColor::Red => "red",
            DisplayColor::Yellow => "yellow",
            DisplayColor::Green => "green",
            DisplayColor::Cyan => "#00FFFF",
            DisplayColor::Blue => "blue",
            DisplayColor::Purple => "purple",
            DisplayColor::White => "white",
            DisplayColor::Jump => "transparent",
            DisplayColor::RGB => "transparent",
        }
    }

    /// Returns the idx value for the color.
    pub fn idx(&self) -> u8 {
        match self {
            DisplayColor::Red => 1,
            DisplayColor::Yellow => 4,
            DisplayColor::Green => 2,
            DisplayColor::Cyan => 5,
            DisplayColor::Blue => 3,
            DisplayColor::Purple => 6,
            DisplayColor::White => 7,
            DisplayColor::Jump => 8,
            DisplayColor::RGB => 9,
        }
    }
}