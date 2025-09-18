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
use std::sync::Arc;
use tokio::sync::Mutex as TokioMutex;
use std::error::Error;
use std::time::Duration;
use tokio::time::{Instant, sleep};
use log::{info, error, debug};

use crate::blue::{self, BlueController};


pub type Characteristic = GattCharacteristic;

type ReceiverCallback = Box<dyn Fn(String) + Send + Sync>;


#[derive(Clone)]
pub struct WinBlueController {
    device_info: Option<DeviceInformation>,
    device: Option<BluetoothLEDevice>,
    write_char: Option<GattCharacteristic>,
    notify_char: Option<GattCharacteristic>,
    service_uuid: Option<GUID>,
    connected: bool,
    buffer: Arc<TokioMutex<String>>,
    last_send_time: Option<Instant>,
    sending: Arc<TokioMutex<()>>,
    notification_token: Option<NotificationToken>,
    cccd_state: Option<GattClientCharacteristicConfigurationDescriptorValue>,
    // New fields matching JS implementation - temporarily marked as unused while implementing
    connect_count: u32,
    ready_to_receive: bool,
    can_send: bool,
    cmd_sending: bool,
    connection_state: i32, // -1=connecting, 0=disconnected, 1=connected, 2=ready
    min_send_interval: Duration,
    ble_call_back: Arc<TokioMutex<Option<ReceiverCallback>>>,
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
            buffer: Arc::new(TokioMutex::new(String::new())),
            last_send_time: None,
            sending: Arc::new(TokioMutex::new(())),
            notification_token: None,
            cccd_state: None,
            connect_count: 0,
            ready_to_receive: false,
            can_send: false,
            cmd_sending: false,
            connection_state: 0,
            min_send_interval: Duration::from_millis(100), // Same as JS 100ms interval
            ble_call_back: Arc::new(TokioMutex::new(None)),
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
            
        } else {
            self.connection_state = 0; // Set to disconnected
        }
        Ok(())
    }

    async fn send_ble(&mut self, data: &[u8]) -> Result<(), String> {
        if let Some(write_char) = &self.write_char {
            let _lock = self.sending.lock().await;
            
            // Log outgoing data
            let hex = data.iter().map(|b| format!("{:02X}", b)).collect::<String>();
            let ascii = data.iter()
                .map(|&b| if b.is_ascii_graphic() { b as char } else { '.' })
                .collect::<String>();
            info!("Sending BLE data - HEX: {} ASCII: {}", hex, ascii);
            
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
        let callback_clone = self.ble_call_back.clone();

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
                        let ascii = value.iter()
                            .map(|&b| if b.is_ascii_graphic() { b as char } else { '.' })
                            .collect::<String>();
                        debug!("Received BLE data - HEX: {} ASCII: {}", hex, ascii);
                        
                        let mut buffer = buffer_clone.blocking_lock();
                        
                        // Handle empty or new buffer
                        if buffer.is_empty() && hex.starts_with("E0E1E2E3") {
                            *buffer = hex;
                        } else if !buffer.is_empty() {
                            buffer.push_str(&hex);
                        }
                        
                        if !buffer.is_empty() {
                            // Find message boundaries
                            let start_idx = buffer.rfind("E0E1E2E3");
                            let end_idx = buffer.rfind("E4E5E6E7");
                            
                            if let (Some(start), Some(end)) = (start_idx, end_idx) {
                                if end > 0 && end == buffer.len() - 8 {
                                    // We have a complete message
                                    let message = buffer[start..end + 8].to_string();
                                    buffer.clear(); // Clear buffer after extracting message
                                    debug!("Complete message received: {}", message);
                                    
                                    // Drop the buffer lock before processing
                                    drop(buffer);
                                    
                                    // Call the callback if available
                                    let callback = callback_clone.blocking_lock();
                                    if let Some(cb) = callback.as_ref() {
                                        cb(message);
                                    }
                                } else {
                                    // Keep from last start marker onwards
                                    *buffer = buffer[start..].to_string();
                                }
                            }
                        }
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
    
    pub async fn add_content(&mut self, content: String) {
        let mut buffer = self.buffer.lock().await;
        *buffer = content;
    }

    pub async fn get_content(&self) -> String {
        let buffer = self.buffer.lock().await;
        buffer.clone()
    }

    pub fn is_connected(&self) -> bool {
        self.connected
    }

    /// Internal method to process a complete BLE message and invoke the registered callback.
    /// Used by the notification handler when a full message has been assembled.
    #[allow(dead_code)]
    async fn process_received_message(&self, message: String) -> Result<(), Box<dyn Error + Send + Sync>> {
        if let Some(callback) = self.ble_call_back.lock().await.as_ref() {
            debug!("Calling callback with message: {}", message);
            callback(message);
        }
        Ok(())
    }

    // Verifies that the expected response is received within the timeout period
    async fn verify_response(&self, expected: &str, timeout_ms: u64) -> Result<(), String> {
        let start = Instant::now();
        let timeout = Duration::from_millis(timeout_ms);
        
        while start.elapsed() < timeout {
            let content = self.get_content().await;
            if content.contains(expected) {
                debug!("Expected response '{}' received", expected);
                return Ok(());
            }
            sleep(Duration::from_millis(50)).await;
        }
        
        Err(format!("Timeout waiting for response containing '{}'", expected))
    }

    async fn send_buffer_sequence(&mut self, buffers: Vec<Vec<u8>>, total_count: usize) -> Result<(), Box<dyn Error + Send + Sync>> {
        info!("Starting buffer sequence send - {} total buffers", total_count);
        
        let platform_send_interval =  Duration::from_millis(20);
     
        let mut last_send = Instant::now();
        let mut remaining_buffers = buffers;
        let mut last_progress = 0;

        while !remaining_buffers.is_empty() {
            let elapsed = last_send.elapsed();
            let delay = if elapsed < platform_send_interval {
                platform_send_interval - elapsed
            } else {
                Duration::from_millis(1)
            };

            // Wait for the appropriate delay
            sleep(delay).await;

            // Get next buffer to send
            if let Some(current_buffer) = remaining_buffers.first() {
                // Log the buffer we're about to send
                let hex = current_buffer.iter().map(|b| format!("{:02X}", b)).collect::<String>();
                let ascii = current_buffer.iter()
                    .map(|&b| if b.is_ascii_graphic() { b as char } else { '.' })
                    .collect::<String>();
                info!("Sending buffer {} of {} - HEX: {} ASCII: {}", 
                    total_count - remaining_buffers.len() + 1,
                    total_count,
                    hex,
                    ascii);
            }

            // Calculate and report progress
            let progress = ((total_count - remaining_buffers.len()) * 100) / total_count;
            if progress != last_progress {
                debug!("Send progress: {}%", progress);
                last_progress = progress;
            }

            // Get next buffer
            let current_buffer = remaining_buffers.remove(0);
            
            if current_buffer.len() == 1 && current_buffer[0] == 0xFF {
                // This is a "split" marker - add extra delay
                sleep(platform_send_interval).await;
                continue;
            }

            // Send the buffer
            if let Some(write_char) = &self.write_char {
                info!("Writing BLE data packet {} of {} ({} bytes)", 
                    total_count - remaining_buffers.len(), 
                    total_count,
                    current_buffer.len());
                    
                let writer = DataWriter::new().map_err(|e| e.to_string())?;
                writer.WriteBytes(&current_buffer).map_err(|e| e.to_string())?;
                let buffer = writer.DetachBuffer().map_err(|e| e.to_string())?;
                
                match write_char.WriteValueWithOptionAsync(
                    &buffer,
                    GattWriteOption::WriteWithoutResponse,
                )?.get() {
                    Ok(GattCommunicationStatus::Success) => {
                        info!("Successfully sent BLE packet {} of {} - HEX: {} ASCII: {}", 
                            total_count - remaining_buffers.len(),
                            total_count,
                            current_buffer.iter().map(|b| format!("{:02X}", b)).collect::<String>(),
                            current_buffer.iter().map(|&b| if b.is_ascii_graphic() { b as char } else { '.' }).collect::<String>());
                        last_send = Instant::now();
                    }
                    Ok(status) => {
                        error!("Failed to write buffer: {:?}", status);
                        return Err(format!("Failed to write buffer: {:?}", status).into());
                    }
                    Err(e) => {
                        error!("Error writing buffer: {:?}", e);
                        return Err(e.into());
                    }
                }
            }
        }

        Ok(())
    }

    pub async fn send(&mut self, cmd: &str) -> Result<(), Box<dyn Error>> {
        // Basic validation
        if cmd.is_empty() {
            return Ok(());
        }

        info!("Initiating BLE command send: {}", cmd);
        
        // Handle non-command data when not ready
        if !cmd.starts_with("E0E1E2E3") {
            if !self.can_send {
                debug!("Simulating send for non-command data");
                sleep(Duration::from_millis(20)).await;
                return Ok(());
            }
            return Err("Invalid command: must start with E0E1E2E3".into());
        }

        // Connection checks
        if !self.is_connected() {
            return Err("Not connected".into());
        }

        // Use a combined check of both flags
        if self.cmd_sending || !self.can_send {
            error!("Last command is still sending or device not ready");
            return Err("Previous command still in progress or device not ready".into());
        }

        self.cmd_sending = true;
        self.can_send = false; // Block new sends until current one completes

        // Prepare buffers
        let bytes = match hex::decode(cmd) {
            Ok(b) => b,
            Err(e) => {
                self.cmd_sending = false;
                return Err(format!("Invalid hex string: {}", e).into());
            }
        };

        // Split into chunks and add split markers
        let mut buffers = Vec::new();
        for chunk in bytes.chunks(20) {
            if !buffers.is_empty() {
                buffers.push(vec![0xFF]); // Split marker
            }
            buffers.push(chunk.to_vec());
        }

        let total_buffers = buffers.len();

        // Clear receive buffer
        {
            let mut buffer = self.buffer.lock().await;
            buffer.clear();
        }

        // Send the sequence and ensure cmd_sending is reset
        let send_result = self.send_buffer_sequence(buffers, total_buffers).await;
        let verify_result: Result<(), Box<dyn Error + Send + Sync>> = if send_result.is_ok() {
            debug!("Send completed successfully");

            // Wait for and verify response
            match self.verify_response("E4E5E6E7", 1000).await {
                Ok(_) => {
                    debug!("Command response verified successfully");
                    Ok(())
                }
                Err(e) => {
                    error!("Failed to verify command response: {}", e);
                    Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)) as Box<dyn Error + Send + Sync>)
                }
            }
        } else {
            send_result.map_err(|e| Box::<dyn Error + Send + Sync>::from(e))
        };

        // Always reset sending state
        self.cmd_sending = false;
        self.can_send = true; // Re-enable sending
        self.last_send_time = Some(Instant::now());

        // Return the result after cleanup
        // Convert Box<dyn Error + Send + Sync> to Box<dyn Error> for type compatibility
        verify_result.map_err(|e| e as Box<dyn Error>)
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


    #[allow(dead_code)]
    fn format_hex_bytes(&mut self,bytes: &[u8]) -> String {
        bytes.iter().enumerate()
        .map(|(i, &b)| {
            if i % 2 == 0 {
                if i > 0 { format!(", 0x{:02X}", b) }
                else { format!("0x{:02X}", b) }
            } else {
                format!("{:02X}", b)
            }
        })
        .collect()
    }
}

