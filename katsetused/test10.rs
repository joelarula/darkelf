use btleplug::api::{
    Central, CentralEvent, Characteristic, CharPropFlags, Manager as _, Peripheral, ScanFilter, Service,
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

    println!("Starting scan...");
    central.start_scan(ScanFilter::default()).await?;

    let mut events = central.events().await?;

    while let Some(event) = events.next().await {
        if let CentralEvent::DeviceDiscovered(id) = event {
            let peripheral = central.peripheral(&id).await?;
            let properties = peripheral.properties().await?;

            if let Some(properties) = properties {
                println!("Found BLE device: {} ({:?})", properties.local_name.unwrap_or_default(), peripheral.address());

                if peripheral.connect().await.is_ok() {
                    println!("Connected to device.");

                    let services = peripheral.services();
                    println!("Services discovered:");
                    for service in &services {
                        println!("  Service UUID: {}", service.uuid);

                        let characteristics = peripheral.characteristics();
                        println!("    Characteristics:");
                        for characteristic in &characteristics {
                            println!(
                                "      Characteristic UUID: {}, Properties: {:?}",
                                characteristic.uuid,
                                characteristic.properties
                            );

                            if characteristic.properties.contains(CharPropFlags::READ) {
                                match peripheral.read(&characteristic).await {
                                    Ok(value) => println!("        Value: {:?}", value),
                                    Err(err) => println!("        Error reading value: {:?}", err),
                                }
                            }
                        }
                    }

                    println!("Disconnecting...");
                    peripheral.disconnect().await?;
                } else {
                    println!("Failed to connect to device.");
                }
            }
        }
    }

    println!("Scan complete.");
    Ok(())
}