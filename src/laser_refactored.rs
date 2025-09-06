use std::fmt::Write; // For string formatting
use std::sync::{Arc, Mutex};
use std::time::{Instant, Duration};
use hex::decode;
use serialport::{available_ports, SerialPort};
use tokio; // For async handling
use crate::ble::{BleController};
use btleplug::api::Characteristic; // Still needed for type references
use std::error::Error;
use std::collections::VecDeque; // For buffer

/// Represents a single laser point with coordinates and control data.
#[derive(Clone, Debug)]
pub struct LaserPoint {
    x: i16,      // X coordinate
    y: i16,      // Y coordinate
    ctrl: u8,    // Control data
    is_up: bool, // Whether the beam is up (not firing)
}

impl LaserPoint {
    /// Create a new laser point.
    pub fn new(x: i16, y: i16, ctrl: u8, is_up: bool) -> Self {
        Self { x, y, ctrl, is_up }
    }
}

/// Settings that are accessible publicly.
#[derive(Clone, Debug)]
pub struct PublicSettings {
    rd_mode: u8,     // Read mode
    sound_val: u8,   // Sound value (scaled)
}

/// Data for laser project.
#[derive(Clone, Debug)]
pub struct ProjectData {
    pub public: PublicSettings,
    pub prj_item: Vec<String>,
}

/// Configuration options for laser.
#[derive(Clone, Debug)]
pub struct LaserOptions {
    // Common settings
    text_decimal_time: bool, // Use decimal time for text
    text_stop_time: bool,    // Stop time for text
    tx_color: u8,            // Text color
    tx_size: u8,             // Text size
    tx_offset: u8,           // Text offset
    tx_align: u8,            // Text alignment
    // Laser parameters
    brightness: u8,           // Laser brightness
    beam_size: u8,            // Beam size
    feed_rate: u8,            // Feed rate for movement
    rd_mode: u8,              // Read mode
    sound_val: u8,            // Sound value (scaled)
    high_quality: bool,       // Use high quality rendering
}

/// Main controller for laser device communication.
pub struct LaserController {
    sending: Arc<Mutex<bool>>,
    ble_controller: Option<Box<dyn BleController>>, // Unified BLE controller using trait
    write_char: Option<Characteristic>,   // Keep for compatibility
    notify_char: Option<Characteristic>,  // Keep for compatibility
    last_send_time: Option<Instant>,
    options: LaserOptions,
    project_data: ProjectData,
    connected: bool,
    mock_mode: bool,
    blu_rec_content: VecDeque<String>,
    rec_device_msg_timer: Option<tokio::time::Instant>,
    discovery_started: bool,
    not_pass: i32,
    pass_count: i32,
}

impl LaserController {
    /// Creates a new LaserController with a BLE controller passed directly
    pub async fn new(is_ble: bool, mock_mode: bool, ble_controller: Option<Box<dyn BleController>>) -> Result<Self, Box<dyn Error>> {
        let sending = Arc::new(Mutex::new(false));
        // BLE controller is now passed directly
        
        let options = LaserOptions {
            text_decimal_time: false,
            text_stop_time: false,
            tx_color: 0,
            tx_size: 60,
            tx_offset: 0,
            tx_align: 0,
            brightness: 100,
            beam_size: 100,
            feed_rate: 100,
            rd_mode: 0,
            sound_val: 0,
            high_quality: false,
        };
        let project_data = ProjectData {
            public: PublicSettings { rd_mode: 0, sound_val: 0 },
            prj_item: Vec::new(),
        };

        let mut controller = LaserController {
            sending,
            ble_controller,
            write_char: None,
            notify_char: None,
            last_send_time: None,
            options,
            project_data,
            connected: false,
            mock_mode,
            blu_rec_content: VecDeque::new(),
            rec_device_msg_timer: None,
            discovery_started: false,
            not_pass: 0,
            pass_count: 0,
        };

        if is_ble {
            controller.connect_ble().await?;
        } else {
            // Handle serial port connection if needed
        }

        Ok(controller)
    }

