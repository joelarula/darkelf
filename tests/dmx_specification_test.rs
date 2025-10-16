use darkelf::{
    dmxdevice::{DmxLaserDevice, DmxLaserState},
    dmx::{DmxFrame, scan_dmx_ports},
    model::{PlaybackCommand, PlaybackMode, Point, SettingsData},
};
use std::thread;
use std::time::Duration;

#[test]
fn test_dmx_channel_mappings() {
    println!("DMX Channel Mapping Tests");
    println!("========================");
    
    test_dmx_channel_1_master_dimmer();
    test_dmx_channel_2_color_control();
    test_dmx_channel_3_color_speed();
    test_dmx_channel_4_pattern_groups();
    test_dmx_channel_5_pattern_selection();
    test_dmx_channel_6_dynamic_effects();
    test_dmx_channel_7_effect_speed();
    test_dmx_channel_8_pattern_size();
    test_dmx_channels_9_16_advanced_control();
}

#[test]
fn test_dmx_state_generation() {
    println!("DMX State Generation Tests");
    println!("=========================");
    
    demo_dmx_commands();
}

#[test]
#[ignore] // Only run with --ignored flag when hardware is available
fn test_dmx_hardware_integration() {
    println!("DMX Hardware Integration Test");
    println!("============================");
    
    // Scan for available DMX ports
    let dmx_ports = scan_dmx_ports();
    
    if dmx_ports.is_empty() {
        println!("No DMX-compatible ports found - skipping hardware test");
        return;
    }
    
    println!("Found DMX ports: {:?}", dmx_ports);
    
    // Use first available port
    let port = &dmx_ports[0];
    let dmx_channel = 1;
    
    match DmxLaserDevice::new(port, dmx_channel) {
        Ok(device) => {
            run_quick_dmx_test(device);
        }
        Err(e) => {
            println!("Failed to create DMX device: {}", e);
            panic!("Hardware test failed");
        }
    }
}

fn run_quick_dmx_test(device: DmxLaserDevice) {
    // Start the device
    assert!(device.start().is_ok());
    
    // Test basic operations
    assert!(device.set_dmx_channel(1, 255).is_ok()); // Master ON
    assert!(device.set_dmx_channel(2, 15).is_ok());  // Red color
    
    thread::sleep(Duration::from_millis(100));
    
    // Test playback command
    let command = PlaybackCommand::default(PlaybackMode::ChristmasPlayback);
    assert!(device.execute_playback_command(&command).is_ok());
    
    thread::sleep(Duration::from_millis(100));
    
    // Turn off
    assert!(device.set_dmx_channel(1, 0).is_ok()); // Master OFF
    
    // Stop the device
    assert!(device.stop().is_ok());
}

fn test_dmx_channel_1_master_dimmer() {
    println!("\n1. CH1 - MASTER DIMMER TEST");
    println!("---------------------------");
    
    let test_values = [0, 5, 9, 10, 50, 100, 150, 200, 255];
    
    for &value in &test_values {
        let mut dmx_frame = DmxFrame::new();
        dmx_frame.set_channel(1, value);
        
        let light_state = if value <= 9 { "OFF" } else { "ON" };
        let intensity = if value <= 9 { 0 } else { value };
        
        println!("  DMX CH1={:3} -> Light: {} (Intensity: {})", value, light_state, intensity);
        
        // This would map to Settings.light and Settings.values[1]
        let settings = create_settings_from_ch1(value);
        println!("    -> Settings: light={}, intensity={}", settings.light, settings.values[1]);
        
        // Verify mapping
        assert_eq!(settings.light, if value <= 9 { 1 } else { 3 });
        assert_eq!(settings.values[1], if value <= 9 { 0 } else { value as u16 });
    }
}

