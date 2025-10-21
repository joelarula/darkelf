
use std::collections::HashMap;
use serde::{Deserialize, Serialize};



pub const MAX_DRAW_POINT_COUNT: usize = 800;

/// Represents the available show/playback modes for the device.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlaybackMode {
    Dmx = 0,
    RandomPlayback = 1,
    LineGeometryPlayback = 2,
    AnimationPlayback = 3,
    TextPlayback = 4,
    ChristmasPlayback = 5,
    OutdoorPlayback = 6,
    Program = 7,
    Draw = 8,
   // Playlist = 9,
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub color: u8,
    pub pen_state: u8,  // 0=pen up/move, 1=pen down/draw
}

impl Point {
    pub fn new(x: f64, y: f64, color: u8, pen_state: u8) -> Self {
        Self { x, y, color, pen_state }
    }
    
    /// Create from drawPs2 output format: [x, y, color, pen_state]
    pub fn from_draw_ps2(arr: [f64; 4]) -> Self {
        Self {
            x: arr[0],
            y: arr[1], 
            color: arr[2] as u8,
            pen_state: arr[3] as u8,
        }
    }
    
    /// Create from JavaScript array format: [x, y, color, pen_state]
    pub fn from_js_array(x: f64, y: f64, color: f64, pen_state: f64) -> Self {
        Self {
            x,
            y,
            color: color as u8,
            pen_state: pen_state as u8,
        }
    }
    
    /// Convert to array format: [x, y, color, pen_state]
    pub fn to_array(&self) -> [f64; 4] {
        [self.x, self.y, self.color as f64, self.pen_state as f64]
    }
    

}


#[derive(Debug, Clone, Serialize, Deserialize)]
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



// cnf_valus[12] playback time configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PisObject {
    #[serde(rename = "txPointTime")]
    pub tx_point_time: u32,
    #[serde(rename = "cnfValus")]
    pub cnf_valus: [u32; 13],
}

impl Default for PisObject {
    fn default() -> Self {
        Self {
            tx_point_time: 50,
            cnf_valus: [0; 13],
        }
    }
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



/// Represents a single point in a drawing path with [x, y, color, pen_state] format
/// Serializes as a 4-element array to match JSON format: [x, y, color, pen_state]
#[derive(Debug, Clone, PartialEq)]
pub struct DrawPoint {
    pub x: f64,
    pub y: f64,
    pub color: u8,      // Color value (0-15)
    pub pen_state: u8,  // Pen state: 0=pen up/move, 1=pen down/draw
}

impl Serialize for DrawPoint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (self.x, self.y, self.color, self.pen_state).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for DrawPoint {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let (x, y, color, pen_state) = <(f64, f64, u8, u8)>::deserialize(deserializer)?;
        Ok(DrawPoint { x, y, color, pen_state })
    }
}

impl DrawPoint {
    pub fn new(x: f64, y: f64, color: u8, pen_state: u8) -> Self {
        Self { x, y, color, pen_state }
    }
    
    /// Create from a 4-element array [x, y, color, pen_state]
    pub fn from_array(arr: [f64; 4]) -> Self {
        Self {
            x: arr[0],
            y: arr[1], 
            color: arr[2] as u8,
            pen_state: arr[3] as u8,
        }
    }
    
    /// Convert to a 4-element array [x, y, color, pen_state]
    pub fn to_array(&self) -> [f64; 4] {
        [self.x, self.y, self.color as f64, self.pen_state as f64]
    }
}

/// Drawing modes that determine how the object is rendered
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrawMode {
    Polylines = -1,     // Multiple polyline paths
    Shape = 2,          // Generic shape (your example uses this)
    Text = 9999,        // Text rendering
}

impl From<i32> for DrawMode {
    fn from(value: i32) -> Self {
        match value {
            -1 => DrawMode::Polylines,
            9999 => DrawMode::Text,
            _ => DrawMode::Shape, // Default for other values like 2
        }
    }
}

impl From<DrawMode> for i32 {
    fn from(mode: DrawMode) -> Self {
        mode as i32
    }
}

impl Serialize for DrawMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i32(*self as i32)
    }
}

impl<'de> Deserialize<'de> for DrawMode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(DrawMode::from(value))
    }
}

