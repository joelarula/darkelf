use std::time::Duration;
use anyhow::Ok;
use btleplug::{
    api::{Central, Manager, Peripheral, PeripheralProperties as _},
    platform::Manager as PlatformManager,
};

#[tokio::main]
#[test]
async fn test_btleplug_api_connect() -> Result<(), anyhow::Error> {
    
        let manager = PlatformManager::new().await?;
        let ble_manager = Some(manager);
        let adapters = ble_manager.as_ref().unwrap().adapters().await?;
        let ble_adapter = Some(adapters.into_iter().next().ok_or_else(|| anyhow::anyhow!("No adapters found"))?);

        let adapter = ble_adapter.as_ref().unwrap();
        adapter.start_scan(Default::default()).await?;
        tokio::time::sleep(Duration::from_secs(15)).await;
        let devices = adapter.peripherals().await?;
        for device in devices {
             println!("Found device: {:?}", device.id());
            //if !device.is_connected().await? {
            //    device.connect().await?;
            //    println!("Connected to device: {:?}", device.properties().await?.and_then(|p| p.local_name));
            //    break;
            //}
        }


   Ok(())
}