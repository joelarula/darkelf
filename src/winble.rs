use std::error::Error;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use log::{info, warn, error,debug};
use tokio::time::Instant;
use windows::{
    core::{HSTRING, Result as WindowsResult},
    Devices::Bluetooth::BluetoothLEDevice,
    Devices::Enumeration::DeviceInformation,
    Devices::Bluetooth::GenericAttributeProfile::{
        GattDeviceService, GattCharacteristic, GattCharacteristicProperties, 
        GattCommunicationStatus, GattWriteOption
    },
    Storage::Streams::{DataWriter, Buffer},
    Foundation,
};
use crate::ble;

pub type Characteristic = GattCharacteristic;

/// A structure to manage BLE connections using Windows API
pub struct BleController {
    device: Option<BluetoothLEDevice>,
    write_char: Option<GattCharacteristic>,
    notify_char: Option<GattCharacteristic>,
    service_uuid: String,
    connected: bool,
    buffer: Arc<Mutex<String>>,
    last_send_time: Option<Instant>,
    sending: Arc<Mutex<bool>>,
    // Temporarily disable notification token to make the code compile
    notification_token: Option<()>, // TODO: Fix this to use proper Windows API type
}

impl BleController {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            device: None,
            write_char: None,
            notify_char: None,
            service_uuid: String::from("0000FF00-0000-1000-8000-00805F9B34FB"), // Default service UUID
            connected: false,
            buffer: Arc::new(Mutex::new(String::new())),
            last_send_time: None,
            sending: Arc::new(Mutex::new(false)),
            notification_token: None,
        })
    }

    pub fn clone(&self) -> Self {
        Self {
            device: None, // Can't clone device reference
            write_char: None, // Can't clone characteristic references
            notify_char: None, // Can't clone characteristic references
            service_uuid: self.service_uuid.clone(),
            connected: self.connected,
            buffer: self.buffer.clone(),
            last_send_time: self.last_send_time,
            sending: self.sending.clone(),
            notification_token: None,
        }
    }

    /// Set target service UUID
    pub fn set_service_uuid(&mut self, uuid: &str) {
        self.service_uuid = uuid.to_uppercase();
    }

    /// Connect to a BLE device with optional name filter
    pub async fn connect(&mut self) -> Result<(), Box<dyn Error>> {
        self.connect_with_name_filter(None).await
    }

    /// Connect to a BLE device with specified name filter
    pub async fn connect_with_name_filter(&mut self, name_filter: Option<&str>) -> Result<(), Box<dyn Error>> {
        // Step 1: Find BLE devices
        let selector = BluetoothLEDevice::GetDeviceSelector()?;
        let devices = DeviceInformation::FindAllAsyncAqsFilter(&selector)?.get()?;

        for i in 0..devices.Size()? {
            let device_info: DeviceInformation = devices.GetAt(i)?;
            let device_name = device_info.Name()?;
            let device_name_str = device_name.to_string_lossy();
            
            // Apply name filter if provided
            if let Some(filter) = name_filter {
                if !device_name_str.starts_with(filter) {
                    continue;
                }
            }
            
            let device_id = device_info.Id()?;
            info!("Found BLE device: {} ({})", device_name, device_id);

            // Step 2: Connect to device
            match BluetoothLEDevice::FromIdAsync(&device_id)?.get() {
                Ok(ble_device) => {
                    self.device = Some(ble_device);
                    self.connected = true;
                    break; // Found and connected, exit loop
                }
                Err(e) => {
                    error!("Failed to connect to device: {:?}", e);
                    continue;
                }
            }
        }

        if self.device.is_none() {
            return Err("No compatible BLE devices found".into());
        }

        Ok(())
    }

    /// Discover characteristics for the connected device
    pub async fn discover_characteristics(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(device) = &self.device {
            // Step 3: Enumerate services
            let services_result = device.GetGattServicesAsync()?.get()?;
            
            for j in 0..services_result.Services()?.Size()? {
                let service: GattDeviceService = services_result.Services()?.GetAt(j)?;
                let service_uuid = service.Uuid()?;
                let service_uuid_str = format!("{:?}", service_uuid).to_uppercase();
                
                info!("  Service UUID: {}", service_uuid_str);
                
                // If we found our target service
                if service_uuid_str == self.service_uuid {
                    // Step 4: Enumerate characteristics
                    let characteristics_result = service.GetCharacteristicsAsync()?.get()?;
                    let characteristics = characteristics_result.Characteristics()?;
                    
                    for k in 0..characteristics.Size()? {
                        let characteristic: GattCharacteristic = characteristics.GetAt(k)?;
                        let char_uuid = characteristic.Uuid()?;
                        let props = characteristic.CharacteristicProperties()?;
                        info!("    Characteristic UUID: {:?}", char_uuid);
                        
                        // Check for write characteristic
                        if props & GattCharacteristicProperties::Write == GattCharacteristicProperties::Write || 
                           props & GattCharacteristicProperties::WriteWithoutResponse == GattCharacteristicProperties::WriteWithoutResponse {
                            self.write_char = Some(characteristic.clone());
                        }
                        
                        // Check for notify characteristic
                        if props & GattCharacteristicProperties::Notify == GattCharacteristicProperties::Notify || 
                           props & GattCharacteristicProperties::Indicate == GattCharacteristicProperties::Indicate {
                            self.notify_char = Some(characteristic.clone());
                            
                            // Set up notification handler
                            self.setup_notifications(&characteristic).await?;
                        }
                    }
                }
            }
        } else {
            return Err("Device not connected".into());
        }

        if self.write_char.is_none() {
            warn!("No writable characteristic found");
        }
        
        if self.notify_char.is_none() {
            warn!("No notify characteristic found");
        }

        Ok(())
    }

    /// Setup notifications for a characteristic
    async fn setup_notifications(&mut self, characteristic: &GattCharacteristic) -> Result<(), Box<dyn Error>> {
        // First try to subscribe to value changed notifications
        // Keeping self.clone() commented as we'll need it when restoring the notification functionality
        // let controller = self.clone();
        let buffer_clone = self.buffer.clone();

        // Create a notification handler using TypedEventHandler
        use windows::Foundation::TypedEventHandler;
        let buffer_clone = buffer_clone.clone();
        let handler = TypedEventHandler::<
            GattCharacteristic,
            windows::Devices::Bluetooth::GenericAttributeProfile::GattValueChangedEventArgs,
        >::new(move |_sender, args| {
            if let Some(args) = args.as_ref() {
                if let Ok(value_buffer) = args.CharacteristicValue() {
                    if let Ok(len) = value_buffer.Length() {
                        let mut value = vec![0u8; len as usize];
                        let data_reader = windows::Storage::Streams::DataReader::FromBuffer(&value_buffer).unwrap();
                        data_reader.ReadBytes(&mut value).unwrap();
                        
                        let hex = value.iter()
                            .map(|b| format!("{:02X}", b))
                            .collect::<String>();
                        
                        info!("Notification received: {}", hex);
                        
                        let mut buffer = buffer_clone.lock().unwrap();
                        *buffer = hex;
                    }
                }
            }
            Ok(())
        });

        // Try to subscribe to notifications
        match characteristic.WriteClientCharacteristicConfigurationDescriptorAsync(
            windows::Devices::Bluetooth::GenericAttributeProfile::GattClientCharacteristicConfigurationDescriptorValue::Notify
        )?.get() {
            Ok(GattCommunicationStatus::Success) => {
                info!("Successfully subscribed to notifications");
                
                // Register the value changed event handler
                let _token = characteristic.ValueChanged(&handler)?;
                // Temporarily ignore the token
                self.notification_token = Some(());
                Ok(())
            },
            Ok(status) => {
                error!("Failed to subscribe to notifications: {:?}", status);
                Err(format!("Failed to subscribe to notifications: {:?}", status).into())
            },
            Err(e) => {
                error!("Error subscribing to notifications: {:?}", e);
                Err(e.into())
            }
        }
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
        let mut guard = self.sending.lock().unwrap();
        if *guard { return Err("Previous send in progress".to_string()); }
        *guard = true;

        if !self.is_connected() { return Err("Not connected".to_string()); }
        
        self.last_send_time = Some(Instant::now());

        let result = self.send_data(bytes).await;

        *guard = false;
        result
    }

    async fn send_data(&self, bytes: &[u8]) -> Result<(), String> {
        if let Some(write_char) = &self.write_char {
            // Create a buffer from the bytes
            let writer = DataWriter::new().map_err(|e| e.to_string())?;
            writer.WriteBytes(bytes).map_err(|e| e.to_string())?;
            let buffer = writer.DetachBuffer().map_err(|e| e.to_string())?;
            
            // Write to the characteristic
            let write_result = write_char.WriteValueAsync(
                &buffer
            ).map_err(|e| e.to_string())?;
            
            // Check the result
            match write_result.get() {
                Ok(GattCommunicationStatus::Success) => Ok(()),
                Ok(status) => Err(format!("Write failed with status: {:?}", status)),
                Err(e) => Err(format!("Write failed with error: {:?}", e)),
            }
        } else {
            Err("Write characteristic not found".to_string())
        }
    }

    /// Disconnect and clean up resources
    pub async fn disconnect(&mut self) -> Result<(), Box<dyn Error>> {
        // Unregister notification handler temporarily disabled
        // TODO: Re-enable notification cleanup once token type is fixed
        // if let (Some(characteristic), Some(token)) = (&self.notify_char, &self.notification_token) {
        //     characteristic.RemoveValueChanged(*token)?;
        // }

        self.device = None;
        self.write_char = None;
        self.notify_char = None;
        self.connected = false;
        self.notification_token = None;
        
        Ok(())
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


pub async fn scan_laser_devices() -> Result<Vec<DeviceInformation>, Box<dyn Error>> {

    let selector = BluetoothLEDevice::GetDeviceSelector()?;
    let devices = DeviceInformation::FindAllAsyncAqsFilter(&selector)?.get()?;

    let mut device_list: Vec<DeviceInformation> = Vec::new();

    for i in 0..devices.Size()? {
        let device_info: DeviceInformation = devices.GetAt(i)?;
        let device_name = device_info.Name()?;
        let device_name_str = device_name.to_string_lossy();
        if !device_name_str.starts_with(ble::LASER_DEVICE_PREFIX) {
            continue;
        }

        let device_id = device_info.Id()?;
        info!("Found Laser device: {} ({})", device_name, device_id);
        let ble_device = BluetoothLEDevice::FromIdAsync(&device_id)?.get()?;
        let services_result = ble_device.GetGattServicesAsync()?.get()?;
        for j in 0..services_result.Services()?.Size()? {

            let service: GattDeviceService = services_result.Services()?.GetAt(j)?;
            let service_uuid = service.Uuid()?;
            let str = format!("{:?}", service_uuid).to_uppercase();
            if ble::LASER_SERVICE_UUID.contains(&str.as_str()) {
                info!("Found Laser service: ({:?}))", service_uuid);
                device_list.push(device_info.clone());
            }
        }
    }
    
    Ok(device_list)
}