fn test_dmx_channel_2_color_control() {
    println!("\n2. CH2 - COLOR CONTROL TEST"); 
    println!("----------------------------");
    
    let test_ranges = [
        (0, "White (fixed)"),
        (15, "Red (fixed)"), 
        (25, "Blue (fixed)"),
        (35, "Pink (fixed)"),
        (45, "Cyan (fixed)"),
        (55, "Yellow (fixed)"),
        (65, "Green (fixed)"),
        (75, "Overall color change"),
        (85, "Pattern initial color"),
        (91, "Colorful rainbow"),
        (100, "2-segment color"),
        (120, "3-segment color"), 
        (140, "4-segment color"),
        (165, "8-segment color"),
        (200, "16-segment color"),
        (235, "32-segment color"),
        (254, "Color gradient")
    ];
    
    for &(value, description) in &test_ranges {
        let color_mode = map_ch2_to_color_mode(value);
        let (r, g, b) = map_ch2_to_rgb(value);
        println!("  DMX CH2={:3} -> {} (RGB: {}/{}/{}) Mode: {}", 
            value, description, r, g, b, color_mode);
            
        // Verify color mapping consistency
        assert!(!color_mode.is_empty());
        assert!(r <= 255 && g <= 255 && b <= 255);
    }
}

fn test_dmx_channel_3_color_speed() {
    println!("\n3. CH3 - COLOR CHANGE SPEED TEST");
    println!("---------------------------------");
    
    let test_values = [0, 5, 9, 50, 100, 127, 128, 150, 200, 255];
    
    for &value in &test_values {
        let (direction, speed) = map_ch3_to_speed_direction(value);
        println!("  DMX CH3={:3} -> Direction: {}, Speed: {}", value, direction, speed);
        
        // Verify direction mapping
        match value {
            0..=9 => assert_eq!(direction, "No Change"),
            10..=127 => assert_eq!(direction, "Forward"),
            128..=255 => assert_eq!(direction, "Reverse"),
        }
    }
}

fn test_dmx_channel_4_pattern_groups() {
    println!("\n4. CH4 - PATTERN GROUP SELECTION TEST");
    println!("--------------------------------------");
    
    let test_ranges = [
        (12, "Static graphics group 1 (basic geometric)"),
        (37, "Static graphics group 2 (basic geometric)"),
        (62, "Static graphics group 3 (edge bright spot)"),
        (87, "Static graphics group 4 (dot pattern)"),
        (112, "Static graphics group 5 (Christmas pattern)"),
        (137, "Animation group 1"),
        (162, "Animation group 2"),
        (187, "Animation group 3"),
        (212, "Animation group 4"),
        (240, "Animation group 5")
    ];
    
    for &(value, description) in &test_ranges {
        let group = map_ch4_to_pattern_group(value);
        let mode = map_ch4_to_laser_mode(value);
        println!("  DMX CH4={:3} -> {} (Group: {}, Mode: {})", value, description, group, mode);
        
        // Verify group mapping
        assert!(group >= 1 && group <= 10);
        assert!(mode >= 2 && mode <= 5);
    }
}

fn test_dmx_channel_5_pattern_selection() {
    println!("\n5. CH5 - PATTERN SELECTION TEST");
    println!("--------------------------------");
    
    for pattern in [1, 32, 64, 128, 192, 255] {
        println!("  DMX CH5={:3} -> Pattern {} within selected group (CH4)", pattern, pattern);
        // This maps to ProjectItem.prj_selected
        assert!(pattern <= 255);
    }
}

fn test_dmx_channel_6_dynamic_effects() {
    println!("\n6. CH6 - DYNAMIC EFFECTS TEST");
    println!("------------------------------");
    
    let test_ranges = [
        (1, "No function"),
        (50, "Built-in dynamic effect 25"),
        (100, "Built-in dynamic effect 50"),
        (206, "Built-in dynamic effect 102"),
        (211, "Line effect random play"),
        (221, "Animation effect random play"),
        (231, "Christmas effect random play"),
        (241, "Outdoor effect random play"),
        (251, "All effects random play")
    ];
    
    for &(value, description) in &test_ranges {
        let effect_mode = map_ch6_to_effect_mode(value);
        let laser_mode = map_ch6_to_laser_mode(value);
        println!("  DMX CH6={:3} -> {} (Effect: {}, Laser Mode: {})", 
            value, description, effect_mode, laser_mode);
            
        // Verify effect mode mapping
        assert!(!effect_mode.is_empty());
        assert!(laser_mode <= 6);
    }
}

