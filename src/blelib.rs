use std::error::Error;
use std::sync::{Mutex, Arc};
use std::time::Duration;
use futures::stream::StreamExt;
use tokio::sync::Mutex as TokioMutex;
use tokio::time::Instant;
use btleplug::api::{Central, Peripheral as BtlePeripheral, Characteristic, WriteType};
use btleplug::platform::{Adapter, Peripheral, Manager as PlatformManager};
use btleplug::api::Manager as _;

/// A structure to manage BLE connections
pub struct BleController {
    manager: Option<PlatformManager>,
    adapter: Option<Adapter>,
    device: Option<Peripheral>,
    write_char: Option<Characteristic>,
    notify_char: Option<Characteristic>,
    connected: bool,
    buffer: Arc<Mutex<String>>,
    last_send_time: Option<Instant>,
    sending: Arc<TokioMutex<()>>,
}

impl BleController {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            manager: None,
            adapter: None,
            device: None,
            write_char: None,
            notify_char: None,
            connected: false,
            buffer: Arc::new(Mutex::new(String::new())),
            last_send_time: None,
            sending: Arc::new(TokioMutex::new(())),
        })
    }

    pub fn clone(&self) -> Self {
        Self {
            manager: None, // These cannot be cloned
            adapter: None, // These cannot be cloned
            device: None,  // These cannot be cloned
            write_char: self.write_char.clone(),
            notify_char: self.notify_char.clone(),
            connected: self.connected,
            buffer: self.buffer.clone(),
            last_send_time: self.last_send_time,
            sending: self.sending.clone(),
        }
    }

    pub async fn connect(&mut self) -> Result<(), Box<dyn Error>> {
        let manager = PlatformManager::new().await?;
        self.manager = Some(manager);
        let adapters = self.manager.as_ref().unwrap().adapters().await?;
        self.adapter = Some(adapters.into_iter().next().ok_or("No adapters found")?);

        let adapter = self.adapter.as_ref().unwrap();
        adapter.start_scan(Default::default()).await?;
        tokio::time::sleep(Duration::from_secs(2)).await;
        let devices = adapter.peripherals().await?;
        
        for device in devices {
            if device.is_connected().await? {
                device.connect().await?;
                self.device = Some(device);
                self.discover_characteristics().await?;
                self.connected = true;
                break;
            }
        }
        
        if self.device.is_none() {
            return Err("No connected BLE devices found".into());
        }
        
        Ok(())
    }

    pub async fn discover_characteristics(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(device) = &self.device {
            device.discover_services().await?;
            let characteristics = device.characteristics();
            
            for characteristic in characteristics {
                let uuid = characteristic.uuid.to_string();
                if uuid == "0000ffe2-0000-1000-8000-00805f9b34fb" || uuid == "0000ff02-0000-1000-8000-00805f9b34fb" {
                    self.write_char = Some(characteristic.clone());
                } else if uuid == "0000ffe1-0000-1000-8000-00805f9b34fb" || uuid == "0000ff01-0000-1000-8000-00805f9b34fb" {
                    self.notify_char = Some(characteristic.clone());
                    let controller = self.clone();
                    let characteristic_clone = characteristic.clone();
                    device.subscribe(&characteristic).await?;
                    let mut controller_clone = controller.clone();
                    let mut notifications = device.notifications().await?;
                    
                    tokio::spawn(async move {
                        while let Some(notification) = notifications.next().await {
                            if notification.uuid == characteristic_clone.uuid {
                                let value = notification.value;
                                let hex = value.iter().map(|b| format!("{:02x}", b)).collect::<String>().to_uppercase();
                                println!("Notification received: {}", hex);
                                controller_clone.add_content(hex);
                            }
                        }
                    });
                }
            }
        }
        
        Ok(())
    }

    pub fn add_content(&mut self, content: String) {
        let mut buffer = self.buffer.lock().unwrap();
        *buffer = content;
    }

    pub fn get_content(&self) -> String {
        let buffer = self.buffer.lock().unwrap();
        buffer.clone()
    }

    pub fn is_connected(&self) -> bool {
        self.connected
    }

    pub async fn send(&mut self, bytes: &[u8]) -> Result<(), String> {
        // Acquire the async-aware mutex. This prevents multiple `send` operations
        // from running concurrently. The lock is held for the duration of the `send_data` await.
        let _guard = self.sending.lock().await;

        if !self.is_connected() {
            return Err("Not connected".to_string());
        }
        
        self.last_send_time = Some(Instant::now());

        self.send_data(bytes).await
    }

    async fn send_data(&self, bytes: &[u8]) -> Result<(), String> {
        if let (Some(device), Some(write_char)) = (&self.device, &self.write_char) {
            device.write(&write_char, bytes, WriteType::WithoutResponse).await.map_err(|e| e.to_string())?;
            Ok(())
        } else {
            Err("BLE not initialized".to_string())
        }
    }
}

/// Helper function to decode hex string to bytes
pub fn decode(hex: &str) -> Result<Vec<u8>, String> {
    if hex.len() % 2 != 0 {
        return Err("Hex string must have an even number of characters".into());
    }
    
    let mut result = Vec::with_capacity(hex.len() / 2);
    let mut chars = hex.chars();
    
    while let (Some(a), Some(b)) = (chars.next(), chars.next()) {
        let byte = match (a.to_digit(16), b.to_digit(16)) {
            (Some(high), Some(low)) => (high << 4) as u8 | low as u8,
            _ => return Err(format!("Invalid hex character in {}", hex)),
        };
        result.push(byte);
    }
    
    Ok(result)
}

/// Helper function to find all BLE devices 
pub async fn scan_devices(timeout_seconds: u64) -> Result<Vec<(Peripheral, String)>, Box<dyn Error>> {
    let manager = PlatformManager::new().await?;
    let adapters = manager.adapters().await?;
    let adapter = adapters.into_iter().next().ok_or("No adapters found")?;
    
    adapter.start_scan(Default::default()).await?;
    tokio::time::sleep(Duration::from_secs(timeout_seconds)).await;
    
    let peripherals = adapter.peripherals().await?;
    let mut device_list = Vec::new();
    
    for peripheral in peripherals {
        if let Ok(Some(properties)) = peripheral.properties().await {
            let name = properties.local_name.unwrap_or_else(|| "Unknown".to_string());
            device_list.push((peripheral, name));
        }
    }
    
    Ok(device_list)
}
