// Test max points capacity at 25 FPS
// Sweeps through increasing PPS/Points configurations to find the limit

use darkelf::dac::helios::{HELIOS_FLAGS_DEFAULT, HeliosDacController, HeliosPoint};
use std::time::{Duration, Instant};

#[test]
fn test_max_points_ring_25fps() -> Result<(), Box<dyn std::error::Error>> {
    let mut controller = HeliosDacController::new()?;
    let num_devices = controller.open_devices()?;

    if num_devices == 0 {
        println!("No devices found!");
        return Ok(());
    }

    println!("\n╔═══════════════════════════════════════════════════════════════╗");
    println!("║ MAX POINTS STRESS TEST @ 25 FPS                              ║");
    println!("║ Ring Shape - Ramping up PPS and Points until failure         ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");

    let target_fps = 25.0;
    let frame_duration = Duration::from_micros(40000); // 1/25 = 40ms

    // Define test steps: (PPS, Points per frame)
    // Points = PPS / 25
    let configs = vec![
        (10000, 400), // Base connection
        (15000, 600),
        (20000, 800),
        (25000, 1000),
        (30000, 1200),
        (35000, 1400),
        (40000, 1600),
        (45000, 1800),
        (50000, 2000),
        (55000, 2200),
        (60000, 2400),
    ];

    for (pps, points) in configs {
        println!("─────────────────────────────────────────────────────────────");
        println!("Testing: {} PPS, {} Points (25 FPS)", pps, points);

        // Generate Ring Shape
        let mut frame = Vec::with_capacity(points);
        for i in 0..points {
            let angle = (i as f32 / points as f32) * 2.0 * std::f32::consts::PI;
            let radius = 0x7FF; // Max radius taking center into account
            let center = 0x7FF;

            let x = (center as f32 + angle.cos() * radius as f32) as u16;
            let y = (center as f32 + angle.sin() * radius as f32) as u16;

            frame.push(HeliosPoint::new(x, y, 255, 255, 255, 200));
        }

        // Run for 2 seconds (50 frames)
        let run_frames = 50;
        let mut status_attempts_sum = 0;
        let mut max_status_attempts = 0;
        let mut frame_times = Vec::new();
        let test_start = Instant::now();
        let mut failed = false;

        for _ in 0..run_frames {
            let frame_start = Instant::now();

            // 1. Wait for buffer readiness
            let mut attempts = 0;
            while attempts < 500 {
                // 500 attempts limit
                match controller.get_status(0) {
                    Ok(true) => break,
                    Ok(false) => attempts += 1,
                    Err(e) => {
                        println!("  ERROR: Status check failed: {}", e);
                        failed = true;
                        break;
                    }
                }
            }

            if attempts >= 500 {
                println!("  ERROR: Buffer timeout (500+ attempts)");
                failed = true;
                break;
            }

            status_attempts_sum += attempts;
            max_status_attempts = max_status_attempts.max(attempts);

            // 2. Write Frame
            if let Err(e) = controller.write_frame_native(0, pps, HELIOS_FLAGS_DEFAULT, &frame) {
                println!("  ERROR: Write frame failed: {}", e);
                failed = true;
                break;
            }

            // 3. Sleep to maintain 25 FPS
            let elapsed = frame_start.elapsed();
            frame_times.push(elapsed.as_micros() as u64);

            if elapsed < frame_duration {
                std::thread::sleep(frame_duration - elapsed);
            }
        }

        let total_duration = test_start.elapsed();
        let actual_fps = run_frames as f32 / total_duration.as_secs_f32();
        let avg_attempts = status_attempts_sum as f32 / run_frames as f32;
        let avg_process_time_ms =
            frame_times.iter().sum::<u64>() as f32 / frame_times.len() as f32 / 1000.0;

        println!("  Result: {} FPS (Target 25.0)", actual_fps);
        println!(
            "  Avg Status Attempts: {:.1} (Max: {})",
            avg_attempts, max_status_attempts
        );
        println!("  Avg Process Time: {:.2}ms / 40.00ms", avg_process_time_ms);

        if failed {
            println!("  [FAILED] - System instability detected at this load.");
            break;
        }

        if actual_fps < 24.0 {
            println!("  [WARN] FPS dropped below 24.0. Reaching limit.");
        }

        if avg_attempts > 20.0 {
            println!("  [WARN] buffer is struggling (high retry count).");
        }

        // Small cooldown between tests
        std::thread::sleep(Duration::from_millis(200));
    }

    controller.stop(0)?;
    controller.close_devices()?;
    Ok(())
}
