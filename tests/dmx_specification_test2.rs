use darkelf::{
    command::CommandGenerator,
    model::{SettingsData, MainCommandData, ProjectData, ProjectItem, PublicData, PisObject},
    dmx::{DmxFrame, DmxController},
};
use std::collections::HashMap;

fn main() {
    println!("DMX Protocol Implementation Analysis - Actual Device Specification");
    println!("================================================================");

    // Test each DMX channel according to the official specification
    test_dmx_channel_1_master_dimmer();
    test_dmx_channel_2_color_control();
    test_dmx_channel_3_color_speed();
    test_dmx_channel_4_pattern_groups();
    test_dmx_channel_5_pattern_selection();
    test_dmx_channel_6_dynamic_effects();
    test_dmx_channel_7_effect_speed();
    test_dmx_channel_8_pattern_size();
    test_dmx_channels_9_16_advanced_control();
    
    // Demonstrate full DMX frame processing
    demonstrate_full_dmx_frame();
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
    }
}

fn test_dmx_channel_3_color_speed() {
    println!("\n3. CH3 - COLOR CHANGE SPEED TEST");
    println!("---------------------------------");
    
    let test_values = [0, 5, 9, 50, 100, 127, 128, 150, 200, 255];
    
    for &value in &test_values {
        let (direction, speed) = map_ch3_to_speed_direction(value);
        println!("  DMX CH3={:3} -> Direction: {}, Speed: {}", value, direction, speed);
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
    }
}

fn test_dmx_channel_5_pattern_selection() {
    println!("\n5. CH5 - PATTERN SELECTION TEST");
    println!("--------------------------------");
    
    for pattern in [1, 32, 64, 128, 192, 255] {
        println!("  DMX CH5={:3} -> Pattern {} within selected group (CH4)", pattern, pattern);
        // This maps to ProjectItem.prj_selected
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
    }
}

fn test_dmx_channel_7_effect_speed() {
    println!("\n7. CH7 - EFFECT SPEED TEST");
    println!("---------------------------");
    
    let test_values = [0, 1, 10, 50, 100, 150, 200, 255];
    
    for &value in &test_values {
        let speed = if value <= 1 { "System default" } else { format!("Manual: {}/255", value) };
        println!("  DMX CH7={:3} -> Speed: {}", value, speed);
    }
}

