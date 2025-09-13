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
    Foundation::TypedEventHandler,
};

// Wrapper type for notification token to avoid direct EventRegistrationToken dependency
#[derive(Clone, Copy)]
struct NotificationToken(i64);
use std::sync::{Arc, Mutex};
use tokio::sync::Mutex as TokioMutex;
use std::error::Error;
use std::time::Duration;
use hex::decode;
use tokio::time::{Instant, sleep};
use log::{info, warn, error, debug};
use serialport::{SerialPort, SerialPortType};

use crate::blue::{self, BlueController};



pub type Characteristic = GattCharacteristic;

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
    notification_token: Option<NotificationToken>,
    cccd_state: Option<GattClientCharacteristicConfigurationDescriptorValue>,
    // New fields matching JS implementation - temporarily marked as unused while implementing
    connect_count: u32,
    ready_to_receive: bool,
    can_send: bool,
    cmd_sending: bool,
    manual_disconnect: bool,
    connection_state: i32, // -1=connecting, 0=disconnected, 1=connected, 2=ready
    min_send_interval: Duration,
}

impl WinBlueController {
    pub async fn new(device_info: Option<&DeviceInformation>) -> Result<Self, Box<dyn Error>> {
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
            cccd_state: None,
            connect_count: 0,
            ready_to_receive: false,
            can_send: false,
            cmd_sending: false,
            manual_disconnect: false,
            connection_state: 0,
            min_send_interval: Duration::from_millis(100), // Same as JS 100ms interval
        })
    }

    pub async fn connect(&mut self) -> Result<(), Box<dyn Error>> {
        debug!("WinBlueController::connect called");
        self.connect_count += 1;
        self.connection_state = -1; // Set to connecting
        
        if let Some(device_info) = self.device_info.as_ref() {
            let device_id = device_info.Id()?;
            debug!("Connecting to BLE device with id: {}", device_id);
            self.device = Some(BluetoothLEDevice::FromIdAsync(&device_id)?.get()?);
            self.discover_characteristics().await?;
            self.connected = true;
            self.connection_state = 1; // Set to connected
            debug!("Device connected");
            
            // Only set ready_to_receive after successful characteristic setup
            self.ready_to_receive = true;
            self.connection_state = 2; // Set to ready
            
            // Send initialization command (if required)
            self.send_init_command().await?;
        } else {
            self.connection_state = 0; // Set to disconnected
        }
        Ok(())
    }

    async fn send_init_command(&mut self) -> Result<(), String> {
        // Clear any pending data
        {
            let mut buffer = self.buffer.lock().unwrap();
            buffer.clear();
        }

        // Send initialization sequence
        let init_cmd = "E0E1E2E38BCE183AE4E5E6E70000000000000000"; // Initial setup command
        let bytes = decode(init_cmd).map_err(|e| format!("Invalid init command: {}", e))?;
        
        // Send init command and wait for response
        self.send_ble(&bytes).await?;
        debug!("First initialization command sent successfully");
        
        // Wait for device to stabilize and verify response
        self.verify_response("E2E3", 500).await?;
        debug!("Received response to first initialization command");
        
        // Wait additional stabilization time
        sleep(Duration::from_millis(500)).await;
        
        // Clear buffer before next command
        {
            let mut buffer = self.buffer.lock().unwrap();
            buffer.clear();
        }
        
        // Send color mode initialization
        let color_init = "E0E1E2E3FF01FF32FF0100FF0000000000000000";
        let color_bytes = decode(color_init).map_err(|e| format!("Invalid color init command: {}", e))?;
        self.send_ble(&color_bytes).await?;
        debug!("Color initialization command sent successfully");
        
        // Verify color init response
        self.verify_response("E2E3", 500).await?;
        debug!("Received response to color initialization command");
        
        Ok(())
    }

    async fn send_ble(&mut self, data: &[u8]) -> Result<(), String> {
        if let Some(write_char) = &self.write_char {
            let _lock = self.sending.lock().await;
            
            // Create the data writer and store the bytes
            let writer = match DataWriter::new() {
                Ok(w) => w,
                Err(e) => return Err(format!("Failed to create DataWriter: {}", e))
            };
            
            if let Err(e) = writer.WriteBytes(data) {
                return Err(format!("Failed to write bytes to buffer: {}", e));
            }
            
            let buffer = match writer.DetachBuffer() {
                Ok(b) => b,
                Err(e) => return Err(format!("Failed to detach buffer: {}", e))
            };
            
            // Write to the characteristic with write without response
            let result = write_char.WriteValueWithOptionAsync(
                &buffer,
                GattWriteOption::WriteWithoutResponse,
            );
            
            let async_op = match result {
                Ok(op) => op,
                Err(e) => return Err(format!("Failed to start write operation: {}", e))
            };
            
            let status = match async_op.get() {
                Ok(s) => s,
                Err(e) => return Err(format!("Failed to complete write operation: {}", e))
            };
            
            match status {
                GattCommunicationStatus::Success => {
                    debug!("Data sent successfully: {:?}", data);
                    self.last_send_time = Some(Instant::now());
                    Ok(())
                }
                _ => {
                    error!("Failed to write data: {:?}", status);
                    Err(format!("Failed to write data: {:?}", status))
                }
            }
        } else {
            error!("Write characteristic not found");
            Err("Write characteristic not found".to_string())
        }
    }

    pub async fn discover_characteristics(&mut self) -> Result<(), Box<dyn Error>> {
        debug!("WinBlueController::discover_characteristics called");
        if let Some(device) = &self.device {
            let services_result = device.GetGattServicesAsync()?.get()?;
            debug!("Enumerating GATT services");
            for j in 0..services_result.Services()?.Size()? {
                let service: GattDeviceService = services_result.Services()?.GetAt(j)?;
                let service_uuid = service.Uuid()?;
                let service_uuid_str = format!("{:?}", service_uuid).to_uppercase();
                debug!("Found service UUID: {}", service_uuid_str);
                if blue::LASER_SERVICE_UUID.contains(&service_uuid_str.as_str()) {
                    self.service_uuid = Some(service_uuid);
                    info!("Service UUID: {:?} Found Laser Service uuid", service_uuid);
                    let characteristics_result = service.GetCharacteristicsAsync()?.get()?;
                    let characteristics = characteristics_result.Characteristics()?;
                    debug!("Enumerating characteristics for service: {}", service_uuid_str);
                    for k in 0..characteristics.Size()? {
                        let characteristic: GattCharacteristic = characteristics.GetAt(k)?;
                        let props = characteristic.CharacteristicProperties()?;
                        let char_uuid: GUID = characteristic.Uuid()?;
                        let char_uuid_str = format!("{:?}", char_uuid).to_uppercase();
                        debug!("Characteristic UUID: {} Properties: {:?}", char_uuid_str, props);
                        if (props & GattCharacteristicProperties::Write == GattCharacteristicProperties::Write ||
                            props & GattCharacteristicProperties::WriteWithoutResponse == GattCharacteristicProperties::WriteWithoutResponse) &&
                            blue::WRITE_UUIDS.contains(&char_uuid_str.as_str()) {
                            info!("Write UUID: {:?} Found Laser Service write uuid", char_uuid);
                                self.write_char = Some(characteristic.clone());
                            }
                        if (props & GattCharacteristicProperties::Notify == GattCharacteristicProperties::Notify ||
                            props & GattCharacteristicProperties::Indicate == GattCharacteristicProperties::Indicate) &&
                            blue::NOTIFY_UUIDS.contains(&char_uuid_str.as_str()) {
                            info!("Notify UUID: {:?} Found Laser Service notification uuid", char_uuid);
                                self.notify_char = Some(characteristic.clone());
                        }
                    }
                }
            }
        } else {
            return Err("Device not connected".into());
        }
        if self.write_char.is_none() || self.notify_char.is_none() {
            return Err("Required characteristics not found".into());
        }
        self.setup_all_notifications().await?;
        Ok(())
    }

    pub async fn setup_all_notifications(&mut self) -> Result<(), Box<dyn Error>> {
        debug!("WinBlueController::setup_all_notifications called");
        if let Some(ref notify_char) = self.notify_char.clone() {
            debug!("Setting up notifications for characteristic");
            self.setup_notifications(notify_char).await?;
        }
        Ok(())
    }

    async fn setup_notifications(&mut self, characteristic: &GattCharacteristic) -> Result<(), Box<dyn Error>> {
        debug!("WinBlueController::setup_notifications called for characteristic");
        let buffer_clone = self.buffer.clone();

        // First verify if notifications are supported
        let properties = characteristic.CharacteristicProperties()?;
        if properties & GattCharacteristicProperties::Notify != GattCharacteristicProperties::Notify &&
           properties & GattCharacteristicProperties::Indicate != GattCharacteristicProperties::Indicate {
            return Err("Characteristic does not support notifications or indications".into());
        }

        // Get current CCCD value
        let cccd_result = characteristic.ReadClientCharacteristicConfigurationDescriptorAsync()?.get();
        debug!("CCCD read result: {:?}", cccd_result);

        let handler = TypedEventHandler::<
            GattCharacteristic,
            windows::Devices::Bluetooth::GenericAttributeProfile::GattValueChangedEventArgs,
        >::new(move |_sender, args| {
            if let Some(args) = args.as_ref() {
                if let Ok(value_buffer) = args.CharacteristicValue() {
                    if let Ok(len) = value_buffer.Length() {
                        let mut value = vec![0u8; len as usize];
                        let data_reader = DataReader::FromBuffer(&value_buffer)?;
                        data_reader.ReadBytes(&mut value)?;
                        let hex = value.iter().map(|b| format!("{:02X}", b)).collect::<String>();
                        debug!("Notification received: {}", hex);
                        
                        let mut buffer = buffer_clone.lock().unwrap();
                        *buffer = hex;
                    }
                }
            }
            Ok(())
        });

        // Attempt to enable notifications
        let desired_value = if properties & GattCharacteristicProperties::Notify == GattCharacteristicProperties::Notify {
            GattClientCharacteristicConfigurationDescriptorValue::Notify
        } else {
            GattClientCharacteristicConfigurationDescriptorValue::Indicate
        };

        match characteristic.WriteClientCharacteristicConfigurationDescriptorAsync(desired_value)?.get() {
            Ok(GattCommunicationStatus::Success) => {
                debug!("Successfully enabled notifications/indications");
                let windows_token = characteristic.ValueChanged(&handler)?;
                self.notification_token = Some(NotificationToken(unsafe { std::mem::transmute(windows_token) }));
                self.cccd_state = Some(desired_value);
                Ok(())
            },
            Ok(status) => {
                let error_msg = format!("Failed to enable notifications/indications: {:?}", status);
                error!("{}", error_msg);
                Err(error_msg.into())
            },
            Err(e) => {
                error!("Error enabling notifications/indications: {:?}", e);
                Err(e.into())
            },
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

    async fn verify_response(&self, expected: &str, timeout_ms: u64) -> Result<(), String> {
        let start = Instant::now();
        let timeout = Duration::from_millis(timeout_ms);
        
        while start.elapsed() < timeout {
            let content = self.get_content();
            if content.contains(expected) {
                return Ok(());
            }
            sleep(Duration::from_millis(50)).await;
        }
        
        Err(format!("Timeout waiting for response containing '{}'", expected))
    }
    
    pub async fn send(&mut self, bytes: &[u8]) -> Result<(), String> {
        // Clear the buffer before sending
        {
            let mut buffer = self.buffer.lock().unwrap();
            buffer.clear();
        }
        
        // Enforce minimum delay between commands
        if let Some(last_time) = self.last_send_time {
            let elapsed = last_time.elapsed();
            if elapsed < self.min_send_interval {
                sleep(self.min_send_interval - elapsed).await;
            }
        }

        debug!("WinBlueController::send called with {} bytes", bytes.len());
        if bytes.len() != 20 || !bytes.starts_with(&[0xE0, 0xE1, 0xE2, 0xE3]) {
            return Err("Invalid command: must be 20 bytes starting with E0E1E2E3".to_string());
        }

        let _guard = self.sending.lock().await;
        if !self.is_connected() {
            return Err("Not connected".to_string());
        }

        // Check if we can send
        if !self.can_send {
            return Err("Device not ready to send".to_string());
        }

        // Set command sending state
        self.cmd_sending = true;

        // Send the data
        let result = if let Some(write_char) = &self.write_char {
            let writer = DataWriter::new().map_err(|e| e.to_string())?;
            writer.WriteBytes(bytes).map_err(|e| e.to_string())?;
            let buffer = writer.DetachBuffer().map_err(|e| e.to_string())?;
            
            let result = write_char.WriteValueWithOptionAsync(
                &buffer,
                GattWriteOption::WriteWithoutResponse,
            ).map_err(|e| e.to_string())?.get();
            
            match result {
                Ok(GattCommunicationStatus::Success) => {
                    debug!("Data sent successfully: {:?}", bytes);
                    self.last_send_time = Some(Instant::now());
                    Ok(())
                }
                Ok(status) => {
                    error!("Failed to write data: {:?}", status);
                    Err(format!("Failed to write data: {:?}", status))
                }
                Err(e) => Err(e.to_string())
            }
        } else {
            Err("Write characteristic not found".to_string())
        };

        // Reset command sending state
        self.cmd_sending = false;

        result
    }



    pub async fn send_animation(&mut self, coordinates: &[[u16; 2]], color: [u8; 3]) -> Result<(), String> {
        for &[x, y] in coordinates {
            let command = vec![
                0xE0, 0xE1, 0xE2, 0xE3,
                0xC0, 0xC1, 0xC2, 0xC3,
                (x & 0xFF) as u8, (x >> 8) as u8,
                (y & 0xFF) as u8, (y >> 8) as u8,
                color[0], color[1], color[2],
                1, // z=1 (laser on)
                0x00, 0x00, 0x00, 0x00,
            ];
            self.send(&command).await?;
        }
        // Send laser off command
        let off_command = vec![
            0xE0, 0xE1, 0xE2, 0xE3,
            0xC0, 0xC1, 0xC2, 0xC3,
            0x00, 0x00, 0x00, 0x00,
            color[0], color[1], color[2],
            0, // z=0 (laser off)
            0x00, 0x00, 0x00, 0x00,
        ];
        self.send(&off_command).await
    }

    pub async fn disconnect(&mut self) -> Result<(), Box<dyn Error>> {
        debug!("WinBlueController::disconnect called");
        
        // Clean up notifications first
        if let (Some(characteristic), Some(token)) = (&self.notify_char, self.notification_token) {
            // Disable notifications
            match characteristic.WriteClientCharacteristicConfigurationDescriptorAsync(
                GattClientCharacteristicConfigurationDescriptorValue::None
            )?.get() {
                Ok(GattCommunicationStatus::Success) => {
                    debug!("Successfully disabled notifications");
                },
                Ok(status) => {
                    error!("Failed to disable notifications: {:?}", status);
                },
                Err(e) => {
                    error!("Error disabling notifications: {:?}", e);
                }
            }
            
            // Remove notification handler
            let windows_token = unsafe { std::mem::transmute(token.0) };
            match characteristic.RemoveValueChanged(windows_token) {
                Ok(_) => debug!("Successfully removed notification handler"),
                Err(e) => error!("Error removing notification handler: {:?}", e)
            }
        }

        // Clean up device resources
        self.device = None;
        self.write_char = None;
        self.notify_char = None;
        self.connected = false;
        self.notification_token = None;
        self.cccd_state = None;

        debug!("Device disconnected and resources cleaned up");
        Ok(())
    }
}

impl BlueController for WinBlueController {
    fn connect<'a>(&'a mut self) -> std::pin::Pin<Box<dyn Future<Output = Result<(), Box<dyn Error>>> + Send + 'a>> {
        Box::pin(async move { self.connect().await })
    }
    
    fn send<'a>(&'a mut self, bytes: &'a [u8]) -> std::pin::Pin<Box<dyn Future<Output = Result<(), String>> + Send + 'a>> {
        Box::pin(async move { self.send(bytes).await })
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
    let mut device_list = Vec::new();

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
                info!("Found laser service: ({:?})", service_uuid);
                device_list.push(device_info.clone());
            }
        }
    }
    Ok(device_list)
}