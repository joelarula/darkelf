// Example Rust implementation matching the C# example

use darkelf::dac::helios::{HELIOS_FLAGS_DEFAULT, HeliosDacController, HeliosPoint};
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Helios DAC Rust Example (matching C# implementation)\n");

    // Make frames - animation of a simple line scanning upward
    println!("Creating frames...");
    use darkelf::ilda::model::{IldaPoint, status};
    let mut frames: Vec<Vec<IldaPoint>> = Vec::with_capacity(30);

    for i in 0..30 {
        // Map 0..30 to -20000..20000 roughly for Y
        let y_val = -20000 + (i * 40000 / 30) as i16;
        let mut frame_points = Vec::with_capacity(1000);

        for j in 0..1000 {
            // Logic: if j < 500, x goes up, else down
            let x_norm = if j < 500 {
                j * 2 // 0..1000
            } else {
                1000 - (j - 500) * 2 // 1000..0
            };
            // Map 0..1000 to -20000..20000
            let x_val = -20000 + (x_norm * 40000 / 1000) as i16;

            // Matching C# colors: R=0xD0, G=0xFF, B=0xD0 -> 208, 255, 208
            frame_points.push(IldaPoint::Format4 {
                x: x_val,
                y: y_val,
                z: 0,
                status: status::NORMAL,
                red: 0xD0,
                green: 0xFF,
                blue: 0xD0,
            });
        }

        frames.push(frame_points);
    }

    // Connect to DACs and output frames
    let mut helios_controller = HeliosDacController::new()?;
    let number_of_devices = match helios_controller.open_devices() {
        Ok(n) => n,
        Err(e) => {
            eprintln!("Failure during detecting and opening of Helios DACs: {}", e);
            return Err(e.into());
        }
    };

    println!("Found {} Helios DACs:", number_of_devices);

    for device_id in 0..number_of_devices {
        let device_id = device_id as u32;

        // Set shutter to open (false = closed, true = open in Rust)
        if let Err(e) = helios_controller.set_shutter(device_id, false) {
            eprintln!(
                "Warning: Could not set shutter for device {}: {}",
                device_id, e
            );
        }

        match helios_controller.get_name(device_id) {
            Ok(name) => println!("  {}", name),
            Err(e) => println!("  Device {}: Error getting name - {}", device_id, e),
        }
    }

    println!("\nSending a test animation to each DAC...");

    // Send frames
    for j in 0..150 {
        for device_id in 0..number_of_devices {
            let device_id = device_id as u32;

            // Wait for ready status (make 50 attempts like C# example)
            let mut is_ready = false;
            for _k in 0..50 {
                match helios_controller.get_status(device_id) {
                    Ok(true) => {
                        is_ready = true;
                        break;
                    }
                    Ok(false) => continue,
                    Err(e) => {
                        eprintln!("Error getting status: {}", e);
                        break;
                    }
                }
            }

            // Send the next frame if received a ready signal
            if is_ready {
                let frame_idx = (j % 30) as usize;
                if let Err(e) = helios_controller.write_frame(
                    device_id,
                    25000,
                    HELIOS_FLAGS_DEFAULT,
                    &frames[frame_idx],
                    4,
                ) {
                    eprintln!("Failure during writing of laser frame to Helios DAC: {}", e);
                }
            } else {
                println!("Warning: Device {} not ready for frame {}", device_id, j);
            }
        }
    }

    println!("\nFreeing connection...");
    helios_controller.close_devices()?;
    println!("Done!");

    Ok(())
}
