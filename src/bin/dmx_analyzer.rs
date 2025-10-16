use darkelf::{
    command::CommandGenerator,
    model::{SettingsData, MainCommandData, ProjectData, ProjectItem, PublicData, PisObject},
    dmx::{DmxFrame, DmxController},
};
use std::collections::HashMap;

fn main() {
    println!("DMX Protocol Analysis for Laser Device Commands");
    println!("================================================");

    // Analyze settings command structure vs DMX
    analyze_settings_command_structure();
    
    // Analyze main command structure
    analyze_main_command_structure();
    
    // Analyze draw command structure
    analyze_draw_command_structure();
    
    // Show potential DMX correlations
    analyze_dmx_correlations();
}

fn analyze_settings_command_structure() {
    println!("\n1. SETTINGS COMMAND STRUCTURE ANALYSIS");
    println!("--------------------------------------");
    
    // Create test settings for channels 1-16
    for channel in 1..=16 {
        let settings = SettingsData {
            values: [channel as u16, 50, 255, 128, 64], // [channel, display_range, r, g, b]
            channel: channel as u8,
            dmx: 0,
            xy: 0,
            light: 3, // RGB mode
            cfg: 0,   // TTL mode
            lang: "en".to_string(),
        };
        
        let cmd = CommandGenerator::get_setting_cmd(&settings);
        println!("Channel {}: {}", channel, cmd);
        
        // Decode the command structure
        decode_settings_command(&cmd, channel);
    }
}

fn decode_settings_command(cmd: &str, channel: u8) {
    // Settings command structure: 00010203 + data + 04050607
    let header = "00010203";
    let footer = "04050607";
    
    if cmd.starts_with(header) && cmd.ends_with(footer) {
        let data_section = &cmd[8..cmd.len()-16]; // Remove header and footer + padding
        
        println!("  Channel {} Command Analysis:", channel);
        println!("    Header: {}", header);
        println!("    Data: {}", data_section);
        println!("    Data breakdown:");
        
        if data_section.len() >= 20 {
            let channel_val = &data_section[0..4];   // 2 bytes - channel value (1-512)
            let ch_field = &data_section[4..6];      // 1 byte - channel field
            let display = &data_section[6..8];       // 1 byte - display range
            let xy = &data_section[8..10];           // 1 byte - xy configuration
            let r_val = &data_section[10..12];       // 1 byte - red value
            let g_val = &data_section[12..14];       // 1 byte - green value
            let b_val = &data_section[14..16];       // 1 byte - blue value
            let light = &data_section[16..18];       // 1 byte - light mode
            let cfg = &data_section[18..20];         // 1 byte - config mode
            
            println!("      Channel Value: {} (hex: {})", 
                u16::from_str_radix(channel_val, 16).unwrap_or(0), channel_val);
            println!("      Ch Field: {} (hex: {})", 
                u8::from_str_radix(ch_field, 16).unwrap_or(0), ch_field);
            println!("      Display: {} (hex: {})", 
                u8::from_str_radix(display, 16).unwrap_or(0), display);
            println!("      XY Config: {} (hex: {})", 
                u8::from_str_radix(xy, 16).unwrap_or(0), xy);
            println!("      RGB: {}/{}/{} (hex: {}/{}/{})", 
                u8::from_str_radix(r_val, 16).unwrap_or(0),
                u8::from_str_radix(g_val, 16).unwrap_or(0),
                u8::from_str_radix(b_val, 16).unwrap_or(0),
                r_val, g_val, b_val);
            println!("      Light Mode: {} (hex: {})", 
                u8::from_str_radix(light, 16).unwrap_or(0), light);
            println!("      Config: {} (hex: {})", 
                u8::from_str_radix(cfg, 16).unwrap_or(0), cfg);
        }
    }
}

