use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use log::{debug, error, info};
use serde::{Deserialize, Serialize};
use tokio::time::{sleep, Duration};
use crate::app_state_manager::DeviceStateManager;
use crate::ble_device_controller::BleDeviceController;
use crate::device_command_utils::DeviceCommandUtils;

/// Function mode with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Function {
    tag: u8,
    name: String,
    show: bool,
}

/// Component state
#[derive(Debug)]
pub struct MainPageState {
    /// Current command being sent
    mode_cmd_send: String,
    /// Available functions
    functions: Vec<Function>,
    /// Device features
    features: DeviceFeatures,
    /// Device power state
    device_on: bool,
    /// Current project index
    prj_index: i32,
    /// Connected device name
    cnn_device: String,
    /// Connection state
    cnn_state: bool,
    /// Random verification bytes
    random_check: Vec<u8>,
    /// Initialization flag
    init_show: bool,
    /// Last command timestamp
    last_cmd_time: u64,
}

/// Main page component controller
pub struct MainPageComponent {
    state: Arc<Mutex<MainPageState>>,
    app_state: Arc<DeviceStateManager>,
    device_controller: Arc<BleDeviceController>,
    command_utils: Arc<DeviceCommandUtils>,
}

impl MainPageComponent {
    /// Create new instance
    pub fn new(
        app_state: Arc<DeviceStateManager>,
        device_controller: Arc<BleDeviceController>,
        command_utils: Arc<DeviceCommandUtils>,
    ) -> Self {
        let state = MainPageState {
            mode_cmd_send: String::new(),
            functions: vec![
                Function {
                    tag: 0,
                    name: "DMX".to_string(),
                    show: true,
                },
                Function {
                    tag: 1, 
                    name: "Random playback".to_string(),
                    show: true,
                },
                Function {
                    tag: 2,
                    name: "Timeline playback".to_string(), 
                    show: true,
                },
                Function {
                    tag: 3,
                    name: "Animation playback".to_string(),
                    show: true,
                },
                Function {
                    tag: 4,
                    name: "Text playback".to_string(),
                    show: true,
                },
                Function {
                    tag: 5,
                    name: "Christmas broadcast".to_string(),
                    show: true,
                },
                Function {
                    tag: 5,
                    name: "ILDA".to_string(),
                    show: false,
                },
                Function {
                    tag: 6,
                    name: "Outdoor playback".to_string(),
                    show: true,
                },
                Function {
                    tag: 7,
                    name: "Personalized programming".to_string(),
                    show: true,
                },
                Function {
                    tag: 8,
                    name: "Hand-drawn doodle".to_string(),
                    show: true,
                },
                Function {
                    tag: 9,
                    name: "Playlist".to_string(),
                    show: true,
                }
            ],
            features: app_state.get_device_features(),
            device_on: false,
            prj_index: -1,
            cnn_device: "Not connected".to_string(),
            cnn_state: false,
            random_check: Vec::new(),
            init_show: false,
            last_cmd_time: 0,
        };

        Self {
            state: Arc::new(Mutex::new(state)),
            app_state,
            device_controller,
            command_utils,
        }
    }

    /// Initialize component
    pub fn on_load(&self) {
        self.gen_random_check();
    }

    /// Show component
    pub fn on_show(&self) {
        let mut state = self.state.lock().unwrap();
        state.features = self.app_state.get_device_features();
        
        if !state.init_show {
            state.init_show = true;
            self.blu_init_pro();
        }
    }

    /// Initialize BLE connection
    pub fn blu_init_pro(&self) {
        let component = Arc::new(self.clone());
        
        self.app_state.set_connection_callback(Box::new(move |status, code| {
            component.blu_cnn_callback(status, code);
        }));

        self.app_state.set_receive_callback(Box::new(move |data| {
            component.blu_rec_callback(data);
        }));

        self.device_controller.cnn_pre_blu();
    }

    /// Send query command
    pub fn go_query_cmd(&self) {
        let state = self.state.lock().unwrap();
        let cmd = self.command_utils.get_query_cmd(&state.random_check);
        self.device_controller.go_send(false, &cmd);
    }

    /// Handle connection callback
    fn blu_cnn_callback(&self, connection_status: i32, result_code: i32) {
        if connection_status != 1 {
            let device = self.app_state.get_ble_device();
            debug!("blu_cnn_callback1 {} {}", connection_status, result_code);

            let mut state = self.state.lock().unwrap();
            if connection_status > 0 && device.is_some() {
                let device = device.unwrap();
                if device.characteristic_id.is_some() {
                    debug!("Connected {}", device.name);
                    state.cnn_device = device.name;
                    state.cnn_state = true;
                    drop(state);
                    self.go_query_cmd();
                    return;
                }
            }
            
            state.cnn_state = false;
            state.device_on = false;
            state.prj_index = -1;
        }
    }

