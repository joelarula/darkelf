use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};
use log::{debug, error, info};
use crate::app_state_manager::DeviceStateManager;
use crate::ble_device_controller::BleDeviceController;
use crate::device_command_utils::DeviceCommandUtils;

/// Color configuration with display properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorConfig {
    pub name: String,
    pub color: String,
    pub order: u8,
    pub idx: u8,
}

/// Public project settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicSettings {
    pub tx_color: u8,    // Color display order
    pub rd_mode: u8,     // Read mode
    pub run_speed: u8,   // Animation speed (1-100)
    pub sound_val: u8,   // Sound sensitivity (0-100)
}

/// Project item settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectItem {
    pub py_mode: u8,                 // Playback mode
    pub prj_selected: Vec<u16>,      // Packed selection bits
    pub ck_values: Vec<bool>,        // Expanded selection bits
}

/// Project configuration state
#[derive(Debug)]
pub struct ProjectConfigState {
    pub prj_index: u8,
    pub send_cmd_parms_timer: Option<tokio::time::Interval>,
    pub show_outdoor_tips: bool,
    pub features: DeviceFeatures,
    pub color_display_order: Vec<ColorConfig>,
    pub public: PublicSettings,
    pub item: ProjectItem,
}

/// Project configuration controller
pub struct BleDeviceProjectConfigPage {
    state: Arc<Mutex<ProjectConfigState>>,
    app_state: Arc<DeviceStateManager>,
    device_controller: Arc<BleDeviceController>,
    command_utils: Arc<DeviceCommandUtils>,
}

impl Default for ProjectConfigState {
    fn default() -> Self {
        Self {
            prj_index: 0,
            send_cmd_parms_timer: None,
            show_outdoor_tips: false,
            features: DeviceFeatures::default(),
            color_display_order: vec![
                ColorConfig {
                    name: "Red".to_string(),
                    color: "red".to_string(),
                    order: 0,
                    idx: 1,
                },
                ColorConfig {
                    name: "Yellow".to_string(),
                    color: "yellow".to_string(),
                    order: 1,
                    idx: 4,
                },
                ColorConfig {
                    name: "Green".to_string(),
                    color: "green".to_string(),
                    order: 2,
                    idx: 2,
                },
                ColorConfig {
                    name: "Cyan".to_string(),
                    color: "#00FFFF".to_string(),
                    order: 3,
                    idx: 5,
                },
                ColorConfig {
                    name: "Blue".to_string(),
                    color: "blue".to_string(),
                    order: 4,
                    idx: 3,
                },
                ColorConfig {
                    name: "Purple".to_string(),
                    color: "purple".to_string(),
                    order: 5,
                    idx: 6,
                },
                ColorConfig {
                    name: "White".to_string(),
                    color: "white".to_string(),
                    order: 6,
                    idx: 7,
                },
                ColorConfig {
                    name: "Jump".to_string(),
                    color: "transparent".to_string(),
                    order: 7,
                    idx: 8,
                },
                ColorConfig {
                    name: "RGB".to_string(),
                    color: "transparent".to_string(),
                    order: 8,
                    idx: 9,
                },
            ],
            public: PublicSettings {
                tx_color: 0,
                rd_mode: 0,
                run_speed: 10,
                sound_val: 20,
            },
            item: ProjectItem {
                py_mode: 0,
                prj_selected: vec![0, 0, 0, 0],
                ck_values: Vec::new(),
            },
        }
    }
}

impl BleDeviceProjectConfigPage {
    /// Create new controller instance
    pub fn new(
        app_state: Arc<DeviceStateManager>,
        device_controller: Arc<BleDeviceController>,
        command_utils: Arc<DeviceCommandUtils>,
    ) -> Self {
        Self {
            state: Arc::new(Mutex::new(ProjectConfigState::default())),
            app_state,
            device_controller,
            command_utils,
        }
    }

