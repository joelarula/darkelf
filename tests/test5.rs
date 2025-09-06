use bleasy::{Error, ScanConfig, Scanner};
use futures::StreamExt;
use btleplug::api::BDAddr;

#[tokio::main]
async fn main() -> Result<(), Error> {
    pretty_env_logger::init();

    // Filter for the target address
    let target_addr = BDAddr::from([0x83, 0x40, 0x09, 0x7B, 0xF0, 0x68]);
    let scan_config = ScanConfig::default()
        .stop_after_timeout(std::time::Duration::from_secs(30));

    let mut scanner = Scanner::new();
    scanner.start(scan_config).await?;

    let mut device_stream = scanner.device_stream();

    while let Some(device) = device_stream.next().await {
        if device.address() == target_addr { // Check address here
            println!("Found TARGET device with address {:?} and name {:?}", device.address(), device.local_name().await);

            // Print RSSI
            println!("RSSI: {:?}", device.rssi().await);


            
          let service_count_result = device.service_count().await;
          match service_count_result {
            Ok(count) => println!("Services: {:?}", count),
             Err(e) => println!("Error getting service count: {:?}", e.to_string()),
        }

            // Print all services and their characteristics
            match device.services().await {
                Ok(services) => {
                    for service in services {
                        println!("Service UUID: {:?}", service.uuid());
                        let characteristics = service.characteristics(); // Get characteristics (no await)
                        for characteristic in characteristics {
                            println!("  Characteristic UUID: {:?}", characteristic.uuid());
                            // Try to read the characteristic value if readable
                            //if characteristic.properties().contains(CharacteristicProperties::READ) {
                            match characteristic.read().await {
                                Ok(value) => println!("    Value: {:?}", value),
                                Err(e) => println!("    Failed to read: {:?}", e),
                            }
                            //}
                        }
                    }
                }
                Err(e) => println!("Failed to get services: {:?}", e),
            }

            // Disconnect after reading
            let _ = device.disconnect().await;
        }
    }

    Ok(())
}