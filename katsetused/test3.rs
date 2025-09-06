// See the "macOS permissions note" in README.md before running this on macOS
// Big Sur or later.

use btleplug::api::{
    bleuuid::BleUuid, Central, CentralEvent, Manager as _, Peripheral, ScanFilter,
};
use btleplug::platform::{Adapter, Manager};
use futures::stream::StreamExt;
use btleplug::api::BDAddr;
use log::*;

async fn get_central(manager: &Manager) -> Adapter {
    let adapters = manager.adapters().await.unwrap();
    adapters.into_iter().nth(0).unwrap()
}
#[test]
fn test_main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        main().await.unwrap();
    });
}
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();

    let manager = Manager::new().await?;

    // get the first bluetooth adapter
    // connect to the adapter
    let central = get_central(&manager).await;

    let central_state = central.adapter_state().await.unwrap();
    println!("CentralState: {:?}", central_state);

    // Each adapter has an event stream, we fetch via events(),
    // simplifying the type, this will return what is essentially a
    // Future<Result<Stream<Item=CentralEvent>>>.
    let mut events = central.events().await?;

    // Target address
    let target_address = "83:40:09:7B:F0:68".parse::<BDAddr>().unwrap();

    // start scanning for devices
    central.start_scan(ScanFilter::default()).await?;

    // Print based on whatever the event receiver outputs. Note that the event
    // receiver blocks, so in a real program, this should be run in its own
    // thread (not task, as this library does not yet use async channels).
    while let Some(event) = events.next().await {
        match event {
            CentralEvent::DeviceDiscovered(id) => {
                let peripheral_result = central.peripheral(&id).await;
                match peripheral_result {
                    Ok(peripheral) => {
                        if peripheral.address() == target_address {
                            let properties_result = peripheral.properties().await;
                            match properties_result {
                                Ok(properties) => {
                                    let name = properties
                                        .and_then(|p| p.local_name)
                                        .map(|local_name| format!("Name: {local_name}"))
                                        .unwrap_or_default();
                                    println!("DeviceDiscovered: {:?} {}", id, name);
                                }
                                Err(e) => error!("Error getting properties: {:?}", e),
                            }
                        }
                    }
                    Err(e) => error!("Error getting peripheral: {:?}", e),
                }
            }
            CentralEvent::StateUpdate(state) => {
                println!("AdapterStatusUpdate {:?}", state);
            }
            CentralEvent::DeviceConnected(id) => {
                let peripheral_result = central.peripheral(&id).await;
                 match peripheral_result {
                    Ok(peripheral) => {
                        if peripheral.address() == target_address {
                            println!("DeviceConnected: {:?}", id);
                        }
                    }
                     Err(e) => error!("Error getting peripheral: {:?}", e),
                 }
            }
            CentralEvent::DeviceDisconnected(id) => {
                let peripheral_result = central.peripheral(&id).await;
                 match peripheral_result {
                    Ok(peripheral) => {
                        if peripheral.address() == target_address {
                            println!("DeviceDisconnected: {:?}", id);
                        }
                    }
                     Err(e) => error!("Error getting peripheral: {:?}", e),
                 }
            }
            CentralEvent::ManufacturerDataAdvertisement {
                id,
                manufacturer_data,
            } => {
                let peripheral_result = central.peripheral(&id).await;
                 match peripheral_result {
                    Ok(peripheral) => {
                        if peripheral.address() == target_address {
                            println!(
                                "ManufacturerDataAdvertisement: {:?}, {:?}",
                                id, manufacturer_data
                            );
                        }
                    }
                     Err(e) => error!("Error getting peripheral: {:?}", e),
                 }
            }
            CentralEvent::ServiceDataAdvertisement { id, service_data } => {
                let peripheral_result = central.peripheral(&id).await;
                 match peripheral_result {
                    Ok(peripheral) => {
                        if peripheral.address() == target_address {
                            println!("ServiceDataAdvertisement: {:?}, {:?}", id, service_data);
                        }
                    }
                     Err(e) => error!("Error getting peripheral: {:?}", e),
                 }
            }
            CentralEvent::ServicesAdvertisement { id, services } => {
                let peripheral_result = central.peripheral(&id).await;
                 match peripheral_result {
                    Ok(peripheral) => {
                        if peripheral.address() == target_address {
                            let services: Vec<String> =
                                services.into_iter().map(|s| s.to_short_string()).collect();
                            println!("ServicesAdvertisement: {:?}, {:?}", id, services);
                        }
                    }
                     Err(e) => error!("Error getting peripheral: {:?}", e),
                 }
            }
            _ => {}
        }
    }
    Ok(())
}