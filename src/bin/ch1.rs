
use darkelf::dmx;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {


    // List all DMX-compatible ports using dmx::available_dmx_ports
    println!("Scanning available serial ports for DMX compatibility:");
    let dmx_ports = dmx::scan_dmx_ports();
    if dmx_ports.is_empty() {
        println!("No DMX-compatible ports found.");
        return Ok(());
    }


    // Select DMX port: auto if one, prompt if multiple
    let port_name = if dmx_ports.len() == 1 {
        println!("\nOnly one DMX-compatible port found: {}", dmx_ports[0]);
        &dmx_ports[0]
    } else {
        println!("\nMultiple DMX-compatible ports found:");
        for (i, p) in dmx_ports.iter().enumerate() {
            println!("  [{}] {}", i + 1, p);
        }
        use std::io::{self, Write};
        loop {
            print!("Select port by number: ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            if let Ok(idx) = input.trim().parse::<usize>() {
                if idx >= 1 && idx <= dmx_ports.len() {
                    break &dmx_ports[idx - 1];
                }
            }
            println!("Invalid selection. Please enter a valid number.");
        }
    };
    println!("\nUsing DMX port: {}", port_name);
    let mut controller = dmx::DmxController::new(port_name, 1)?;
    println!("Connected to DMX adapter. Testing CH1 (Shutter) features...");



    Ok(())
}