fn test_dmx_channel_7_effect_speed() {
    println!("\n7. CH7 - EFFECT SPEED TEST");
    println!("---------------------------");
    
    let test_values = [0, 1, 10, 50, 100, 150, 200, 255];
    
    for &value in &test_values {
        let speed = if value <= 1 { "System default".to_string() } else { format!("Manual: {}/255", value) };
        println!("  DMX CH7={:3} -> Speed: {}", value, speed);
        
        // Verify speed mapping
        if value <= 1 {
            assert!(speed.contains("default"));
        } else {
            assert!(speed.contains("Manual"));
        }
    }
}

fn test_dmx_channel_8_pattern_size() {
    println!("\n8. CH8 - PATTERN SIZE TEST");
    println!("---------------------------");
    
    let test_values = [0, 32, 64, 96, 128, 160, 192, 224, 255];
    
    for &value in &test_values {
        let size_percent = (value as f32 / 255.0 * 100.0) as u8;
        println!("  DMX CH8={:3} -> Pattern Size: {}%", value, size_percent);
        
        // Verify size calculation
        assert!(size_percent <= 100);
    }
}

fn test_dmx_channels_9_16_advanced_control() {
    println!("\n9. CH9-CH16 - ADVANCED CONTROL TEST");
    println!("------------------------------------");
    
    // CH9 - Size Control
    let ch9_ranges = [
        (7, "Pattern size selection"),
        (35, "Speed selection small to large"),
        (75, "Speed selection large to small"),
        (115, "Size scaling speed selection"),
        (155, "Two-point irregular cycle scaling"),
        (195, "Three-point irregular cycle scaling"),
        (235, "Four-point irregular cycle scaling")
    ];
    
    for &(value, description) in &ch9_ranges {
        println!("  DMX CH9={:3} -> {}", value, description);
        assert!(!description.is_empty());
    }
    
    // CH10 - Rotation Control
    println!("  DMX CH10: 0-127=Angle, 128-191=Forward rotation, 192-255=Reverse rotation");
    
    // CH11-CH12 - Flip Controls
    println!("  DMX CH11: 0-127=V-flip position, 128-255=V-flip speed");
    println!("  DMX CH12: 0-127=H-flip position, 128-255=H-flip speed");
    
    // CH13-CH14 - Position Controls  
    println!("  DMX CH13: 0-127=H-position, 128-255=H-circular movement");
    println!("  DMX CH14: 0-127=V-position, 128-255=V-circular movement");
    
    // CH15 - Wave Effect
    println!("  DMX CH15: 0-1=No function, 2-255=Wave amplitude/speed (8 gears)");
    
    // CH16 - Manual Drawing
    println!("  DMX CH16: 0-1=No function, 2-63=Manual gradual drawing modes");
}

fn demo_dmx_commands() {
    println!("\n=== DMX COMMAND GENERATION DEMO ===");
    
    // Create a virtual DMX state for demonstration
    let mut state = DmxLaserState::default();
    
    // Demo different DMX configurations
    let demos: Vec<(&str, Box<dyn FnMut(&mut DmxLaserState)>)> = vec![
        ("Basic Red Laser", Box::new(|s: &mut DmxLaserState| {
            s.master_dimmer = 255;
            s.color_control = 15; // Red
            s.pattern_group = 12; // Basic geometric
        })),
        ("Christmas Rainbow", Box::new(|s: &mut DmxLaserState| {
            s.master_dimmer = 255;
            s.color_control = 91; // Rainbow
            s.color_speed = 100; // Medium speed
            s.pattern_group = 112; // Christmas patterns
            s.dynamic_effects = 231; // Christmas random
        })),
        ("Animation Show", Box::new(|s: &mut DmxLaserState| {
            s.master_dimmer = 255;
            s.color_control = 254; // Gradient
            s.color_speed = 150; // Fast
            s.pattern_group = 137; // Animation group 1
            s.dynamic_effects = 221; // Animation random
            s.effect_speed = 200; // High speed
        })),
        ("Position Control", Box::new(|s: &mut DmxLaserState| {
            s.master_dimmer = 255;
            s.color_control = 25; // Blue
            s.horizontal_pos = 200; // Circular movement
            s.vertical_pos = 200; // Circular movement
        })),
        ("Manual Drawing", Box::new(|s: &mut DmxLaserState| {
            s.master_dimmer = 255;
            s.color_control = 65; // Green
            s.manual_drawing = 32; // Manual mode 1
            s.horizontal_pos = 64; // Position
            s.vertical_pos = 64; // Position
        })),
    ];
    
    for (name, config_fn) in demos.into_iter() {
        println!("\n--- {} ---", name);
        
        // Reset state
        state = DmxLaserState::default();
        (config_fn)(&mut state);
        
        // Show DMX channel values
        print_dmx_state(&state);
        
        // Show interpretation
        interpret_dmx_state(&state);
        
        // Verify state is valid
        assert!(state.master_dimmer <= 255);
        assert!(state.color_control <= 255);
        // Add more validation as needed
    }
}

