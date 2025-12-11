// Example Rust implementation matching the C# example

use darkelf::heliosdac::{
    HeliosDacController, HeliosPoint, HELIOS_FLAGS_DEFAULT,
};
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Helios DAC Rust Example (matching C# implementation)\n");

    // Make frames - animation of a simple line scanning upward
    println!("Creating frames...");
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
            
            // Matching C# colors: R=0xD0, G=0xFF, B=0xD0, I=0xFF
            frame_points.push(HeliosPoint::new(x, y, 0xD0, 0xFF, 0xD0, 0xFF));
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
            eprintln!("Warning: Could not set shutter for device {}: {}", device_id, e);
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
                if let Err(e) = helios_controller.write_frame(device_id, 25000, HELIOS_FLAGS_DEFAULT, &frames[frame_idx]) {
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
