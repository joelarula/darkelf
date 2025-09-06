use std::error::Error;
use darkelf::ble_provider::{BleImplementation, get_ble_controller};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("BLE Provider Demo - Choose your implementation");
    println!("-----------------------------------------");
    
    // Choose Windows implementation
    #[cfg(target_os = "windows")]
    let implementation = BleImplementation::Windows;
    
    // Choose btleplug implementation for non-Windows platforms
    #[cfg(not(target_os = "windows"))]
    let implementation = BleImplementation::Btleplug;
    
    println!("\nUsing implementation: {:?}", implementation);
    
    // Get a controller with the chosen implementation
    let mut controller = get_ble_controller(implementation).await?;
    
    // Connect to device
    println!("\nConnecting to BLE device...");
    match controller.connect().await {
        Ok(_) => println!("Connected successfully!"),
        Err(e) => {
            println!("Failed to connect: {}", e);
            return Ok(());
        }
    }
    
    // Discover characteristics
    println!("\nDiscovering characteristics...");
    match controller.discover_characteristics().await {
        Ok(_) => println!("Characteristics discovered successfully!"),
        Err(e) => {
            println!("Failed to discover characteristics: {}", e);
            return Ok(());
        }
    }
    
    // Send data
    let command = "01020304";
    println!("\nSending command: {}", command);
    let bytes = hex::decode(command).unwrap();
    match controller.send(&bytes).await {
        Ok(_) => println!("Command sent successfully!"),
        Err(e) => println!("Failed to send command: {}", e),
    }
    
    // Wait for response
    println!("\nWaiting for response...");
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    println!("Response: {}", controller.get_content());
    
    println!("\nDemo completed!");
    Ok(())
}
