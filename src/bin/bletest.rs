use std::error::Error;
use darkelf::blelib;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting BLE device scan...");
    
    // Scan for devices for 5 seconds
    let devices = blelib::scan_devices(5).await?;
    
    println!("Found {} devices:", devices.len());
    for (i, (peripheral, name)) in devices.iter().enumerate() {
        println!("{}. {} (ID: {:?})", i+1, name, peripheral.id());
    }
    
    if !devices.is_empty() {
        // Example - you could connect to the first device
        println!("\nYou could connect to these devices using the BleController.");
        println!("Example usage:");
        println!("
let mut controller = blelib::BleController::new().await?;
controller.connect().await?;
controller.discover_characteristics().await?;

// Send data to the device
let command = \"01020304\"; // example command in hex
let bytes = blelib::decode(command)?;
controller.send(&bytes).await?;
        ");
    }
    
    Ok(())
}
