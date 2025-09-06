use std::error::Error;
use darkelf::blelib::BleController;
use darkelf::blelib;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("BLE Demo - Using the new BleController");
    println!("--------------------------------------");
    
    // Step 1: Scan for devices
    println!("\n1. Scanning for BLE devices...");
    let devices = blelib::scan_devices(5).await?;
    
    println!("Found {} devices:", devices.len());
    for (i, (peripheral, name)) in devices.iter().enumerate() {
        println!("  {}. {} (ID: {:?})", i+1, name, peripheral.id());
    }
    
    // Step 2: Create and connect BLE controller
    println!("\n2. Creating BLE controller...");
    let mut controller = BleController::new().await?;
    
    println!("3. Connecting to device...");
    match controller.connect().await {
        Ok(_) => println!("   Connected successfully!"),
        Err(e) => {
            println!("   Failed to connect: {}", e);
            return Ok(());
        }
    }
    
    // Step 3: Discover characteristics
    println!("\n4. Discovering characteristics...");
    match controller.discover_characteristics().await {
        Ok(_) => println!("   Characteristics discovered!"),
        Err(e) => println!("   Failed to discover characteristics: {}", e),
    }
    
    // Step 4: Send a command
    let hex_command = "0102030405";  // Example command
    println!("\n5. Sending command: {}", hex_command);
    
    let bytes = blelib::decode(hex_command)?;
    match controller.send(&bytes).await {
        Ok(_) => println!("   Command sent successfully!"),
        Err(e) => println!("   Failed to send command: {}", e),
    }
    
    // Step 5: Listen for notifications
    println!("\n6. Waiting for response...");
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    println!("   Response: {}", controller.get_content());
    
    println!("\nDemo completed!");
    Ok(())
}
