
use crate::device::LaserDevice;
use crate::dmx::{DmxFrame, DmxController, DmxCommand};
use crate::dmxchannel::{DIMMER_CHANNEL, OFF,ON};
use crate::model::{
    DeviceInfo, DmxLaserState, DrawData, DrawItem, MainCommandData, PisObject, PlaybackCommand, PlaybackMode, Point, SettingsData
};

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use log::{info, error, debug, warn};

/// DMX-based laser device implementation following the official 16-channel DMX specification
pub struct DmxLaserDevice {
    dmx_controller: Arc<Mutex<DmxController>>,
    device_info: DeviceInfo,
    current_state: Arc<Mutex<DmxLaserState>>,
    dmx_start_channel: usize,
    is_running: Arc<Mutex<bool>>,
}


impl DmxLaserDevice {

    pub fn new(port_name: &str, dmx_start_channel: usize) -> Result<Self, Box<dyn std::error::Error>> {
        info!("Creating DMX laser device on port: {}, starting at channel: {}", port_name, dmx_start_channel);
        
        let dmx_controller = DmxController::new(port_name, dmx_start_channel)?;
        
        let device_info = DeviceInfo {
            device_on: true,
            device_type: "DMX_LASER_PROJECTOR".to_string(),
            version: "1.0.0".to_string(),
            user_type: "DMX_COMPATIBLE".to_string(),
        };

        Ok(Self {
            dmx_controller: Arc::new(Mutex::new(dmx_controller)),
            device_info,
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
                
                // Log DMX values every 100 frames (about every 2-3 seconds)
                frame_count += 1;
                if frame_count % 100 == 0 {
                    debug!("DMX Frame #{}: CH1={} CH2={} CH3={} CH4={} CH5={} CH6={}", 
                        frame_count,
                        current_state.master_dimmer,
                        current_state.color_control, 
                        current_state.color_speed,
                        current_state.pattern_group,
                        current_state.pattern_select,
                        current_state.dynamic_effects
                    );
                }
                
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

    /// Execute a playback command using DMX channels
    pub fn execute_playback_command(&self, command: &PlaybackCommand) -> Result<(), Box<dyn std::error::Error>> {
        info!("Executing DMX playback command: {:?}", command.mode);
        
        let mut state = self.current_state.lock().unwrap();
        
        // Map playback command to DMX channels
        self.playback_command_to_dmx_state(command, &mut state);
        
        Ok(())
    }

    /// Send draw points using DMX position channels
    pub fn send_draw_points(&self, points: &[Point]) -> Result<(), Box<dyn std::error::Error>> {
        info!("Sending {} draw points via DMX position control", points.len());
        
        if points.is_empty() {
            return Ok(());
        }

        let mut state = self.current_state.lock().unwrap();
        
        // Enable light for drawing
        state.master_dimmer = 255;
        
        // Use first point for static position (DMX doesn't handle point sequences directly)
        let point = &points[0];
        
        // Map X coordinate (0.0-800.0) to DMX range (0-127 for position)
        state.horizontal_pos = ((point.x / 800.0 * 127.0) as u8).min(127);
        
        // Map Y coordinate (0.0-800.0) to DMX range (0-127 for position)  
        state.vertical_pos = ((point.y / 800.0 * 127.0) as u8).min(127);
        
        // Set color from point
        state.color_control = self.laser_color_to_dmx_color(point.color);
        
        // For point sequences, we could implement manual drawing mode
        if points.len() > 1 {
            state.manual_drawing = 32; // Manual gradual drawing mode 1
        }
        
        Ok(())
    }

    /// Draw method compatible with LaserDevice interface - async version
    /// This follows the same pattern as the Bluetooth device's encodeDrawPointCommand
    pub async fn draw(&self, points: Vec<Point>, config: PisObject) {
        info!("DMX draw: {} points (following encodeDrawPointCommand pattern)", points.len());
        
        if points.is_empty() {
            warn!("No points provided for drawing");
            return;
        }

        let mut state = self.current_state.lock().unwrap();
        
        // Enable light for drawing (equivalent to command header)
        state.master_dimmer = 255;
        
        // Apply PisObject configuration to DMX channels 9-16 (matches config.cnfValus[0-11])
        self.apply_pis_object_to_dmx(&mut state, &config);
        
        // Configure drawing based on points (matches point encoding)
        self.encode_points_to_dmx(&mut state, &points, &config);
        
        info!("DMX draw configured using encodeDrawPointCommand pattern");
    }

    /// Encode points to DMX following the encodeDrawPointCommand pattern
    fn encode_points_to_dmx(&self, state: &mut DmxLaserState, points: &[Point], config: &PisObject) {
        if points.is_empty() {
            return;
        }
        
        // Enable manual drawing mode (equivalent to drawing command activation)
        state.manual_drawing = 48; // Manual drawing mode 2
        
        // Process first point for initial positioning
        let first_point = &points[0];
        
        // Map coordinates from Point to DMX channels (following point[0], point[1] encoding)
        state.horizontal_pos = ((first_point.x / 800.0 * 127.0) as u8).min(127);
        state.vertical_pos = ((first_point.y / 800.0 * 127.0) as u8).min(127);
        
        // Map color from Point to DMX (following point[2] encoding)
        state.color_control = self.laser_color_to_dmx_color(first_point.color);
        
        // Handle pen state (following point[3] encoding)
        // pen_state: 0=move, 1=draw
        if first_point.pen_state == 0 {
            // Move operation - reduce intensity but keep light on
            state.master_dimmer = 64; // Dim for move
        } else {
            // Draw operation - full intensity
            state.master_dimmer = 255;
        }
        
        // For multi-point drawings, enable advanced drawing features
        if points.len() > 1 {
            // Analyze drawing path for automatic configuration
            self.configure_multipoint_drawing(state, points, config);
        }
        
        debug!("Encoded {} points to DMX: pos=({},{}), color={}, pen_state={}", 
               points.len(), state.horizontal_pos, state.vertical_pos, 
               first_point.color, first_point.pen_state);
    }

    /// Configure DMX for multi-point drawing (following encodeDrawPointCommand's batch processing)
    fn configure_multipoint_drawing(&self, state: &mut DmxLaserState, points: &[Point], config: &PisObject) {
        // Calculate drawing bounds for auto-scaling
        let (min_x, max_x, min_y, max_y) = points.iter().fold(
            (f64::INFINITY, f64::NEG_INFINITY, f64::INFINITY, f64::NEG_INFINITY),
            |(min_x, max_x, min_y, max_y), point| {
                (min_x.min(point.x), max_x.max(point.x), 
                 min_y.min(point.y), max_y.max(point.y))
            }
        );
        
        let width = max_x - min_x;
        let height = max_y - min_y;
        let drawing_size = width.max(height);
        
        // Auto-scale pattern size based on drawing bounds (follows JavaScript auto-sizing logic)
        let size_factor = (drawing_size / 800.0).clamp(0.1, 2.0);
        state.pattern_size = (size_factor * 255.0) as u8;
        
        // Set drawing speed based on point count (equivalent to pointTimeValue)
        let point_density = points.len() as f64 / drawing_size.max(1.0);
        let speed_factor = (point_density * 10.0).clamp(1.0, 255.0);
        state.effect_speed = speed_factor as u8;
        
        // Check for color changes in the path
        let color_changes = points.windows(2).filter(|pair| pair[0].color != pair[1].color).count();
        if color_changes > 0 {
            // Enable color cycling if multiple colors detected
            state.color_speed = 150; // Medium color change speed
        }
        
        // Detect drawing patterns for advanced effects
        let pen_changes = points.windows(2).filter(|pair| pair[0].pen_state != pair[1].pen_state).count();
        if pen_changes > points.len() / 4 {
            // Many pen up/down changes - likely a complex drawing
            state.wave_effect = 30; // Add wave effect for dynamic appearance
        }
        
        debug!("Multi-point drawing config: size={}, speed={}, color_changes={}, pen_changes={}", 
               size_factor, speed_factor, color_changes, pen_changes);
    }

    /// Execute a draw command using DMX manual drawing and position control
    pub fn draw_command(&self, draw_data: &DrawData) -> Result<(), Box<dyn std::error::Error>> {
        info!("Executing DMX draw command with {} draw items", draw_data.draw_points.len());
        
        if draw_data.draw_points.is_empty() {
            warn!("No draw items provided");
            return Ok(());
        }

        let mut state = self.current_state.lock().unwrap();
        
        // Enable light for drawing
        state.master_dimmer = 255;
        
        // Enable manual drawing mode
        state.manual_drawing = 32; // Manual gradual drawing mode 1
        
        // Get the first draw item for initial positioning and configuration
        let first_item = &draw_data.draw_points[0];
        self.configure_dmx_from_draw_item(&mut state, first_item);
        
        // Apply PisObject configuration to advanced DMX channels
        self.apply_pis_object_to_dmx(&mut state, &draw_data.pis_obj);
        
        info!("DMX draw command configured - manual drawing mode enabled");
        Ok(())
    }

    /// Execute a sequence of draw items with timing control
    pub fn draw_sequence(&self, draw_items: &[DrawItem], timing_ms: u64) -> Result<(), Box<dyn std::error::Error>> {
        info!("Executing DMX draw sequence with {} items, {}ms timing", draw_items.len(), timing_ms);
        
        for (index, item) in draw_items.iter().enumerate() {
            info!("Drawing item {} of {}", index + 1, draw_items.len());
            
            let mut state = self.current_state.lock().unwrap();
            
            // Configure state for this draw item
            self.configure_dmx_from_draw_item(&mut state, item);
            
            // Release lock before sleeping
            drop(state);
            
            // Wait between draw items
            if index < draw_items.len() - 1 {
                thread::sleep(Duration::from_millis(timing_ms));
            }
        }
        
        Ok(())
    }

    /// Update settings using DMX channels
    pub fn update_settings(&self, settings: &SettingsData) -> Result<(), Box<dyn std::error::Error>> {
        info!("Updating DMX settings");
        
        let mut state = self.current_state.lock().unwrap();
        
        // Map settings to DMX channels
        state.master_dimmer = if settings.light > 1 { 255 } else { 0 };
        
        // Map RGB values to color control
        let (r, g, b) = (settings.values[2] as u8, settings.values[3] as u8, settings.values[4] as u8);
        state.color_control = self.rgb_to_dmx_color(r, g, b);
        
        // Map XY config to rotation
        state.rotation = settings.xy.saturating_mul(2); // Scale to DMX range
        
        Ok(())
    }

    /// Get current device information
    pub fn get_device_info(&self) -> &DeviceInfo {
        &self.device_info
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

    /// Map playback command to DMX state
    fn playback_command_to_dmx_state(&self, command: &PlaybackCommand, state: &mut DmxLaserState) {
        // Enable light
        //state.master_dimmer = 255;
        
        // Map playback mode to pattern group and dynamic effects
        match command.mode {
            PlaybackMode::Dmx => {
                state.dynamic_effects = 0; // No function - pure DMX control
            },
            PlaybackMode::RandomPlayback => {
                state.dynamic_effects = 251; // All effects random play
            },
            PlaybackMode::LineGeometryPlayback => {
                state.pattern_group = 12; // Static group 1 (geometric)
                state.dynamic_effects = 211; // Line effect random play
            },
            PlaybackMode::AnimationPlayback => {
                state.pattern_group = 137; // Animation group 1
                state.dynamic_effects = 221; // Animation effect random play
            },
            PlaybackMode::TextPlayback => {
                state.pattern_group = 12; // Static group for text
                state.manual_drawing = 32; // Manual drawing for text
            },
            PlaybackMode::ChristmasPlayback => {
                state.pattern_group = 112; // Christmas patterns
                state.dynamic_effects = 231; // Christmas effect random play
            },
            PlaybackMode::OutdoorPlayback => {
                state.pattern_group = 25; // Static group 2
                state.dynamic_effects = 241; // Outdoor effect random play
            },
            PlaybackMode::Program => {
                state.dynamic_effects = 100; // Built-in dynamic effect
            },
            PlaybackMode::Draw => {
                state.manual_drawing = 32; // Manual drawing mode
            },
            PlaybackMode::Playlist => {
                state.dynamic_effects = 251; // All effects random play
            },
        }
        
        // Apply command parameters
        if let Some(color) = command.color {
            state.color_control = self.laser_color_to_dmx_color(color);
        }
        
        if let Some(speed) = command.playback_speed {
            state.effect_speed = speed;
            state.color_speed = speed;
        }
        
        if let Some(audio_sens) = command.audio_sensitivity {
            // Map audio sensitivity to wave effect (approximate)
            state.wave_effect = audio_sens;
        }
    }

    /// Map laser color index to DMX color control value
    fn laser_color_to_dmx_color(&self, color: u8) -> u8 {
        match color {
            0 => 0,  // Black/Off -> White (will be controlled by dimmer)
            1 => 15, // Red
            2 => 65, // Green  
            3 => 25, // Blue
            4 => 55, // Yellow
            5 => 45, // Cyan
            6 => 35, // Pink/Magenta
            7 => 0,  // White
            _ => 91, // Default to rainbow for unknown colors
        }
    }

    /// Map RGB values to closest DMX color control value
    fn rgb_to_dmx_color(&self, r: u8, g: u8, b: u8) -> u8 {
        // Find closest fixed color match
        if r > 200 && g > 200 && b > 200 { return 0; }  // White
        if r > 200 && g < 100 && b < 100 { return 15; } // Red
        if r < 100 && g > 200 && b < 100 { return 65; } // Green
        if r < 100 && g < 100 && b > 200 { return 25; } // Blue
        if r > 200 && g > 200 && b < 100 { return 55; } // Yellow
        if r < 100 && g > 200 && b > 200 { return 45; } // Cyan
        if r > 200 && g < 100 && b > 200 { return 35; } // Pink
        
        // Default to gradient mode for complex colors
        254
    }

    /// Create DMX command for setting all channels
    pub fn create_full_dmx_command(&self, state: &DmxLaserState) -> Box<dyn DmxCommand> {
        let start_channel = self.dmx_start_channel;
        let state_clone = state.clone();
        
        Box::new(move |dmx_frame: &mut DmxFrame| {
            Self::state_to_dmx_frame(&state_clone, dmx_frame, start_channel);
        })
    }

    /// Generate laser command from current DMX state (for hybrid operation)
    pub fn dmx_state_to_laser_command(&self, state: &DmxLaserState) -> (SettingsData, MainCommandData, PisObject) {
        // Settings from DMX channels
        let settings = SettingsData {
            values: [
                self.dmx_start_channel as u16,
                if state.master_dimmer > 9 { 255 } else { 0 },
                self.dmx_color_to_rgb_r(state.color_control) as u16,
                self.dmx_color_to_rgb_g(state.color_control) as u16,
                self.dmx_color_to_rgb_b(state.color_control) as u16,
            ],
            channel: self.dmx_start_channel as u8,
            dmx: 1, // DMX mode enabled
            xy: (state.rotation / 2).min(7), // Scale down to XY config range
            light: if state.master_dimmer > 9 { 3 } else { 1 },
            cfg: 0, // TTL mode
            lang: "en".to_string(),
        };

        // Main command from DMX channels
        let main_data = MainCommandData {
            current_mode: self.dmx_to_laser_mode(state.pattern_group, state.dynamic_effects),
            text_color: (state.color_control % 10).min(7),
            text_size: state.pattern_size,
            run_speed: if state.effect_speed <= 1 { 128 } else { state.effect_speed },
            text_distance: 50,
            audio_mode: if state.wave_effect > 1 { 1 } else { 0 },
            sound_value: state.wave_effect,
            text_point_time: 50,
            draw_point_time: 50,
            run_direction: if state.color_speed > 127 { 1 } else { 0 },
        };

        // PisObject from advanced DMX channels
        let pis_obj = PisObject {
            tx_point_time: 50,
            cnf_valus: [
                state.size_control as u32,
                state.rotation as u32,
                state.vertical_flip as u32,
                state.horizontal_flip as u32,
                state.horizontal_pos as u32,
                state.vertical_pos as u32,
                state.wave_effect as u32,
                state.manual_drawing as u32,
                0, 0, 0, 0, 0
            ],
        };

        (settings, main_data, pis_obj)
    }

    /// Map DMX color value back to RGB components
    fn dmx_color_to_rgb_r(&self, dmx_color: u8) -> u8 {
        match dmx_color {
            0..=9 => 255,   // White
            10..=19 => 255, // Red
            20..=29 => 0,   // Blue  
            30..=39 => 255, // Pink
            40..=49 => 0,   // Cyan
            50..=59 => 255, // Yellow
            60..=69 => 0,   // Green
            _ => 128        // Dynamic/other
        }
    }

    fn dmx_color_to_rgb_g(&self, dmx_color: u8) -> u8 {
        match dmx_color {
            0..=9 => 255,   // White
            10..=19 => 0,   // Red
            20..=29 => 0,   // Blue
            30..=39 => 192, // Pink
            40..=49 => 255, // Cyan
            50..=59 => 255, // Yellow
            60..=69 => 255, // Green
            _ => 128        // Dynamic/other
        }
    }

    fn dmx_color_to_rgb_b(&self, dmx_color: u8) -> u8 {
        match dmx_color {
            0..=9 => 255,   // White
            10..=19 => 0,   // Red
            20..=29 => 255, // Blue
            30..=39 => 203, // Pink  
            40..=49 => 255, // Cyan
            50..=59 => 0,   // Yellow
            60..=69 => 0,   // Green
            _ => 128        // Dynamic/other
        }
    }

    /// Map DMX pattern group and effects to laser mode
    fn dmx_to_laser_mode(&self, pattern_group: u8, dynamic_effects: u8) -> u8 {
        if dynamic_effects > 1 {
            match dynamic_effects {
                207..=216 => 2, // Line effects
                217..=226 => 3, // Animation effects
                227..=236 => 5, // Christmas effects
                237..=246 => 6, // Outdoor effects
                247..=255 => 1, // Random play
                _ => 0          // DMX mode for built-in effects
            }
        } else {
            match pattern_group {
                100..=124 => 5, // Christmas patterns
                125..=255 => 3, // Animation patterns
                _ => 2          // Geometric patterns
            }
        }
    }

    /// Configure DMX state from a vector of points
    fn configure_dmx_from_points(&self, state: &mut DmxLaserState, points: &[Point]) {
        if points.is_empty() {
            return;
        }
        
        // Use first point for initial position and color
        let first_point = &points[0];
        
        // Set color from first point
        state.color_control = self.laser_color_to_dmx_color(first_point.color);
        
        // Map X coordinate (0.0-800.0) to DMX range (0-127 for position)
        state.horizontal_pos = ((first_point.x / 800.0 * 127.0) as u8).min(127);
        
        // Map Y coordinate (0.0-800.0) to DMX range (0-127 for position)  
        state.vertical_pos = ((first_point.y / 800.0 * 127.0) as u8).min(127);
        
        // If we have multiple points, enable manual drawing mode for path following
        if points.len() > 1 {
            state.manual_drawing = 32; // Manual gradual drawing mode 1
            
            // Calculate drawing bounds for auto-sizing
            let (min_x, max_x) = points.iter()
                .map(|p| p.x)
                .fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), x| (min.min(x), max.max(x)));
            let (min_y, max_y) = points.iter()
                .map(|p| p.y)
                .fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), y| (min.min(y), max.max(y)));
            
            let width = max_x - min_x;
            let height = max_y - min_y;
            let drawing_size = width.max(height);
            
            // Auto-scale pattern size based on drawing bounds
            let size_factor = (drawing_size / 800.0).clamp(0.1, 2.0);
            state.pattern_size = (size_factor * 127.0) as u8;
        }
        
        debug!("Configured DMX from {} points: pos=({},{}), color={}, manual_drawing={}", 
               points.len(), state.horizontal_pos, state.vertical_pos, first_point.color, state.manual_drawing);
    }

