use std::env;

use darkelf::{device::LaserDevice, dmxchannel::{DIMMER_CHANNEL, OFF, ON}, dmxdevice::DmxLaserDevice, util};
use log::{info, warn};

#[tokio::main]
#[test]
async fn test_quick_laser_check() -> Result<(), anyhow::Error> {
    util::setup_logging();
    unsafe {
        env::set_var("RUST_LOG", "debug");
    }

    info!("=== Quick Laser Device Check ===");
    let dmx_ports = darkelf::dmx::scan_dmx_ports();
    assert!(!dmx_ports.is_empty(), "No DMX-compatible ports found");
    
    // Always use COM4 (USB Serial Port) for consistent testing
    let port = "COM4";
    let dmx_channel = 1;
    
    // Verify COM4 is available in the scanned ports
    if !dmx_ports.contains(&port.to_string()) {
        panic!("COM4 (USB Serial Port) not found in available DMX ports: {:?}", dmx_ports);
    }
    
    info!("Creating DMX laser device on port: {} (USB Serial Port)", port);
    
    match DmxLaserDevice::new(port, dmx_channel) {
        Ok(device) => {
            info!("Created DMX laser device on {} starting at channel {}", port, dmx_channel);
            
            if let Ok(()) = device.start() {
                info!("Device started - DMX output active");
                


                let _ = device.set_dmx_channel(DIMMER_CHANNEL, 45);  
                let _ = device.set_dmx_channel(2, 0);    // No color control  
                let _ = device.set_dmx_channel(3, 0);    // No color speed (static)
                let _ = device.set_dmx_channel(4, 41);   // Pattern group
                let _ = device.set_dmx_channel(5, 90);   // Pattern select
                let _ = device.set_dmx_channel(6, 255);  // Maximum effects
                let _ = device.set_dmx_channel(7, 128);  // Medium effect speed
                let _ = device.set_dmx_channel(8, 255);  // Maximum pattern size
                

                std::thread::sleep(std::time::Duration::from_secs(1));
                device.off().await;
                std::thread::sleep(std::time::Duration::from_secs(1));
                device.on().await;
     

            
                for i in (10..255) {
                    let dimmer_value = i as u8;
                    info!("Setting master dimmer to {}", dimmer_value);
                    let _ = device.set_dmx_channel(1, dimmer_value);
                    std::thread::sleep(std::time::Duration::from_millis(100));
                }

                
                std::thread::sleep(std::time::Duration::from_secs(10));

                // Stop the device after the extended setup phase
                if let Ok(()) = device.stop() {
                    info!("Device stopped successfully");
                }
                
            } else {
                warn!("Failed to setup device");
            }
        },
        Err(e) => {
            warn!("Failed to create DMX device: {}", e);
        }
    }
    
    Ok(())
}