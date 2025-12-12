// Test 25 fps with precise timing using sleep
// This test uses sleep to control frame rate instead of relying solely on DAC buffer status

use darkelf::heliosdac::{HeliosDacController, HeliosPoint};
use std::time::{Duration, Instant};

#[test]
fn test_25fps_with_sleep() -> Result<(), Box<dyn std::error::Error>> {
    let mut controller = HeliosDacController::new()?;
    let num_devices = controller.open_devices()?;
    
    if num_devices == 0 {
        println!("No devices found!");
        return Ok(());
    }

    println!("\n╔═══════════════════════════════════════════════════════════════╗");
    println!("║ 25 FPS Test with Sleep-based Timing                          ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");

    // Test configurations for 25 fps
    let test_configs = vec![
        (10000, 400, "10K PPS, 400 pts = 25 fps (optimal for Test 6 galvo)"),
        (12500, 500, "12.5K PPS, 500 pts = 25 fps (higher detail)"),
        (8000, 320, "8K PPS, 320 pts = 25 fps (brighter, lower speed)"),
    ];

    for (test_idx, (pps, points_per_frame, description)) in test_configs.iter().enumerate() {
        println!("─────────────────────────────────────────────────────────────");
        println!("Test {}: {}", test_idx + 1, description);
        println!("─────────────────────────────────────────────────────────────");
        
        // Create 30 animated frames
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
                
                frame_points.push(HeliosPoint::new(x, y, 255, 255, 255, 200));
            }
            
            frames.push(frame_points);
        }

        // Calculate frame timing
        let target_fps = 25.0;
        let frame_duration_ms = 1000.0 / target_fps;
        let frame_duration = Duration::from_micros((frame_duration_ms * 1000.0) as u64);
        
        println!("  Target FPS: {}", target_fps);
        println!("  Frame duration: {:.2}ms", frame_duration_ms);
        println!("  PPS: {}, Points: {}", pps, points_per_frame);
        println!("  Point display time: {:.2}ms", (*points_per_frame as f32 / *pps as f32) * 1000.0);
        println!("\n  Playing 250 frames (10 loops @ 25 fps = 10 seconds)...\n");

        let test_start = Instant::now();
        let mut frame_times = Vec::new();
        let mut status_attempts_history = Vec::new();
        let mut sleep_times_us = Vec::new();
        
        for i in 0..250 {
            let frame_start = Instant::now();
            let device_id = 0u32;
            
            // Check status
            let mut status_attempts = 0;
            while status_attempts < 100 {
                match controller.get_status(device_id) {
                    Ok(true) => break,
                    Ok(false) => status_attempts += 1,
                    Err(e) => {
                        eprintln!("Error getting status: {}", e);
                        break;
                    }
                }
            }
            status_attempts_history.push(status_attempts);
            
            // Send the frame
            let frame_idx = i % 30;
            if let Err(e) = controller.write_frame(device_id, *pps, 0, &frames[frame_idx]) {
                eprintln!("Error writing frame {}: {}", i, e);
            }
            
            let processing_time = frame_start.elapsed();
            frame_times.push(processing_time.as_micros() as u64);
            
            // Sleep to maintain 25 fps
            if processing_time < frame_duration {
                let sleep_time = frame_duration - processing_time;
                sleep_times_us.push(sleep_time.as_micros() as u64);
                std::thread::sleep(sleep_time);
            } else {
                sleep_times_us.push(0);
            }
            
            // Print every 25 frames (1 second)
            if i > 0 && i % 25 == 0 {
                let elapsed = test_start.elapsed().as_secs_f32();
                let actual_fps = i as f32 / elapsed;
                let recent_attempts: Vec<_> = status_attempts_history.iter().rev().take(25).copied().collect();
                let avg_attempts = recent_attempts.iter().sum::<u32>() as f32 / recent_attempts.len() as f32;
                
                println!("    Frame {:3} ({:.1}s): Actual FPS: {:.2}, Avg StatusAttempts: {:.1}", 
                    i, elapsed, actual_fps, avg_attempts);
            }
        }

        let total_duration = test_start.elapsed();
        let actual_fps = 250.0 / total_duration.as_secs_f32();
        
        // Calculate statistics
        let avg_status_attempts = status_attempts_history.iter().sum::<u32>() as f32 / status_attempts_history.len() as f32;
        let max_status_attempts = *status_attempts_history.iter().max().unwrap();
        let avg_processing_time = frame_times.iter().sum::<u64>() as f32 / frame_times.len() as f32;
        let max_processing_time = *frame_times.iter().max().unwrap();
        let avg_sleep_time = sleep_times_us.iter().sum::<u64>() as f32 / sleep_times_us.len() as f32;
        let frames_without_sleep = sleep_times_us.iter().filter(|&&t| t == 0).count();
        
        println!("\n  Results:");
        println!("    Total duration: {:.2}s (expected 10.0s)", total_duration.as_secs_f32());
        println!("    Actual FPS: {:.2} (target {:.1})", actual_fps, target_fps);
        println!("    FPS accuracy: {:.2}%", (actual_fps / target_fps) * 100.0);
        println!("    Avg status attempts: {:.2} (max: {})", avg_status_attempts, max_status_attempts);
        println!("    Avg processing time: {:.0}µs ({:.2}ms)", avg_processing_time, avg_processing_time / 1000.0);
        println!("    Max processing time: {}µs ({:.2}ms)", max_processing_time, max_processing_time as f32 / 1000.0);
        println!("    Avg sleep time: {:.0}µs ({:.2}ms)", avg_sleep_time, avg_sleep_time / 1000.0);
        println!("    Frames without sleep: {} ({:.1}%)", frames_without_sleep, (frames_without_sleep as f32 / 250.0) * 100.0);
        
        let rating = if actual_fps >= 24.8 && actual_fps <= 25.2 && avg_status_attempts < 5.0 {
            "★★★★★ Excellent - Perfect timing and buffer performance"
        } else if actual_fps >= 24.5 && actual_fps <= 25.5 && avg_status_attempts < 8.0 {
            "★★★★☆ Very Good - Good timing and buffer performance"
        } else if actual_fps >= 24.0 && actual_fps <= 26.0 {
            "★★★☆☆ Good - Acceptable timing"
        } else {
            "★★☆☆☆ Fair - Timing drift detected"
        };
        
        println!("    Rating: {}\n", rating);
        
        if test_idx < test_configs.len() - 1 {
            std::thread::sleep(Duration::from_secs(1));
        }
    }

    println!("═══════════════════════════════════════════════════════════════");

    controller.stop(0)?;
    controller.close_devices()?;
    
    Ok(())
}