    /// Configure DMX state from a DrawItem
    fn configure_dmx_from_draw_item(&self, state: &mut DmxLaserState, item: &DrawItem) {
        // Set color from line color
        state.color_control = self.laser_color_to_dmx_color(item.line_color);
        
        // Map draw item transformations to DMX channels
        
        // Position: Apply translation (x0, y0) to position channels
        // Map drawing coordinates (-400 to 400) to DMX position range (0-127)
        let x_normalized = ((item.x0 + 400.0) / 800.0 * 127.0).clamp(0.0, 127.0) as u8;
        let y_normalized = ((item.y0 + 400.0) / 800.0 * 127.0).clamp(0.0, 127.0) as u8;
        
        state.horizontal_pos = x_normalized;
        state.vertical_pos = y_normalized;
        
        // Rotation: Map angle (0-360 degrees) to DMX rotation (0-127)
        let rotation_normalized = ((item.ang % 360.0) / 360.0 * 127.0) as u8;
        state.rotation = rotation_normalized;
        
        // Scale: Map z factor to pattern size
        let size_normalized = (item.z.clamp(0.1, 5.0) / 5.0 * 255.0) as u8;
        state.pattern_size = size_normalized;
        
        // Draw mode influences pattern group selection
        match item.draw_mode {
            crate::model::DrawMode::Shape => {
                state.pattern_group = 12; // Static group 1 (geometric shapes)
            }
            crate::model::DrawMode::Polylines => {
                state.pattern_group = 25; // Static group 2 (lines)
                state.manual_drawing = 32; // Enable manual drawing for polylines
            }
            crate::model::DrawMode::Text => {
                state.pattern_group = 12; // Static group 1 for text
                state.manual_drawing = 48; // Manual drawing mode for text
            }
        }
        
        debug!("Configured DMX from DrawItem: pos=({},{}), rotation={}, size={}, color={}", 
               x_normalized, y_normalized, rotation_normalized, size_normalized, item.line_color);
    }

