use darkelf::{
    dmxdevice::{DmxLaserDevice, DmxLaserState},
    dmx::scan_dmx_ports,
    model::{PlaybackCommand, PlaybackMode, Point, SettingsData},
};
use std::thread;
use std::time::Duration;
use log::info;

fn main() {
    env_logger::init();
    
    println!("DMX Laser Device Test");
    println!("====================");
    
    // Scan for available DMX ports
    println!("Scanning for DMX-compatible ports...");
    let dmx_ports = scan_dmx_ports();
    
    if dmx_ports.is_empty() {
        println!("No DMX-compatible ports found!");
        println!("This demo will show the DMX command generation without hardware.");
        demo_dmx_commands();
        return;
    }
    
    println!("Found DMX ports: {:?}", dmx_ports);
    
    // Use first available port
    let port = &dmx_ports[0];
    let dmx_channel = 1; // Start at DMX channel 1
    
    match DmxLaserDevice::new(port, dmx_channel) {
        Ok(device) => {
            println!("Created DMX laser device on {} starting at channel {}", port, dmx_channel);
            run_dmx_device_demo(device);
        }
        Err(e) => {
            println!("Failed to create DMX device: {}", e);
            println!("Running command generation demo instead...");
            demo_dmx_commands();
        }
    }
}

fn run_dmx_device_demo(device: DmxLaserDevice) {
    println!("\n=== DMX DEVICE DEMO ===");
    
    // Start the device
    if let Err(e) = device.start() {
        println!("Failed to start device: {}", e);
        return;
    }
    
    println!("Device started - DMX output active");
    
    // Demo 1: Basic light control
    println!("\n1. Basic Light Control");
    device.set_dmx_channel(1, 255).unwrap(); // Master ON
    device.set_dmx_channel(2, 0).unwrap();   // White color
    thread::sleep(Duration::from_secs(2));
    
    // Demo 2: Color cycling
    println!("2. Color Cycling");
    let colors = [15, 25, 65, 55, 45, 35]; // Red, Blue, Green, Yellow, Cyan, Pink
    for &color in &colors {
        device.set_dmx_channel(2, color).unwrap();
        thread::sleep(Duration::from_millis(500));
    }
    
    // Demo 3: Pattern selection
    println!("3. Pattern Selection");
    device.set_dmx_channel(4, 112).unwrap(); // Christmas patterns
    device.set_dmx_channel(5, 32).unwrap();  // Pattern 32
    thread::sleep(Duration::from_secs(2));
    
    // Demo 4: Dynamic effects
    println!("4. Dynamic Effects");
    device.set_dmx_channel(6, 221).unwrap(); // Animation effect random play
    device.set_dmx_channel(7, 200).unwrap(); // High speed
    thread::sleep(Duration::from_secs(3));
    
    // Demo 5: Position control
    println!("5. Position Control");
    device.set_dmx_channel(13, 0).unwrap();   // Left position
    thread::sleep(Duration::from_millis(500));
    device.set_dmx_channel(13, 127).unwrap(); // Center position
    thread::sleep(Duration::from_millis(500));
    device.set_dmx_channel(13, 255).unwrap(); // Right position
    thread::sleep(Duration::from_millis(500));
    
    // Demo 6: Playback commands
    println!("6. Playback Commands");
    test_playback_commands(&device);
    
    // Demo 7: Draw points
    println!("7. Draw Points");
    test_draw_points(&device);
    
    // Demo 8: Draw command (advanced)
    println!("8. Draw Command (Advanced)");
    if let Err(e) = demo_draw_command(&device) {
        println!("Draw command demo failed: {}", e);
    }
    
    // Demo 9: Settings update
    println!("9. Settings Update");
    test_settings_update(&device);
    
    // Turn off
    println!("10. Turning Off");
    device.set_dmx_channel(1, 0).unwrap(); // Master OFF
    thread::sleep(Duration::from_secs(1));
    
    // Stop the device
    if let Err(e) = device.stop() {
        println!("Error stopping device: {}", e);
    } else {
        println!("Device stopped successfully");
    }
}

fn test_playback_commands(device: &DmxLaserDevice) {
    let test_commands = [
        PlaybackCommand::default(PlaybackMode::ChristmasPlayback),
        PlaybackCommand::default(PlaybackMode::AnimationPlayback),
        PlaybackCommand::default(PlaybackMode::LineGeometryPlayback),
        PlaybackCommand::default(PlaybackMode::RandomPlayback),
    ];
    
    for command in &test_commands {
        println!("  Executing: {:?}", command.mode);
        device.execute_playback_command(command).unwrap();
        thread::sleep(Duration::from_secs(2));
    }
}

fn test_draw_points(device: &DmxLaserDevice) {
    let points = vec![
        Point::new(100.0, 100.0, 1, 0), // Red, pen up (move)
        Point::new(200.0, 100.0, 1, 1), // Red, pen down (draw)
        Point::new(200.0, 200.0, 2, 1), // Green, pen down
        Point::new(100.0, 200.0, 3, 1), // Blue, pen down
        Point::new(100.0, 100.0, 1, 1), // Red, pen down (close)
    ];
    
    println!("  Sending {} draw points", points.len());
    device.send_draw_points(&points).unwrap();
    thread::sleep(Duration::from_secs(3));
}

