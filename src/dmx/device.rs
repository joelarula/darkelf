

use crate::dmx::controller::{DmxFrame, DmxController, DmxCommand};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use log::{info, error, debug, warn};


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
            master_dimmer: 0,      
            color_control: 0,      
            color_speed: 0,       
            pattern_group: 0,     
            pattern_select: 0,     
            dynamic_effects: 0,    
            effect_speed: 0,     
            pattern_size: 0,     
            size_control: 0,      
            rotation: 0,         
            vertical_flip: 0,     
            horizontal_flip: 0,   
            horizontal_pos: 0,   
            vertical_pos: 0,     
            wave_effect: 0,        
            manual_drawing: 0,    
        }
    }
}

/// DMX-based laser device implementation following the official 16-channel DMX specification
pub struct DmxLaserDevice {
    dmx_controller: Arc<Mutex<DmxController>>,
    current_state: Arc<Mutex<DmxLaserState>>,
    dmx_start_channel: usize,
    is_running: Arc<Mutex<bool>>,
}


impl DmxLaserDevice {

    pub fn new(port_name: &str, dmx_start_channel: usize) -> Result<Self, Box<dyn std::error::Error>> {
        info!("Creating DMX laser device on port: {}, starting at channel: {}", port_name, dmx_start_channel);
        
        let dmx_controller = DmxController::new(port_name, dmx_start_channel)?;
        
        Ok(Self {
            dmx_controller: Arc::new(Mutex::new(dmx_controller)),
            current_state: Arc::new(Mutex::new(DmxLaserState::default())),
            dmx_start_channel,
            is_running: Arc::new(Mutex::new(false)),
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
        let state = Arc::clone(&self.current_state);
        let is_running_clone = Arc::clone(&self.is_running);
        let start_channel = self.dmx_start_channel;

        thread::spawn(move || {
            info!("DMX output thread started");
            let mut frame_count = 0;
            
            while *is_running_clone.lock().unwrap() {
                let current_state = {
                    let state_guard = state.lock().unwrap();
                    state_guard.clone()
                };
                
                // Create DMX frame from current state
                let mut dmx_frame = DmxFrame::new();
                Self::state_to_dmx_frame(&current_state, &mut dmx_frame, start_channel);
                             
                if let Ok(mut controller) = controller.lock() {
                    if let Err(e) = controller.send_frame(&dmx_frame) {
                        error!("Failed to send DMX frame: {}", e);
                    }
                } else {
                    error!("Failed to acquire DMX controller lock");
                }
                
                // DMX refresh rate (standard 44Hz = ~22.7ms per frame)
                // send_frame includes break timing (~200μs) + MAB (~20μs) + frame transmission (~20ms at 250kbaud)
                // Sleep for remaining time to maintain consistent refresh rate
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

    /// Get current DMX state
    pub fn get_current_state(&self) -> DmxLaserState {
        self.current_state.lock().unwrap().clone()
    }

    /// Set individual DMX channel value
    pub fn set_dmx_channel(&self, channel: u8, value: u8) -> Result<(), Box<dyn std::error::Error>> {
        if channel < 1 || channel > 16 {
            return Err("DMX channel must be 1-16".into());
        }
        
        let mut state = self.current_state.lock().unwrap();
        
        match channel {
            1 => state.master_dimmer = value,
            2 => state.color_control = value,
            3 => state.color_speed = value,
            4 => state.pattern_group = value,
            5 => state.pattern_select = value,
            6 => state.dynamic_effects = value,
            7 => state.effect_speed = value,
            8 => state.pattern_size = value,
            9 => state.size_control = value,
            10 => state.rotation = value,
            11 => state.vertical_flip = value,
            12 => state.horizontal_flip = value,
            13 => state.horizontal_pos = value,
            14 => state.vertical_pos = value,
            15 => state.wave_effect = value,
            16 => state.manual_drawing = value,
            _ => unreachable!(),
        }
        
        debug!("Set DMX CH{}: {}", channel, value);
        Ok(())
    }

    /// Convert laser state to DMX frame
    fn state_to_dmx_frame(state: &DmxLaserState, dmx_frame: &mut DmxFrame, start_channel: usize) {
        dmx_frame.set_channel(start_channel, state.master_dimmer);
        dmx_frame.set_channel(start_channel + 1, state.color_control);
        dmx_frame.set_channel(start_channel + 2, state.color_speed);
        dmx_frame.set_channel(start_channel + 3, state.pattern_group);
        dmx_frame.set_channel(start_channel + 4, state.pattern_select);
        dmx_frame.set_channel(start_channel + 5, state.dynamic_effects);
        dmx_frame.set_channel(start_channel + 6, state.effect_speed);
        dmx_frame.set_channel(start_channel + 7, state.pattern_size);
        dmx_frame.set_channel(start_channel + 8, state.size_control);
        dmx_frame.set_channel(start_channel + 9, state.rotation);
        dmx_frame.set_channel(start_channel + 10, state.vertical_flip);
        dmx_frame.set_channel(start_channel + 11, state.horizontal_flip);
        dmx_frame.set_channel(start_channel + 12, state.horizontal_pos);
        dmx_frame.set_channel(start_channel + 13, state.vertical_pos);
        dmx_frame.set_channel(start_channel + 14, state.wave_effect);
        dmx_frame.set_channel(start_channel + 15, state.manual_drawing);
    }


    /// Create DMX command for setting all channels
    pub fn create_full_dmx_command(&self, state: &DmxLaserState) -> Box<dyn DmxCommand> {
        let start_channel = self.dmx_start_channel;
        let state_clone = state.clone();
        
        Box::new(move |dmx_frame: &mut DmxFrame| {
            Self::state_to_dmx_frame(&state_clone, dmx_frame, start_channel);
        })
    }


}

impl Drop for DmxLaserDevice {
    fn drop(&mut self) {
        if let Err(e) = self.stop() {
            error!("Error stopping DMX laser device: {}", e);
        }
    }
}