/// Flexible point structure that can handle both simple points and nested polylines
#[derive(Debug, Clone)]
pub enum DrawPoints {
    /// Simple array of points for regular shapes/text
    Simple(Vec<DrawPoint>),
    /// Nested arrays for polylines mode
    Polylines(Vec<Vec<DrawPoint>>),
}

impl DrawPoints {
    /// Get all points as a flattened vector for processing
    pub fn flatten(&self) -> Vec<DrawPoint> {
        match self {
            DrawPoints::Simple(points) => points.clone(),
            DrawPoints::Polylines(polylines) => {
                polylines.iter().flat_map(|polyline| polyline.iter()).cloned().collect()
            }
        }
    }
    
    /// Get the total number of points
    pub fn len(&self) -> usize {
        match self {
            DrawPoints::Simple(points) => points.len(),
            DrawPoints::Polylines(polylines) => polylines.iter().map(|p| p.len()).sum(),
        }
    }
    
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Custom serialization/deserialization for DrawPoints
mod flexible_points {
    use super::{DrawPoints, DrawPoint};
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use serde_json::Value;

    pub fn serialize<S>(points: &DrawPoints, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match points {
            DrawPoints::Simple(points) => points.serialize(serializer),
            DrawPoints::Polylines(polylines) => polylines.serialize(serializer),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DrawPoints, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        
        if let Value::Array(arr) = value {
            if arr.is_empty() {
                return Ok(DrawPoints::Simple(vec![]));
            }
            
            // Check the first element to determine structure
            match &arr[0] {
                // If first element is an array, check if it's nested (polylines) or flat (simple points)
                Value::Array(inner_arr) => {
                    // Check if this is nested arrays (polylines) or flat point arrays (simple)
                    if !inner_arr.is_empty() && matches!(inner_arr[0], Value::Array(_)) {
                        // This is polylines - nested arrays like [[[x,y,c,p], [x,y,c,p]], [[x,y,c,p]]]
                        let mut polylines = Vec::new();
                        for polyline_value in arr {
                            if let Value::Array(polyline_arr) = polyline_value {
                                let mut polyline_points = Vec::new();
                                for point_value in polyline_arr {
                                    if let Value::Array(point_arr) = point_value {
                                        if point_arr.len() == 4 {
                                            let x = point_arr[0].as_f64().unwrap_or(0.0);
                                            let y = point_arr[1].as_f64().unwrap_or(0.0);
                                            let color = point_arr[2].as_u64().unwrap_or(0) as u8;
                                            let pen_state = point_arr[3].as_u64().unwrap_or(0) as u8;
                                            polyline_points.push(DrawPoint { x, y, color, pen_state });
                                        }
                                    }
                                }
                                polylines.push(polyline_points);
                            }
                        }
                        Ok(DrawPoints::Polylines(polylines))
                    } else {
                        // This is simple points - flat array like [[x,y,c,p], [x,y,c,p]]
                        let mut points = Vec::new();
                        for point_value in arr {
                            if let Value::Array(point_arr) = point_value {
                                if point_arr.len() == 4 {
                                    let x = point_arr[0].as_f64().unwrap_or(0.0);
                                    let y = point_arr[1].as_f64().unwrap_or(0.0);
                                    let color = point_arr[2].as_u64().unwrap_or(0) as u8;
                                    let pen_state = point_arr[3].as_u64().unwrap_or(0) as u8;
                                    points.push(DrawPoint { x, y, color, pen_state });
                                }
                            }
                        }
                        Ok(DrawPoints::Simple(points))
                    }
                }
                // If first element is a number, this is simple points format
                Value::Number(_) => {
                    // This is simple points - flat array of [x, y, color, pen_state] arrays
                    let mut points = Vec::new();
                    for point_value in arr {
                        if let Value::Array(point_arr) = point_value {
                            if point_arr.len() == 4 {
                                let x = point_arr[0].as_f64().unwrap_or(0.0);
                                let y = point_arr[1].as_f64().unwrap_or(0.0);
                                let color = point_arr[2].as_u64().unwrap_or(0) as u8;
                                let pen_state = point_arr[3].as_u64().unwrap_or(0) as u8;
                                points.push(DrawPoint { x, y, color, pen_state });
                            }
                        }
                    }
                    Ok(DrawPoints::Simple(points))
                }
                _ => {
                    // Fallback to empty simple points
                    Ok(DrawPoints::Simple(vec![]))
                }
            }
        } else {
            Ok(DrawPoints::Simple(vec![]))
        }
    }
}

/// Represents a single drawable object with geometry and transformation parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrawItem {
    /// Drawing points - can be either simple points or nested polylines depending on draw_mode
    #[serde(with = "flexible_points")]
    pub ps: DrawPoints,
    
