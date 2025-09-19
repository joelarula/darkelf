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
    pub text_size: u8,
    pub run_speed: u8,
    pub text_distance: u8,
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