    /// Handle receive callback
    fn blu_rec_callback(&self, data: &str) {
        debug!("blu_rec_callback");
        let state = self.state.lock().unwrap();
        
        if self.check_rcv_data(data, &state.random_check) {
            self.device_controller.set_can_send(true);
            self.device_controller.set_cmd_data(data);
            drop(state);
            let mut state = self.state.lock().unwrap();
            state.prj_index = self.app_state.get_cmd_mode();
        } else {
            error!("Abnormality in reading device parameters");
        }
    }

    /// Generate random verification bytes
    fn gen_random_check(&self) {
        let mut state = self.state.lock().unwrap();
        state.random_check = (0..4)
            .map(|_| rand::random::<u8>())
            .collect();
    }

    /// Validate received data
    fn check_rcv_data(&self, data: &str, random_verify: &[u8]) -> bool {
        if random_verify.len() != 4 || data.len() < 24 {
            return false;
        }

        // Extract and validate checksum
        let checksum = &data[data.len()-24..data.len()-16];
        let mut expected = Vec::with_capacity(4);
        
        for (i, &c) in random_verify.iter().enumerate() {
            let val = match i {
                0 => ((c as i32 + 55) >> 1) - 10,
                1 => 7 + ((c as i32 - 68) << 1),
                2 => 15 + ((c as i32 + 97) >> 1),
                3 => 87 + ((c as i32 - 127) >> 1),
                _ => unreachable!()
            } & 0xFF;
            expected.push(val as u8);
        }

        let received: Vec<u8> = (0..4)
            .map(|i| u8::from_str_radix(&checksum[i*2..i*2+2], 16).unwrap())
            .collect();

        if expected != received {
            return false;
        }

        let mut state = self.state.lock().unwrap();
        
        // Parse device status
        let power = u8::from_str_radix(&data[data.len()-16..data.len()-14], 16).unwrap();
        state.device_on = power != 0;

        // Parse device info
        let device_type = &data[data.len()-14..data.len()-12];
        let version = &data[data.len()-12..data.len()-10];
        let user_type = &data[data.len()-10..data.len()-8];

        self.app_state.set_device_info(
            device_type,
            version,
            user_type
        );

        state.features = self.app_state.get_device_features();

        true
    }

    /// Connect laser device
    pub fn cnn_laser(&self) {
        self.device_controller.cnn_laser();
    }

    /// Handle settings click
    pub fn setting_click(&self, tag: u8) {
        let state = self.state.lock().unwrap();
        
        if tag == 0 && !state.device_on {
            return;
        }

        if state.prj_index != tag as i32 && tag == 0 {
            drop(state);
            let mut state = self.state.lock().unwrap();
            state.prj_index = tag as i32;
            self.app_state.set_cmd_mode(tag as i32);
            self.send_cmd();
        } else {
            // Navigate to settings page
            // Implementation depends on UI framework
        }
    }

    /// Toggle device power
    pub fn on_off_change(&self) -> Result<(), String> {
        if !self.device_controller.get_can_send() {
            let state = self.state.lock().unwrap();
            if state.cnn_state {
                return Err("The current device cannot be identified".to_string());
            } else {
                return Err("Please connect first Bluetooth".to_string());
            }
        }

        let mut state = self.state.lock().unwrap();
        state.device_on = !state.device_on;
        
        let cmd = if state.device_on {
            "B0B1B2B3FFB4B5B6B7"
        } else {
            "B0B1B2B300B4B5B6B7"
        };

        self.device_controller.go_send(false, cmd);
        Ok(())
    }

    /// Send command
    pub fn send_cmd(&self) {
        let state = self.state.lock().unwrap();
        let cmd = self.command_utils.get_cmd_str(
            self.app_state.get_cmd_data(),
            &state.features
        );
        
        drop(state);
        let mut state = self.state.lock().unwrap();
        state.mode_cmd_send = cmd;
        self.do_send_cmd();
    }

    /// Execute command send with retry
    async fn do_send_cmd(&self) {
        let state = self.state.lock().unwrap();
        if state.mode_cmd_send.is_empty() {
            return;
        }

        let cmd = state.mode_cmd_send.clone();
        drop(state);

        let result = self.device_controller.go_send(false, &cmd);
        
        if result {
            let mut state = self.state.lock().unwrap();
            state.mode_cmd_send.clear();
        } else {
            sleep(Duration::from_millis(100)).await;
            self.do_send_cmd().await;
        }
    }
}

// Implement Drop for cleanup
impl Drop for MainPageComponent {
    fn drop(&mut self) {
        // Cleanup callbacks
        self.app_state.clear_callbacks();
    }
}