fn test_dmx_channel_8_pattern_size() {
    println!("\n8. CH8 - PATTERN SIZE TEST");
    println!("---------------------------");
    
    let test_values = [0, 32, 64, 96, 128, 160, 192, 224, 255];
    
    for &value in &test_values {
        let size_percent = (value as f32 / 255.0 * 100.0) as u8;
        println!("  DMX CH8={:3} -> Pattern Size: {}%", value, size_percent);
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

fn demonstrate_full_dmx_frame() {
    println!("\n10. FULL DMX FRAME DEMONSTRATION");
    println!("=================================");
    
    // Create a typical DMX frame for laser control
    let mut dmx_frame = DmxFrame::new();
    dmx_frame.set_channel(1, 255);   // Master ON
    dmx_frame.set_channel(2, 91);    // Colorful rainbow
    dmx_frame.set_channel(3, 100);   // Forward color change, medium speed
    dmx_frame.set_channel(4, 112);   // Christmas patterns
    dmx_frame.set_channel(5, 64);    // Pattern 64
    dmx_frame.set_channel(6, 221);   // Animation effect random play
    dmx_frame.set_channel(7, 150);   // Medium effect speed
    dmx_frame.set_channel(8, 200);   // Large pattern size
    dmx_frame.set_channel(9, 115);   // Size scaling speed
    dmx_frame.set_channel(10, 160);  // Forward rotation
    dmx_frame.set_channel(11, 50);   // Vertical flip position
    dmx_frame.set_channel(12, 50);   // Horizontal flip position
    dmx_frame.set_channel(13, 127);  // Center horizontal position
    dmx_frame.set_channel(14, 127);  // Center vertical position
    dmx_frame.set_channel(15, 100);  // Wave effect gear 3
    dmx_frame.set_channel(16, 32);   // Manual drawing mode 1
    
    println!("DMX Frame Configuration:");
    for ch in 1..=16 {
        if let Some(value) = dmx_frame.get_channel(ch) {
            println!("  CH{:2}: {:3} - {}", ch, value, get_channel_description(ch, value));
        }
    }
    
    // Show how this maps to laser commands
    println!("\nLaser Command Mapping:");
    let (settings, main_data, pis_obj) = dmx_to_laser_command(&dmx_frame, 1);
    
    println!("  Settings Command:");
    println!("    Light: {} (from CH1)", settings.light);
    println!("    Color Mode: {} (from CH2)", map_ch2_to_color_mode(91));
    println!("    XY Config: {} (from CH10)", settings.xy);
    
    println!("  Main Command:");
    println!("    Mode: {} (from CH4+CH6)", main_data.current_mode);
    println!("    Speed: {} (from CH3+CH7)", main_data.run_speed);
    println!("    Size: {} (from CH8)", main_data.text_size);
    
    println!("  PisObject Config:");
    println!("    Config values derived from CH9-CH16 advanced controls");
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

fn dmx_to_laser_command(dmx_frame: &DmxFrame, start_channel: usize) -> (SettingsData, MainCommandData, PisObject) {
    let ch1 = dmx_frame.get_channel(start_channel).unwrap_or(0);
    let ch2 = dmx_frame.get_channel(start_channel + 1).unwrap_or(0);
    let ch3 = dmx_frame.get_channel(start_channel + 2).unwrap_or(0);
    let ch4 = dmx_frame.get_channel(start_channel + 3).unwrap_or(0);
    let ch6 = dmx_frame.get_channel(start_channel + 5).unwrap_or(0);
    let ch7 = dmx_frame.get_channel(start_channel + 6).unwrap_or(0);
    let ch8 = dmx_frame.get_channel(start_channel + 7).unwrap_or(0);
    let ch10 = dmx_frame.get_channel(start_channel + 9).unwrap_or(0);
    
    let (r, g, b) = map_ch2_to_rgb(ch2);
    
    let settings = SettingsData {
        values: [start_channel as u16, if ch1 > 9 { 255 } else { 0 }, r as u16, g as u16, b as u16],
        channel: start_channel as u8,
        dmx: 1,
        xy: if ch10 <= 127 { ch10 } else { ((ch10 - 128) * 2) % 256 },
        light: if ch1 > 9 { 3 } else { 1 },
        cfg: 0,
        lang: "en".to_string(),
    };
    
    let main_data = MainCommandData {
        current_mode: map_ch6_to_laser_mode(ch6),
        text_color: (ch2 % 10).min(7),
        text_size: ch8,
        run_speed: if ch7 <= 1 { 128 } else { ch7 },
        text_distance: 50,
        audio_mode: 0,
        sound_value: 0, 
        text_point_time: 50,
        draw_point_time: 50,
        run_direction: if ch3 > 127 { 1 } else { 0 },
    };
    
    let pis_obj = PisObject {
        tx_point_time: 50,
        cnf_valus: [ch10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    };
    
    (settings, main_data, pis_obj)
}

fn get_channel_description(channel: usize, value: u8) -> &'static str {
    match channel {
        1 => if value <= 9 { "Light OFF" } else { "Light ON" },
        2 => map_ch2_to_color_mode(value),
        3 => if value <= 9 { "No color change" } 
             else if value <= 127 { "Forward color change" }
             else { "Reverse color change" },
        4 => match value {
            0..=24 => "Static group 1",
            25..=49 => "Static group 2", 
            50..=74 => "Static group 3",
            75..=99 => "Static group 4",
            100..=124 => "Static group 5 (Christmas)",
            125..=149 => "Animation group 1",
            150..=174 => "Animation group 2",
            _ => "Animation group 3+"
        },
        5 => "Pattern selection",
        6 => map_ch6_to_effect_mode(value),
        7 => if value <= 1 { "Default speed" } else { "Manual speed" },
        8 => "Pattern size",
        9 => "Size control mode",
        10 => if value <= 127 { "Rotation angle" } else { "Rotation speed" },
        11 => "Vertical flip",
        12 => "Horizontal flip", 
        13 => "Horizontal position",
        14 => "Vertical position",
        15 => "Wave effect",
        16 => "Manual drawing",
        _ => "Unknown"
    }
}