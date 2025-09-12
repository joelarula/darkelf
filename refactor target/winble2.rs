use windows::{
    core::{HSTRING, Result as WindowsResult, GUID},
    Devices::Bluetooth::BluetoothLEDevice,
    Devices::Enumeration::DeviceInformation,
    Devices::Bluetooth::GenericAttributeProfile::{
        GattDeviceService, GattCharacteristic, GattCharacteristicProperties,
        GattCommunicationStatus, GattWriteOption,
        GattClientCharacteristicConfigurationDescriptorValue
    },
    Storage::Streams::{DataWriter, DataReader},
    Foundation::{TypedEventHandler, EventRegistrationToken},
};
use std::sync::{Arc, Mutex};
use tokio::sync::Mutex as TokioMutex;
use std::error::Error;
use std::time::Duration;
use tokio::time::{Instant, sleep};
use log::{info, warn, error, debug};
use serialport::{SerialPort, SerialPortType};

// Assuming these are defined in the `blue` module
pub mod blue {
    pub const LASER_DEVICE_PREFIX: &str = "Laser"; // Adjust based on actual device name
    pub const LASER_SERVICE_UUID: &[&str] = &[
        "0000FF00-0000-1000-8000-00805F9B34FB",
        "0000FFE0-0000-1000-8000-00805F9B34FB",
    ];
    pub const WRITE_UUIDS: &[&str] = &[
        "0000FF02-0000-1000-8000-00805F9B34FB",
        "0000FFE1-0000-1000-8000-00805F9B34FB",
    ];
    pub const NOTIFY_UUIDS: &[&str] = &[
        "0000FF01-0000-1000-8000-00805F9B34FB",
        "0000FFE1-0000-1000-8000-00805F9B34FB",
    ];
}

pub trait BlueController {
    fn connect<'a>(&'a mut self) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn Error>>> + Send + 'a>>;
    fn send<'a>(&'a mut self, bytes: &'a [u8]) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send + 'a>>;
    fn get_content(&self) -> String;
    fn is_connected(&self) -> bool;
}

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
    notification_token: Option<EventRegistrationToken>,
    dmx_port: Option<Box<dyn SerialPort>>,
    muuid_sel: u8, // 0 or 1, based on 0b2a module
}

