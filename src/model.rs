
use std::collections::HashMap;

pub const MAX_DRAW_POINT_COUNT: usize = 800;

/// Represents the available show/playback modes for the device.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlaybackMode {
    Dmx = 0,
    RandomPlayback = 1,
    LineGeometryPlayback = 2,
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
    pub text_color: u8,
    pub text_size: u8, // text size 10 - 100
    pub run_speed: u8, // speed 0 - 100
    pub text_distance: u8, // text distance 10 - 100
    pub audio_mode: u8,
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
    pub light: u8,         // 1=single, 2=dual, 3=full rgb
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
    pub prj_data: Option<ProjectData>,
    pub pis_obj: Option<PisObject>,
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

#[derive(Debug, Clone)]
pub struct PlaybackCommand {
    pub mode: PlaybackMode,
    pub color: Option<u8>,
    pub audio_mode: Option<bool>,
    pub audio_sensitivity: Option<u8>,
    pub playback_speed: Option<u8>,
    pub tick_playback: Option<bool>,
    pub selected_shows: Option<Vec<u8>>,
}

impl PlaybackCommand {
    pub fn default(mode: PlaybackMode) -> Self {
        PlaybackCommand {
            mode,
            color: Some(DisplayColor::RGB as u8 ),
            audio_mode: None,
            audio_sensitivity: None,
            playback_speed: None,
            tick_playback: None,
            selected_shows: None,
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
    pub cur_mode: u8,
    pub text_data: TextData,
    pub prj_data: ProjectData,
}

#[derive(Debug)]
pub struct TextData {
    pub tx_color: u8,
    pub tx_size: u8,
    pub run_speed: u8,
    pub tx_dist: u8,
    pub tx_point_time: u8,
    pub run_dir: u8,
}

#[derive(Debug, Clone)]
pub struct ProjectData {
    pub public: PublicData,
    pub prj_item: HashMap<i32, ProjectItem>,
}

#[derive(Debug, Clone)]
pub struct PublicData {
    pub rd_mode: u8, // audio trigger mode
    pub sound_val: u8, // sound sensitivity
}

#[derive(Debug, Clone)]
pub struct ProjectItem {
    pub py_mode: u8, //  playBackMode  0 : 128;
    pub prj_selected: Vec<u16>, // selected shows
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

// cnf_valus[12] playback time configuration.
#[derive(Debug, Clone)]
pub struct PisObject {
    pub tx_point_time: u32,
    pub cnf_valus: [u32; 13],
}


/// Represents a color option from the colorDisplayOrder array.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayColor {
    Red = 1,
    Yellow = 4,
    Green = 2,
    Cyan = 5,
    Blue = 3,
    Purple = 6,
    White = 7,
    Jump = 8,
    RGB = 9,
}

impl DisplayColor {
    pub fn from_u8(val: u8) -> Option<Self> {
        match val {
            1 => Some(DisplayColor::Red),
            2 => Some(DisplayColor::Green),
            3 => Some(DisplayColor::Blue),
            4 => Some(DisplayColor::Yellow),
            5 => Some(DisplayColor::Cyan),
            6 => Some(DisplayColor::Purple),
            7 => Some(DisplayColor::White),
            8 => Some(DisplayColor::Jump),
            9 => Some(DisplayColor::RGB),
            _ => None,
        }
    }

    pub fn value(&self) -> u8 {
        *self as u8
    }
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


#[derive(Debug, Clone)]
pub enum PisObjNote {
    PatternSelection(Vec<(u16, &'static str)>),
    PatternType(Vec<(u16, &'static str)>),
    Color(Vec<(u16, &'static str)>),
    WaterFlow(Vec<(u16, &'static str)>),
    PatternSize(Vec<(u16, &'static str)>),
    Expansion(Vec<(u16, &'static str)>),
    Rotation(Vec<(u16, &'static str)>),
    WaterSurfaceRotation(Vec<(u16, &'static str)>),
    VerticalRotation(Vec<(u16, &'static str)>),
    WaterSurfaceMotion(Vec<(u16, &'static str)>),
    VerticalMotion(Vec<(u16, &'static str)>),
    WaterWidth(Vec<(u16, &'static str)>),
}

impl PisObjNote {
    pub fn get_notes(category: usize) -> Option<PisObjNote> {
        match category {
            0 => Some(PisObjNote::PatternSelection(vec![
                (256, "Pattern Selection"),
            ])),
            1 => Some(PisObjNote::PatternType(vec![
                (25, "Straight Line Pattern"),
                (25, "Arc Pattern"),
                (25, "Bright Spot Pattern"),
                (25, "Dot Pattern"),
                (25, "Christmas Pattern"),
                (25, "Animation Group 1"),
                (25, "Animation Group 2"),
                (25, "Animation Group 3"),
                (25, "Animation Group 4"),
                (31, "Animation Group 5"),
            ])),
            2 => Some(PisObjNote::Color(vec![
                (10, "White"),
                (10, "Red"),
                (10, "Blue"),
                (10, "Pink"),
                (10, "Cyan"),
                (10, "Yellow"),
                (10, "Green"),
                (10, "Overall Color Change"),
                (13, "Rainbow Colors"),
                (18, "2-Segment Color"),
                (21, "3-Segment Color"),
                (18, "4-Segment Color"),
                (33, "8-Segment Color"),
                (36, "16-Segment Color"),
                (35, "32-Segment Color"),
                (2, "Color Gradient Drawing"),
            ])),
            3 => Some(PisObjNote::WaterFlow(vec![
                (10, "Non-Flowing Water"),
                (118, "Forward Flowing Water"),
                (128, "Reverse Flowing Water"),
            ])),
            4 => Some(PisObjNote::PatternSize(vec![
                (256, "Pattern Size"),
            ])),
            5 => Some(PisObjNote::Expansion(vec![
                (16, "Expand Manual Selection"),
                (40, "From Small to Large Expansion"),
                (40, "From Large to Small Expansion"),
                (40, "Large to Small Expansion"),
                (120, "Preview No Function"),
            ])),
            6 => Some(PisObjNote::Rotation(vec![
                (128, "Rotation Angle"),
                (64, "Forward Rotation Speed"),
                (64, "Reverse Rotation Speed"),
            ])),
            7 => Some(PisObjNote::WaterSurfaceRotation(vec![
                (128, "Water Surface Rotation Position"),
                (128, "Water Surface Rotation Speed"),
            ])),
            8 => Some(PisObjNote::VerticalRotation(vec![
                (128, "Vertical Rotation Position"),
                (128, "Vertical Rotation Speed"),
            ])),
            9 => Some(PisObjNote::WaterSurfaceMotion(vec![
                (128, "Water Surface Rotation"),
                (128, "Water Surface Movement"),
            ])),
            10 => Some(PisObjNote::VerticalMotion(vec![
                (128, "Vertical Rotation Position"),
                (128, "Vertical Rotation Movement"),
            ])),
            11 => Some(PisObjNote::WaterWidth(vec![
                (2, "Non-Flowing Water"),
                (31, "Flowing Water Width 1"),
                (32, "Flowing Water Width 2"),
                (32, "Flowing Water Width 3"),
                (32, "Flowing Water Width 4"),
                (32, "Flowing Water Width 5"),
                (32, "Flowing Water Width 6"),
                (32, "Flowing Water Width 7"),
                (31, "Flowing Water Width 8"),
            ])),
            _ => None,
        }
    }
}