    /// Apply PisObject configuration to advanced DMX channels
    /// This follows the exact mapping from encodeDrawPointCommand's config.cnfValus[0-11]
    fn apply_pis_object_to_dmx(&self, state: &mut DmxLaserState, pis_obj: &PisObject) {
        // Direct mapping from encodeDrawPointCommand pattern:
        // config.cnfValus[0-11] -> DMX CH9-16 advanced control
        
        if pis_obj.cnf_valus.len() >= 12 {
            // Map configuration values exactly as encodeDrawPointCommand does
            // cnfValus[0] -> CH9: size_control
            state.size_control = (pis_obj.cnf_valus[0].min(255)) as u8;
            
            // cnfValus[1] -> CH10: rotation (0-360 degrees -> 0-127 DMX range)
            state.rotation = (pis_obj.cnf_valus[1].min(360) * 127 / 360) as u8;
            
            // cnfValus[2] -> CH11: vertical_flip
            state.vertical_flip = (pis_obj.cnf_valus[2].min(255)) as u8;
            
            // cnfValus[3] -> CH12: horizontal_flip  
            state.horizontal_flip = (pis_obj.cnf_valus[3].min(255)) as u8;
            
            // cnfValus[4] -> CH13: horizontal_pos (0-100 -> 0-127 DMX position range)
            state.horizontal_pos = (pis_obj.cnf_valus[4].min(100) * 127 / 100) as u8;
            
            // cnfValus[5] -> CH14: vertical_pos (0-100 -> 0-127 DMX position range)
            state.vertical_pos = (pis_obj.cnf_valus[5].min(100) * 127 / 100) as u8;
            
            // cnfValus[6] -> CH15: wave_effect
            state.wave_effect = (pis_obj.cnf_valus[6].min(255)) as u8;
            
            // cnfValus[7] -> CH16: manual_drawing
            state.manual_drawing = (pis_obj.cnf_valus[7].min(255)) as u8;
            
            // Additional config values (cnfValus[8-11]) can be used for extended features
            if pis_obj.cnf_valus.len() >= 12 {
                // cnfValus[8] could modify pattern_group
                if pis_obj.cnf_valus[8] > 0 {
                    state.pattern_group = (pis_obj.cnf_valus[8].min(255)) as u8;
                }
                
                // cnfValus[9] could modify dynamic_effects
                if pis_obj.cnf_valus[9] > 0 {
                    state.dynamic_effects = (pis_obj.cnf_valus[9].min(255)) as u8;
                }
                
                // cnfValus[10] could modify color_speed
                if pis_obj.cnf_valus[10] > 0 {
                    state.color_speed = (pis_obj.cnf_valus[10].min(255)) as u8;
                }
                
                // cnfValus[11] could modify pattern_size
                if pis_obj.cnf_valus[11] > 0 {
                    state.pattern_size = (pis_obj.cnf_valus[11].min(255)) as u8;
                }
            }
        }
        
        // Map tx_point_time to effect speed (following encodeDrawPointCommand's timing logic)
        // JavaScript: 10 * config.cnfValus[12] or 10 * pointTimeValue
        let effect_speed = if pis_obj.tx_point_time > 0 {
            // Convert point time to effect speed (matches JavaScript timing calculation)
            let timing_factor = (pis_obj.tx_point_time * 10).min(2550); // 10x multiplier like JS
            (255 - (timing_factor / 10).min(255)) as u8 // Invert for speed (higher time = slower)
        } else {
            128 // Default medium speed
        };
        state.effect_speed = effect_speed;
        
        debug!("Applied PisObject (encodeDrawPointCommand pattern): tx_point_time={} -> effect_speed={}", 
               pis_obj.tx_point_time, effect_speed);
    }

