use btleplug::api::{
    BDAddr, Central, CentralEvent, Characteristic, Manager as _, Peripheral as _,
    PeripheralProperties, ScanFilter, Service,
};
use btleplug::platform::{Adapter, Manager, PeripheralId, Peripheral};
use futures::stream::StreamExt;
use std::collections::HashMap;
use std::error::Error;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::select;
use tokio::task;
use tokio::time::{sleep, Duration as TokioDuration};
use uuid::Uuid;

// TD5322A is a Bluetooth 5.1 chip from Tudasemi,
const DEVICE_NAME_PREFIX: &str = "TD5322A_";
const GENERIC_ACCESS_SERVICE_UUID: Uuid = Uuid::from_u128(0x00001800_0000_1000_8000_00805f9b34fb);
const DEVICE_NAME_CHARACTERISTIC_UUID: Uuid = Uuid::from_u128(0x00002A00_0000_1000_8000_00805f9b34fb);

#[derive(Debug, Clone)]
pub struct BluetoothDevice {
    pub id: PeripheralId,
    pub name: Option<String>,
    pub address: Option<BDAddr>,
    pub manufacturer_data: HashMap<u16, Vec<u8>>,
    pub services: Vec<Service>,
    pub is_connected: bool,
    // Add other relevant information here
}

impl BluetoothDevice {
    pub fn new(id: PeripheralId) -> Self {
        BluetoothDevice {
            id,
            name: None,
            address: None,
            manufacturer_data: HashMap::new(),
            services: Vec::new(),
            is_connected: false,
        }
    }

    // Method to update device information
    pub fn update_info(&mut self, properties: &PeripheralProperties) {
        self.name = properties.local_name.clone();
        self.address = Some(properties.address);
        self.manufacturer_data = properties.manufacturer_data.clone();
    }

    pub fn update_services(&mut self, services: Vec<Service>) {
        self.services = services;
    }

    pub fn update_connection_status(&mut self, is_connected: bool) {
        self.is_connected = is_connected;
    }

    pub fn print_all(&self) {
        println!("Device ID: {:?}", self.id);
        println!("Device Name: {:?}", self.name);
        println!("Device Address: {:?}", self.address);
        println!("Manufacturer Data: {:?}", self.manufacturer_data);
        println!("Services: {:?}", self.services);
        println!("Is Connected: {:?}", self.is_connected);
    }
}

/// Scans for Bluetooth devices and returns a list of devices with names starting with "TD5322A_".
pub async fn scan_ble_devices(duration: u64) -> Result<Vec<BluetoothDevice>, Box<dyn Error>> {
    
    let manager = Manager::new().await?;
    let adapters = manager.adapters().await?;
    let central = adapters
        .into_iter()
        .nth(0)
        .ok_or("No Bluetooth adapters found")?;

    let adapter_info = central.adapter_info().await?;
    println!("Using adapter: {}", adapter_info);

    central.start_scan(ScanFilter::default()).await?;
    println!("Scanning for {} seconds...", duration);

    let mut events = central.events().await?;
    let scan_duration = Duration::from_secs(duration);
    let mut timeout: Pin<Box<tokio::time::Sleep>> = Box::pin(sleep(scan_duration));

    let discovered_devices: Arc<Mutex<HashMap<PeripheralId, BluetoothDevice>>> =
        Arc::new(Mutex::new(HashMap::new()));

    loop {
        select! {
            Some(event) = events.next() => {
                match event {
                    CentralEvent::DeviceDiscovered(id) => {
                        let mut devices = discovered_devices.lock().unwrap();
                        if !devices.contains_key(&id) {
                            println!("Found device: {}", id.to_string());
                            devices.insert(id.clone(), BluetoothDevice::new(id.clone()));
                        }
                    }
                    CentralEvent::DeviceUpdated(id) => {
                        let peripheral_result = central.peripheral(&id).await;
                        if let Ok(peripheral) = peripheral_result {
                            if let Ok(Some(properties)) = peripheral.properties().await {
                                let mut devices = discovered_devices.lock().unwrap();
                                if let Some(device) = devices.get_mut(&id) {
                                    device.update_info(&properties);
                                }
                            }
                        }
                    }
                    _ => {}
                }
            },
            _ = &mut timeout => {
                println!("\nScan finished.");
                central.stop_scan().await?;
                break;
            }
        }
    }

    // Connect to peripherals and discover services
    let mut devices_vec: Vec<BluetoothDevice> = {
        let devices = discovered_devices.lock().unwrap();
        devices.values().cloned().collect()
    };

    for device in &mut devices_vec {
        let peripheral_result = central.peripheral(&device.id).await;
        if let Ok(peripheral) = peripheral_result {
            if !peripheral.is_connected().await? {
                println!("Connecting to peripheral {:?}...", device.name);
                if let Err(err) = peripheral.connect().await {
                    eprintln!("Error connecting to peripheral, skipping: {}", err);
                    continue;
                }
            }

            device.update_connection_status(peripheral.is_connected().await?);
            println!("Discovering services for peripheral {:?}...", device.name);
            if let Err(err) = peripheral.discover_services().await {
                eprintln!("Error discovering services for peripheral, skipping: {}", err);
                continue;
            }

            let services = peripheral.services().into_iter().collect::<Vec<_>>();
            device.update_services(services);

            if peripheral.is_connected().await? {
                println!("Disconnecting from peripheral {:?}...", device.name);
                if let Err(err) = peripheral.disconnect().await {
                    eprintln!("Error disconnecting from peripheral: {}", err);
                }
            }
        }
    }

   // for device in &devices_vec {
   //     device.print_all();
   // }

    Ok(devices_vec)
}