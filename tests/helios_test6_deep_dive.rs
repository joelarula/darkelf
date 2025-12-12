// Deep dive investigation of Test 6 configuration (10K PPS, 100 pts, 100 fps)
// This showed the best buffer performance with only 3-4 status attempts

use darkelf::heliosdac::{HeliosDacController, HeliosPoint};
use std::time::{Duration, Instant};

#[test]
fn test_6_detailed_timing() -> Result<(), Box<dyn std::error::Error>> {
    let mut controller = HeliosDacController::new()?;
    let num_devices = controller.open_devices()?;
    
    if num_devices == 0 {
        println!("No devices found!");
        return Ok(());
    }

    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║ Test 6 Deep Dive: 10K PPS, 100 pts, 100 fps                 ║");
    println!("║ Detailed timing and buffer analysis                          ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");

    let pps = 10000;
    let points_per_frame = 100;
    let expected_fps = pps as f32 / points_per_frame as f32;
    let expected_frame_time_us = (points_per_frame as f64 / pps as f64 * 1_000_000.0) as u64;

    println!("Configuration:");
    println!("  PPS: {}", pps);
    println!("  Points per frame: {}", points_per_frame);
    println!("  Expected frame rate: {:.1} fps", expected_fps);
    println!("  Expected frame time: {} µs ({:.2} ms)", expected_frame_time_us, expected_frame_time_us as f32 / 1000.0);
    println!("  Theoretical min interval: {:.2} ms\n", expected_frame_time_us as f32 / 1000.0);

    // Create 30 frames with different patterns
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

    println!("═══════════════════════════════════════════════════════════════");
    println!("Running 1000 frames with detailed metrics...\n");

    let mut status_attempts_history = Vec::new();
    let mut frame_times_us = Vec::new();
    let mut actual_intervals_us = Vec::new();
    let start_time = Instant::now();
    let mut last_frame_time = start_time;

    for i in 0..1000 {
        let frame_start = Instant::now();
        let device_id = 0u32;
        
        // Track status attempts
        let mut status_attempts = 0;
        while status_attempts < 512 {
            match controller.get_status(device_id) {
                Ok(true) => break,
                Ok(false) => status_attempts += 1,
                Err(e) => {
                    eprintln!("Error getting status at frame {}: {}", i, e);
                    break;
                }
            }
        }
        
        status_attempts_history.push(status_attempts);

        // Track actual interval since last frame
        let interval_since_last = last_frame_time.elapsed().as_micros() as u64;
        if i > 0 {
            actual_intervals_us.push(interval_since_last);
        }
        
        // Send the frame
        let frame_idx = i % 30;
        if let Err(e) = controller.write_frame(device_id, pps, 0, &frames[frame_idx]) {
            eprintln!("Error writing frame {}: {}", i, e);
        }
        
        let frame_duration = frame_start.elapsed().as_micros() as u64;
        frame_times_us.push(frame_duration);
        
        // Print detailed info every 100 frames
        if i > 0 && i % 100 == 0 {
            let recent_attempts: Vec<_> = status_attempts_history.iter().rev().take(100).copied().collect();
            let avg_attempts = recent_attempts.iter().sum::<u32>() as f32 / recent_attempts.len() as f32;
            let max_attempts = *recent_attempts.iter().max().unwrap();
            let min_attempts = *recent_attempts.iter().min().unwrap();
            
            let recent_times: Vec<_> = frame_times_us.iter().rev().take(100).copied().collect();
            let avg_time = recent_times.iter().sum::<u64>() as f32 / recent_times.len() as f32;
            
            let recent_intervals: Vec<_> = actual_intervals_us.iter().rev().take(100).copied().collect();
            let avg_interval = recent_intervals.iter().sum::<u64>() as f32 / recent_intervals.len() as f32;
            
            println!("Frame {:4}: StatusAttempts={:2} (avg={:.1}, min={}, max={}), FrameTime={:4}µs, Interval={:5.0}µs",
                i, status_attempts, avg_attempts, min_attempts, max_attempts, frame_duration, avg_interval);
        }
        
        last_frame_time = Instant::now();
    }

    let total_duration = start_time.elapsed();
    
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("ANALYSIS RESULTS:");
    println!("═══════════════════════════════════════════════════════════════\n");
    
    // Overall timing
    let actual_fps = 1000.0 / total_duration.as_secs_f32();
    println!("Overall Performance:");
    println!("  Total time: {:.2}s", total_duration.as_secs_f32());
    println!("  Actual FPS: {:.2} (expected {:.1})", actual_fps, expected_fps);
    println!("  FPS efficiency: {:.1}%\n", (actual_fps / expected_fps) * 100.0);
    
    // Status attempts statistics
    let avg_attempts = status_attempts_history.iter().sum::<u32>() as f32 / status_attempts_history.len() as f32;
    let max_attempts = *status_attempts_history.iter().max().unwrap();
    let min_attempts = *status_attempts_history.iter().min().unwrap();
    let zero_attempts = status_attempts_history.iter().filter(|&&x| x == 0).count();
    
    println!("Status Check Statistics:");
    println!("  Average attempts: {:.2}", avg_attempts);
    println!("  Min attempts: {}", min_attempts);
    println!("  Max attempts: {}", max_attempts);
    println!("  Frames with 0 attempts: {} ({:.1}%)", zero_attempts, (zero_attempts as f32 / 1000.0) * 100.0);
    println!("  Buffer ready rate: {:.1}%\n", (zero_attempts as f32 / 1000.0) * 100.0);
    
    // Frame processing time statistics
    let avg_frame_time = frame_times_us.iter().sum::<u64>() as f32 / frame_times_us.len() as f32;
    let max_frame_time = *frame_times_us.iter().max().unwrap();
    let min_frame_time = *frame_times_us.iter().min().unwrap();
    
    println!("Frame Processing Time:");
    println!("  Average: {:.1}µs ({:.2}ms)", avg_frame_time, avg_frame_time / 1000.0);
    println!("  Min: {}µs ({:.2}ms)", min_frame_time, min_frame_time as f32 / 1000.0);
    println!("  Max: {}µs ({:.2}ms)", max_frame_time, max_frame_time as f32 / 1000.0);
    println!("  API overhead: {:.1}%\n", (avg_frame_time / expected_frame_time_us as f32) * 100.0);
    
    // Actual interval statistics
    let avg_interval = actual_intervals_us.iter().sum::<u64>() as f32 / actual_intervals_us.len() as f32;
    let max_interval = *actual_intervals_us.iter().max().unwrap();
    let min_interval = *actual_intervals_us.iter().min().unwrap();
    
    println!("Actual Frame Intervals:");
    println!("  Expected: {}µs ({:.2}ms)", expected_frame_time_us, expected_frame_time_us as f32 / 1000.0);
    println!("  Actual average: {:.0}µs ({:.2}ms)", avg_interval, avg_interval / 1000.0);
    println!("  Min: {}µs ({:.2}ms)", min_interval, min_interval as f32 / 1000.0);
    println!("  Max: {}µs ({:.2}ms)", max_interval, max_interval as f32 / 1000.0);
    println!("  Timing accuracy: {:.2}%\n", (expected_frame_time_us as f32 / avg_interval) * 100.0);
    
    // Distribution analysis
    let mut attempt_distribution = vec![0; 21];
    for &attempts in &status_attempts_history {
        if attempts <= 20 {
            attempt_distribution[attempts as usize] += 1;
        }
    }
    
    println!("Status Attempts Distribution:");
    for (attempts, count) in attempt_distribution.iter().enumerate() {
        if *count > 0 {
            let percentage = (*count as f32 / 1000.0) * 100.0;
            let bar = "█".repeat((percentage / 2.0) as usize);
            println!("  {:2} attempts: {:4} frames ({:5.1}%) {}", attempts, count, percentage, bar);
        }
    }

    println!("\n═══════════════════════════════════════════════════════════════");
    println!("RECOMMENDATIONS:");
    println!("═══════════════════════════════════════════════════════════════\n");
    
    if avg_attempts < 5.0 {
        println!("✓ Excellent buffer performance! Very stable configuration.");
    } else if avg_attempts < 10.0 {
        println!("✓ Good buffer performance. Configuration is working well.");
    } else {
        println!("⚠ High average status attempts. Consider reducing frame rate.");
    }
    
    if zero_attempts as f32 / 1000.0 > 0.5 {
        println!("✓ Buffer ready >50% of the time - headroom available.");
    } else if zero_attempts as f32 / 1000.0 > 0.2 {
        println!("✓ Buffer ready >20% of the time - reasonable headroom.");
    } else {
        println!("⚠ Buffer rarely ready - operating near limit.");
    }
    
    let timing_accuracy = (expected_frame_time_us as f32 / avg_interval) * 100.0;
    if timing_accuracy > 95.0 {
        println!("✓ Excellent timing accuracy (>95%).");
    } else if timing_accuracy > 85.0 {
        println!("✓ Good timing accuracy (>85%).");
    } else {
        println!("⚠ Timing drift detected. Check system load.");
    }

    controller.stop(0)?;
    controller.close_devices()?;
    
    Ok(())
}

