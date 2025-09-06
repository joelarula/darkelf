use std::error::Error;
use darkelf::winble::BleController;
use darkelf::winble;
use log::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();
    info!("Windows BLE API Demo");
    info!("-------------------");
    
    // Step 1: Scan for devices
    info!("\n1. Scanning for BLE devices...");
    let devices = winble::scan_devices(5).await?;
    
    info!("Found {} devices:", devices.len());
    for (i, (name, addr)) in devices.iter().enumerate() {
        info!("  {}. {} (Address: {})", i+1, name, addr);
    }
    
    // Step 2: Create and connect BLE controller
    info!("\n2. Creating BLE controller...");
    let mut controller = BleController::new().await?;
    
    // Optional: Set target service UUID
    controller.set_service_uuid("0000FF00-0000-1000-8000-00805F9B34FB");
    
    // Connect with optional name filter (e.g., for TD5322A devices)
    info!("\n3. Connecting to device with name starting with 'TD5322A'...");
    match controller.connect_with_name_filter(Some("TD5322A")).await {
        Ok(_) => info!("   Connected successfully!"),
        Err(e) => {
            info!("   Failed to connect: {}", e);
            info!("\nTrying again without name filter...");
            
            match controller.connect().await {
                Ok(_) => info!("   Connected to first available device!"),
                Err(e) => {
                    info!("   Failed to connect: {}", e);
                    return Ok(());
                }
            }
        }
    }
    
    // Step 3: Discover characteristics
    info!("\n4. Discovering characteristics...");
    match controller.discover_characteristics().await {
        Ok(_) => info!("   Characteristics discovered!"),
        Err(e) => info!("   Failed to discover characteristics: {}", e),
    }
    
    // Step 4: Send a command
    let hex_command = "0102030405";  // Example command
    info!("\n5. Sending command: {}", hex_command);
    
    let bytes = winble::decode(hex_command)?;
    match controller.send(&bytes).await {
        Ok(_) => info!("   Command sent successfully!"),
        Err(e) => info!("   Failed to send command: {}", e),
    }
    
    // Step 5: Listen for notifications
    info!("\n6. Waiting for response...");
    info!("   (Listening for notifications for 10 seconds)");
    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    info!("   Response: {}", controller.get_content());
    
    // Step 6: Disconnect
    info!("\n7. Disconnecting...");
    controller.disconnect().await?;
    info!("   Disconnected successfully!");
    
    info!("\nDemo completed!");
    Ok(())
}
