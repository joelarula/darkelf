# DarkElf DMX Console

A cross-platform DMX device control and channel editor UI for Laser light 8340 device written in rust.



## Features
- DMX device detection and port selection (auto-selects if only one port)
- Real-time DMX channel editing with tooltips for channel capabilities
- Two-column channel layout with fixed-width labels and DragValue widgets
- Status bar for device and operation feedback
- Modern, responsive UI using egui-taffy flexbox patterns

## Getting Started

### Prerequisites
- A supported DMX device
- FTDI USB DMX Converter 
- JSON fixture file  `laser_light_8340` in Open Fixture Format in `assets/fixtures/` ([see Open Fixture Library](https://open-fixture-library.org/)) anlog for UKING ZQ03268 LASER



### Build and Run
```sh
cargo run --bin dmxconsole
```

### Usage
- Select a DMX port from the dropdown at the bottom of the UI
- Edit channel values using the DragValue widgets
- Hover over channel labels for detailed capability tooltips
- Status bar displays device and operation status


## License
MIT

## Author
Joel Arula