impl BlueController for WinBlueController {
    fn connect<'a>(&'a mut self) -> std::pin::Pin<Box<dyn Future<Output = Result<(), Box<dyn Error + Send + Sync>>> + Send + 'a>> {
        Box::pin(async move { 
            match self.connect().await {
                Ok(()) => Ok(()),
                Err(e) => Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())) as Box<dyn Error + Send + Sync>)
            }
        })
    }

    fn send<'a>(&'a mut self, command: &str) -> std::pin::Pin<Box<dyn Future<Output = Result<(), Box<dyn Error + Send + Sync>>> + Send + 'a>> {
        let command = command.to_owned();
        Box::pin(async move {
            // Log the outgoing command
            info!("Sending command - HEX: {} ASCII: {}", 
                command,
                command.chars().map(|c| if c.is_ascii_graphic() { c } else { '.' }).collect::<String>());
            
            // Pre-validate before any async operations
            if command.is_empty() {
                return Ok(());
            }

            // All error conditions are checked before any async operations or mutex locks
            if !command.starts_with("E0E1E2E3") {
                if !self.can_send {
                    tokio::time::sleep(Duration::from_millis(20)).await;
                    return Ok(());
                }
                return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, 
                    "Invalid command: must start with E0E1E2E3")) as Box<dyn Error + Send + Sync>);
            }

            if !self.is_connected() {
                error!("Attempted to send command while disconnected");
                return Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotConnected, "Not connected")) as Box<dyn Error + Send + Sync>);
            }

            if self.cmd_sending {
                error!("Attempted to send command while previous command is still in progress");
                return Err(Box::new(std::io::Error::new(std::io::ErrorKind::WouldBlock, 
                    "Previous command still in progress")) as Box<dyn Error + Send + Sync>);
            }
            
            info!("Device status check passed, proceeding with send...");

            self.cmd_sending = true;
            // Convert hex string to bytes
            let bytes: Vec<u8> = match hex::decode(&command) {
                Ok(b) => b,
                Err(e) => {
                    self.cmd_sending = false;
                    return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, 
                        format!("Invalid hex string: {}", e))) as Box<dyn Error + Send + Sync>);
                }
            };
            
            info!("Preparing to send command {} ({} bytes)", command, bytes.len());
            
            // Split into chunks and add split markers
            let mut buffers = Vec::new();
            for chunk in bytes.chunks(20) {
                if !buffers.is_empty() {
                    buffers.push(vec![0xFF]); // Split marker
                }
                buffers.push(chunk.to_vec());
            }

            let total_buffers = buffers.len();
            info!("Split command into {} buffers", total_buffers);

            let result = match self.send_buffer_sequence(buffers, total_buffers).await {
                Ok(_) => Ok(()),
                Err(e) => Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())) as Box<dyn Error + Send + Sync>)
            };
            self.cmd_sending = false;
            
            result
        })
    }
    
    fn is_connected(&self) -> bool {
        self.is_connected()
    }

    fn set_receiver_callback(&mut self, callback: Box<dyn Fn(String) + Send + Sync>) {
        // Use try_lock() instead of blocking_lock() to avoid blocking the async runtime
        if let Ok(mut cb) = self.ble_call_back.try_lock() {
            *cb = Some(callback);
        }
    }
    
    fn clear_receiver_callback(&mut self) {
        let mut cb = self.ble_call_back.blocking_lock();
        *cb = None;
    }

    fn disconnect<'a>(&'a mut self) -> std::pin::Pin<Box<dyn Future<Output = Result<(), Box<dyn Error + Send + Sync>>> + Send + 'a>> {
        Box::pin(async move { 
            match self.disconnect().await {
                Ok(()) => Ok(()),
                Err(e) => Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())) as Box<dyn Error + Send + Sync>)
            }
        })
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