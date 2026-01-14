// Example Rust implementation matching the Python example

use darkelf::dac::helios::{HELIOS_FLAGS_DEFAULT, HeliosDacController, HeliosPoint};

#[test]
fn test_helios_python_example() -> Result<(), Box<dyn std::error::Error>> {
    // Create controller and open devices
    let mut controller = HeliosDacController::new()?;
    let num_devices = controller.open_devices()?;
    println!("Found {} Helios DACs", num_devices);

    if num_devices == 0 {
        println!("No devices found!");
        return Ok(());
    }

    // Print device names
    // Note: GetName function appears to cause crashes with this DLL version, skip it
    // for device_id in 0..num_devices {
    //     match controller.get_name(device_id as u32) {
    //         Ok(name) => println!("Device {}: {}", device_id, name),
    //         Err(e) => println!("Device {}: Error getting name - {}", device_id, e),
    //     }
    // }

    // Create sample frames - 30 frames of 1000 points each
    // This creates a horizontal line scanning vertically upward
    println!("\nCreating sample frames...");
    let mut frames: Vec<Vec<HeliosPoint>> = Vec::with_capacity(30);

    for i in 0..30 {
        let y = ((i * 0xFFF) / 30) as u16;
        let mut frame_points = Vec::with_capacity(1000);

        for j in 0..1000 {
            let x = if j < 500 {
                ((j * 0xFFF) / 500) as u16
            } else {
                (0xFFF - ((j - 500) * 0xFFF / 500)) as u16
            };

            frame_points.push(HeliosPoint::new(x, y, 255, 255, 255, 130));
        }

        frames.push(frame_points);
    }

    println!("Playing frames on DAC...");
    println!("Press Ctrl+C to stop\n");

    // Play frames on DAC
    for i in 0..1500 {
        for device_id in 0..num_devices {
            let device_id = device_id as u32;

            // Wait for ready status (make 512 attempts)
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

            let status = controller.get_status(device_id).unwrap_or(false);
            println!(
                "Frame {}, Device {}, PPS: 30000, Flags: 0, Points: 1000, StatusAttempts: {}, Status: {}",
                i,
                device_id,
                status_attempts,
                if status { 1 } else { 0 }
            );

            // Send the frame
            let frame_idx = i % 30;
            if let Err(e) = controller.write_frame_native(
                device_id,
                25000,
                HELIOS_FLAGS_DEFAULT,
                &frames[frame_idx],
            ) {
                eprintln!("Error writing frame: {}", e);
            }
        }
    }

    println!("\nClosing devices...");
    controller.close_devices()?;

    Ok(())
}
