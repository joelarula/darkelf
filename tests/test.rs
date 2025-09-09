use btleplug::api::{Central, Manager as _, Peripheral as _};
use btleplug::platform::Manager;
use std::error::Error;
use std::time::Duration;
use tokio::time;
use anyhow::Ok;

#[test]
fn test_bluetooth_connection() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        test_bluetooth_connection_async().await.unwrap();
    });
}

async fn test_bluetooth_connection_async() -> Result<(), anyhow::Error> {
    // Initialize Bluetooth manager and adapter
    let manager = Manager::new().await?;
    let adapters = manager.adapters().await?;
    let adapter = adapters.into_iter().next().ok_or_else(|| anyhow::anyhow!("No Bluetooth adapters found"))?;

    // Start scanning for devices
    adapter.start_scan(btleplug::api::ScanFilter::default()).await?;
    time::sleep(Duration::from_secs(5)).await; // Wait for some devices to appear

    // Find your device (replace "D5322A" with your actual advertised name or use UUID)
    let peripherals = adapter.peripherals().await?;
    let mut device = None;
    for p in peripherals {
        println!("Peripheral: {:?}", p.id());
        if let Some(props) = p.properties().await.unwrap() {
            if let Some(name) = props.local_name {
               
                if name.contains("D5322A") {
                    device = Some(p);
                    break;
                }
            }
        }
    }

    let device = match device {
        Some(d) => d,
        None => {
            println!("Device not found!");
            return Ok(());
        }
    };

    // Connect to the device
    device.connect().await?;
    println!("Connected to device!");

    // Discover services and characteristics
    device.discover_services().await?;
    let services = device.services();
    for service in services {
        println!("Service UUID: {:?}", service.uuid);
        for ch in &service.characteristics {
            println!("  Characteristic UUID: {:?}", ch.uuid);
        }
    }

    // Disconnect when done
    device.disconnect().await?;
    println!("Disconnected.");

    Ok(())
}