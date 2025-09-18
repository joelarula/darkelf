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
use std::{pin::Pin, sync::{Arc, Mutex}};
use tokio::sync::Mutex as TokioMutex;
use std::error::Error;
use std::time::Duration;
use hex::decode;
use tokio::time::{Instant, sleep};
use log::{info, warn, error, debug};
use std::collections::VecDeque;
use std::future::Future;

pub const LASER_DEVICE_PREFIX: &str = "TD5322A";

pub const GENERIC_ACCESS_SERVICE_UUID: &str = "00001800-0000-1000-8000-00805F9B34FB";

pub const DEVICE_INFORMATION_SERVICE_UUID: &str = "0000180A-0000-1000-8000-00805F9B34FB";

pub const LASER_SERVICE_UUID: [&str; 2] = [
    "0000FF00-0000-1000-8000-00805F9B34FB",
    "0000FFE0-0000-1000-8000-00805F9B34FB1"
];

// UUIDs from JavaScript example
pub const WRITE_UUIDS: [&str; 2] = [
    "0000FFE2-0000-1000-8000-00805F9B34FB",
    "0000FF02-0000-1000-8000-00805F9B34FB"
];
pub const NOTIFY_UUIDS: [&str; 2] = [
    "0000FFE1-0000-1000-8000-00805F9B34FB",
    "0000FF01-0000-1000-8000-00805F9B34FB"
];


pub type Characteristic = GattCharacteristic;

pub type AsyncResult<T> = Pin<Box<dyn Future<Output = Result<T, Box<dyn Error + Send + Sync>>> + Send>>;

pub trait DeviceController: Send + Sync  {
    fn connect(&mut self) -> Result<(), Box<dyn Error>> ;

    fn disconnect(&mut self) -> Result<(), Box<dyn Error>> ;
    
    fn send(&mut self, cmd: &str) -> AsyncResult<()>;
    
    fn has_complete_message(&self) -> bool;

    fn take_complete_message(&mut self) -> Option<String>;
    
    fn set_receiver_callback(&mut self, callback: Box<dyn Fn(String) + Send + Sync>);
    
    fn clear_receiver_callback(&mut self);
    
    fn is_connected(&self) -> bool;

    
}


type ReceiverCallback = Box<dyn Fn(String) + Send + Sync>;



#[derive(Clone)]
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
    // Callback and fragment handling fields
    blu_rec_call_back: Arc<Mutex<Option<ReceiverCallback>>>,
    blu_rec_content: Arc<Mutex<VecDeque<String>>>,
    blu_rec_content_complete: Arc<Mutex<Option<String>>>,
    // Connection state management
    connect_count: u32,
    ready_to_receive: bool,
    can_send: bool,
    cmd_sending: bool,
    connection_state: i32, // -1=connecting, 0=disconnected, 1=connected, 2=ready
    min_send_interval: Duration,
    last_fragment_time: Option<Instant>,
    fragment_interval: Duration,
}

// Helper function to handle hex strings with 'Z' split markers
#[derive(Debug)]
enum BufferSegment {
    Data(Vec<u8>),
    Split,
}


