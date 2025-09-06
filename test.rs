use btleplug::api::{Central, CentralEvent, Manager as _, Peripheral as _, ScanFilter};
use btleplug::platform::{Manager, Peripheral, PeripheralId};
use futures::stream::StreamExt;
use tokio::select;
use tokio::time::{sleep, Duration};
use std::collections::HashMap;
use std::pin::Pin;
use std::error::Error;
use uuid::Uuid;

const DEVICE_NAME_UUID: Uuid = Uuid::from_u128(0x00002a00_0000_1000_8000_00805f9b34fb);

/// Handles the logic for a newly discovered device.
async fn handle_device(peripheral: Peripheral) -> Result<(), Box<dyn Error>> {
    let properties = peripheral.properties().await?
        .ok_or("Failed to get peripheral properties")?;

    let mut device_name = properties.local_name.clone();
    let device_address = properties.address;

    // If the local_name is not present, some devices put it in the manufacturer data.
    // Let's check there. The name "TD5322A" might be part of this data.
    if device_name.is_none() {
        for (_company_id, data) in properties.manufacturer_data.iter() {
            let name_from_manu = String::from_utf8_lossy(data).to_string();
            if name_from_manu.starts_with("TD5322A") {
                device_name = Some(name_from_manu);
                break;
            }
        }
    }

    let mut final_name = device_name.clone().unwrap_or_else(|| "Unknown Device".to_string());
    println!("Discovered: {} ({})", final_name, device_address);
    // Connect to the device to discover its GATT services.
    println!("  Attempting to connect...");
    if let Err(e) = peripheral.connect().await {
        eprintln!("  -> Failed to connect: {}", e);
        return Ok(()); // Continue to next device
    }
    println!("  Connected successfully.");

    // If the name is still unknown, try to read the Device Name characteristic.
    if device_name.is_none() {
        println!("  Discovering services to find device name...");
        if let Err(e) = peripheral.discover_services().await {
            eprintln!("  -> Failed to discover services: {}", e);
        } else {
            for characteristic in peripheral.characteristics() {
                if characteristic.uuid == DEVICE_NAME_UUID {
                    if let Ok(value) = peripheral.read(&characteristic).await {
                        final_name = String::from_utf8_lossy(&value).to_string();
                        println!("  -> Found name via GATT: {}", final_name.trim());
                    }
                    break; // Found it
                }
            }
        }
    }

    println!("  Disconnecting from {}...", final_name.trim());
    peripheral.disconnect().await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Get the Bluetooth manager for the platform.
    let manager = Manager::new().await?;
    let adapters = manager.adapters().await?;
    let central = adapters.into_iter().nth(0)
        .ok_or("No Bluetooth adapters found")?;

    // Start a scan for new devices.
    println!("Starting BLE scan...");
    central.start_scan(ScanFilter::default()).await?;

    // Create a HashMap to keep track of discovered devices to avoid duplicates.
    let mut discovered_devices: HashMap<PeripheralId, bool> = HashMap::new();

    // The event stream will notify us of new devices as they are found.
    let mut events = central.events().await?;

    // We'll run the scan for 30 seconds.
    let scan_duration = Duration::from_secs(30);
    println!("Scanning for devices for the next {} seconds...", scan_duration.as_secs());
    let mut timeout: Pin<Box<tokio::time::Sleep>> = Box::pin(sleep(scan_duration));

    loop {
        select! {
            // Wait for a new device to be discovered.
            Some(event) = events.next() => {
                if let CentralEvent::DeviceDiscovered(id) = event {
                    if !discovered_devices.contains_key(&id) {
                        discovered_devices.insert(id.clone(), true);
                        if let Ok(peripheral) = central.peripheral(&id).await {
                            if let Err(err) = handle_device(peripheral).await {
                                eprintln!("Error handling device: {}", err);
                            }
                        }
                    }
                }
            },
            // The scan has timed out.
            _ = &mut timeout => {
                println!("\nScan timed out. Stopping scan.");
                central.stop_scan().await?;
                break;
            }
        }
    }

    Ok(())
}
