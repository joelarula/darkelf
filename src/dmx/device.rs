
use crate::dmx::controller::{DmxFrame, DmxController};
use crate::dmx::model::Fixture;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use log::{info, error, debug, warn};


pub struct DmxDevice {
    pub fixture: Fixture,
    dmx_controller: Arc<Mutex<DmxController>>,
    current_frame: Arc<Mutex<DmxFrame>>,
    pub dmx_start_channel: usize,
    is_running: Arc<Mutex<bool>>,
    
}

impl DmxDevice {


    pub fn new(port_name: &str, dmx_start_channel: usize, fixture: Fixture) -> Result<Self, Box<dyn std::error::Error>> {
        info!("Creating DMX laser device on port: {}, starting at channel: {}", port_name, dmx_start_channel);
        
        let dmx_controller = DmxController::new(port_name, dmx_start_channel)?;
        
        Ok(Self {
            dmx_controller: Arc::new(Mutex::new(dmx_controller)),
            current_frame: Arc::new(Mutex::new(DmxFrame::new())),
            dmx_start_channel,
            is_running: Arc::new(Mutex::new(false)),
            fixture: fixture,
        })
    }

    pub fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Starting DMX laser device");     
        let mut is_running = self.is_running.lock().unwrap();
        if *is_running {
            warn!("DMX device already running");
            return Ok(());
        }
        *is_running = true;
        
        // Start DMX output thread
        let controller = Arc::clone(&self.dmx_controller);
        let frame = Arc::clone(&self.current_frame);
        let is_running_clone = Arc::clone(&self.is_running);

        thread::spawn(move || {
            info!("DMX output thread started");
            while *is_running_clone.lock().unwrap() {
                let current_frame = {
                    let frame_guard = frame.lock().unwrap();
                    frame_guard.clone()
                };
                if let Ok(mut controller) = controller.lock() {
                    if let Err(e) = controller.send_frame(&current_frame) {
                        error!("Failed to send DMX frame: {}", e);
                    }
                } else {
                    error!("Failed to acquire DMX controller lock");
                }
                thread::sleep(Duration::from_millis(22));
            }
            info!("DMX output thread stopped");
        });

        Ok(())
    }

    /// Stop the DMX device
    pub fn stop(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Stopping DMX laser device");
        
        let mut is_running = self.is_running.lock().unwrap();
        *is_running = false;
        
        // Send all-zero frame to turn off laser
        let dmx_frame = DmxFrame::new();
        if let Ok(mut controller) = self.dmx_controller.lock() {
            controller.send_frame(&dmx_frame)?;
        }
        
        Ok(())
    }


    /// Get current DMX frame
    pub fn get_current_frame(&self) -> DmxFrame {
        self.current_frame.lock().unwrap().clone()
    }


    /// Set individual DMX channel value (relative to device's start channel)
    pub fn set_dmx_channel(&self, channel: usize, value: u8) -> Result<(), Box<dyn std::error::Error>> {
        let abs_channel = self.dmx_start_channel + channel - 1;
        if abs_channel < 1 || abs_channel > 512 {
            return Err("DMX channel must be 1-512".into());
        }
        let mut frame = self.current_frame.lock().unwrap();
        frame.set_channel(abs_channel, value);
        debug!("Set DMX CH{} (abs {}): {}", channel, abs_channel, value);
        Ok(())
    }

    /// Get individual DMX channel value (relative to device's start channel)
    pub fn get_dmx_channel(&self, channel: usize) -> Option<u8> {
        let abs_channel = self.dmx_start_channel + channel - 1;
        let frame = self.current_frame.lock().unwrap();
        frame.get_channel(abs_channel)
    }
}

impl Drop for DmxDevice {
    fn drop(&mut self) {
        if let Err(e) = self.stop() {
            error!("Error stopping DMX laser device: {}", e);
        }
    }
}

