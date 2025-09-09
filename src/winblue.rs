use crate::blue::BlueController;
use std::future::Future;
use std::pin::Pin;
use std::error::Error;
use std::sync::{Mutex, Arc};
use std::time::Duration;
use tokio::sync::Mutex as TokioMutex;
use log::{info, warn, error,debug};
use tokio::time::Instant;
use windows::core::GUID;
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
use crate::blue;

pub type Characteristic = GattCharacteristic;



/// A structure to manage BLE connections using Windows API
pub struct WinBlueController {
    device_info: Option<DeviceInformation>,
    device: Option<BluetoothLEDevice>,
    write_char: Option<GattCharacteristic>,
    notify_char: Option<GattCharacteristic>,
    service_uuid: Option<GUID>,
    connected: bool,
    buffer: Arc<Mutex<String>>,
    last_send_time: Option<Instant>,
    sending: Arc<TokioMutex<()>>,
    // Temporarily disable notification token to make the code compile
    notification_token: Option<()>, // TODO: Fix this to use proper Windows API type
}

impl WinBlueController {
    pub async fn new(device_info: Option<&DeviceInformation> ) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            device_info: device_info.cloned(),
            device: None,
            write_char: None,
            notify_char: None,
            service_uuid: None,            
            connected: false,
            buffer: Arc::new(Mutex::new(String::new())),
            last_send_time: None,
            sending: Arc::new(TokioMutex::new(())),
            notification_token: None,
        })
    }



    /// Connect to a BLE device 
    pub async fn connect(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(device_info) = self.device_info.as_ref() {
            let device_id = device_info.Id()?;
            self.device = Some(BluetoothLEDevice::FromIdAsync(&device_id)?.get()?);
            self.connected = true;
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
                if blue::LASER_SERVICE_UUID.contains(&service_uuid_str.as_str()) {
                    self.service_uuid = Some(service_uuid);
                    info!("    Service UUID: {:?} Found Laser Service uuid", service_uuid);  
                    let characteristics_result = service.GetCharacteristicsAsync()?.get()?;
                    let characteristics = characteristics_result.Characteristics()?;
                   

                    for k in 0..characteristics.Size()? {
                        let characteristic: GattCharacteristic = characteristics.GetAt(k)?;
                        let props = characteristic.CharacteristicProperties()?;
                        let char_uuid: GUID = characteristic.Uuid()?;
                        let char_uuid_str = format!("{:?}", char_uuid).to_uppercase();
                        

                        // If writable and matches UUID, save for writing
                        if props & GattCharacteristicProperties::Write == GattCharacteristicProperties::Write || 
                           props & GattCharacteristicProperties::WriteWithoutResponse == GattCharacteristicProperties::WriteWithoutResponse {
                            if blue::WRITE_UUIDS.contains(&char_uuid_str.as_str()) {
                                info!("    Write UUID: {:?} Found Laser Service write uuid", char_uuid);  
                                self.write_char = Some(characteristic.clone());
                            }
                        }

                        // If notifiable/indicatable and matches UUID, enable notifications
                        if (props & GattCharacteristicProperties::Notify == GattCharacteristicProperties::Notify ||
                            props & GattCharacteristicProperties::Indicate == GattCharacteristicProperties::Indicate) {
                            if blue::NOTIFY_UUIDS.contains(&char_uuid_str.as_str()) {
                                info!("    Write UUID: {:?} Found Laser Service notification uuid", char_uuid);  
                                self.notify_char = Some(characteristic.clone());
                            }
                        }
                    }

                }

            }
        } else {
            return Err("Device not connected".into());
        }

        if self.write_char.is_none() {
            return Err("No writable characteristic found".into());
        }
        
        if self.notify_char.is_none() {
            return Err("No notify characteristic found".into());
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


// Implementation of the common trait for btleplug
impl BlueController for WinBlueController {
    
    fn connect<'a>(&'a mut self) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn Error>>> + Send + 'a>> {
        Box::pin(async move {
            self.connect().await
        })
    }
    
    fn send<'a>(&'a mut self, bytes: &'a [u8]) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send + 'a>> {
        Box::pin(async move {
            self.send(bytes).await
        })
    }
    
    fn get_content(&self) -> String {
        self.get_content()
    }
    
    fn is_connected(&self) -> bool {
        self.is_connected()
    }
}



pub async fn scan_laser_devices() -> Result<Vec<DeviceInformation>, Box<dyn Error>> {

    let selector = BluetoothLEDevice::GetDeviceSelector()?;
    let devices = DeviceInformation::FindAllAsyncAqsFilter(&selector)?.get()?;

    let mut device_list: Vec<DeviceInformation> = Vec::new();

    for i in 0..devices.Size()? {
        let device_info: DeviceInformation = devices.GetAt(i)?;
        let device_name = device_info.Name()?;
        let device_name_str = device_name.to_string_lossy();
        if !device_name_str.starts_with(blue::LASER_DEVICE_PREFIX) {
            continue;
        }

        let device_id = device_info.Id()?;
        info!("Found laser device: {} ({})", device_name, device_id);
        let ble_device = BluetoothLEDevice::FromIdAsync(&device_id)?.get()?;
        let services_result = ble_device.GetGattServicesAsync()?.get()?;
        for j in 0..services_result.Services()?.Size()? {

            let service: GattDeviceService = services_result.Services()?.GetAt(j)?;
            let service_uuid = service.Uuid()?;
            let str = format!("{:?}", service_uuid).to_uppercase();
            if blue::LASER_SERVICE_UUID.contains(&str.as_str()) {
                info!("Found laser service: ({:?}))", service_uuid);
                device_list.push(device_info.clone());
            }
        }
    }
    
    Ok(device_list)
}