#[test]
fn test_25fps_variable_patterns() -> Result<(), Box<dyn std::error::Error>> {
    let mut controller = HeliosDacController::new()?;
    let num_devices = controller.open_devices()?;
    
    if num_devices == 0 {
        println!("No devices found!");
        return Ok(());
    }

    println!("\n╔═══════════════════════════════════════════════════════════════╗");
    println!("║ 25 FPS with Variable Pattern Complexity                      ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");

    let pps = 10000;
    let target_fps = 25.0;
    let frame_duration = Duration::from_micros(40000); // 1/25 = 40ms
    
    println!("Testing variable point counts at constant 25 fps:");
    println!("  PPS: {}", pps);
    println!("  Frame duration: 40.00ms\n");

    // Test different point counts
    let point_counts = vec![
        (200, "Simple pattern (200 points, 20ms display)"),
        (300, "Medium pattern (300 points, 30ms display)"),
        (400, "Complex pattern (400 points, 40ms display)"),
        (350, "Mixed complexity (350 points, 35ms display)"),
    ];

    for (points_per_frame, description) in point_counts {
        println!("  Testing: {}", description);
        
        // Create frame
        let mut frame = Vec::with_capacity(points_per_frame);
        for j in 0..points_per_frame {
            let angle = (j as f32 / points_per_frame as f32) * 2.0 * std::f32::consts::PI;
            let radius = 0x7FF;
            let center = 0x7FF;
            let x = (center as f32 + (angle.cos() * radius as f32)) as u16;
            let y = (center as f32 + (angle.sin() * radius as f32)) as u16;
            frame.push(HeliosPoint::new(x, y, 255, 255, 255, 200));
        }
        
        let test_start = Instant::now();
        let mut timing_errors = 0;
        let mut status_sum = 0;
        
        // Run 100 frames (4 seconds)
        for i in 0..100 {
            let frame_start = Instant::now();
            
            // Check status
            let mut status_attempts = 0;
            while status_attempts < 100 {
                match controller.get_status(0) {
                    Ok(true) => break,
                    Ok(false) => status_attempts += 1,
                    Err(_) => break,
                }
            }
            status_sum += status_attempts;
            
            // Send frame
            if let Err(e) = controller.write_frame(0, pps, 0, &frame) {
                eprintln!("Error: {}", e);
            }
            
            let processing_time = frame_start.elapsed();
            
            // Sleep to maintain timing
            if processing_time < frame_duration {
                std::thread::sleep(frame_duration - processing_time);
            } else {
                timing_errors += 1;
            }
        }
        
        let total_duration = test_start.elapsed();
        let actual_fps = 100.0 / total_duration.as_secs_f32();
        let avg_status = status_sum as f32 / 100.0;
        
        println!("    Duration: {:.2}s, FPS: {:.2}, Avg status: {:.1}, Timing errors: {}",
            total_duration.as_secs_f32(), actual_fps, avg_status, timing_errors);
        
        std::thread::sleep(Duration::from_millis(500));
    }

    println!("\n═══════════════════════════════════════════════════════════════");

    controller.stop(0)?;
    controller.close_devices()?;
    
    Ok(())
}

