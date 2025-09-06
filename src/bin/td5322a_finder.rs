use std::error::Error;
use darkelf::blelib;
use btleplug::api::Peripheral;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting BLE device scan for TD5322A devices...");
    
    // Scan for devices for 5 seconds
    let devices = blelib::scan_devices(5).await?;
    
    // Filter for devices with names starting with TD5322A
    let td_devices: Vec<_> = devices.iter()
        .filter(|(_, name)| name.starts_with("TD5322A"))
        .collect();
    
    println!("Found {} TD5322A devices:", td_devices.len());
    for (i, (peripheral, name)) in td_devices.iter().enumerate() {
        println!("{}. {} (ID: {:?})", i+1, name, peripheral.id());
    }
    
    // Try to connect to a TD5322A device
    if !td_devices.is_empty() {
        println!("\nAttempting to connect to the first TD5322A device...");
        
        // Create a BleController instance
        let mut controller = blelib::BleController::new().await?;
        
        // In this example, we'd need more code to connect to a specific device
        // The current BleController implementation connects to already connected devices
        // Here we would add functionality to connect to a specific device
        
        println!("Note: The current BleController implementation needs to be extended to connect to specific devices.");
        println!("This would require modifying the connect() function to take a device ID parameter.");
    } else {
        println!("No TD5322A devices found.");
    }
    
    Ok(())
}
