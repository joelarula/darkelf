use std::time::Duration;
use btleplug::{
    api::{Central, Manager, Peripheral, PeripheralProperties as _},
    platform::Manager as PlatformManager,
};
use darkelf::util;
use log::{error, info};

#[tokio::main]
#[test]
async fn test_btleplug_api_connect() -> Result<(), anyhow::Error> {
    
        util::setup_logging();

        let manager = PlatformManager::new().await?;
        let ble_manager = Some(manager);
        let adapters = ble_manager.as_ref().unwrap().adapters().await?;
        let ble_adapter = Some(adapters.into_iter().next().ok_or_else(|| anyhow::anyhow!("No adapters found"))?);

        let adapter = ble_adapter.as_ref().unwrap();
        adapter.start_scan(Default::default()).await?;
        tokio::time::sleep(Duration::from_secs(15)).await;
        let devices = adapter.peripherals().await?;
        for device in devices {
            info!("Found device: {:?}", device.id());
            if !device.is_connected().await? {
                match device.connect().await {
                    Ok(_) => {
                        info!("Connected to device: {:?}", device.properties().await?.and_then(|p| p.local_name));
                    },
                    Err(e) => {
                        error!("Failed to connect to device {:?}: {e:?}", device.id());
                    }
                }
            }
        }


   Ok(())
}