impl WinBlueController {
    /// Splits a hex string into buffers, handling 'Z' markers like JavaScript implementation
    fn split_hex_string_to_buffers(&self, hex_string: &str, chunk_size: usize) -> Vec<BufferSegment> {
        let mut result = Vec::new();
        
        // Split on 'Z' first, like JavaScript hexStringToBufferSequence
        for (i, segment) in hex_string.split('Z').enumerate() {
            if !segment.is_empty() {
                // Add split marker between segments
                if i > 0 {
                    result.push(BufferSegment::Split);
                }
                
                // Convert hex string to bytes
                if let Ok(bytes) = hex::decode(segment) {
                    // Split into chunks like JavaScript splitHexStringToBuffers
                    let mut offset = 0;
                    let mut remaining = bytes.len();
                    
                    while remaining > 0 {
                        let size = remaining.min(chunk_size);
                        let chunk = bytes[offset..offset + size].to_vec();
                        result.push(BufferSegment::Data(chunk));
                        offset += size;
                        remaining -= size;
                    }
                }
            }
        }
        
        result
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
                if LASER_SERVICE_UUID.contains(&service_uuid_str.as_str()) {
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
                            WRITE_UUIDS.contains(&char_uuid_str.as_str()) {
                            info!("Write UUID: {:?} Found Laser Service write uuid", char_uuid);
                                self.write_char = Some(characteristic.clone());
                            }
                        if (props & GattCharacteristicProperties::Notify == GattCharacteristicProperties::Notify ||
                            props & GattCharacteristicProperties::Indicate == GattCharacteristicProperties::Indicate) &&
                            NOTIFY_UUIDS.contains(&char_uuid_str.as_str()) {
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
            // Initialize new fragment and callback fields
            blu_rec_call_back: Arc::new(Mutex::new(None)),
            blu_rec_content: Arc::new(Mutex::new(VecDeque::with_capacity(10))), // Keep last 10 fragments
            blu_rec_content_complete: Arc::new(Mutex::new(None)),
            // Connection state fields
            connect_count: 0,
            ready_to_receive: false,
            can_send: false,
            cmd_sending: false,
            connection_state: 0,
            min_send_interval: Duration::from_millis(100),
            last_fragment_time: None,
            fragment_interval: Duration::from_millis(50), // 50ms between fragments
        })
    }
    
    /// Process a received data fragment, handling message reassembly and callbacks
    fn process_received_data_fragment(&self, data_fragment: &str) -> Result<(), Box<dyn Error>> {
        debug!("Processing received data fragment: {}", data_fragment);
        
        // Get mutex locks for our buffers
        let mut content = self.blu_rec_content.lock().unwrap();
        let mut complete = self.blu_rec_content_complete.lock().unwrap();
        
        // If this is a new message (starts with E0E1E2E3), clear previous content
        if data_fragment.contains("E0E1E2E3") {
            content.clear();
            *complete = None;
        }
        
        // Add the new fragment to our collection
        content.push_back(data_fragment.to_string());
        
        // Maintain max 10 fragments
        while content.len() > 10 {
            content.pop_front();
        }
        
        // Join all fragments
        let full_content: String = content.iter().cloned().collect();
        debug!("Current accumulated content: {}", full_content);
        
        // Check if we have a complete message
        if let (Some(start_idx), Some(end_idx)) = (
            full_content.rfind("E0E1E2E3"),
            full_content.rfind("E4E5E6E7")
        ) {
            if end_idx > start_idx && end_idx == full_content.len() - 8 {
                // We have a complete message ending at the buffer end
                let complete_msg = full_content[start_idx..=end_idx+7].to_string();
                *complete = Some(complete_msg.clone());
                
                // Notify via callback if registered
                if let Some(callback) = self.blu_rec_call_back.lock().unwrap().as_ref() {
                    callback(complete_msg);
                }
                
                // Clear the fragment buffer after successful processing
                content.clear();
            } else if end_idx > start_idx {
                // We have a complete message but there's more data after
                // Keep only data after the last start marker
                let remaining = full_content[start_idx..].to_string();
                content.clear();
                content.push_back(remaining);
            }
        }
        
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
        // Process through fragment handler
        if let Err(e) = self.process_received_data_fragment(&content) {
            error!("Error processing fragment: {}", e);
        }
    }


