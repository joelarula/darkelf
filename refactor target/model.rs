// Rust data models extracted from app-service-minimal.js

#[derive(Debug, Clone)]
pub struct ShowCtr {
    pub light1: bool,
    pub light2: bool,
    pub light3: bool,
    pub light_ext: bool,
}

#[derive(Debug, Clone)]
pub struct DeviceInfo {
    pub device_type: u8,
    pub version: u8,
    pub user_type: u8,
}

#[derive(Debug, Clone)]
pub struct Features {
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
    pub animation_fix: bool,
}

#[derive(Debug, Clone)]
pub struct ColorDisplayOrderItem {
    pub name: String,
    pub color: String,
    pub order: u8,
    pub idx: u8,
}

#[derive(Debug, Clone)]
pub struct PublicConfig {
    pub tx_color: u8,
    pub rd_mode: u8,
    pub run_speed: u8,
    pub sound_val: u8,
}

#[derive(Debug, Clone)]
pub struct ItemConfig {
    pub py_mode: u8,
    pub prj_selected: [u16; 4],
    pub ck_values: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct DeviceConfig {
    pub show_ctr: ShowCtr,
    pub device_info: DeviceInfo,
    pub features: Features,
    pub set_cmd_send: String,
    pub version: String,
    pub machine: String,
    pub dmx: u8,
    pub ch: u16,
    pub xy: u8,
    pub light: u8,
    pub cfg: u8,
    pub val_arr: [u16; 5],
    pub val_range: [[u16; 2]; 5],
}

#[derive(Debug, Clone)]
pub struct ProjectConfig {
    pub prj_index: u8,
    pub send_cmd_parms_timer: Option<u64>,
    pub show_outdoor_tips: bool,
    pub features: Features,
    pub color_display_order: Vec<ColorDisplayOrderItem>,
    pub public: PublicConfig,
    pub item: ItemConfig,
}
