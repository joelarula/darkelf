use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};
use log::{debug, error, info};
use crate::app_state_manager::DeviceStateManager;
use crate::ble_device_controller::BleDeviceController;
use crate::device_command_utils::DeviceCommandUtils;

/// Light control configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightControl {
    light1: bool,
    light2: bool,
    light3: bool,
    light_ext: bool,
}

/// Value range definition for device parameters
#[derive(Debug, Clone)]
pub struct ValueRange {
    pub min: u16,
    pub max: u16,
}

/// Device settings data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceSettings {
    pub ch: u16,      // channel: 1-512
    pub dmx: u8,      // 0 or 1
    pub xy: u8,       // 0-7 Normal: X+Y+ X+Y- X-Y- X-Y+ Interchange: X+Y+ X+Y- X-Y- X-Y+
    pub light: u8,    // 1 single, 2 dual, 3 full
    pub cfg: u8,      // 0 ttl 255 analog
    pub lang: u8,
    pub val_arr: Vec<u16>, // [channel, display_range, r, g, b]
}

/// Configuration page state
#[derive(Debug)]
pub struct DeviceConfigState {
    show_ctr: LightControl,
    device_info: i32,
    features: DeviceFeatures,
    set_cmd_send: String,
    version: String,
    machine: String,
    settings: DeviceSettings,
    val_range: Vec<ValueRange>,
}

/// Device configuration controller
pub struct DeviceConfigPageController {
    state: Arc<Mutex<DeviceConfigState>>,
    app_state: Arc<DeviceStateManager>,
    device_controller: Arc<BleDeviceController>,
    command_utils: Arc<DeviceCommandUtils>,
}

impl Default for DeviceSettings {
    fn default() -> Self {
        Self {
            ch: 0,
            dmx: 0,
            xy: 0,
            light: 1,
            cfg: 0,
            lang: 0,
            val_arr: vec![1, 10, 10, 10, 10],
        }
    }
}

impl Default for LightControl {
    fn default() -> Self {
        Self {
            light1: true,
            light2: true,
            light3: true,
            light_ext: false,
        }
    }
}

impl DeviceConfigPageController {
    /// Create new controller instance
    pub fn new(
        app_state: Arc<DeviceStateManager>,
        device_controller: Arc<BleDeviceController>,
        command_utils: Arc<DeviceCommandUtils>
    ) -> Self {
        let state = DeviceConfigState {
            show_ctr: LightControl::default(),
            device_info: 0,
            features: app_state.get_device_features(),
            set_cmd_send: String::new(),
            version: String::new(),
            machine: String::new(),
            settings: DeviceSettings::default(),
            val_range: vec![
                ValueRange { min: 1, max: 512 },   // channel
                ValueRange { min: 10, max: 100 },  // display range
                ValueRange { min: 0, max: 255 },   // R
                ValueRange { min: 0, max: 255 },   // G
                ValueRange { min: 0, max: 255 },   // B
            ],
        };

        Self {
            state: Arc::new(Mutex::new(state)),
            app_state,
            device_controller,
            command_utils,
        }
    }

    /// Initialize controller with DMX setting
    pub fn on_load(&self, dmx: u8) -> Result<(), String> {
        let mut state = self.state.lock().map_err(|e| e.to_string())?;
        
        // Get current version
        state.version = self.get_runtime_version()?;
        
        // Load settings
        let settings = self.app_state.get_cmd_data("settingData")?;
        state.settings = settings;
        state.settings.dmx = dmx;

        // Get device info
        let device_info = self.app_state.get_device_info()?;
        state.device_info = device_info.device_type;
        
        // Format machine string
        state.machine = format!("{:02} - {:02}", 
            device_info.device_type,
            device_info.version
        );

        self.init_data()?;
        
        Ok(())
    }

    /// Save settings on unload
    pub fn on_unload(&self) -> Result<(), String> {
        let state = self.state.lock().map_err(|e| e.to_string())?;
        
        self.app_state.set_cmd_data("settingData", &state.settings)?;
        
        Ok(())
    }

