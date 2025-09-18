use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::sync::Mutex as TokioMutex;
use windows::{
    Devices::Bluetooth::{BluetoothLEDevice, GenericAttributeProfile::*},
    Devices::Enumeration::DeviceInformation,
    Foundation::TypedEventHandler,
    Storage::Streams::{DataReader, DataWriter},
};
use log::{debug, error};

// Wrapper type for notification token to avoid direct EventRegistrationToken dependency
#[derive(Clone, Copy)]
struct NotificationToken(i64);

#[derive(Debug, thiserror::Error)]
pub enum DeviceError {
    #[error("Device not connected")]
    NotConnected,
    #[error("Windows API error: {0}")]
    WindowsError(String),
    #[error("Invalid command format: {0}")]
    InvalidCommand(String),
    #[error("Communication error: {0}")]
    CommunicationError(String),
    #[error("Timeout error")]
    Timeout,
}

pub struct PendingCommand {
    command_id: String,
    response_sender: tokio::sync::oneshot::Sender<Result<String, DeviceError>>,
    timeout: Duration,
    sent_at: Instant,
}

#[derive(Debug)]
pub struct FragmentBuffer {
    buffer: String,
    last_update: Instant,
    timeout: Duration,
}

impl FragmentBuffer {
    fn new(timeout: Duration) -> Self {
        Self {
            buffer: String::new(),
            last_update: Instant::now(),
            timeout,
        }
    }

    fn add_fragment(&mut self, fragment: &str) -> bool {
        self.buffer.push_str(fragment);
        self.last_update = Instant::now();
        self.is_complete()
    }

    fn is_complete(&self) -> bool {
        self.buffer.starts_with("E0E1E2E3") && self.buffer.ends_with("E4E5E6E7")
    }

    fn is_timed_out(&self) -> bool {
        self.last_update.elapsed() > self.timeout
    }

    fn take_message(&mut self) -> Option<String> {
        if self.is_complete() {
            let message = self.buffer.clone();
            self.buffer.clear();
            Some(message)
        } else {
            None
        }
    }

    fn clear(&mut self) {
        self.buffer.clear();
    }
}

pub struct WinDeviceController {
    device_info: Option<DeviceInformation>,
    device: Option<BluetoothLEDevice>,
    write_char: Option<GattCharacteristic>,
    notify_char: Option<GattCharacteristic>,
    service_uuid: Option<windows::core::GUID>,
    connected: bool,
    send_buffer: Arc<Mutex<VecDeque<Vec<u8>>>>,
    receive_buffer: Arc<Mutex<FragmentBuffer>>,
    sending: Arc<TokioMutex<()>>,
    pending_commands: Arc<Mutex<VecDeque<PendingCommand>>>,
    notification_token: Option<NotificationToken>,
    min_send_interval: Duration,
    command_timeout: Duration,
}

impl WinDeviceController {
    pub async fn new(device_info: Option<&DeviceInformation>) -> Result<Self, DeviceError> {
        Ok(Self {
            device_info: device_info.cloned(),
            device: None,
            write_char: None,
            notify_char: None,
            service_uuid: None,
            connected: false,
            send_buffer: Arc::new(Mutex::new(VecDeque::new())),
            receive_buffer: Arc::new(Mutex::new(FragmentBuffer::new(Duration::from_secs(5)))),
            sending: Arc::new(TokioMutex::new(())),
            pending_commands: Arc::new(Mutex::new(VecDeque::new())),
            notification_token: None,
            min_send_interval: Duration::from_millis(100),
            command_timeout: Duration::from_secs(5),
        })
    }

    pub async fn connect(&mut self) -> Result<(), DeviceError> {
        if let Some(device_info) = self.device_info.as_ref() {
            let device_id = device_info.Id()
                .map_err(|e| DeviceError::WindowsError(e.to_string()))?;
            
            self.device = Some(BluetoothLEDevice::FromIdAsync(&device_id)
                .map_err(|e| DeviceError::WindowsError(e.to_string()))?
                .get()
                .map_err(|e| DeviceError::WindowsError(e.to_string()))?);
            
            self.discover_characteristics().await?;
            self.connected = true;
            
            Ok(())
        } else {
            Err(DeviceError::NotConnected)
        }
    }

    async fn discover_characteristics(&mut self) -> Result<(), DeviceError> {
        if let Some(device) = &self.device {
            let services_result = device.GetGattServicesAsync()
                .map_err(|e| DeviceError::WindowsError(e.to_string()))?
                .get()
                .map_err(|e| DeviceError::WindowsError(e.to_string()))?;

            for i in 0..services_result.Services()?.Size()? {
                let service = services_result.Services()?.GetAt(i)?;
                let service_uuid = service.Uuid()?;
                // Check service UUID against your known service UUID
                self.setup_service_characteristics(&service).await?;
            }

            if self.write_char.is_none() || self.notify_char.is_none() {
                return Err(DeviceError::CommunicationError("Required characteristics not found".into()));
            }

            self.setup_notifications().await?;
            Ok(())
        } else {
            Err(DeviceError::NotConnected)
        }
    }