    /// Internal implementation of send that handles low-level buffer manipulation
    pub async fn send_internal(&mut self, bytes: &[u8]) -> Result<(), String> {
        // Clear the buffer before sending
        {
            let mut buffer = self.buffer.lock().unwrap();
            buffer.clear();
        }

        // Convert bytes to hex string for processing
        let hex_string = hex::encode_upper(bytes);
        debug!("WinBlueController::send_internal called with hex string: {}", hex_string);
        
        // Process as hex string to handle potential 'Z' markers
        let segments = self.split_hex_string_to_buffers(&hex_string, 20);
        
        for segment in segments {
            match segment {
                BufferSegment::Data(data) => {
                    // Only validate E0E1E2E3 for 20-byte buffers
                    if data.len() == 20 && !data.starts_with(&[0xE0, 0xE1, 0xE2, 0xE3]) {
                        return Err("Invalid command: 20-byte buffer must start with E0E1E2E3".to_string());
                    }
                    
                    // Enforce minimum delay between commands
                    if let Some(last_time) = self.last_send_time {
                        let elapsed = last_time.elapsed();
                        if elapsed < Duration::from_millis(50) {
                            sleep(Duration::from_millis(50) - elapsed).await;
                        }
                    }
                    
                    // Send the data
                    if let Some(write_char) = &self.write_char {
                        let writer = DataWriter::new()
                            .map_err(|e| format!("Failed to create DataWriter: {:?}", e))?;
                        writer.WriteBytes(&data)
                            .map_err(|e| format!("Failed to write bytes: {:?}", e))?;
                        let buffer = writer.DetachBuffer()
                            .map_err(|e| format!("Failed to detach buffer: {:?}", e))?;

                        write_char.WriteValueWithOptionAsync(&buffer, GattWriteOption::WriteWithoutResponse)
                            .map_err(|e| format!("Failed to write characteristic: {:?}", e))?
                            .get()
                            .map_err(|e| format!("Failed to complete write: {:?}", e))?;
                    } else {
                        return Err("Write characteristic not found".to_string());
                    }
                }
                BufferSegment::Split => {
                    // Add delay for split marker
                    sleep(Duration::from_millis(100)).await;
                }
            }
        }

        // Update last send time
        self.last_send_time = Some(Instant::now());
        Ok(())
    }


    fn send(&mut self, cmd: &str) -> AsyncResult<()> {
        // Validate command format
        if !self.can_send {
            return Box::pin(async move { Err("Device not ready to send".into()) });
        }
        if cmd.len() == 0 {
            return Box::pin(async move { Err("Empty command".into()) });
        }
        if !cmd.starts_with("E0E1E2E3") {
            return Box::pin(async move { Err("Invalid command: must start with E0E1E2E3".into()) });
        }

        // Clone necessary state
        let write_char = self.write_char.clone();
        let sending = self.sending.clone();
        
        // Convert hex string to bytes
        let bytes = match hex::decode(cmd) {
            Ok(b) => {
                // Log bytes in JavaScript format
                debug!("{}", format_hex_bytes(&b));
                Arc::new(b)
            },
            Err(e) => return Box::pin(async move { Err(format!("Invalid hex string: {}", e).into()) })
        };

        Box::pin(async move {
            let _guard = sending.lock().await;
            
            if let Some(write_char) = write_char {
                let writer = DataWriter::new()
                    .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;
                
                writer.WriteBytes(&bytes)
                    .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;
                
                let buffer = writer.DetachBuffer()
                    .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;
                
                let result = write_char.WriteValueWithOptionAsync(
                    &buffer,
                    GattWriteOption::WriteWithoutResponse,
                )
                .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?
                .get()
                .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;

                match result {
                    GattCommunicationStatus::Success => {
                        debug!("Data sent successfully: {:?}", bytes);
                        Ok(())
                    }
                    status => {
                        error!("Failed to write data: {:?}", status);
                        Err(format!("Failed to write data: {:?}", status).into())
                    }
                }
            } else {
                Err("Write characteristic not found".into())
            }
        })
    }
    
    fn has_complete_message(&self) -> bool {
        self.blu_rec_content_complete.lock().unwrap().is_some()
    }
    
    fn take_complete_message(&mut self) -> Option<String> {
        self.blu_rec_content_complete.lock().unwrap().take()
    }
    
    fn set_receiver_callback(&mut self, callback: Box<dyn Fn(String) + Send + Sync>) {
        let mut cb = self.blu_rec_call_back.lock().unwrap();
        *cb = Some(callback);
    }
    
    fn clear_receiver_callback(&mut self) {
        let mut cb = self.blu_rec_call_back.lock().unwrap();
        *cb = None;
    }
    
