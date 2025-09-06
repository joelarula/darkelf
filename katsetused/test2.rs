use std::time::Duration;

use darkelf::bleadapter::BluetoothManager;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let bt_manager = BluetoothManager::new().await?;
    let devices = bt_manager.discover_devices(Some(vec![]), Duration::from_secs(15)).await?;

    println!("Discovered {} devices", devices.len());

    for device in devices {
        
        let name = match device.get_name().await {
            Ok(n) => n,
            Err(e) => format!("Error: {}", e),
        };
        println!("Device: {} ({})", name, device.get_address());

    }
    Ok(())
}