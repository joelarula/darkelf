use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use log::{debug, info, error};

/// Represents the device project data structure
#[derive(Debug, Clone)]
pub struct ProjectData {
    pub prj_index: i32,
    pub public: ProjectPublicData,
    pub prj_item: HashMap<i32, ProjectItem>,
}

#[derive(Debug, Clone)]
pub struct ProjectPublicData {
    pub rd_mode: u8,    // audio trigger mode, 0 or 255
    pub run_speed: i32, // default 50
    pub tx_color: u8,   // colorDisplayOrder, default 9
    pub sound_val: i32, // sound sensitivity, default 20
}

#[derive(Debug, Clone)]
pub struct ProjectItem {
    pub py_mode: u8,              // 0,255 loop playback, tick play
    pub prj_selected: Vec<i32>,   // array of 4 numbers
    pub ck_values: Vec<i32>,      // selected items
}

#[derive(Debug, Clone)]
pub struct SettingData {
    pub dmx: i32,
    pub ch: i32,
    pub xy: i32,
    pub light: i32,
    pub cfg: i32,
    pub lang: i32,
    pub val_arr: Vec<i32>,
}

#[derive(Debug, Clone)]
pub struct CommandData {
    pub cur_mode: i32,
    pub setting_data: SettingData,
    pub prj_data: ProjectData,
}

#[derive(Debug, Clone)]
pub struct DeviceInfo {
    pub device_type: i32,
    pub version: i32,
    pub user_type: i32,
}

#[derive(Debug, Clone)]
pub struct DeviceFeatures {
    pub text_stop_time: bool,
    pub text_decimal_time: bool,
    pub display_type: i32,
    pub show_outdoor_tips: bool,
    pub xy_cnf: bool,
    pub arb_play: bool,
    pub ilda: bool,
    pub ttl_an: bool,
    pub pics_play: bool,
    pub text_up_down: bool,
    pub animation_fix: bool,
    pub text_mode_fix01: bool,
}

#[derive(Debug)]
pub struct BleDevice {
    pub device_id: String,
    pub service_id: Option<String>,
    pub characteristic_id: Option<String>,
}

/// Main state manager for device control
pub struct DeviceStateManager {
    /// BLE service UUIDs to connect to
    mservice_uuids: Vec<String>,
    /// BLE characteristic UUIDs for transmitting (TX) data
    mtxd_uuids: Vec<String>,
    /// BLE characteristic UUIDs for receiving (RX) data
    mrxd_uuids: Vec<String>,
    /// UUID selection (0,1,2)
    muuid_sel: i32,
    /// BLE manual disconnect flag
    ble_manual_disc_conn: bool,
    /// BLE connection state change set flag
    ble_connection_state_change_set: bool,
    /// Bluetooth adapter state
    bluetooth_adapter_open: bool,
    /// Current BLE device
    ble_device: Option<BleDevice>,
    /// BLE state (0,1,2)
    blu_state: i32,
    /// Stop BLE operations flag
    blu_connect_stop: bool,
    /// BLE connection state
    blu_connected: i32,
    /// Ready to receive flag
    blu_ready_rec: bool,
    /// BLE data can be sent flag
    blu_data_can_send: bool,
    /// Command sending in progress flag
    blu_data_cmd_sending: bool,
    /// Data send interval in ms
    blu_data_send_interval: u64,
    /// Device information
    device_info: DeviceInfo,
    /// Command data
    cmd: CommandData,
    /// Connection callback
    blu_cnn_callback: Option<Arc<Mutex<Box<dyn Fn(i32, bool) + Send + Sync>>>>,
    /// Receive callback
    blu_rec_callback: Option<Arc<Mutex<Box<dyn Fn(&str) + Send + Sync>>>>,
    /// Receive content buffer
    blu_rec_content: Option<String>,
}

impl Default for DeviceStateManager {
    fn default() -> Self {
        Self::new()
    }
}

impl DeviceStateManager {
    /// Create a new instance with default values
    pub fn new() -> Self {
        DeviceStateManager {
            mservice_uuids: Vec::new(),
            mtxd_uuids: Vec::new(),
            mrxd_uuids: Vec::new(),
            muuid_sel: 0,
            ble_manual_disc_conn: false,
            ble_connection_state_change_set: false,
            bluetooth_adapter_open: false,
            ble_device: None,
            blu_state: 0,
            blu_connect_stop: false,
            blu_connected: 0,
            blu_ready_rec: false,
            blu_data_can_send: false,
            blu_data_cmd_sending: false,
            blu_data_send_interval: 100,
            device_info: DeviceInfo {
                device_type: 0,
                version: 0,
                user_type: 0,
            },
            cmd: CommandData {
                cur_mode: 0,
                setting_data: SettingData {
                    dmx: 0,
                    ch: 0,
                    xy: 0,
                    light: 1,
                    cfg: 0,
                    lang: 0,
                    val_arr: vec![1, 10, 10, 10, 10],
                },
                prj_data: ProjectData {
                    prj_index: 0,
                    public: ProjectPublicData {
                        rd_mode: 0,
                        run_speed: 50,
                        tx_color: 9,
                        sound_val: 20,
                    },
                    prj_item: {
                        let mut items = HashMap::new();
                        for &i in &[2, 3, 5, 6] {
                            items.insert(i, ProjectItem {
                                py_mode: 0,
                                prj_selected: vec![0, 0, 0, 0],
                                ck_values: Vec::new(),
                            });
                        }
                        items
                    },
                },
            },
            blu_cnn_callback: None,
            blu_rec_callback: None,
            blu_rec_content: None,
        }
    }

    /// Set BLE data send interval
    pub fn set_blu_data_send_interval(&mut self, interval: u64) {
        self.blu_data_send_interval = interval;
    }

