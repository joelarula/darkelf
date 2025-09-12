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
    core::GUID,
    Devices::Bluetooth::BluetoothLEDevice,
    Devices::Enumeration::DeviceInformation,
    Devices::Bluetooth::GenericAttributeProfile::{
        GattDeviceService, GattCharacteristic, GattCharacteristicProperties, 
        GattCommunicationStatus, GattWriteOption,
        GattClientCharacteristicConfigurationDescriptorValue
    },
    Storage::Streams::{DataWriter, DataReader},
    Foundation::{EventRegistrationToken, TypedEventHandler},
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
    notification_token: Option<windows::Foundation::EventRegistrationToken>,
}

impl WinBlueController {
    pub async fn new(device_info: Option<&DeviceInformation> ) -> Result<Self, Box<dyn Error>> {
        debug!("WinBlueController::new called");
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
        debug!("WinBlueController::connect called");
        if let Some(device_info) = self.device_info.as_ref() {
            let device_id = device_info.Id()?;
            debug!("Connecting to BLE device with id: {}", device_id);
            self.device = Some(BluetoothLEDevice::FromIdAsync(&device_id)?.get()?);
            
            if self.write_char.is_none() || self.notify_char.is_none() {
                debug!("Discovering characteristics after connect");
                self.discover_characteristics().await?;
            }

            self.connected = true;
            debug!("Device connected");
        }
        Ok(())
    }

