/*
use std::sync::{Arc, Mutex};
use std::time::{Instant, Duration};
use hex::decode;
use serialport::{available_ports, SerialPort, new};
use windows::core::HSTRING;
use windows::Devices::Bluetooth::GenericAttributeProfile::{
    GattCharacteristic, GattClientCharacteristicConfigurationDescriptorValue,
    GattCommunicationStatus,
};
use windows::Devices::Bluetooth::{
    BluetoothLEDevice,
};
use windows::Devices::Enumeration::DeviceInformation;
use windows::Foundation::{TypedEventHandler, EventRegistrationToken};
// use windows::Storage::Streams::{Buffer, IBuffer};
use windows::Win32::Storage::FileSystem::{IBuffer};
use futures::StreamExt;
use tokio; // For async handling
use std::error::Error;
use std::collections::VecDeque; // For buffer
use regex::Regex;



#[derive(Debug, Clone)]
pub struct LaserOptions {
    tx_color: u8,
    tx_size: u8,
    run_speed: u8,
    tx_dist: u8,
    rd_mode: u8,
    sound_val: u8,
}

#[derive(Debug, Clone)]
pub struct PrjItem {
    py_mode: u8,
    prj_selected: [u16; 4],
}

#[derive(Debug, Clone)]
pub struct ProjectData {
    pub public: PublicSettings,
    pub prj_item: Vec<PrjItem>,
}

#[derive(Debug, Clone)]
pub struct PublicSettings {
    rd_mode: u8,
    sound_val: u8,
}

#[derive(Debug, Clone)]
pub struct DeviceTesting {
    device_id: String,
    service_id: String,
    read_uuid: String,
    write_uuid: String,
    test_status: String,
    test_result: String,
    test_msg: String,
    can_send: bool,
    conn: bool,
}

#[derive(Clone)]
pub struct LaserController {
    sending: Arc<Mutex<bool>>,
    port: Option<Box<dyn SerialPort>>, // For DMX/Serial
    ble_device: Option<BluetoothLEDevice>, // For Windows BLE
    write_char: Option<GattCharacteristic>,
    notify_char: Option<GattCharacteristic>,
    is_ble: bool,
    last_send_time: Option<Instant>,
    options: LaserOptions,
    project_data: ProjectData,
    connected: bool,
    mock_mode: bool,
    notification_token: Option<EventRegistrationToken>,
    blu_rec_content: VecDeque<String>, // Buffer for accumulated hex data
    device_testing: Option<DeviceTesting>,
    rec_device_msg_timer: Option<tokio::time::Instant>,
    device_tested: Vec<DeviceTesting>,
    discovery_started: bool,
    testing_idx: usize,
    not_pass: i32,
    pass_count: i32,
}

// Manual Clone implementation to handle non-cloneable fields
impl Clone for LaserController {
    fn clone(&self) -> Self {
        LaserController {
            sending: Arc::clone(&self.sending),
            port: None, // SerialPort is not Clone, so set to None or handle as needed
            ble_device: None, // BluetoothLEDevice is not Clone, so set to None or handle as needed
            write_char: None, // GattCharacteristic is not Clone, so set to None or handle as needed
            notify_char: None, // GattCharacteristic is not Clone, so set to None or handle as needed
            is_ble: self.is_ble,
            last_send_time: self.last_send_time,
            options: self.options.clone(),
            project_data: ProjectData {
                public: PublicSettings {
                    rd_mode: self.project_data.public.rd_mode,
                    sound_val: self.project_data.public.sound_val,
                },
                prj_item: self.project_data.prj_item.clone(),
            },
            connected: self.connected,
            mock_mode: self.mock_mode,
            notification_token: None, // EventRegistrationToken is not Clone
            blu_rec_content: self.blu_rec_content.clone(),
            device_testing: self.device_testing.clone(),
            rec_device_msg_timer: self.rec_device_msg_timer,
            device_tested: self.device_tested.clone(),
            discovery_started: self.discovery_started,
            testing_idx: self.testing_idx,
            not_pass: self.not_pass,
            pass_count: self.pass_count,
        }
    }
}

impl LaserController {
    pub async fn new(is_ble: bool, mock_mode: bool) -> Result<Self, Box<dyn Error>> {
        let sending = Arc::new(Mutex::new(false));
        let options = LaserOptions {
            tx_color: 0,
            tx_size: 60,
            run_speed: 128,
            tx_dist: 60,
            rd_mode: 0,
            sound_val: 0,
        };
        let project_data = ProjectData {
            public: PublicSettings { rd_mode: 0, sound_val: 0 },
            prj_item: Vec::new(),
        };

        let mut controller = LaserController {
            sending,
            port: None,
            ble_device: None,
            write_char: None,
            notify_char: None,
            is_ble,
            last_send_time: None,
            options,
            project_data,
            connected: false,
            mock_mode,
            notification_token: None,
            blu_rec_content: VecDeque::new(),
            device_testing: None,
            rec_device_msg_timer: None,
            device_tested: Vec::new(),
            discovery_started: false,
            testing_idx: 0,
            not_pass: 0,
            pass_count: 0,
        };

        if is_ble {
            controller.connect_ble().await?;
        } else {
            let ports = available_ports()?;
            if let Some(port_info) = ports.into_iter().next() {
                controller.port = Some(new(&port_info.port_name, 250_000).open()?); // DMX512 baud rate
            }
        }
        controller.connected = controller.is_connected();
        Ok(controller)
    }

    async fn connect_ble(&mut self) -> Result<(), Box<dyn Error>> {
        let selector = BluetoothLEDevice::GetDeviceSelector()?;
       let devices = windows::Devices::Enumeration::DeviceInformation::FindAllAsyncAqsFilter(&selector)?.get()?;

        if let Some(device_info) = devices.First()? {
            let ble_device = BluetoothLEDevice::FromIdAsync(&device_info.Id()?)?.await?;
            self.ble_device = Some(ble_device);
            self.discover_characteristics().await?;
            self.connected = true;
        } else {
            return Err("No connected BLE devices found".into());
        }
        Ok(())
    }

    async fn discover_characteristics(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(ble_device) = &self.ble_device {
            let services_result = ble_device.GetGattServicesAsync()?.await?;
            let services = services_result.Services()?;
            for i in 0..services.Size()? {
                let service = services.GetAt(i)?;
                let characteristics = service.GetCharacteristicsAsync()?.await?;
                let chars = characteristics.Characteristics()?;
                for j in 0..chars.Size()? {
                    let char = chars.GetAt(j)?;
                    let uuid = char.Uuid()?.to_string()?;
                    if uuid == "0000ffe2-0000-1000-8000-00805f9b34fb" || uuid == "0000ff02-0000-1000-8000-00805f9b34fb" {
                        self.write_char = Some(char);
                    } else if uuid == "0000ffe1-0000-1000-8000-00805f9b34fb" || uuid == "0000ff01-0000-1000-8000-00805f9b34fb" {
                        self.notify_char = Some(char.clone());
                        let config = GattClientCharacteristicConfigurationDescriptorValue::Notify;
                        char.WriteClientCharacteristicConfigurationDescriptorAsync(&config)?.await?;
                        let controller = self.clone();
                        let handler = TypedEventHandler::new(move |_, args| {
                            if let Ok(value) = args.Value() {
                                let data = value.Value()?.to_vec();
                                let hex = data.iter().map(|b| format!("{:02x}", b)).collect::<String>().to_uppercase();
                                println!("Notification: {}", hex);
                                controller.add_content(hex);
                            }
                            Ok(())
                        });
                        let token = char.ValueChanged(&handler)?;
                        self.notification_token = Some(token);
                    }
                }
            }
        }
        Ok(())
    }

    pub async fn send(&mut self, cmd_hex: &str, show_loading: bool, callback: Option<&mut dyn FnMut(i8, u8)>) -> Result<(), String> {
        if cmd_hex.is_empty() { return Err("Empty command".to_string()); }

        let mut guard = self.sending.lock().unwrap();
        if *guard { return Err("Previous send in progress".to_string()); }
        *guard = true;

        if !self.is_connected() { return Err("Not connected".to_string()); }

        if let Some(cb) = callback { cb(0, 0); }
        if show_loading && !self.mock_mode {
            self.last_send_time = Some(Instant::now());
        }

        let bytes = decode(cmd_hex).map_err(|e| e.to_string())?;
        if bytes.is_empty() { *guard = false; return Err("Invalid hex".to_string()); }

        let result = if self.mock_mode {
            tokio::time::sleep(Duration::from_millis(20)).await;
            Ok(())
        } else if self.is_ble {
            self.send_ble(&bytes).await
        } else {
            drop(guard); // Drop the lock before mutable borrow
            self.send_serial(&bytes)
        };

        // Re-lock if needed after send_serial
        let mut guard = self.sending.lock().unwrap();
        *guard = false;
        if let Some(cb) = callback {
            if result.is_ok() { cb(1, 100); } else { cb(-1, 0); }
        }
        result
    }

    async fn send_ble(&self, bytes: &[u8]) -> Result<(), String> {
        if let (Some(_ble_device), Some(write_char)) = (&self.ble_device, &self.write_char) {
            // Create a Buffer from the bytes
            let mut buffer = Buffer::Create(bytes.len() as u32)?;
            buffer.as_mut_slice().copy_from_slice(bytes);
            let ibuffer: IBuffer = buffer.into();
            let result = write_char.WriteValueWithResultAsync(&ibuffer)?.await?;
            if result.Status()? == GattCommunicationStatus::Success {
                Ok(())
            } else {
                Err(format!("Write failed with status: {:?}", result.Status()?))
            }
        } else {
            Err("BLE not initialized".to_string())
        }
    }

    fn send_serial(&mut self, bytes: &[u8]) -> Result<(), String> {
        if let Some(port) = self.port.as_mut() {
            port.write_all(bytes).map_err(|e| e.to_string())?;
            Ok(())
        } else {
            Err("Serial port not initialized".to_string())
        }
    }

    fn is_connected(&self) -> bool {
        if self.is_ble {
            self.ble_device.is_some() && self.write_char.is_some()
        } else {
            self.port.is_some()
        }
    }

    pub fn parse_response(&mut self, hex: &str) -> Result<(), String> {
    let c_section = get_cmd_value("C0C1C2C3", "C4C5C6C7", hex)?;
        let bytes = hex::decode(&c_section).map_err(|e| e.to_string())?;

        self.project_data.public.rd_mode = clamp(bytes.get(9).cloned().unwrap_or(0), 0, 255);
        self.options.tx_color = clamp(bytes.get(3).cloned().unwrap_or(0), 0, 9);
        self.options.tx_size = clamp(((bytes.get(4).cloned().unwrap_or(0) as f32 / 255.0) * 100.0).round() as u8, 10, 100);
        self.options.run_speed = clamp(((bytes.get(6).cloned().unwrap_or(0) as f32 / 255.0) * 100.0).round() as u8, 0, 255);
        self.options.tx_dist = clamp(((bytes.get(8).cloned().unwrap_or(0) as f32 / 255.0) * 100.0).round() as u8, 10, 100);
        self.project_data.public.sound_val = clamp(((bytes.get(10).cloned().unwrap_or(0) as f32 / 255.0) * 100.0).round() as u8, 0, 255);

        let mut offset = 17;
        if let Some(item) = self.project_data.prj_item.get_mut(0) {
            item.py_mode = clamp(bytes.get(offset).cloned().unwrap_or(0), 0, 255);
            item.prj_selected[3] = clamp(bytes.get(offset + 1).cloned().unwrap_or(0) as u16, 0, 65535);
            item.prj_selected[2] = clamp(bytes.get(offset + 3).cloned().unwrap_or(0) as u16, 0, 65535);
            item.prj_selected[1] = clamp(bytes.get(offset + 5).cloned().unwrap_or(0) as u16, 0, 65535);
            item.prj_selected[0] = clamp(bytes.get(offset + 7).cloned().unwrap_or(0) as u16, 0, 65535);
            offset += 9;
        }

        let xy_offset = offset;
        self.options.tx_dist = clamp(bytes.get(xy_offset).cloned().unwrap_or(0), 0, 255); // Placeholder
        // Expand XY config as needed

    let b_section = get_cmd_value("00010203", "04050607", hex)?;
        let b_bytes = hex::decode(&b_section).map_err(|e| e.to_string())?;
        // Expand settings parsing

    let d_section = get_cmd_value("D0D1D2D3", "D4D5D6D7", hex)?;
        if !d_section.is_empty() {
            let d_bytes = hex::decode(&d_section).map_err(|e| e.to_string())?;
            let count = clamp(d_bytes.get(1).cloned().unwrap_or(0) & 0x7F, 0, 127);
            for i in 0..count as usize {
                let item_offset = 3 + i * 22;
                if item_offset + 22 <= d_bytes.len() {
                    let mut item = PrjItem {
                        py_mode: clamp(d_bytes[item_offset], 0, 255),
                        prj_selected: [0; 4],
                    };
                    for j in 0..4 {
                        item.prj_selected[j] = clamp(d_bytes[item_offset + 1 + j * 2] as u16, 0, 65535);
                    }
                    self.project_data.prj_item.push(item);
                }
            }
        }
        Ok(())
    }

    fn add_content(&mut self, hex: String) {
        println!("addContent: {}", hex);
        let mut content = self.blu_rec_content.back().cloned().unwrap_or_else(String::new);
        if content.is_empty() && hex.starts_with("E0E1E2E3") {
            content = hex;
        } else {
            content.push_str(&hex);
        }

        if !content.is_empty() {
            let last_start = content.rfind("E0E1E2E3").unwrap_or(0);
            let last_end = content.rfind("E4E5E6E7").unwrap_or(0);
            let mut processed = content.clone();

            if last_end > 0 {
                if last_end == content.len() - 8 {
                    let packet = content[last_start..last_end + 8].to_string();
                    self.data_received(packet);
                    processed = String::new();
                } else {
                    processed = content[last_start..].to_string();
                }
            }
            self.blu_rec_content.clear();
            self.blu_rec_content.push_back(processed);
        }
    }

    async fn data_received(&mut self, packet: String) {
        println!("Data received: {}", packet);
        if let Some(timer) = self.rec_device_msg_timer {
            tokio::time::sleep_until(timer.into()).await;
            self.rec_device_msg_timer = None;
        }

    let c_section = get_cmd_value("C0C1C2C3", "C4C5C6C7", &packet).unwrap_or_default();
        let new_cmd = format!("C0C1C2C304{}{}C4C5C6C7",
                             &c_section[2..20], // Slice from index 2 to 20 (18 chars)
                             &c_section[28..]); // Slice from index 28 to end

        println!("Constructed command: {}", new_cmd);

        if let Some(testing) = &mut self.device_testing {
            if testing.read_uuid == "0000ffe1-0000-1000-8000-00805f9b34fb" { // Example UUID
                testing.test_status = "Sending program command".to_string();
                println!("Test status updated");

                let controller = self.clone();
                tokio::spawn(async move {
                    let result = controller.send(&new_cmd, false, None).await;
                    if let Err(e) = result {
                        testing.test_status = "Test ended".to_string();
                        testing.test_result = "Not passed".to_string();
                        println!("Sending program command failed: {}", e);
                        testing.test_msg = format!("Sending program command failed: {}", e);
                        println!("Test message updated");
                    } else {
                        testing.test_status = "Test ended".to_string();
                        testing.test_result = "Passed".to_string();
                        testing.test_msg = "Sending program command successful".to_string();
                        controller.not_pass -= 1;
                        controller.pass_count += 1;
                        println!("Test passed, counts updated");
                    }
                });
            } else {
                println!("Characteristic value does not match");
            }
        }
    }

    pub async fn start_test(&mut self, test: &mut DeviceTesting, max_retries: i32) {
        if max_retries < 0 {
            test.test_status = "Test ended".to_string();
            test.test_result = "Not passed".to_string();
            println!("Send command failed: timeout");
            test.test_msg = "Send command failed - timeout".to_string();
            println!("Test status updated");
            return;
        }

        if let Some(testing) = self.device_testing.as_mut() {
            if testing.can_send {
                tokio::time::sleep(Duration::from_secs(4)).await;
                let mut testing_clone = testing.clone();
                drop(testing); // Release the mutable borrow before calling .await
                self.do_start_test(&mut testing_clone).await;
                // Optionally update self.device_testing with testing_clone if needed
            } else {
                tokio::time::sleep(Duration::from_secs(1)).await;
                self.start_test(test, max_retries - 1).await;
            }
        }
    }

    pub async fn do_start_test(&mut self, test: &mut DeviceTesting) {
        test.test_status = "Sending query command".to_string();
        println!("Test status updated");

        let result = self.send("E0E1E2E3F776CD3AE4E5E6E7", false, None).await;
        if let Err(e) = result {
            test.test_status = "Test ended".to_string();
            test.test_result = "Not passed".to_string();
            println!("Sending query command failed: {}", e);
            test.test_msg = "Sending query command failed".to_string();
            println!("Test status updated");
        } else {
            test.test_status = "Sending query command successful, waiting for device response".to_string();
            println!("Test status updated");
            if let Some(timer) = self.rec_device_msg_timer {
                tokio::time::sleep_until(timer.into()).await;
            }
            self.rec_device_msg_timer = Some(tokio::time::Instant::now() + Duration::from_secs(10));
            let controller = self.clone();
            tokio::spawn(async move {
                tokio::time::sleep(Duration::from_secs(10)).await;
                if let Some(testing) = &mut controller.device_testing {
                    //                    testing.rec_device_msg_timer = None;
                    if testing.conn {
                        testing.test_status = "Test ended".to_string();
                        testing.test_result = "Not passed".to_string();
                        println!("Read device timeout");
                        testing.test_msg = "Read device timeout".to_string();
                        println!("Test status updated");
                    }
                }
            });
        }
    }

    pub async fn close_cnn_and_run(&mut self, callback: Option<Box<dyn Fn()>>) {
        let mut connected_device = None;
        for device in &self.device_tested {
            if device.conn {
                connected_device = Some(device.clone());
                break;
            }
        }
        println!("dev==== {:?}", connected_device);
        if let Some(device) = connected_device {
            if device.conn {
                if let Some(ble_device) = &self.ble_device {
                    ble_device.Close().ok();
                    if let Some(cb) = callback {
                        cb();
                    }
                }
            } else if let Some(cb) = callback {
                cb();
            }
        } else if let Some(cb) = callback {
            cb();
        }
    }

    pub async fn do_start(&mut self, index: usize) {
        if index > self.device_tested.len() - 1 {
            if self.discovery_started {
                tokio::time::sleep(Duration::from_secs(1)).await;
                self.do_start(index).await;
            }
            return;
        }

        self.testing_idx = index;
        self.device_testing = Some(self.device_tested[index].clone());
        if let Some(testing) = &mut self.device_testing {
            testing.test_status = "Testing".to_string();
            testing.test_result = "Testing".to_string();
            testing.test_msg = "Testing".to_string();
            println!("startTest ========: {}, {:?}", self.testing_idx, testing);
        }

        let controller = self.clone();
        self.close_cnn_and_run(Some(Box::new(move || {
            let mut test = controller.device_testing.clone().unwrap();
            let controller_clone = controller.clone();
            tokio::spawn(async move {
                if let Ok(()) = controller_clone.connect_ble().await {
                    test.conn = true;
                    controller_clone.do_log();
                    controller_clone.do_start_test(&mut test).await;
                } else {
                    if let Some(mut testing) = controller_clone.device_testing.as_mut() {
                        testing.test_status = "Test ended".to_string();
                        testing.test_result = "Not passed".to_string();
                        testing.test_msg = "Connection failed".to_string();
                        controller_clone.do_log();
                        controller_clone.device_testing = None;
                        if controller_clone.discovery_started {
                            controller_clone.do_start(controller_clone.testing_idx + 1).await;
                        }
                    }
                }
            });
        }))).await;
    }

    fn do_log(&self) {
        println!("Log: {:?}", self.device_testing);
    }

    pub async fn test_click(&mut self) {
        if let Some(testing) = &self.device_testing {
            self.send("E0E1E2E3F776CD3AE4E5E6E7", false, None).await.ok();
        }
    }

    pub async fn send_data(&mut self, test: &DeviceTesting, cmd_hex: &str, callback: Option<Box<dyn Fn(String)>>) {
        let chunks = covert2_send_data(cmd_hex).unwrap_or_default();
        self.do_send_data(test, &chunks, 0, callback).await;
    }

    pub async fn do_send_data(&mut self, test: &DeviceTesting, chunks: &[Vec<u8>], index: usize, callback: Option<Box<dyn Fn(String)>>) {
        if index >= chunks.len() {
            if let Some(cb) = callback {
                cb(String::new());
            }
            return;
        }

        let result = self.send_ble(&chunks[index]).await;
        if let Err(e) = result {
            if let Some(cb) = callback {
                cb(e);
            }
        } else if index < chunks.len() - 1 {
            tokio::time::sleep(Duration::from_millis(20)).await;
            self.do_send_data(test, chunks, index + 1, callback).await;
        } else if let Some(cb) = callback {
            cb(String::new());
        }
    }

    pub async fn write_ble_characteristic_value(&mut self, test: &DeviceTesting, data: &[u8], callback: Option<Box<dyn Fn(String)>>) {
        let result = self.send_ble(data).await;
        if let Some(cb) = callback {
            cb(result.err().unwrap_or_default());
        }
    }

    pub async fn close_bluetooth_adapter(&mut self) {
        if let Some(ble_device) = &self.ble_device {
            ble_device.Close().ok();
        }
    }
}



fn clamp<T: Ord>(val: T, min: T, max: T) -> T {
    val.max(min).min(max)
}

fn get_cmd_value(start: &str, end: &str, input: &str) -> Result<String, String> {
    let re = format!("{}(.+?){}", regex::escape(start), regex::escape(end));
    let regex = Regex::new(&re).map_err(|e| format!("Regex error: {}", e))?;
    regex.captures(input)
        .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()))
        .ok_or_else(|| {
            println!("No matching string found at getCmdValue");
            "No matching string found at getCmdValue".to_string()
        })
}

fn covert2_send_data(hex: &str) -> Result<Vec<Vec<u8>>, String> {
    let bytes = hex::decode(hex.replace(" ", "")).map_err(|e| e.to_string())?;
    let mut chunks = Vec::new();
    for i in (0..bytes.len()).step_by(20) {
        let end = std::cmp::min(i + 20, bytes.len());
        chunks.push(bytes[i..end].to_vec());
    }
    Ok(chunks)
}

    */