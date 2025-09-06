use btleplug::api::{
    Central, CentralEvent, Characteristic, Manager as _, Peripheral, ScanFilter, Service, BDAddr,
};
use btleplug::platform::Manager;
use futures::stream::StreamExt;
use uuid::Uuid;

async fn get_central(manager: &Manager) -> btleplug::platform::Adapter {
    let adapters = manager.adapters().await.unwrap();
    adapters.into_iter().nth(0).unwrap()
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();

    let manager = Manager::new().await?;
    let central = get_central(&manager).await;

    // Replace with the target device's address
    let target_address = BDAddr::from_str_delim("83:40:09:7B:F0:68").unwrap();
    let target_name = "TD5322A_V3.1.2BLE";

    println!("Starting scan...");
    central.start_scan(ScanFilter::default()).await?;

    let mut events = central.events().await?;

    while let Some(event) = events.next().await {
        if let CentralEvent::DeviceDiscovered(id) = event {
            let peripheral = central.peripheral(&id).await?;
            let properties = peripheral.properties().await?;

            if let Some(properties) = properties {
                if let Some(name) = properties.local_name {
                      println!("Found  device: {} ({:?})", name, peripheral.address());

                    if peripheral.address() == target_address {
                        println!("Found target device: {} ({:?})", name, peripheral.address());

                        if peripheral.connect().await.is_ok() {
                            println!("Connected to device.");

                            let services = peripheral.services();
                            println!("Services discovered:");
                            for service in &services {
                                println!("  Service UUID: {}", service.uuid);

                                let characteristics = peripheral.characteristics();
                                println!("    Characteristics:");
                                for characteristic in &characteristics {
                                    if characteristic.service_uuid == service.uuid {
                                        println!(
                                            "      Characteristic UUID: {}, Properties: {:?}",
                                            characteristic.uuid,
                                            characteristic.properties
                                        );
                                    }
                                }
                            }

                            println!("Disconnecting...");
                            peripheral.disconnect().await?;
                            break; // Stop scanning after finding and processing the device
                        } else {
                            println!("Failed to connect to device.");
                        }
                    }
                }
            }
        }
    }

    println!("Scan complete.");
    Ok(())
}