// Minimal test to isolate the crash

use darkelf::dac::helios::{HeliosDacController, HeliosPoint};

#[test]
fn test_helios_minimal() -> Result<(), Box<dyn std::error::Error>> {
    println!("Creating controller...");
    let mut controller = HeliosDacController::new()?;
    
    println!("Opening devices...");
    let num_devices = controller.open_devices()?;
    println!("Found {} Helios DACs", num_devices);

    if num_devices == 0 {
        println!("No devices found!");
        return Ok(());
    }

    // Skip get_name for now, it might be causing the crash
    // println!("Test: Getting device 0 name...");
    // match controller.get_name(0) {
    //     Ok(name) => println!("Device 0 name: {}", name),
    //     Err(e) => println!("Error getting name: {}", e),
    // }
    
    println!("Test: Getting device 0 status...");
    match controller.get_status(0) {
        Ok(ready) => println!("Device 0 status: ready={}", ready),
        Err(e) => println!("Error getting status: {}", e),
    }

    println!("Closing devices...");
    controller.close_devices()?;
    println!("Test completed successfully!");
    
    Ok(())
}