    /// Set receive callback handler
    pub fn set_rec_callback<F>(&mut self, callback: F)
    where
        F: Fn(&str) + Send + Sync + 'static,
    {
        self.blu_rec_callback = Some(Arc::new(Mutex::new(Box::new(callback))));
    }

    /// Set BLE connection state
    pub fn set_blu_cnn_state(&mut self, connection_state: i32, is_manual_change: bool) {
        self.blu_connected = connection_state;
        if self.blu_connected == 2 {
            self.save_device();
        }
        if let Some(callback) = &self.blu_cnn_callback {
            if let Ok(cb) = callback.lock() {
                cb(connection_state, is_manual_change);
            }
        }
    }

    /// Set command mode
    pub fn set_cmd_mode(&mut self, mode: i32) {
        self.cmd.cur_mode = mode;
        self.cmd.prj_data.prj_index = mode;
    }

    /// Update command data
    pub fn set_cmd_data(&mut self, key: &str, data: CommandData) {
        match key {
            "prj_data" => {
                self.cmd.prj_data.public = data.prj_data.public.clone();
                if data.prj_data.prj_index != 1 {
                    if let Some(item) = data.prj_data.prj_item.get(&data.prj_data.prj_index) {
                        self.cmd.prj_data.prj_item.insert(data.prj_data.prj_index, item.clone());
                    }
                }
            }
            "text_data" => {
                // Handle text data updates
                self.cmd.prj_data.public.run_speed = data.prj_data.public.run_speed;
                self.cmd.prj_data.public.tx_color = data.prj_data.public.tx_color;
            }
            _ => {}
        }
    }

    /// Read BLE settings
    pub fn read_setting(&mut self) {
        self.muuid_sel = self.read_data("lastsel").unwrap_or(0);
        match self.muuid_sel {
            0 => {
                self.mservice_uuids = vec!["0000FF00-0000-1000-8000-00805F9B34FB".to_string()];
                self.mtxd_uuids = vec!["0000FF02-0000-1000-8000-00805F9B34FB".to_string()];
                self.mrxd_uuids = vec!["0000FF01-0000-1000-8000-00805F9B34FB".to_string()];
            }
            1 => {
                self.mservice_uuids = vec!["0000FFE0-0000-1000-8000-00805F9B34FB".to_string()];
                self.mtxd_uuids = vec!["0000FFE1-0000-1000-8000-00805F9B34FB".to_string()];
                self.mrxd_uuids = vec!["0000FFE1-0000-1000-8000-00805F9B34FB".to_string()];
            }
            2 => {
                self.mservice_uuids = vec!["0000FF00-0000-1000-8000-00805F9B34FB".to_string()];
                self.mtxd_uuids = vec!["0000FF02-0000-1000-8000-00805F9B34FB".to_string()];
                self.mrxd_uuids = vec!["0000FF01-0000-1000-8000-00805F9B34FB".to_string()];
            }
            _ => {}
        }
    }

    /// Get device features based on type and version
    pub fn get_device_features(&self) -> DeviceFeatures {
        let mut features = DeviceFeatures {
            text_stop_time: false,
            text_decimal_time: false,
            display_type: 0,
            show_outdoor_tips: false,
            xy_cnf: false,
            arb_play: false,
            ilda: false,
            ttl_an: false,
            pics_play: false,
            text_up_down: false,
            animation_fix: false,
            text_mode_fix01: false,
        };

        let device_type = self.device_info.device_type;
        let version = self.device_info.version;

        // Configure features based on device type and version
        if (device_type == 1 && version >= 1) || 
           (device_type == 0 && version >= 2) || 
           device_type >= 2 {
            features.text_stop_time = true;
            features.text_decimal_time = true;
        }

        if (device_type == 1 && version >= 2) || device_type > 1 {
            features.show_outdoor_tips = true;
        }

        if device_type == 1 && version == 1 {
            features.text_mode_fix01 = true;
        }

        if device_type == 2 {
            features.xy_cnf = true;
        }

        if device_type == 1 || device_type == 2 {
            features.ilda = true;
            features.ttl_an = true;
        }

        if device_type >= 2 || version >= 3 {
            features.arb_play = true;
        }

        if device_type >= 3 || version >= 4 {
            features.text_up_down = true;
        }

        if device_type >= 3 || version >= 5 {
            features.pics_play = true;
        }

        if device_type == 1 {
            features.animation_fix = true;
        }

        features.display_type = device_type;
        features
    }

    // Storage interface - implementations would be provided separately
    fn read_data(&self, key: &str) -> Option<i32> {
        // Implementation would interact with persistent storage
        None
    }

    fn save_data(&self, key: &str, value: i32) {
        // Implementation would interact with persistent storage
    }

    fn save_device(&self) {
        // Implementation would save device info to persistent storage
    }
}

// Implement additional traits for async operations and error handling
#[async_trait]
impl DeviceStateManagerAsync for DeviceStateManager {
    async fn create_ble_connection(&mut self, device_id: &str) -> Result<(), BleError> {
        // Implementation would handle async BLE connection
        todo!()
    }

    async fn close_ble_connection(&mut self) -> Result<(), BleError> {
        // Implementation would handle async BLE disconnection
        todo!()
    }
}

/// Error types for BLE operations
#[derive(Debug)]
pub enum BleError {
    ConnectionFailed(String),
    Timeout,
    InvalidState,
    DeviceNotFound,
    AdapterError(String),
}

/// Async operations trait
#[async_trait]
pub trait DeviceStateManagerAsync {
    async fn create_ble_connection(&mut self, device_id: &str) -> Result<(), BleError>;
    async fn close_ble_connection(&mut self) -> Result<(), BleError>;
}