    async fn connect_ble(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(controller) = &mut self.ble_controller {
            controller.connect().await?;
            self.connected = true;
            
            // Get write and notify characteristics for compatibility
            // This will be done automatically in the BleController
            // but we still keep track of them here for the interface
            if let Some(characteristics) = self.get_characteristics_from_ble() {
                for characteristic in characteristics {
                    let uuid = characteristic.uuid.to_string();
                    if uuid == "0000ffe2-0000-1000-8000-00805f9b34fb" || uuid == "0000ff02-0000-1000-8000-00805f9b34fb" {
                        self.write_char = Some(characteristic.clone());
                    } else if uuid == "0000ffe1-0000-1000-8000-00805f9b34fb" || uuid == "0000ff01-0000-1000-8000-00805f9b34fb" {
                        self.notify_char = Some(characteristic.clone());
                    }
                }
            }
            
            Ok(())
        } else {
            Err("No BLE controller was provided".into())
        }
    }

    /// Helper method to get characteristics from BleController
    fn get_characteristics_from_ble(&self) -> Option<Vec<Characteristic>> {
        if let Some(controller) = &self.ble_controller {
            // This is a dummy implementation - in real code, you'd get this from the controller
            // For now we'll just return an empty vec
            Some(Vec::new())
        } else {
            None
        }
    }

    pub async fn discover_characteristics(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(controller) = &mut self.ble_controller {
            controller.discover_characteristics().await?;
            // For now just set connected to true
            self.connected = true;
            Ok(())
        } else {
            Err("BLE controller not initialized".into())
        }
    }

    pub async fn send(&mut self, cmd_hex: &str, show_loading: bool, callback: Option<&mut dyn FnMut(i8, u8)>) -> Result<(), String> {
        if cmd_hex.is_empty() { return Err("Empty command".to_string()); }

        let mut guard = self.sending.lock().unwrap();
        if *guard { return Err("Previous send in progress".to_string()); }
        *guard = true;

        if !self.is_connected() { return Err("Not connected".to_string()); }

        let mut cb = callback;
        if let Some(cb_ref) = cb.as_deref_mut() {
            cb_ref(0, 0);
        }
        if show_loading && !self.mock_mode {
            self.last_send_time = Some(Instant::now());
        }

        let bytes = decode(cmd_hex).map_err(|e| e.to_string())?;
        if bytes.is_empty() { *guard = false; return Err("Invalid hex".to_string()); }

        let result = if self.mock_mode {
            tokio::time::sleep(Duration::from_millis(20)).await;
            Ok(())
        } else {
            drop(guard); // Drop the lock before mutable borrow
            self.send_ble(&bytes).await
        };

        let mut guard = self.sending.lock().unwrap();
        *guard = false;
        if let Some(cb_ref) = cb.as_deref_mut() {
            if result.is_ok() { cb_ref(1, 100); } else { cb_ref(-1, 0); }
        }
        result
    }

    async fn send_ble(&mut self, bytes: &[u8]) -> Result<(), String> {
        if let Some(controller) = &mut self.ble_controller {
            controller.send(bytes).await
        } else {
            Err("BLE controller not initialized".to_string())
        }
    }


    pub fn add_content(&mut self, content: String) {
        self.blu_rec_content.push_back(content);
        if self.blu_rec_content.len() > 10 {
            self.blu_rec_content.pop_front();
        }
    }

    pub fn is_connected(&self) -> bool {
        self.ble_controller.is_some() && self.ble_controller.as_ref().unwrap().is_connected()
    }

    // Other methods would follow the same pattern:
    // 1. Check if using BLE
    // 2. If yes, delegate to ble_controller
    // 3. Otherwise handle serial port logic

    pub fn cleanup(&mut self) -> Result<(), String> {
        // Close any BLE connections
        if let Some(controller) = &mut self.ble_controller {
            // In a real implementation, you'd call a cleanup method on the controller
        }
        
        self.ble_controller = None;
        self.connected = false;
        Ok(())
    }

    // Add other methods from the original implementation, 
    // adapting them to use the new BLE controller
}