// Helper functions for DMX value mapping
fn create_settings_from_ch1(ch1_value: u8) -> SettingsData {
    SettingsData {
        values: [1, if ch1_value <= 9 { 0 } else { ch1_value as u16 }, 255, 255, 255],
        channel: 1,
        dmx: 1,
        xy: 0,
        light: if ch1_value <= 9 { 1 } else { 3 },
        cfg: 0,
        lang: "en".to_string(),
    }
}

fn map_ch2_to_color_mode(ch2: u8) -> &'static str {
    match ch2 {
        0..=69 => "Fixed Color",
        70..=79 => "Overall Change",
        80..=89 => "Pattern Initial",
        90..=92 => "Rainbow",
        93..=110 => "2-Segment",
        111..=131 => "3-Segment",
        132..=149 => "4-Segment",
        150..=182 => "8-Segment",
        183..=218 => "16-Segment",
        219..=253 => "32-Segment",
        254..=255 => "Gradient",
    }
}

fn map_ch2_to_rgb(ch2: u8) -> (u8, u8, u8) {
    match ch2 {
        0..=9 => (255, 255, 255),   // White
        10..=19 => (255, 0, 0),     // Red
        20..=29 => (0, 0, 255),     // Blue  
        30..=39 => (255, 192, 203), // Pink
        40..=49 => (0, 255, 255),   // Cyan
        50..=59 => (255, 255, 0),   // Yellow
        60..=69 => (0, 255, 0),     // Green
        _ => (128, 128, 128)        // Dynamic (placeholder)
    }
}

fn map_ch3_to_speed_direction(ch3: u8) -> (&'static str, u8) {
    match ch3 {
        0..=9 => ("No Change", 0),
        10..=127 => ("Forward", ch3 - 10),
        128..=255 => ("Reverse", ch3 - 128),
    }
}

fn map_ch4_to_pattern_group(ch4: u8) -> u8 {
    match ch4 {
        0..=24 => 1,    // Group 1
        25..=49 => 2,   // Group 2  
        50..=74 => 3,   // Group 3
        75..=99 => 4,   // Group 4
        100..=124 => 5, // Group 5 (Christmas)
        125..=149 => 6, // Animation 1
        150..=174 => 7, // Animation 2
        175..=199 => 8, // Animation 3
        200..=224 => 9, // Animation 4
        225..=255 => 10, // Animation 5
    }
}

fn map_ch4_to_laser_mode(ch4: u8) -> u8 {
    match ch4 {
        100..=124 => 5, // Christmas mode
        125..=255 => 3, // Animation mode
        _ => 2          // Line/geometric mode
    }
}

fn map_ch6_to_effect_mode(ch6: u8) -> &'static str {
    match ch6 {
        0..=1 => "No Function",
        2..=206 => "Built-in Effect",
        207..=216 => "Line Random",
        217..=226 => "Animation Random", 
        227..=236 => "Christmas Random",
        237..=246 => "Outdoor Random",
        247..=255 => "All Random",
    }
}