#[test]
fn test_25fps_long_run() -> Result<(), Box<dyn std::error::Error>> {
    let mut controller = HeliosDacController::new()?;
    let num_devices = controller.open_devices()?;
    
    if num_devices == 0 {
        println!("No devices found!");
        return Ok(());
    }

    println!("\n╔═══════════════════════════════════════════════════════════════╗");
    println!("║ 25 FPS Long Run Stability Test (60 seconds)                  ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");

    let pps = 10000;
    let points_per_frame = 400;
    let target_fps = 25.0;
    let frame_duration = Duration::from_micros(40000);
    let total_frames = 1500; // 60 seconds at 25 fps
    
    println!("Configuration:");
    println!("  PPS: {}", pps);
    println!("  Points per frame: {}", points_per_frame);
    println!("  Target FPS: {}", target_fps);
    println!("  Total frames: {} (60 seconds)", total_frames);
    println!("  Frame interval: 40.00ms\n");

    // Create 30 frames
    let mut frames: Vec<Vec<HeliosPoint>> = Vec::with_capacity(30);
    for i in 0..30 {
        let y = ((i * 0xFFF) / 30) as u16;
        let mut frame_points = Vec::with_capacity(points_per_frame);
        
        for j in 0..points_per_frame {
            let x = if j < points_per_frame / 2 {
                ((j * 0xFFF) / (points_per_frame / 2)) as u16
            } else {
                (0xFFF - ((j - points_per_frame / 2) * 0xFFF / (points_per_frame / 2))) as u16
            };
            frame_points.push(HeliosPoint::new(x, y, 255, 255, 255, 200));
        }
        frames.push(frame_points);
    }

    let test_start = Instant::now();
    let mut max_drift = 0i64;
    let mut status_history = Vec::new();
    
    println!("Running test...\n");

    for i in 0..total_frames {
        let expected_time_ms = (i as f64 / target_fps as f64) * 1000.0;
        let frame_start = Instant::now();
        
        // Status check
        let mut status_attempts = 0;
        while status_attempts < 100 {
            match controller.get_status(0) {
                Ok(true) => break,
                Ok(false) => status_attempts += 1,
                Err(_) => break,
            }
        }
        status_history.push(status_attempts);
        
        // Send frame
        let frame_idx = i % 30;
        if let Err(e) = controller.write_frame(0, pps, 0, &frames[frame_idx]) {
            eprintln!("Error at frame {}: {}", i, e);
        }
        
        // Calculate timing drift
        let actual_time_ms = test_start.elapsed().as_millis() as f64;
        let drift_ms = (actual_time_ms - expected_time_ms) as i64;
        max_drift = max_drift.max(drift_ms.abs());
        
        // Sleep for precise timing
        let processing_time = frame_start.elapsed();
        if processing_time < frame_duration {
            std::thread::sleep(frame_duration - processing_time);
        }
        
        // Report every 5 seconds (125 frames)
        if i > 0 && i % 125 == 0 {
            let elapsed = test_start.elapsed().as_secs_f32();
            let actual_fps = i as f32 / elapsed;
            let recent_status: Vec<_> = status_history.iter().rev().take(125).copied().collect();
            let avg_status = recent_status.iter().sum::<u32>() as f32 / 125.0;
            
            println!("  {:2}s: Frame {:4}, FPS: {:.2}, Drift: {:+}ms, Avg status: {:.1}",
                elapsed.round(), i, actual_fps, drift_ms, avg_status);
        }
    }

    let total_duration = test_start.elapsed();
    let actual_fps = total_frames as f32 / total_duration.as_secs_f32();
    let avg_status = status_history.iter().sum::<u32>() as f32 / status_history.len() as f32;
    
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("FINAL RESULTS:");
    println!("═══════════════════════════════════════════════════════════════\n");
    println!("  Total duration: {:.2}s (expected 60.0s)", total_duration.as_secs_f32());
    println!("  Actual FPS: {:.3} (target {:.1})", actual_fps, target_fps);
    println!("  FPS accuracy: {:.3}%", (actual_fps / target_fps) * 100.0);
    println!("  Max timing drift: {}ms", max_drift);
    println!("  Avg status attempts: {:.2}", avg_status);
    
    if actual_fps >= 24.9 && actual_fps <= 25.1 && max_drift < 50 {
        println!("\n  ✓ Excellent long-term stability!");
    } else if actual_fps >= 24.7 && actual_fps <= 25.3 && max_drift < 100 {
        println!("\n  ✓ Good long-term stability");
    } else {
        println!("\n  ⚠ Timing drift detected - check system load");
    }

    controller.stop(0)?;
    controller.close_devices()?;
    
    Ok(())
}
