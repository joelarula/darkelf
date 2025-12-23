# DAC - Raspberry Pi Pico RP2040 Project

Rust firmware for RP2040 microcontroller (Raspberry Pi Pico).

## Setup

```powershell
# Install ARM Cortex-M0+ target
rustup target add thumbv6m-none-eabi

# Install probe-rs for flashing/debugging
cargo install probe-rs --features cli

# Or use elf2uf2-rs for drag-and-drop UF2 flashing
cargo install elf2uf2-rs
```

## Build & Flash

```powershell
cd dac

# Build
cargo build --release

# Flash with probe-rs (requires debug probe)
cargo run --release

# OR convert to UF2 and copy to Pico in bootloader mode
elf2uf2-rs target/thumbv6m-none-eabi/release/dac dac.uf2
# Then copy dac.uf2 to the Pico USB drive (BOOTSEL mode)
```

## UF2 Bootloader Mode

1. Hold BOOTSEL button on Pico
2. Connect USB cable
3. Release BOOTSEL button
4. Pico appears as USB drive
5. Copy dac.uf2 to the drive
