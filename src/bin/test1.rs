use bluest::{Adapter, Device};
use futures::stream::StreamExt;
use std::error::Error;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Get the default Bluetooth adapter
    let adapter = Adapter::default().await.ok_or("No Bluetooth adapter found")?;

    // Start discovery
    println!("Starting Bluetooth discovery...");
    let mut discovery = adapter.discover_devices(&[]).await?;

    // We'll collect discovered devices in a vector
    let mut devices: Vec<Device> = Vec::new();

    // Scan for 10 seconds
    let scan_duration = Duration::from_secs(10);
    let scan_timeout = sleep(scan_duration);
    tokio::pin!(scan_timeout);

    loop {
        tokio::select! {
            Some(device_result) = discovery.next() => {
                match device_result {
                    Ok(device) => {
                        let name = device.name().unwrap_or_else(|_| "(unknown)".into());
                        println!("Discovered: {} ({:?})", name, device.id());
                        devices.push(device);
                    }
                    Err(e) => {
                        println!("Failed to discover device: {}", e);
                    }
                }
            },
            _ = &mut scan_timeout => {
                break;
            }
        }
    }
    println!("Discovery finished. {} devices found.", devices.len());

    // List services and characteristics for each device
    for device in devices {
        let name = device.name()?;
        println!("\nInspecting: {} ({:?})", name, device.id());

        // Discover services and characteristics using helper function
        //discover_services_and_characteristics(&device).await;
    }
    Ok(())
}



// New helper function to discover and print services and characteristics
async fn discover_services_and_characteristics(device: &Device) {
    match device.discover_services().await {
        Ok(services) => {
            for service in services {
                println!("  Service: {}", service.uuid());
                // Discover characteristics for each service
                match service.discover_characteristics().await {
                    Ok(characteristics) => {
                        for characteristic in characteristics {
                            match characteristic.properties().await {
                                Ok(props) => {
                                    println!("    Characteristic: {} (properties: {:?})", characteristic.uuid(), props);
                                }
                                Err(e) => {
                                    println!("    Characteristic: {} (failed to get properties: {})", characteristic.uuid(), e);
                                }
                            }
                        }
                    }
                    Err(e) => println!("    Failed to discover characteristics: {}", e),
                }
            }
        }
        Err(e) => println!("  Failed to discover services: {}", e),
    }
}