    /// Load project configuration
    pub fn on_load(&self, tag: u8) -> Result<(), String> {
        let mut state = self.state.lock().map_err(|e| e.to_string())?;
        
        let prj_data = self.app_state.get_cmd_data("prjData")?;
        let public_settings = prj_data.public;

        // Configure project based on index
        if tag == 1 {
            state.prj_index = tag;
            state.public = public_settings;
        } else {
            let item = prj_data.prj_item.get(&(tag as i32))
                .ok_or_else(|| "Project item not found".to_string())?;

            state.prj_index = tag;
            state.public = public_settings;
            state.item = item.clone();
            
            // Expand selection bits
            state.item.ck_values = self.get_ck_values(&state.item.prj_selected);
        }

        // Set outdoor tips visibility
        if tag == 6 && state.features.show_outdoor_tips {
            state.show_outdoor_tips = true;
        }

        Ok(())
    }

    /// Send command with parameters
    pub async fn send_cmd(&self, command_params: Option<HashMap<String, Value>>) -> Result<bool, String> {
        let state = self.state.lock().map_err(|e| e.to_string())?;
        
        // Update project data
        self.app_state.set_cmd_data("prjData", &json!({
            "prjIndex": state.prj_index,
            "public": state.public,
            "item": state.item,
        }))?;

        // Prepare command parameters
        let mut params = command_params.unwrap_or_default();
        params.insert("features".to_string(), 
            serde_json::to_value(self.app_state.get_device_features()?)?
        );

        // Generate and send command
        let command = self.command_utils.get_cmd_str(
            self.app_state.get_cmd_data("cmd")?,
            &params
        )?;

        self.device_controller.go_send(false, &command)
    }

    /// Extract individual bits from packed integers
    fn get_ck_values(&self, packed: &[u16]) -> Vec<bool> {
        let mut bits = Vec::new();
        for &value in packed {
            for bit in 0..16 {
                bits.push(((value >> bit) & 1) == 1);
            }
        }
        bits
    }

    /// Pack individual bits into integers
    fn get_prj_selected(&self, bits: &[bool]) -> Vec<u16> {
        let mut result = vec![0; 4];
        for (i, &bit) in bits.iter().enumerate() {
            if bit {
                let word_idx = i / 16;
                let bit_pos = i % 16;
                result[word_idx] |= 1 << bit_pos;
            }
        }
        result
    }

    /// Handle auto-selection button click
    pub async fn select_auto_btn_click(&self, action: u8) -> Result<(), String> {
        let mut state = self.state.lock().map_err(|e| e.to_string())?;
        
        // Update selection bits based on action
        for bit in state.item.ck_values.iter_mut() {
            *bit = match action {
                2 => !*bit,           // Invert
                3 => false,          // Clear all
                _ => true,           // Set all
            };
        }

        // Pack bits and update state
        state.item.prj_selected = self.get_prj_selected(&state.item.ck_values);
        
        // Send updated command
        self.send_cmd(None).await?;
        
        Ok(())
    }

    /// Handle checkbox group changes
    pub async fn checkbox_change(&self, indices: Vec<usize>) -> Result<(), String> {
        let mut state = self.state.lock().map_err(|e| e.to_string())?;
        
        let mut packed = vec![0u16; 4];
        for &idx in &indices {
            let word_idx = (idx - 1) / 16;
            let bit_pos = (idx - 1) % 16;
            packed[word_idx] |= 1 << bit_pos;
        }

        state.item.prj_selected = packed;
        self.send_cmd(None).await?;
        
        Ok(())
    }

    /// Handle selection button click
    pub async fn btn_select_click(&self, index: usize) -> Result<(), String> {
        let mut state = self.state.lock().map_err(|e| e.to_string())?;
        
        if state.item.py_mode == 0 {
            return Ok(());
        }

        // Toggle bit and prepare command parameters
        state.item.ck_values[index] = !state.item.ck_values[index];
        
        let command_params = if state.item.ck_values[index] {
            Some(json!({
                "prjParm": {
                    "prjIndex": state.prj_index,
                    "selIndex": index + 1
                }
            }))
        } else {
            None
        };

        // Update packed bits
        state.item.prj_selected = self.get_prj_selected(&state.item.ck_values);
        
        // Send command with parameters
        self.send_cmd(command_params).await?;
        
        Ok(())
    }
}

impl Drop for BleDeviceProjectConfigPage {
    fn drop(&mut self) {
        if let Ok(state) = self.state.lock() {
            if let Some(timer) = &state.send_cmd_parms_timer {
                // Cleanup timer
            }
        }
    }
}