fn analyze_main_command_structure() {
    println!("\n2. MAIN COMMAND STRUCTURE ANALYSIS");
    println!("-----------------------------------");
    
    // Create test configuration with different modes
    let modes = [0, 1, 2, 3, 4, 5, 6]; // DMX, Random, Line, Animation, Text, Christmas, Outdoor
    
    for mode in modes {
        println!("\nMode {}: {}", mode, get_mode_name(mode));
        
        let mut prj_item = HashMap::new();
        prj_item.insert(2, ProjectItem { py_mode: 128, prj_selected: vec![0, 0, 0, 1] });
        prj_item.insert(3, ProjectItem { py_mode: 128, prj_selected: vec![0, 0, 0, 2] });
        prj_item.insert(5, ProjectItem { py_mode: 128, prj_selected: vec![0, 0, 0, 3] });
        prj_item.insert(6, ProjectItem { py_mode: 128, prj_selected: vec![0, 0, 0, 4] });
        
        let config = darkelf::model::CommandConfig {
            cur_mode: mode,
            text_data: darkelf::model::TextData {
                tx_color: 5,
                tx_size: 50,
                run_speed: 50,
                tx_dist: 50,
                tx_point_time: 10,
                run_dir: 1,
            },
            prj_data: ProjectData {
                public: PublicData {
                    rd_mode: 1,
                    sound_val: 77,
                },
                prj_item,
            },
        };
        
        let cmd = CommandGenerator::get_cmd_str(&config);
        println!("  Command: {}", cmd);
        decode_main_command(&cmd, mode);
    }
}

fn decode_main_command(cmd: &str, mode: u8) {
    let header = "C0C1C2C3";
    let footer = "C4C5C6C7";
    
    if cmd.starts_with(header) && cmd.ends_with(footer) {
        let data_section = &cmd[8..cmd.len()-8];
        println!("    Mode {} Data Analysis:", mode);
        
        if data_section.len() >= 4 {
            let mode_byte = &data_section[0..2];
            let reserved = &data_section[2..4];
            
            println!("      Mode Byte: {} (hex: {})", 
                u8::from_str_radix(mode_byte, 16).unwrap_or(0), mode_byte);
            println!("      Reserved: {} (hex: {})", 
                u8::from_str_radix(reserved, 16).unwrap_or(0), reserved);
                
            // Continue parsing based on structure
            if data_section.len() >= 20 {
                let color = &data_section[4..6];
                let tx_size_a = &data_section[6..8];
                let tx_size_b = &data_section[8..10];
                let run_speed = &data_section[10..12];
                
                println!("      Color: {} (hex: {})", 
                    u8::from_str_radix(color, 16).unwrap_or(0), color);
                println!("      TX Size A: {} (hex: {})", 
                    u8::from_str_radix(tx_size_a, 16).unwrap_or(0), tx_size_a);
                println!("      TX Size B: {} (hex: {})", 
                    u8::from_str_radix(tx_size_b, 16).unwrap_or(0), tx_size_b);
                println!("      Run Speed: {} (hex: {})", 
                    u8::from_str_radix(run_speed, 16).unwrap_or(0), run_speed);
            }
        }
    }
}

fn analyze_draw_command_structure() {
    println!("\n3. DRAW COMMAND STRUCTURE ANALYSIS");
    println!("-----------------------------------");
    
    // Create test points for analysis
    let test_points = vec![
        darkelf::model::Point::new(100.0, 100.0, 1, 0), // Move to start
        darkelf::model::Point::new(200.0, 100.0, 1, 1), // Draw line
        darkelf::model::Point::new(200.0, 200.0, 1, 1), // Draw line
        darkelf::model::Point::new(100.0, 200.0, 1, 1), // Draw line  
        darkelf::model::Point::new(100.0, 100.0, 1, 1), // Close shape
    ];
    
    let pis_config = PisObject {
        tx_point_time: 50,
        cnf_valus: [10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130],
    };
    
    let draw_cmd = CommandGenerator::get_draw_cmd_str(&test_points, &pis_config);
    println!("Draw Command: {}", draw_cmd);
    decode_draw_command(&draw_cmd);
}

fn decode_draw_command(cmd: &str) {
    let header = "F0F1F2F3";
    let footer = "F4F5F6F7";
    
    if cmd.starts_with(header) && cmd.ends_with(footer) {
        let data_section = &cmd[8..cmd.len()-8];
        println!("  Draw Command Analysis:");
        println!("    Header: {}", header);
        println!("    Data Length: {} hex chars", data_section.len());
        
        // Config section is typically first 30-32 hex chars
        if data_section.len() > 32 {
            let config_section = &data_section[0..30];
            let point_count_section = &data_section[30..34];
            let points_section = &data_section[34..];
            
            println!("    Config Section: {}", config_section);
            println!("    Point Count: {} ({})", 
                point_count_section,
                u16::from_str_radix(point_count_section, 16).unwrap_or(0));
            println!("    Points Data: {} chars", points_section.len());
            println!("    Points per byte: {:.1}", points_section.len() as f32 / 2.0 / 5.0); // Assuming 5 points
        }
    }
}