    /// Send configuration command
    pub async fn send_cmd(&self) -> Result<(), String> {
        let mut state = self.state.lock().map_err(|e| e.to_string())?;
        
        // Update global settings
        self.app_state.set_cmd_data("settingData", &state.settings)?;
        
        // Generate command
        let command = self.command_utils.get_setting_cmd(
            &self.app_state.get_cmd_data("settingData")?
        )?;
        
        state.set_cmd_send = command;
        self.do_send_cmd().await?;
        
        Ok(())
    }

    /// Execute command send with retry
    async fn do_send_cmd(&self) -> Result<(), String> {
        let mut state = self.state.lock().map_err(|e| e.to_string())?;
        
        if state.set_cmd_send.is_empty() {
            return Ok(());
        }

        let cmd = state.set_cmd_send.clone();
        drop(state);

        let result = self.device_controller.go_send(false, &cmd);
        if result {
            let mut state = self.state.lock().map_err(|e| e.to_string())?;
            state.set_cmd_send.clear();
        } else {
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            self.do_send_cmd().await?;
        }

        Ok(())
    }

    /// Initialize device-specific data
    fn init_data(&self) -> Result<(), String> {
        let mut state = self.state.lock().map_err(|e| e.to_string())?;
        let device_info = self.app_state.get_device_info()?;

        // Adjust ranges for device type 1
        if device_info.device_type == 1 {
            state.val_range = vec![
                ValueRange { min: 1, max: 512 },   // channel
                ValueRange { min: 10, max: 100 },  // display range
                ValueRange { min: 0, max: 100 },   // R
                ValueRange { min: 0, max: 100 },   // G
                ValueRange { min: 0, max: 100 },   // B
            ];

            // Clamp values to new ranges
            for i in 2..5 {
                if state.settings.val_arr[i] > state.val_range[i].max {
                    state.settings.val_arr[i] = state.val_range[i].max;
                }
            }
        }

        // Update light control based on device type/version
        if device_info.device_type == 1 || 
           (device_info.device_type == 0 && device_info.version >= 1) {
            state.show_ctr = LightControl {
                light1: false,
                light2: false,
                light3: false,
                light_ext: true,
            };
        }

        Ok(())
    }

    /// Get runtime version
    fn get_runtime_version(&self) -> Result<String, String> {
        // Implementation would depend on platform
        Ok("1.0.0".to_string())
    }

    /// Update a setting value with range validation
    pub fn update_setting(&self, key: &str, value: u16) -> Result<(), String> {
        let mut state = self.state.lock().map_err(|e| e.to_string())?;
        
        match key {
            "ch" => {
                if value >= state.val_range[0].min && value <= state.val_range[0].max {
                    state.settings.ch = value;
                }
            },
            "dmx" => state.settings.dmx = (value & 1) as u8,
            "xy" => state.settings.xy = (value & 7) as u8,
            "light" => {
                if value >= 1 && value <= 3 {
                    state.settings.light = value as u8;
                }
            },
            "cfg" => state.settings.cfg = if value == 0 { 0 } else { 255 },
            _ => return Err(format!("Invalid setting key: {}", key)),
        }

        Ok(())
    }

    /// Update a value array element with range validation
    pub fn update_val_arr(&self, index: usize, value: u16) -> Result<(), String> {
        let mut state = self.state.lock().map_err(|e| e.to_string())?;
        
        if index < state.val_range.len() {
            let range = &state.val_range[index];
            let clamped = value.clamp(range.min, range.max);
            state.settings.val_arr[index] = clamped;
            Ok(())
        } else {
            Err("Invalid value array index".to_string())
        }
    }
}

// Implement Drop for cleanup
impl Drop for DeviceConfigPageController {
    fn drop(&mut self) {
        // Ensure settings are saved
        if let Err(e) = self.on_unload() {
            error!("Failed to save settings on drop: {}", e);
        }
    }
}