    fn is_connected(&self) -> bool {
        self.connected
    }

    
    /// Core implementation of the send functionality that handles both string and byte input
    async fn send_impl(&mut self, bytes: &[u8]) -> Result<(), Box<dyn Error + Send + Sync>> {
        // Check connection and readiness
        if !self.is_connected() {
            return Err("Not connected".into());
        }
        if !self.can_send {
            return Err("Device not ready to send".into());
        }

        // Clear the buffer before sending
        {
            let mut buffer = self.buffer.lock().unwrap();
            buffer.clear();
        }

        // Convert to hex for processing
        let hex_string = hex::encode_upper(bytes);
        debug!("WinBlueController::send called with hex string: {}", hex_string);
        
        // Process as hex string to handle potential 'Z' markers
        let segments = self.split_hex_string_to_buffers(&hex_string, 20);
        
        // Acquire send lock and set state
        let _guard = self.sending.lock().await;
        self.cmd_sending = true;
        
        for segment in segments {
            match segment {
                BufferSegment::Data(chunk) => {
                    // Only validate E0E1E2E3 for 20-byte buffers
                    if chunk.len() == 20 && !chunk.starts_with(&[0xE0, 0xE1, 0xE2, 0xE3]) {
                        self.cmd_sending = false;
                        return Err("Invalid command: 20-byte buffer must start with E0E1E2E3".into());
                    }
                    
                    // Enforce minimum delay between commands
                    if let Some(last_time) = self.last_send_time {
                        let elapsed = last_time.elapsed();
                        if elapsed < self.min_send_interval {
                            sleep(self.min_send_interval - elapsed).await;
                        }
                    }
                    
                    // Send the chunk
                    let write_char = self.write_char.as_ref()
                        .ok_or_else(|| -> Box<dyn Error + Send + Sync> { "Write characteristic not found".into() })?;
                        
                    let writer = DataWriter::new()
                        .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;
                    writer.WriteBytes(&chunk)
                        .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;
                    let buffer = writer.DetachBuffer()
                        .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;

                    let result = write_char.WriteValueWithOptionAsync(&buffer, GattWriteOption::WriteWithoutResponse)
                        .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?
                        .get()
                        .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;

                    match result {
                        GattCommunicationStatus::Success => {
                            debug!("Chunk sent successfully: {:?}", chunk);
                        }
                        status => {
                            self.cmd_sending = false;
                            error!("Failed to write chunk: {:?}", status);
                            return Err(format!("Failed to write chunk: {:?}", status).into());
                        }
                    }
                }
                BufferSegment::Split => {
                    sleep(self.fragment_interval).await;
                }
            }
        }

        // Update state after successful send
        self.last_send_time = Some(Instant::now());
        self.cmd_sending = false;
        
        Ok(())
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

impl DeviceController for WinBlueController {
    fn connect(&mut self) -> Result<(), Box<dyn Error>> {
        match futures::executor::block_on(self.connect()) {
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        }
    }

    fn send(&mut self, cmd: &str) -> AsyncResult<()> {
        // Clone self since we need to move it into the async block
        let this = self.clone();
        
        // Validate command format
        if cmd.is_empty() {
            return Box::pin(async move { Err("Empty command".into()) });
        }
        if !cmd.starts_with("E0E1E2E3") {
            return Box::pin(async move { Err("Invalid command: must start with E0E1E2E3".into()) });
        }

        let bytes = match hex::decode(cmd) {
            Ok(b) => b,
            Err(e) => return Box::pin(async move { Err(format!("Invalid hex string: {}", e).into()) })
        };

        // Use internal send implementation
        Box::pin(async move { 
            this.send_impl(&bytes).await 
        })
    }

    fn has_complete_message(&self) -> bool {
        self.blu_rec_content_complete.lock().unwrap().is_some()
    }

    fn take_complete_message(&mut self) -> Option<String> {
        self.blu_rec_content_complete.lock().unwrap().take()
    }

    fn set_receiver_callback(&mut self, callback: Box<dyn Fn(String) + Send + Sync>) {
        *self.blu_rec_call_back.lock().unwrap() = Some(callback);
    }

    fn clear_receiver_callback(&mut self) {
        *self.blu_rec_call_back.lock().unwrap() = None;
    }

    fn is_connected(&self) -> bool {
        self.connected
    }
    
    fn disconnect(&mut self) -> Result<(), Box<dyn Error>> {
        match futures::executor::block_on(self.disconnect()) {
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        }
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
        if !device_name_str.starts_with(LASER_DEVICE_PREFIX) {
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
            if LASER_SERVICE_UUID.contains(&str.as_str()) {
                info!("Found laser service: ({:?})", service_uuid);
                device_list.push(device_info.clone());
            }
        }
    }
    Ok(device_list)
}


/// Format bytes in the same way as JavaScript's logHexBytes function
fn format_hex_bytes(bytes: &[u8]) -> String {
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