    /// Discover characteristics for the connected device
    pub async fn discover_characteristics(&mut self) -> Result<(), Box<dyn Error>> {
        debug!("WinBlueController::discover_characteristics called");
        if let Some(device) = &self.device {
            // Step 3: Enumerate services
            let services_result = device.GetGattServicesAsync()?.get()?;
            debug!("Enumerating GATT services");
            for j in 0..services_result.Services()?.Size()? {
                let service: GattDeviceService = services_result.Services()?.GetAt(j)?;
                let service_uuid = service.Uuid()?;
                let service_uuid_str: String = format!("{:?}", service_uuid).to_uppercase();
                debug!("Found service UUID: {}", service_uuid_str);
                if blue::LASER_SERVICE_UUID.contains(&service_uuid_str.as_str()) {
                    self.service_uuid = Some(service_uuid);
                    info!("    Service UUID: {:?} Found Laser Service uuid", service_uuid);  
                    let characteristics_result = service.GetCharacteristicsAsync()?.get()?;
                    let characteristics = characteristics_result.Characteristics()?;
                    debug!("Enumerating characteristics for service: {}", service_uuid_str);
                    for k in 0..characteristics.Size()? {
                        let characteristic: GattCharacteristic = characteristics.GetAt(k)?;
                        let props = characteristic.CharacteristicProperties()?;
                        let char_uuid: GUID = characteristic.Uuid()?;
                        let char_uuid_str = format!("{:?}", char_uuid).to_uppercase();
                        debug!("Characteristic UUID: {} Properties: {:?}", char_uuid_str, props);
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
            debug!("No writable characteristic found after discovery");
            return Err("No writable characteristic found".into());
        }
        if self.notify_char.is_none() {
            debug!("No notify characteristic found after discovery");
            return Err("No notify characteristic found".into());
        }
        debug!("Characteristic discovery complete");
        Ok(())
    }

    /// Setup notifications for all discovered notification characteristics
    pub async fn setup_all_notifications(&mut self) -> Result<(), Box<dyn Error>> {
        debug!("WinBlueController::setup_all_notifications called");
        if let Some(ref notify_char) = self.notify_char.clone() {
            debug!("Setting up notifications for characteristic");
            self.setup_notifications(notify_char).await?;
        } else {
            debug!("No notification characteristic available");
        }
        Ok(())
    }

    /// Setup notifications for a characteristic by writing to its CCCD
    async fn setup_notifications(&mut self, characteristic: &GattCharacteristic) -> Result<(), Box<dyn Error>> {
        debug!("WinBlueController::setup_notifications called for characteristic");
        let buffer_clone = self.buffer.clone();

        // Create a notification handler using TypedEventHandler
        use windows::Foundation::TypedEventHandler;
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
                        
                        debug!("Notification received: {}", hex);
                        
                        let mut buffer = buffer_clone.lock().unwrap();
                        *buffer = hex;
                    }
                }
            }
            Ok(())
        });

        // Write 0x0001 to the CCCD to enable notifications (matches Frame 265 in the BT log)
        debug!("Writing 0x0001 to characteristic's CCCD to enable notifications");
        match characteristic.WriteClientCharacteristicConfigurationDescriptorAsync(
            windows::Devices::Bluetooth::GenericAttributeProfile::GattClientCharacteristicConfigurationDescriptorValue::Notify
        )?.get() {
            Ok(GattCommunicationStatus::Success) => {
                debug!("Successfully enabled notifications on characteristic");
                let token = characteristic.ValueChanged(&handler)?;
                self.notification_token = Some(token);
                Ok(())
            },
            Ok(status) => {
                error!("Failed to enable notifications: {:?}", status);
                Err(format!("Failed to enable notifications: {:?}", status).into())
            },
            Err(e) => {
                error!("Error enabling notifications: {:?}", e);
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
        debug!("WinBlueController::send called with {} bytes", bytes.len());
        if bytes.len() != 20 || !bytes.starts_with(&[0xE0, 0xE1, 0xE2, 0xE3]) {
            return Err("Invalid command: must be 20 bytes starting with E0E1E2E3".to_string());
        }
        // Acquire the async-aware mutex. This prevents multiple `send` operations
        // from running concurrently. The lock is held for the duration of the `send_data` await.
        let _guard = self.sending.lock().await;

        if !self.is_connected() {
            debug!("Send called but controller is not connected");
            return Err("Not connected".to_string());
        }

        if let Some(last_send) = self.last_send_time {
            let elapsed = Instant::now().duration_since(last_send);
            if elapsed < Duration::from_millis(20) {
                sleep(Duration::from_millis(20) - elapsed).await;
            }
        }
        self.last_send_time = Some(Instant::now());
        debug!("Calling send_data");
        self.send_data(bytes).await
    }

    async fn send_data(&self, bytes: &[u8]) -> Result<(), String> {
        debug!("WinBlueController::send_data called");
        if let Some(write_char) = &self.write_char {
            // Debug: print the command being written
            let hex_str = bytes.iter().map(|b| format!("{:02X}", b)).collect::<Vec<_>>().join("");
            debug!("Writing command to BLE: {}", hex_str);

            // Create a buffer from the bytes
            let writer = DataWriter::new().map_err(|e| e.to_string())?;
            writer.WriteBytes(bytes).map_err(|e| e.to_string())?;
            let buffer = writer.DetachBuffer().map_err(|e| e.to_string())?;
            
            // Write to the characteristic
            debug!("Calling WriteValueAsync");
            let write_result = write_char.WriteValueAsync(
                &buffer
            ).map_err(|e| e.to_string())?;
            
            // Check the result
            match write_result.get() {
                Ok(GattCommunicationStatus::Success) => {
                    debug!("Write to BLE characteristic succeeded");
                    Ok(())
                },
                Ok(status) => {
                    debug!("Write to BLE characteristic failed with status: {:?}", status);
                    Err(format!("Write failed with status: {:?}", status))
                },
                Err(e) => {
                    debug!("Write to BLE characteristic failed with error: {:?}", e);
                    Err(format!("Write failed with error: {:?}", e))
                },
            }
        } else {
            debug!("send_data called but write_char is None");
            Err("Write characteristic not found".to_string())
        }
    }

    /// Disconnect and clean up resources
    pub async fn disconnect(&mut self) -> Result<(), Box<dyn Error>> {
        debug!("WinBlueController::disconnect called");
        if let (Some(characteristic), Some(token)) = (&self.notify_char, &self.notification_token) {
            characteristic.RemoveValueChanged(*token)?;
        }

        self.device = None;
        self.write_char = None;
        self.notify_char = None;
        self.connected = false;
        self.notification_token = None;
        debug!("Device disconnected and resources cleaned up");
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
