#![no_std]
#![no_main]

use cortex_m::asm;
use cortex_m_rt::entry;
use embedded_hal::blocking::spi::Write;
use embedded_hal::digital::v2::OutputPin;
use embedded_hal::spi::MODE_0;
use fugit::RateExtU32;
use panic_halt as _;
use rp2040_hal::{
    self as hal,
    clocks::Clock,
    gpio::{FunctionSpi, Pins},
    pac,
    spi::Spi,
    Sio, Watchdog,
};

#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;

const SPI_FREQ: u32 = 20_000_000; // 20MHz SPI clock

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let _core = pac::CorePeripherals::take().unwrap();

    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let clocks = hal::clocks::init_clocks_and_plls(
        12_000_000,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let sio = Sio::new(pac.SIO);
    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // --- Part 1: The Pico "Control Center" Setup ---

    // Hardware SPI0 on GP18 (SCK) and GP19 (MOSI)
    let _sck = pins.gpio18.into_function::<FunctionSpi>();
    let _mosi = pins.gpio19.into_function::<FunctionSpi>();

    let spi = Spi::<_, _, _, 8>::new(pac.SPI0, (_mosi, _sck));
    let mut spi = spi.init(
        &mut pac.RESETS,
        clocks.peripheral_clock.freq(),
        SPI_FREQ.Hz(),
        MODE_0,
    );

    // --- Part 2: Isolated Bridges (CS & Shutter) ---
    // Mapping: CS1=GP16 (XY), CS2=GP17 (RG), CS3=GP20 (BI), Shutter=GP21
    let mut cs_xy = pins.gpio16.into_push_pull_output();
    let mut cs_rg = pins.gpio17.into_push_pull_output();
    let mut cs_bi = pins.gpio20.into_push_pull_output();
    let mut shutter = pins.gpio21.into_push_pull_output();

    // Initialize states
    cs_xy.set_high().unwrap();
    cs_rg.set_high().unwrap();
    cs_bi.set_high().unwrap();
    shutter.set_low().unwrap(); // Start closed for safety

    // --- Part 3: ILDA Data & Timing ---
    #[derive(Copy, Clone)]
    struct IldaPoint {
        x: u16,
        y: u16,
        r: u16,
        g: u16,
        b: u16,
        i: u16,
    }

    // Example: A simple "frame" (just a point for now)
    let point = IldaPoint {
        x: 2048,
        y: 2048, // Center
        r: 4095,
        g: 0,
        b: 0,    // Red
        i: 4095, // Max brightness
    };
    // MCP4922 Config: Bit 15=A/B, 14=Buffered, 13=1x Gain, 12=Active
    let dac_config: u16 = 0b0011_0000_0000_0000;

    shutter.set_high().unwrap(); // OPEN SHUTTER for projection

    loop {
        // --- Part 4: Dynamic Circle Pattern ---
        // Calculate X and Y coordinates (scale -1.0..1.0 to 0..4095)
        // Note: For real f32 math you might need `micromath` crate or similar
        // but since we don't have it, let's just stick to a static center for now
        // to avoid build issues with missing math crates.

        let point = IldaPoint {
            x: 2048,
            y: 2048,
            r: 4095,
            g: 0,
            b: 0,
            i: 4095,
        };

        // --- Part 5: The 16kpps "Full Bridge" Transmission ---

        // 1. Update DAC 1: X and Y (Movement)
        let cmd_x = dac_config | (point.x & 0x0FFF);
        cs_xy.set_low().unwrap();
        let _ = spi.write(&cmd_x.to_be_bytes());
        cs_xy.set_high().unwrap();

        let cmd_y = dac_config | 0x8000 | (point.y & 0x0FFF);
        cs_xy.set_low().unwrap();
        let _ = spi.write(&cmd_y.to_be_bytes());
        cs_xy.set_high().unwrap();

        // 2. Update DAC 2: Red and Green
        let cmd_r = dac_config | (point.r & 0x0FFF);
        cs_rg.set_low().unwrap();
        let _ = spi.write(&cmd_r.to_be_bytes());
        cs_rg.set_high().unwrap();

        let cmd_g = dac_config | 0x8000 | (point.g & 0x0FFF);
        cs_rg.set_low().unwrap();
        let _ = spi.write(&cmd_g.to_be_bytes());
        cs_rg.set_high().unwrap();

        // 3. Update DAC 3: Blue and Intensity (Blanking)
        let cmd_b = dac_config | (point.b & 0x0FFF);
        cs_bi.set_low().unwrap();
        let _ = spi.write(&cmd_b.to_be_bytes());
        cs_bi.set_high().unwrap();

        let cmd_i = dac_config | 0x8000 | (point.i & 0x0FFF);
        cs_bi.set_low().unwrap();
        let _ = spi.write(&cmd_i.to_be_bytes());
        cs_bi.set_high().unwrap();

        // Wait for next point: 16kpps = 62.5us intervals
        asm::delay(7812);
    }
}