    /// Create a DMX-compatible version of encodeDrawPointCommand
    /// Returns a string similar to the JavaScript function's hex output
    pub fn encode_draw_point_command_dmx(&self, points: &[Point], config: &PisObject) -> String {
        let mut command = String::new();
        
        // Add configuration header (mimics JavaScript config encoding)
        for i in 0..12 {
            let value = if i < config.cnf_valus.len() {
                config.cnf_valus[i].min(255) as u8
            } else {
                0
            };
            command.push_str(&format!("{:02X}", value));
        }
        
        // Add point time configuration (index 13-14 equivalent)
        command.push_str(&format!("{:02X}", (config.tx_point_time * 10).min(255) as u8));
        command.push_str("00"); // Padding equivalent to JavaScript
        
        // Add point count
        command.push_str(&format!("{:02X}", points.len().min(255)));
        
        // Add points data (following JavaScript point encoding)
        for (i, point) in points.iter().enumerate() {
            // X coordinate (point[0])
            let x = (point.x as u16).min(65535);
            command.push_str(&format!("{:04X}", x));
            
            // Y coordinate (point[1]) 
            let y = (point.y as u16).min(65535);
            command.push_str(&format!("{:04X}", y));
            
            // Color and pen state combined (point[2] and point[3])
            let color = point.color & 0x0F;
            let pen_state = point.pen_state & 0x0F;
            let combined = (color << 4) | pen_state;
            command.push_str(&format!("{:02X}", combined));
        }
        
        debug!("Generated DMX draw command (encodeDrawPointCommand format): {} chars", command.len());
        command
    }
}

impl Drop for DmxLaserDevice {
    fn drop(&mut self) {
        if let Err(e) = self.stop() {
            error!("Error stopping DMX laser device: {}", e);
        }
    }
}

impl LaserDevice for DmxLaserDevice {

    async fn setup(&self) {
        if let Err(e) = self.start() {
            error!("Failed to start DMX laser device: {}", e);
        }
    }
    
    async fn on(&self) {
        let _ = self.set_dmx_channel(DIMMER_CHANNEL, ON);
    }
    
    async fn off(&self) {
        let _ = self.set_dmx_channel(DIMMER_CHANNEL, OFF);
    }
    
    fn get_settings(&self) -> Option<SettingsData> {
        todo!()
    }
    
    async fn set_settings(&self, new_settings: SettingsData) {
        todo!()
    }
    
    async fn set_playback_mode(&self, command: PlaybackCommand) {
        todo!()
    }
    
    fn is_on(&self) -> bool {
        todo!()
    }
    
    async fn draw(&self, points: Vec<Point>, config: PisObject) {
        todo!()
    }
    
}