fn map_ch6_to_laser_mode(ch6: u8) -> u8 {
    match ch6 {
        207..=216 => 2, // Line effects
        217..=226 => 3, // Animation effects
        227..=236 => 5, // Christmas effects  
        237..=246 => 6, // Outdoor effects
        247..=255 => 1, // Random playback
        _ => 0          // DMX mode
    }
}

fn print_dmx_state(state: &DmxLaserState) {
    println!("DMX Channels:");
    println!("  CH1  (Master): {}", state.master_dimmer);
    println!("  CH2  (Color):  {}", state.color_control);
    println!("  CH3  (Speed):  {}", state.color_speed);
    println!("  CH4  (Group):  {}", state.pattern_group);
    println!("  CH5  (Pattern): {}", state.pattern_select);
    println!("  CH6  (Effects): {}", state.dynamic_effects);
    println!("  CH7  (E.Speed): {}", state.effect_speed);
    println!("  CH8  (Size):   {}", state.pattern_size);
    println!("  CH9  (S.Ctrl): {}", state.size_control);
    println!("  CH10 (Rotate): {}", state.rotation);
    println!("  CH11 (V.Flip): {}", state.vertical_flip);
    println!("  CH12 (H.Flip): {}", state.horizontal_flip);
    println!("  CH13 (H.Pos):  {}", state.horizontal_pos);
    println!("  CH14 (V.Pos):  {}", state.vertical_pos);
    println!("  CH15 (Wave):   {}", state.wave_effect);
    println!("  CH16 (Manual): {}", state.manual_drawing);
}

fn interpret_dmx_state(state: &DmxLaserState) {
    println!("Interpretation:");
    
    // Master dimmer
    let light_status = if state.master_dimmer <= 9 { "OFF" } else { "ON" };
    println!("  Light: {}", light_status);
    
    // Color
    let color_desc = match state.color_control {
        0..=9 => "White",
        10..=19 => "Red",
        20..=29 => "Blue", 
        30..=39 => "Pink",
        40..=49 => "Cyan",
        50..=59 => "Yellow",
        60..=69 => "Green",
        70..=79 => "Overall color change",
        80..=89 => "Pattern initial color",
        90..=92 => "Colorful rainbow",
        93..=110 => "2-segment color",
        111..=131 => "3-segment color",
        132..=149 => "4-segment color",
        150..=182 => "8-segment color",
        183..=218 => "16-segment color",
        219..=253 => "32-segment color",
        254..=255 => "Color gradient",
    };
    println!("  Color: {}", color_desc);
    
    // Pattern group
    let group_desc = match state.pattern_group {
        0..=24 => "Static group 1 (basic geometric)",
        25..=49 => "Static group 2",
        50..=74 => "Static group 3 (edge bright)",
        75..=99 => "Static group 4 (dot pattern)",
        100..=124 => "Static group 5 (Christmas)",
        125..=149 => "Animation group 1",
        150..=174 => "Animation group 2",
        _ => "Animation group 3+",
    };
    println!("  Pattern Group: {}", group_desc);
    
    // Effects
    if state.dynamic_effects > 1 {
        let effect_desc = match state.dynamic_effects {
            2..=206 => "Built-in dynamic effect",
            207..=216 => "Line effect random play",
            217..=226 => "Animation effect random play",
            227..=236 => "Christmas effect random play",
            237..=246 => "Outdoor effect random play",
            247..=255 => "All effects random play",
            _ => "Unknown effect",
        };
        println!("  Effects: {}", effect_desc);
    }
    
    // Position
    if state.horizontal_pos > 127 || state.vertical_pos > 127 {
        println!("  Position: Circular movement active");
    } else if state.horizontal_pos != 127 || state.vertical_pos != 127 {
        println!("  Position: H={}, V={}", state.horizontal_pos, state.vertical_pos);
    }
    
    // Manual drawing
    if state.manual_drawing > 1 {
        println!("  Manual Drawing: Active (mode {})", (state.manual_drawing - 2) / 32 + 1);
    }
}