mod ble;

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Searching for laser devices");
    let devices = ble::scan_ble_devices(15  ).await?;

    if devices.is_empty() {
        println!("\nNo devices found.");
    } else {
        println!("\n--- Found {} matching devices ---", devices.len());
        for device in devices {
            println!("- {} {}",  device.address.unwrap_or_default() ,device.name.unwrap_or("Unknown".to_string()));
        }
    }

    Ok(())
}
