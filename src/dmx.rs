
/// DMX-512 baud rate (250000 baud as per specification)
pub const DMX_BAUD_RATE: u32 = 250_000;
pub const DMX_FRAME_SIZE: usize = 512;
pub const DMX_TEST_FRAME: [u8; DMX_FRAME_SIZE] = [0u8; DMX_FRAME_SIZE];

use serialport::{DataBits, FlowControl, Parity, StopBits};
use std::thread;
use std::time::Duration;
use std::io::Write;
use std::error::Error;


pub struct DmxFrame {
    pub channels: Vec<u8>, // or [u8; 512] for a full DMX universe
}

impl DmxFrame {
    pub fn new() -> Self {
        DmxFrame {
            channels: vec![0u8; DMX_FRAME_SIZE],
        }
    }

    pub fn set_channel(&mut self, channel: usize, value: u8) {
        if channel > 0 && channel <= self.channels.len() {
            self.channels[channel - 1] = value;
        }
    }

    pub fn get_channel(&self, channel: usize) -> Option<u8> {
        if channel > 0 && channel <= self.channels.len() {
            Some(self.channels[channel - 1])
        } else {
            None
        }
    }
}

pub trait DmxCommand {
    fn configure( &self, state: &mut DmxFrame);
}

impl<F> DmxCommand for F
where
    F: Fn(&mut DmxFrame),
{
    fn configure(&self, state: &mut DmxFrame) {
        self(state);
    }
}

pub struct DmxController {
    port: Box<dyn serialport::SerialPort>,
    address: usize,  // Starting channel (1-based)
}

impl DmxController {


    pub fn new(port_name: &str, address: usize) -> Result<Self, Box<dyn Error>> {
        println!("Opening DMX port: {}", port_name);
		let mut port = serialport::new(port_name, DMX_BAUD_RATE)
            .data_bits(DataBits::Eight)
            .flow_control(FlowControl::None)
            .parity(Parity::None)
            .stop_bits(StopBits::Two)
            .timeout(Duration::from_millis(10))
            .open()?;

        Ok(DmxController { port, address: address - 1 })  // 0-based index
    }


	pub fn send(&mut self, commands: &[Box< dyn DmxCommand>]) -> Result<(), Box<dyn Error>> {
        let mut frame = DmxFrame::new();
        for cmd in commands {         
			cmd.configure(&mut frame);
        }
        self.send_frame(&frame)
    }


    pub fn send_frame(&mut self, state: &DmxFrame) -> Result<(), Box<dyn Error>> {
        // DMX-512 Break: minimum 176μs (we use 200μs for safety)
        self.port.set_break()?;
        thread::sleep(Duration::from_micros(200));
        
        // Mark After Break (MAB): minimum 12μs (we use 20μs for safety)
        self.port.clear_break()?;
        thread::sleep(Duration::from_micros(20));

        let mut frame: Vec<u8> = vec![0x00];  // Start code
        frame.resize(DMX_FRAME_SIZE + 1, 0);  // 1 + 512

        for (i, &val) in state.channels.iter().enumerate() {
            let idx = self.address + i + 1;  // +1 for after start
            if idx < frame.len() {
                frame[idx] = val;
            }
        }

        self.port.write_all(&frame)?;
        Ok(())
    }
}

/// Returns a Vec of DMX-compatible serial port names (ports that can be opened at 250_000 baud, 2 stop bits, and accept a DMX frame).
pub fn scan_dmx_ports() -> Vec<String> {
	let mut dmx_ports = Vec::new();
	match serialport::available_ports() {
		Ok(ports) => {
			for p in &ports {
				println!("Testing port: {} (type: {:?}, info: {:?})", p.port_name, p.port_type, p); // Print port info
				let result = serialport::new(&p.port_name, DMX_BAUD_RATE)
					.timeout(Duration::from_millis(10))
					.stop_bits(serialport::StopBits::Two)
					.open()
					.and_then(|mut port| {
						port.write_all(&DMX_TEST_FRAME)
							.map_err(|e| serialport::Error::new(serialport::ErrorKind::Io(std::io::ErrorKind::Other), format!("write failed: {}", e)))
					});
				if result.is_ok() {
					dmx_ports.push(p.port_name.clone());
				}
			}
		}
		Err(_) => {}
	}
	dmx_ports
}
