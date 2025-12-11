# Helios DAC Rust Implementation

This directory contains a Rust implementation of the Helios Laser DAC SDK, providing safe bindings to the native library.

## Structure

- `src/heliosdac.rs` - Main Rust wrapper around the Helios DAC C library
- `tests/helios_example.rs` - Example matching the Python implementation
- `tests/helios_example_csharp_style.rs` - Example matching the C# implementation

## Prerequisites

You need the Helios DAC shared library installed:
- **Windows**: `HeliosLaserDAC.dll`
- **Linux**: `libHeliosLaserDAC.so`
- **macOS**: `libHeliosLaserDAC.dylib`

The library should be either:
1. In the same directory as your executable
2. In your system's library path
3. Specified via environment variables

## Building

The library uses FFI to link to the native Helios DAC library. Make sure the library is available before building.

```bash
# Build the library
cargo build --release

# Run the Python-style example
cargo test --test helios_example -- --nocapture --test-threads=1

# Run the C#-style example
cargo test --test helios_example_csharp_style -- --nocapture --test-threads=1
```

## Usage

```rust
use darkelf::heliosdac::{HeliosDacController, HeliosPoint};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create controller and open devices
    let mut controller = HeliosDacController::new();
    let num_devices = controller.open_devices()?;
    println!("Found {} devices", num_devices);

    // Create some points
    let points = vec![
        HeliosPoint::new(0x7FF, 0x7FF, 255, 0, 0, 255),  // Red center
        HeliosPoint::new(0xFFF, 0x7FF, 0, 255, 0, 255),  // Green right
    ];

    // Wait for device to be ready
    if controller.wait_for_ready(0, 512)? {
        // Write frame at 30000 points per second
        controller.write_frame(0, 30000, 0, &points)?;
    }

    controller.close_devices()?;
    Ok(())
}
```

## API Overview

### Core Types

- `HeliosPoint` - Standard resolution point (x, y: 12-bit, r/g/b/i: 8-bit)
- `HeliosPointHighRes` - High resolution point (x, y, r, g, b: 16-bit)

### Main Controller

- `open_devices()` - Scan and open all connected DACs
- `close_devices()` - Close all connections
- `get_status(dac_num)` - Check if DAC is ready for new frame
- `write_frame(dac_num, pps, flags, points)` - Send frame to DAC
- `wait_for_ready(dac_num, max_attempts)` - Helper to wait for ready status
- `stop(dac_num)` - Stop output
- `set_shutter(dac_num, open)` - Control laser shutter
- `get_name(dac_num)` - Get device name

### Constants

- `HELIOS_MAX_POINTS` - Maximum points per frame (0xFFF = 4095)
- `HELIOS_MAX_PPS` - Maximum points per second (0xFFFF = 65535)
- `HELIOS_MIN_PPS` - Minimum points per second (7)
- `HELIOS_FLAGS_DEFAULT` - Recommended default flags

## Examples Comparison

### Python Example
The `helios_example.rs` matches the Python implementation:
- 1500 frames total
- 30 frame buffers cycling
- 30000 PPS
- 512 status check attempts

### C# Example
The `helios_example_csharp_style.rs` matches the C# implementation:
- 150 frames total
- 30 frame buffers cycling
- 25000 PPS
- 50 status check attempts
- Shutter control

Both create the same animation: a horizontal line scanning vertically upward.

## Notes

- Always call `get_status()` and wait for ready before calling `write_frame()`
- Use `HELIOS_FLAGS_DEFAULT` for cross-platform compatibility
- Frames should be at least 10ms in duration to account for timing jitter
- The controller automatically closes devices when dropped