impl WinBlueController {
    pub async fn new(device_info: Option<&DeviceInformation>, dmx_port_path: &str, muuid_sel: u8) -> Result<Self, Box<dyn Error>> {
        debug!("WinBlueController::new called with muuid_sel: {}", muuid_sel);
        let dmx_port = serialport::new(dmx_port_path, 250_000)
            .data_bits(serialport::DataBits::Eight)
            .parity(serialport::Parity::None)
            .stop_bits(serialport::StopBits::Two)
            .open()?;
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
            dmx_port: Some(dmx_port),
            muuid_sel,
        })
    }

    pub async fn connect(&mut self) -> Result<(), Box<dyn Error>> {
        debug!("WinBlueController::connect called");
        if let Some(device_info) = self.device_info.as_ref() {
            let device_id = device_info.Id()?;
            debug!("Connecting to BLE device with id: {}", device_id);
            self.device = Some(BluetoothLEDevice::FromIdAsync(&device_id)?.get()?);
            self.discover_characteristics().await?;
            self.connected = true;
            debug!("Device connected");
            // Send initialization command (if required)
            self.send_init_command().await?;
        }
        Ok(())
    }

    async fn send_init_command(&mut self) -> Result<(), Box<dyn Error>> {
        // Placeholder: Add actual initialization command from Vue code or Wireshark
        let init_command = vec![
            0xE0, 0xE1, 0xE2, 0xE3,
            0x00, 0x00, 0x00, 0x00, // Initialize command type
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ];
        self.send(&init_command).await.map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)))?;
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
        let dmx_port_clone = self.dmx_port.as_mut().map(|port| Box::new(port.try_clone().unwrap()) as Box<dyn SerialPort>);

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

                        // Map to DMX
                        if let Some(ref mut dmx_port) = dmx_port_clone.as_ref() {
                            let mut dmx_frame = [0u8; 512];
                            if value.len() >= 16 && value.starts_with(&[0xE0, 0xE1, 0xE2, 0xE3]) {
                                dmx_frame[0] = value[12]; // Red (channel 1)
                                dmx_frame[1] = value[13]; // Green (channel 2)
                                dmx_frame[2] = value[14]; // Blue (channel 3)
                                dmx_frame[3] = value[8]; // x low byte (channel 4)
                                dmx_frame[4] = value[9]; // x high byte (channel 5)
                                dmx_frame[5] = value[10]; // y low byte (channel 6)
                                dmx_frame[6] = value[11]; // y high byte (channel 7)
                                dmx_frame[7] = value[15]; // z (laser on/off, channel 8)
                            }
                            dmx_port.write(&[0x00]).unwrap_or_else(|e| error!("DMX break error: {}", e));
                            dmx_port.write(&dmx_frame).unwrap_or_else(|e| error!("DMX write error: {}", e));
                        }
                    }
                }
            }
            Ok(())
        });

        match characteristic.WriteClientCharacteristicConfigurationDescriptorAsync(
            GattClientCharacteristicConfigurationDescriptorValue::Notify
        )?.get() {
            Ok(GattCommunicationStatus::Success) => {
                debug!("Successfully enabled notifications");
                let token = characteristic.ValueChanged(&handler)?;
                self.notification_token = Some(token);
                Ok(())
            },
            Ok(status) => Err(format!("Failed to enable notifications: {:?}", status).into()),
            Err(e) => Err(e.into()),
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

        let _guard = self.sending.lock().await;
        if !self.is_connected() {
            return Err("Not connected".to_string());
        }

        if let Some(last_send) = self.last_send_time {
            let elapsed = Instant::now().duration_since(last_send);
            if elapsed < Duration::from_millis(20) {
                sleep(Duration::from_millis(20) - elapsed).await;
            }
        }
        self.last_send_time = Some(Instant::now());
        self.send_data(bytes).await
    }

    async fn send_data(&self, bytes: &[u8]) -> Result<(), String> {
        if let Some(write_char) = &self.write_char {
            let hex_str = bytes.iter().map(|b| format!("{:02X}", b)).collect::<Vec<_>>().join("");
            debug!("Writing command to BLE: {}", hex_str);

            let writer = DataWriter::new().map_err(|e| e.to_string())?;
            writer.WriteBytes(bytes).map_err(|e| e.to_string())?;
            let buffer = writer.DetachBuffer().map_err(|e| e.to_string())?;

            let write_result = write_char.WriteValueAsync(&buffer)?.get();
            match write_result {
                Ok(GattCommunicationStatus::Success) => {
                    debug!("Write to BLE characteristic succeeded");
                    // Send to DMX
                    if let Some(ref mut dmx_port) = self.dmx_port.as_ref() {
                        let mut dmx_frame = [0u8; 512];
                        dmx_frame[0] = bytes[12]; // Red (channel 1)
                        dmx_frame[1] = bytes[13]; // Green (channel 2)
                        dmx_frame[2] = bytes[14]; // Blue (channel 3)
                        dmx_frame[3] = bytes[8]; // x low byte (channel 4)
                        dmx_frame[4] = bytes[9]; // x high byte (channel 5)
                        dmx_frame[5] = bytes[10]; // y low byte (channel 6)
                        dmx_frame[6] = bytes[11]; // y high byte (channel 7)
                        dmx_frame[7] = bytes[15]; // z (laser on/off, channel 8)
                        dmx_port.write(&[0x00]).map_err(|e| e.to_string())?;
                        dmx_port.write(&dmx_frame).map_err(|e| e.to_string())?;
                    }
                    Ok(())
                },
                Ok(status) => Err(format!("Write failed with status: {:?}", status)),
                Err(e) => Err(format!("Write failed with error: {:?}", e)),
            }
        } else {
            Err("Write characteristic not found".to_string())
        }
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
        if let (Some(characteristic), Some(token)) = (&self.notify_char, self.notification_token) {
            characteristic.RemoveValueChanged(token)?;
        }
        self.device = None;
        self.write_char = None;
        self.notify_char = None;
        self.connected = false;
        self.notification_token = None;
        self.dmx_port = None;
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