#[test]
fn test_6_vs_variations() -> Result<(), Box<dyn std::error::Error>> {
    let mut controller = HeliosDacController::new()?;
    let num_devices = controller.open_devices()?;
    
    if num_devices == 0 {
        println!("No devices found!");
        return Ok(());
    }

    println!("\n╔═══════════════════════════════════════════════════════════════╗");
    println!("║ Test 6 Variations: Exploring nearby configurations           ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");

    // Test variations around the optimal Test 6 config
    let test_configs = vec![
        (10000, 100, "10K PPS, 100 pts = 100 fps (original Test 6)"),
        (10000, 90, "10K PPS,  90 pts = 111 fps (fewer points)"),
        (10000, 110, "10K PPS, 110 pts =  91 fps (more points)"),
        (9000, 100, " 9K PPS, 100 pts =  90 fps (lower PPS)"),
        (11000, 100, "11K PPS, 100 pts = 110 fps (higher PPS)"),
        (10000, 80, "10K PPS,  80 pts = 125 fps (minimal points)"),
        (10000, 120, "10K PPS, 120 pts =  83 fps (more detail)"),
    ];

    for (pps, points_per_frame, description) in test_configs {
        println!("─────────────────────────────────────────────────────────────");
        println!("Testing: {}", description);
        println!("─────────────────────────────────────────────────────────────");
        
        // Create simple test frame
        let mut frame = Vec::with_capacity(points_per_frame);
        for j in 0..points_per_frame {
            let x = ((j * 0xFFF) / points_per_frame) as u16;
            let y = 0x7FF;  // Middle
            frame.push(HeliosPoint::new(x, y, 255, 255, 255, 200));
        }
        
        let mut status_attempts_sum = 0;
        let mut max_attempts = 0;
        let mut error_count = 0;
        let start = Instant::now();
        
        // Run 200 frames
        for i in 0..200 {
            let mut attempts = 0;
            while attempts < 100 {
                match controller.get_status(0) {
                    Ok(true) => break,
                    Ok(false) => attempts += 1,
                    Err(_) => {
                        error_count += 1;
                        break;
                    }
                }
            }
            
            status_attempts_sum += attempts;
            max_attempts = max_attempts.max(attempts);
            
            if let Err(_) = controller.write_frame(0, pps, 0, &frame) {
                error_count += 1;
            }
        }
        
        let duration = start.elapsed();
        let actual_fps = 200.0 / duration.as_secs_f32();
        let avg_attempts = status_attempts_sum as f32 / 200.0;
        
        println!("  Results:");
        println!("    Duration: {:.2}s", duration.as_secs_f32());
        println!("    Actual FPS: {:.1}", actual_fps);
        println!("    Avg status attempts: {:.2}", avg_attempts);
        println!("    Max status attempts: {}", max_attempts);
        println!("    Errors: {}", error_count);
        
        let rating = if avg_attempts < 3.0 && error_count == 0 {
            "★★★★★ Excellent"
        } else if avg_attempts < 5.0 && error_count == 0 {
            "★★★★☆ Very Good"
        } else if avg_attempts < 8.0 && error_count == 0 {
            "★★★☆☆ Good"
        } else if error_count == 0 {
            "★★☆☆☆ Fair"
        } else {
            "★☆☆☆☆ Poor (errors detected)"
        };
        
        println!("    Rating: {}\n", rating);
        
        std::thread::sleep(Duration::from_millis(500));
    }

    println!("═══════════════════════════════════════════════════════════════");

    controller.stop(0)?;
    controller.close_devices()?;
    
    Ok(())
}
