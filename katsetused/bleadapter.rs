use btleplug::api::{Central, CentralEvent, Manager as _, Peripheral as _, WriteType, Characteristic, ValueNotification, ScanFilter, Service, PeripheralProperties, BDAddr};
use btleplug::platform::{Adapter, Manager, PeripheralId, Peripheral};
use futures::stream::StreamExt;
use std::collections::HashMap;
use std::error::Error;
use std::time::Duration;
use tokio::sync::mpsc::{Receiver, Sender, channel};
use uuid::Uuid;
use tokio;
use futures::select;
use futures::pin_mut;
use std::fmt::Debug;

pub struct BluetoothManager {
    adapter: Adapter,
}

#[derive(Debug, Clone)]
pub struct BluetoothDevice {
    peripheral_id: PeripheralId,
    address: BDAddr,
    name: String,
    manufacturer_data: HashMap<u16, Vec<u8>>,
}

impl BluetoothManager {
    pub async fn new() -> Result<Self, Box<dyn Error + Send + Sync>> {
        let manager = Manager::new().await?;
        let adapters = manager.adapters().await?;
        let adapter = adapters.into_iter().next().ok_or("No Bluetooth adapters found")?;
        Ok(BluetoothManager { adapter })
    }

    pub async fn discover_devices(&self, _filter_names: Option<Vec<String>>, timeout: Duration) -> Result<Vec<BluetoothDevice>, Box<dyn Error + Send + Sync>> {
        self.adapter.start_scan(ScanFilter::default()).await?;

        let event_stream = self.adapter.events().await?;
        pin_mut!(event_stream); // Make event_stream movable

        let mut devices = Vec::new();
        let mut tasks: Vec<tokio::task::JoinHandle<Result<BluetoothDevice, Box<dyn Error + Send + Sync>>>> = Vec::new();

        let mut timeout_fut = tokio::time::sleep(timeout);
        tokio::pin!(timeout_fut);

        loop {
            tokio::select! {
                event = event_stream.next() => {
                    match event {
                        Some(CentralEvent::DeviceDiscovered(id)) => {
                            let central = self.adapter.clone();
                            match central.peripheral(&id).await {
                                Ok(p) => {
                                    match p.properties().await {
                                        Ok(Some(props)) => {
                                            let mut device_name = props.local_name;

                                            // If the local_name is not present, some devices put it in the manufacturer data.
                                            // Let's check there. The name "TD5322A" might be part of this data.
                                            if device_name.is_none() {
                                                for (_company_id, data) in props.manufacturer_data.iter() {
                                                    let name_from_manu = String::from_utf8_lossy(data).to_string();
                                                    if name_from_manu.starts_with("TD5322A") {
                                                        println!("Discovered matching device: {}", name_from_manu);
                                                        device_name = Some(name_from_manu);
                                                        break;
                                                    }
                                                }
                                            }

                                            let final_name = device_name.clone().unwrap_or_else(|| "Unknown Device".to_string());
                                            let device = BluetoothDevice::new(p.id(), props.address, final_name.clone(), props.manufacturer_data.clone());
                                            devices.push(device);
                                        }
                                        Ok(None) => {
                                            eprintln!("Device has no properties");
                                        }
                                        Err(e) => {
                                            eprintln!("Error getting properties: {}", e);
                                        }
                                    }
                                }
                                Err(e) => {
                                    eprintln!("Error getting peripheral: {}", e);
                                }
                            }
                        },
                        _ => {}
                    }
                },
                _ = &mut timeout_fut => {
                    println!("Discovery timed out.");
                    break;
                },
            }
        }
        self.adapter.stop_scan().await?; // Stop scanning after timeout

        Ok(devices)
    }
}

impl BluetoothDevice {
    pub fn new(peripheral_id: PeripheralId, address: BDAddr, name: String, manufacturer_data: HashMap<u16, Vec<u8>>) -> Self {
        BluetoothDevice {
            peripheral_id,
            address,
            name,
            manufacturer_data,
        }
    }

    pub async fn print_info(&self, peripheral: &Peripheral) -> Result<(), Box<dyn Error + Send + Sync>> {
        println!("Checking connection status...");
        if peripheral.is_connected().await? {
            println!("Device is connected.");
            println!("Device Name: {:?}", self.name);
            println!("Device Address: {:?}", self.address);
            println!("Device ID: {:?}", self.peripheral_id);
            println!("Manufacturer Data: {:?}", self.manufacturer_data);

            println!("Attempting to discover services...");
            peripheral.discover_services().await?;
            let services = peripheral.services();
            println!("Services:");
            for service in &services {
                println!("  Service UUID: {}", service.uuid);
                let characteristics = peripheral.characteristics();
                println!("    Characteristics:");
                for characteristic in &characteristics {
                    println!("      Characteristic UUID: {}", characteristic.uuid);
                    println!("        Properties: {:?}", characteristic.properties);
                }
            }
        } else {
            println!("Device is not connected.");
        }

        Ok(())
    }

    pub async fn get_name(&self) -> Result<String, Box<dyn Error + Send + Sync>> {
        Ok(self.name.clone())
    }

    pub fn get_address(&self) -> String {
        format!("{}", self.address)
    }

    pub async fn write(&self, service_uuid: &str, char_uuid: &str, data: &[u8], peripheral: &Peripheral) -> Result<(), Box<dyn Error + Send + Sync>> {
        let services = peripheral.services();
        let mut service_map = HashMap::new();
        for service in services {
            service_map.insert(
                format!("{}", service.uuid),
                peripheral.characteristics().into_iter().collect::<Vec<_>>(),
            );
        }
        let chars = service_map.get(service_uuid).ok_or("Service not found")?;
        let characteristic = chars.iter().find(|c| format!("{}", c.uuid) == char_uuid).ok_or("Characteristic not found")?;
        peripheral.write(characteristic, data, WriteType::WithResponse).await?;
        Ok(())
    }

    pub async fn notifications(&self, service_uuid: &str, char_uuid: &str, peripheral: &Peripheral) -> Result<Receiver<ValueNotification>, Box<dyn Error + Send + Sync>> {
        let services = peripheral.services();
        let mut service_map = HashMap::new();
        for service in services {
            service_map.insert(
                format!("{}", service.uuid),
                peripheral.characteristics().into_iter().collect::<Vec<_>>(),
            );
        }
        let chars = service_map.get(service_uuid).ok_or("Service not found")?;
        let characteristic = chars.iter().find(|c| format!("{}", c.uuid) == char_uuid).ok_or("Characteristic not found")?;
        peripheral.subscribe(characteristic).await?;
        // You'd set up a channel and spawn a task here to forward notifications.
        // For brevity, we'll just stub this.
        unimplemented!()
    }

    pub async fn disconnect(&self, peripheral: &Peripheral) -> Result<(), Box<dyn Error + Send + Sync>> {
        peripheral.disconnect().await?;
        Ok(())
    }
}