fn analyze_dmx_correlations() {
    println!("\n4. DMX PROTOCOL CORRELATIONS");
    println!("-----------------------------");
    
    println!("Potential DMX Channel Mappings:");
    println!("  Channels 1-3: RGB Color Control");
    println!("    - Channel 1: Red (0-255)");
    println!("    - Channel 2: Green (0-255)");
    println!("    - Channel 3: Blue (0-255)");
    
    println!("  Channels 4-6: Position Control");
    println!("    - Channel 4: X Position (0-255)");
    println!("    - Channel 5: Y Position (0-255)");
    println!("    - Channel 6: Z/Intensity (0-255)");
    
    println!("  Channels 7-10: Pattern Control");
    println!("    - Channel 7: Pattern Select (0-255)");
    println!("    - Channel 8: Pattern Speed (0-255)");
    println!("    - Channel 9: Pattern Size (0-255)");
    println!("    - Channel 10: Pattern Rotation (0-255)");
    
    println!("  Channels 11-16: Advanced Features"); 
    println!("    - Channel 11: Strobe/Flash (0-255)");
    println!("    - Channel 12: Dimmer (0-255)");
    println!("    - Channel 13: Audio Sensitivity (0-255)");
    println!("    - Channel 14: Show Mode (0-255)");
    println!("    - Channel 15: Beam Control (0-255)");
    println!("    - Channel 16: Master Control (0-255)");
    
    // Show example DMX frame
    println!("\nExample DMX Frame for Laser Control:");
    let mut dmx_frame = DmxFrame::new();
    dmx_frame.set_channel(1, 255);  // Red full
    dmx_frame.set_channel(2, 128);  // Green half
    dmx_frame.set_channel(3, 64);   // Blue quarter
    dmx_frame.set_channel(4, 127);  // X center
    dmx_frame.set_channel(5, 127);  // Y center
    dmx_frame.set_channel(6, 200);  // Intensity high
    dmx_frame.set_channel(7, 10);   // Pattern 10
    dmx_frame.set_channel(8, 100);  // Speed medium
    
    for ch in 1..=16 {
        if let Some(value) = dmx_frame.get_channel(ch) {
            if value > 0 {
                println!("  Channel {}: {} ({})", ch, value, get_channel_function(ch));
            }
        }
    }
    
    // Command correlation analysis
    println!("\nCommand to DMX Correlation Analysis:");
    analyze_command_to_dmx_mapping();
}

fn get_mode_name(mode: u8) -> &'static str {
    match mode {
        0 => "DMX Mode",
        1 => "Random Playback",
        2 => "Line Geometry Playback", 
        3 => "Animation Playback",
        4 => "Text Playback",
        5 => "Christmas Playback",
        6 => "Outdoor Playback",
        _ => "Unknown Mode",
    }
}

fn get_channel_function(channel: usize) -> &'static str {
    match channel {
        1 => "Red",
        2 => "Green", 
        3 => "Blue",
        4 => "X Position",
        5 => "Y Position",
        6 => "Intensity",
        7 => "Pattern",
        8 => "Speed",
        9 => "Size",
        10 => "Rotation",
        11 => "Strobe",
        12 => "Dimmer",
        13 => "Audio Sens",
        14 => "Show Mode",
        15 => "Beam Ctrl",
        16 => "Master",
        _ => "Extended",
    }
}

fn analyze_command_to_dmx_mapping() {
    println!("Settings Command RGB values -> DMX Channels 1-3:");
    println!("Main Command Color field -> DMX Channel selection");
    println!("Main Command Speed -> DMX Channel 8 (Speed)");
    println!("Main Command Audio -> DMX Channel 13 (Audio Sensitivity)");
    println!("Draw Command Points -> DMX Channels 4-5 (X/Y Position)");
    println!("Config Values -> DMX Channels 7-16 (Pattern/Control)");
    
    // Show hex pattern analysis
    println!("\nHex Pattern Analysis:");
    println!("Settings Header: 00010203 -> Potential DMX Universe/Channel addressing");
    println!("Main Header: C0C1C2C3 -> Potential Command Type identifier");  
    println!("Draw Header: F0F1F2F3 -> Potential Point Data identifier");
    println!("Footers: *4*5*6*7 -> Command termination patterns");
    
    println!("\nByte Position Correlations:");
    println!("Position 1-2: Channel Address (1-512 DMX range)");
    println!("Position 3: Sub-channel or function select"); 
    println!("Position 4-6: RGB values (DMX channels 1-3)");
    println!("Position 7: XY configuration (position modes)");
    println!("Position 8: Light mode (beam configuration)");
    println!("Position 9: Config mode (TTL/Analog like DMX)");
}