    async fn setup_service_characteristics(&mut self, service: &GattDeviceService) -> Result<(), DeviceError> {
        let characteristics = service.GetCharacteristicsAsync()
            .map_err(|e| DeviceError::WindowsError(e.to_string()))?
            .get()
            .map_err(|e| DeviceError::WindowsError(e.to_string()))?;

        for i in 0..characteristics.Characteristics()?.Size()? {
            let characteristic = characteristics.Characteristics()?.GetAt(i)?;
            let props = characteristic.CharacteristicProperties()?;
            
            // Setup write characteristic
            if props & GattCharacteristicProperties::Write == GattCharacteristicProperties::Write ||
               props & GattCharacteristicProperties::WriteWithoutResponse == GattCharacteristicProperties::WriteWithoutResponse {
                self.write_char = Some(characteristic.clone());
            }
            
            // Setup notify characteristic
            if props & GattCharacteristicProperties::Notify == GattCharacteristicProperties::Notify ||
               props & GattCharacteristicProperties::Indicate == GattCharacteristicProperties::Indicate {
                self.notify_char = Some(characteristic.clone());
            }
        }
        
        Ok(())
    }

    async fn setup_notifications(&mut self) -> Result<(), DeviceError> {
        if let Some(notify_char) = &self.notify_char {
            let receive_buffer = self.receive_buffer.clone();
            let pending_commands = self.pending_commands.clone();

            let handler = TypedEventHandler::new(move |_sender, args| {
                if let Some(args) = args {
                    if let Ok(buffer) = args.CharacteristicValue() {
                        if let Ok(reader) = DataReader::FromBuffer(&buffer) {
                            let len = buffer.Length().unwrap_or(0) as usize;
                            let mut data = vec![0u8; len];
                            if reader.ReadBytes(&mut data).is_ok() {
                                let hex = data.iter()
                                    .map(|b| format!("{:02X}", b))
                                    .collect::<String>();
                                
                                let mut recv_buffer = receive_buffer.lock().unwrap();
                                if recv_buffer.add_fragment(&hex) {
                                    if let Some(message) = recv_buffer.take_message() {
                                        // Process complete message
                                        let mut commands = pending_commands.lock().unwrap();
                                        if let Some(cmd) = commands.pop_front() {
                                            let _ = cmd.response_sender.send(Ok(message));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                Ok(())
            });

            notify_char.WriteClientCharacteristicConfigurationDescriptorAsync(
                GattClientCharacteristicConfigurationDescriptorValue::Notify
            )
            .map_err(|e| DeviceError::WindowsError(e.to_string()))?
            .get()
            .map_err(|e| DeviceError::WindowsError(e.to_string()))?;

            let token = notify_char.ValueChanged(&handler)
                .map_err(|e| DeviceError::WindowsError(e.to_string()))?;
            
            self.notification_token = Some(NotificationToken(unsafe { std::mem::transmute(token) }));
        }
        Ok(())
    }

    pub async fn send_command(&mut self, data: &[u8]) -> Result<String, DeviceError> {
        if !self.connected {
            return Err(DeviceError::NotConnected);
        }

        let (tx, rx) = tokio::sync::oneshot::channel();
        let command_id = hex::encode(&data[..4]);

        {
            let mut commands = self.pending_commands.lock().unwrap();
            commands.push_back(PendingCommand {
                command_id,
                response_sender: tx,
                timeout: self.command_timeout,
                sent_at: Instant::now(),
            });
        }

        // Split data into 20-byte chunks and queue them
        {
            let mut send_buffer = self.send_buffer.lock().unwrap();
            for chunk in data.chunks(20) {
                send_buffer.push_back(chunk.to_vec());
            }
        }

        // Send all fragments
        self.process_send_queue().await?;

        // Wait for response with timeout
        tokio::time::timeout(self.command_timeout, rx)
            .await
            .map_err(|_| DeviceError::Timeout)?
            .map_err(|_| DeviceError::CommunicationError("Response channel closed".into()))?
    }

    async fn process_send_queue(&mut self) -> Result<(), DeviceError> {
        let _lock = self.sending.lock().await;
        
        while let Some(chunk) = self.send_buffer.lock().unwrap().pop_front() {
            self.send_chunk(&chunk).await?;
            tokio::time::sleep(self.min_send_interval).await;
        }
        
        Ok(())
    }

    async fn send_chunk(&self, chunk: &[u8]) -> Result<(), DeviceError> {
        if let Some(write_char) = &self.write_char {
            let writer = DataWriter::new()
                .map_err(|e| DeviceError::WindowsError(e.to_string()))?;
            
            writer.WriteBytes(chunk)
                .map_err(|e| DeviceError::WindowsError(e.to_string()))?;
            
            let buffer = writer.DetachBuffer()
                .map_err(|e| DeviceError::WindowsError(e.to_string()))?;
            
            write_char.WriteValueWithOptionAsync(&buffer, GattWriteOption::WriteWithoutResponse)
                .map_err(|e| DeviceError::WindowsError(e.to_string()))?
                .get()
                .map_err(|e| DeviceError::WindowsError(e.to_string()))?;
            
            Ok(())
        } else {
            Err(DeviceError::NotConnected)
        }
    }

    pub async fn cleanup_timeouts(&mut self) {
        let now = Instant::now();
        let mut commands = self.pending_commands.lock().unwrap();
        commands.retain(|cmd| {
            if now.duration_since(cmd.sent_at) > cmd.timeout {
                let _ = cmd.response_sender.send(Err(DeviceError::Timeout));
                false
            } else {
                true
            }
        });
    }

    pub fn is_connected(&self) -> bool {
        self.connected
    }
}

impl Drop for WinDeviceController {
    fn drop(&mut self) {
        if let (Some(characteristic), Some(token)) = (&self.notify_char, self.notification_token) {
            let _ = characteristic.RemoveValueChanged(unsafe { std::mem::transmute(token.0) });
        }
    }
}