fn test_settings_update(device: &DmxLaserDevice) {
    let settings = SettingsData {
        values: [1, 255, 255, 128, 64], // Channel 1, full intensity, red=255, green=128, blue=64
        channel: 1,
        dmx: 1,
        xy: 3, // Some rotation
        light: 3, // RGB mode
        cfg: 0, // TTL
        lang: "en".to_string(),
    };
    
    println!("  Updating settings with orange color and rotation");
    device.update_settings(&settings).unwrap();
    thread::sleep(Duration::from_secs(2));
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
    
    for (name, mut config_fn) in demos.into_iter() {
        println!("\n--- {} ---", name);
        
        // Reset state
        state = DmxLaserState::default();
        config_fn(&mut state);
        
        // Show DMX channel values
        print_dmx_state(&state);
        
        // Show interpretation
        interpret_dmx_state(&state);
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

fn demo_draw_command(device: &DmxLaserDevice) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== DMX Draw Command Demo ===");
    
    use darkelf::model::{DrawData, DrawItem, DrawPoints, DrawPoint, DrawMode, PisObject};
    use std::pin::Pin;
    use std::future::Future;
    
    // Demo 1: LaserDevice-compatible async draw method
    println!("1. Testing async draw method (LaserDevice-compatible)");
    
    let star_points = vec![
        Point::new(400.0, 100.0, 1, 0), // Red, move to top
        Point::new(300.0, 300.0, 1, 1), // Red, draw to bottom left
        Point::new(500.0, 200.0, 1, 1), // Red, draw to right  
        Point::new(300.0, 200.0, 1, 1), // Red, draw to left
        Point::new(500.0, 300.0, 1, 1), // Red, draw to bottom right
        Point::new(400.0, 100.0, 1, 1), // Red, draw back to top (close star)
    ];
    
    let config = PisObject {
        tx_point_time: 50, // 50ms point time
        cnf_valus: [
            20,  // size_control
            45,  // rotation (45 degrees)
            64,  // vertical_flip center
            64,  // horizontal_flip center
            64,  // horizontal_pos center
            64,  // vertical_pos center
            10,  // wave_effect
            48,  // manual_drawing mode 2
            0, 0, 0, 0, 0
        ],
    };
    
    // Use a simple runtime to execute the async draw method
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(device.draw(star_points, config));
    
    thread::sleep(Duration::from_secs(2));
    
    // Demo 2: DrawData-based draw command
    println!("2. Testing DrawData-based draw command");
    
    // Create a simple rectangle drawing
    let mut rectangle_item = DrawItem::new();
    rectangle_item.line_color = 1; // Red
    rectangle_item.draw_mode = DrawMode::Shape;
    rectangle_item.x0 = 0.0; // Center position
    rectangle_item.y0 = 0.0;
    rectangle_item.z = 1.5; // 150% size
    rectangle_item.ang = 45.0; // 45 degree rotation
    
    // Add rectangle points
    let rectangle_points = vec![
        DrawPoint { x: -100.0, y: -100.0, color: 1, pen_state: 0 }, // Move to start
        DrawPoint { x: 100.0, y: -100.0, color: 1, pen_state: 1 },  // Draw to corner
        DrawPoint { x: 100.0, y: 100.0, color: 1, pen_state: 1 },   // Draw to corner
        DrawPoint { x: -100.0, y: 100.0, color: 1, pen_state: 1 },  // Draw to corner
        DrawPoint { x: -100.0, y: -100.0, color: 1, pen_state: 1 }, // Close rectangle
    ];
    rectangle_item.ps = DrawPoints::Simple(rectangle_points);
    
    // Create a circle drawing
    let mut circle_item = DrawItem::new();
    circle_item.line_color = 2; // Green
    circle_item.draw_mode = DrawMode::Polylines;
    circle_item.x0 = 200.0; // Offset position
    circle_item.y0 = 0.0;
    circle_item.z = 1.0; // Normal size
    circle_item.ang = 0.0; // No rotation
    
    // Create circle points (simplified - just 8 points)
    let mut circle_points = Vec::new();
    for i in 0..8 {
        let angle = (i as f64) * std::f64::consts::PI * 2.0 / 8.0;
        let x = angle.cos() * 80.0;
        let y = angle.sin() * 80.0;
        let pen_state = if i == 0 { 0 } else { 1 }; // Move to first, draw rest
        circle_points.push(DrawPoint { x, y, color: 2, pen_state });
    }
    // Close the circle
    circle_points.push(DrawPoint { x: 80.0, y: 0.0, color: 2, pen_state: 1 });
    circle_item.ps = DrawPoints::Simple(circle_points);
    
    // Create PisObject with custom timing
    let pis_obj = PisObject {
        tx_point_time: 30, // 30ms point time
        cnf_valus: [
            10,  // size_control
            0,   // rotation override
            64,  // vertical_flip
            64,  // horizontal_flip  
            64,  // horizontal_pos
            64,  // vertical_pos
            0,   // wave_effect
            32,  // manual_drawing mode
            0, 0, 0, 0, 0 // unused
        ],
    };
    
    // Create DrawData
    let draw_data = DrawData {
        draw_points: vec![rectangle_item, circle_item],
        pis_obj,
    };
    
    println!("Executing draw command with rectangle and circle");
    device.draw_command(&draw_data)?;
    
    thread::sleep(Duration::from_secs(3));
    
    // Demo 3: Draw sequence with individual items
    println!("3. Testing draw sequence (rectangle -> circle)");
    device.draw_sequence(&draw_data.draw_points, 1500)?;
    
    thread::sleep(Duration::from_secs(2));
    
    println!("Draw command demo completed");
    Ok(())
}