    /// X-axis translation offset
    pub x0: f64,
    
    /// Y-axis translation offset  
    pub y0: f64,
    
    /// Scale factor (z-axis or zoom)
    pub z: f64,
    
    /// Drawing mode that determines rendering method
    #[serde(rename = "drawMode")]
    pub draw_mode: DrawMode,
    
    /// Rotation angle in degrees
    pub ang: f64,
    
    /// Line color value (0-15 for direct colors, >=8 for special color modes)
    #[serde(rename = "lineColor")]
    pub line_color: u8,
}

impl DrawItem {
    pub fn new() -> Self {
        Self {
            ps: DrawPoints::Simple(Vec::new()),
            x0: 0.0,
            y0: 0.0,
            z: 1.0,
            draw_mode: DrawMode::Shape,
            ang: 0.0,
            line_color: 1,
        }
    }
    
    /// Add a point to the drawing path
    pub fn add_point(&mut self, point: DrawPoint) {
        match &mut self.ps {
            DrawPoints::Simple(points) => points.push(point),
            DrawPoints::Polylines(_) => {
                // Convert to simple and add point
                let mut flattened = self.ps.flatten();
                flattened.push(point);
                self.ps = DrawPoints::Simple(flattened);
            }
        }
    }
    
    /// Get all points as a flattened vector
    pub fn get_all_points(&self) -> Vec<DrawPoint> {
        self.ps.flatten()
    }
}

impl Default for DrawItem {
    fn default() -> Self {
        Self::new()
    }
}



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrawData {

    #[serde(rename = "drawPoints")]
    pub draw_points: Vec<DrawItem>,
    
    #[serde(rename = "pisObj")]
    pub pis_obj: PisObject,
    
}

impl DrawData {
    pub fn new() -> Self {
        Self {
            draw_points: Vec::new(),
            pis_obj: PisObject::default(),
        }
    }
    
    pub fn add_draw_object(&mut self, obj: DrawItem) {
        self.draw_points.push(obj);
    }
}

impl Default for DrawData {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct DmxLaserState {
    pub master_dimmer: u8,      // CH1: 0-9=OFF, 10-255=ON
    pub color_control: u8,      // CH2: Color selection and modes
    pub color_speed: u8,        // CH3: Color change speed and direction
    pub pattern_group: u8,      // CH4: Pattern group selection
    pub pattern_select: u8,     // CH5: Individual pattern within group
    pub dynamic_effects: u8,    // CH6: Dynamic effects and random play
    pub effect_speed: u8,       // CH7: Effect speed control
    pub pattern_size: u8,       // CH8: Pattern size
    pub size_control: u8,       // CH9: Advanced size control modes
    pub rotation: u8,           // CH10: Rotation angle and speed
    pub vertical_flip: u8,      // CH11: Vertical flip position/speed
    pub horizontal_flip: u8,    // CH12: Horizontal flip position/speed
    pub horizontal_pos: u8,     // CH13: Horizontal position/movement
    pub vertical_pos: u8,       // CH14: Vertical position/movement
    pub wave_effect: u8,        // CH15: Wave amplitude and speed
    pub manual_drawing: u8,     // CH16: Manual drawing modes
}

impl Default for DmxLaserState {
    fn default() -> Self {
        Self {
            master_dimmer: 0,      // Light OFF
            color_control: 0,      // White
            color_speed: 0,        // No color change
            pattern_group: 12,     // Static group 1
            pattern_select: 0,     // First pattern
            dynamic_effects: 0,    // No function
            effect_speed: 128,     // Medium speed
            pattern_size: 128,     // Medium size
            size_control: 7,       // Basic size selection
            rotation: 64,          // Center rotation
            vertical_flip: 64,     // Center vertical
            horizontal_flip: 64,   // Center horizontal
            horizontal_pos: 127,   // Center horizontal position
            vertical_pos: 127,     // Center vertical position
            wave_effect: 0,        // No wave
            manual_drawing: 0,     // No manual drawing
        }
    }
}
