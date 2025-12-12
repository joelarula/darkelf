// Test different PPS rates suitable for 15kpps galvanometer
// Tests various point rates to find optimal settings for galvo performance

use darkelf::heliosdac::{
    HeliosDacController, HeliosPoint,
};

#[test]
fn test_galvo_15kpps() -> Result<(), Box<dyn std::error::Error>> {
    // Create controller and open devices
    let mut controller = HeliosDacController::new()?;
    let num_devices = controller.open_devices()?;
    println!("Found {} Helios DACs", num_devices);

    if num_devices == 0 {
        println!("No devices found!");
        return Ok(());
    }

    // Test configurations for 15kpps galvanometer
    // Format: (pps, points_per_frame, description)
    let test_configs = vec![
        (15000, 500, "15K PPS, 500 pts = 30 fps (max galvo rate)"),
        (12000, 400, "12K PPS, 400 pts = 30 fps (safe rate)"),
        (10000, 333, "10K PPS, 333 pts = 30 fps (conservative)"),
        (15000, 250, "15K PPS, 250 pts = 60 fps (max galvo, high refresh)"),
        (12000, 200, "12K PPS, 200 pts = 60 fps (safe, high refresh)"),
        (10000, 100, "10K PPS, 100 pts = 100 fps (very smooth)"),
    ];

    for (test_idx, (pps, points_per_frame, description)) in test_configs.iter().enumerate() {
        println!("\n╔═══════════════════════════════════════════════════════════════╗");
        println!("║ Test {}: {}                                    ", test_idx + 1, description);
        println!("║ PPS: {}, Points: {}, Frame Rate: {:.1} fps                    ",
                 pps, points_per_frame, *pps as f32 / *points_per_frame as f32);
        println!("╚═══════════════════════════════════════════════════════════════╝");

        // Create a simple horizontal line that moves vertically
        // This tests galvo movement on both axes
        let mut frames: Vec<Vec<HeliosPoint>> = Vec::with_capacity(30);
        
        for i in 0..30 {
            let y = ((i * 0xFFF) / 30) as u16;
            let mut frame_points = Vec::with_capacity(*points_per_frame);
            
            for j in 0..*points_per_frame {
                let x = if j < points_per_frame / 2 {
                    ((j * 0xFFF) / (points_per_frame / 2)) as u16
                } else {
                    (0xFFF - ((j - points_per_frame / 2) * 0xFFF / (points_per_frame / 2))) as u16
                };
                
                // Bright white line for visibility
                frame_points.push(HeliosPoint::new(x, y, 255, 255, 255, 200));
            }
            
            frames.push(frame_points);
        }

        println!("\nPlaying 150 frames (5 loops)...");
        
        // Play 150 frames for this test (5 complete loops)
        for i in 0..150 {
            let device_id = 0u32;
            
            // Wait for ready status
            let mut status_attempts = 0;
            while status_attempts < 512 {
                match controller.get_status(device_id) {
                    Ok(true) => break,
                    Ok(false) => status_attempts += 1,
                    Err(e) => {
                        eprintln!("Error getting status: {}", e);
                        break;
                    }
                }
            }
            
            // Print progress every 30 frames
            if i % 30 == 0 {
                let status = controller.get_status(device_id).unwrap_or(false);
                println!(
                    "  Frame {:3}, PPS: {:5}, Points: {:4}, StatusAttempts: {:3}, Status: {}",
                    i, pps, points_per_frame, status_attempts, if status { 1 } else { 0 }
                );
            }
            
            // Send the frame
            let frame_idx = i % 30;
            if let Err(e) = controller.write_frame(device_id, *pps, 0, &frames[frame_idx]) {
                eprintln!("Error writing frame: {}", e);
            }
        }
        
        println!("Test {} completed.", test_idx + 1);
        
        // Small pause between tests
        if test_idx < test_configs.len() - 1 {
            println!("\nPausing 1 second before next test...");
            std::thread::sleep(std::time::Duration::from_millis(1000));
        }
    }

    println!("\n╔═══════════════════════════════════════════════════════════════╗");
    println!("║ All galvo tests completed!                                   ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");

    // Stop and close
    controller.stop(0)?;
    controller.close_devices()?;
    
    Ok(())
}

#[test]
fn test_galvo_stress() -> Result<(), Box<dyn std::error::Error>> {
    // Stress test at maximum safe galvo rate
    let mut controller = HeliosDacController::new()?;
    let num_devices = controller.open_devices()?;
    
    if num_devices == 0 {
        println!("No devices found!");
        return Ok(());
    }

    println!("\n╔═══════════════════════════════════════════════════════════════╗");
    println!("║ Galvo Stress Test: 15K PPS Continuous                        ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");

    let pps = 15000;
    let points_per_frame = 300;  // 50 fps
    
    // Create complex pattern (circle with radial lines)
    let mut frame = Vec::with_capacity(points_per_frame);
    let center_x = 0xFFF / 2;
    let center_y = 0xFFF / 2;
    let radius = 0xFFF / 3;
    
    for i in 0..points_per_frame {
        let angle = (i as f32 / points_per_frame as f32) * 2.0 * std::f32::consts::PI;
        let x = (center_x as f32 + (angle.cos() * radius as f32)) as u16;
        let y = (center_y as f32 + (angle.sin() * radius as f32)) as u16;
        
        // Color shifts through spectrum
        let r = ((angle.sin() + 1.0) * 127.5) as u8;
        let g = ((angle.cos() + 1.0) * 127.5) as u8;
        let b = (((angle * 2.0).sin() + 1.0) * 127.5) as u8;
        
        frame.push(HeliosPoint::new(x, y, r, g, b, 180));
    }

    println!("Drawing continuous circle at 15K PPS, {} points, {:.1} fps",
             points_per_frame, pps as f32 / points_per_frame as f32);
    println!("Running 500 frames...\n");

    for i in 0..500 {
        let device_id = 0u32;
        
        // Quick status check
        let mut status_attempts = 0;
        while status_attempts < 100 {
            match controller.get_status(device_id) {
                Ok(true) => break,
                Ok(false) => status_attempts += 1,
                Err(_) => break,
            }
        }
        
        if i % 50 == 0 {
            println!("  Frame {:3}, StatusAttempts: {:2}", i, status_attempts);
        }
        
        if let Err(e) = controller.write_frame(device_id, pps, 0, &frame) {
            eprintln!("Error at frame {}: {}", i, e);
        }
    }

    println!("\nStress test completed!");
    
    controller.stop(0)?;
    controller.close_devices()?;
    
    Ok(())
}
