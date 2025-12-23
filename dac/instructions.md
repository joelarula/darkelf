# Isolated 16kpps ILDA DAC - Build Instructions

This is the final blueprint for your Isolated 16kpps ILDA DAC. This design ensures your PC is safe through galvanic isolation, your signals are clean with proper analog buffering, and your color depth is a full 12-bit for professional laser control.

## 1. Bill of Materials (BOM)

### Microcontroller
The Raspberry Pi Pico serves as the brain of this system, utilizing its RP2040 chip with PIO (Programmable I/O) capabilities to achieve the high-speed 16kpps update rate required for smooth laser scanning.

### Isolation Components
The design employs two critical isolation barriers. For data isolation, we use the ADuM1401BRWZ, a 4-channel digital isolator in SOIC-16 package that safely transfers SPI signals across the isolation boundary. For power isolation, the RECOM RS-0512D provides a 5V to ±12V isolated DC-DC conversion, ensuring complete galvanic isolation between your computer and the high-voltage laser system.

### Digital-to-Analog Conversion
Three MCP4922-E/P chips provide the core digital-to-analog conversion. These 12-bit dual DACs in DIP-14 packages give you six independent channels: two for X/Y positioning and four for RGBI color control. The 12-bit resolution provides 4096 steps per channel, essential for smooth gradients and precise beam positioning.

### Signal Conditioning
The TL074IDT quad op-amp handles signal amplification and buffering. It converts the 0-4.096V DAC outputs to the ±5V differential signals required by the ILDA standard for X/Y, and provides clean buffering for the RGB color channels.

### Power Regulation
An L7805 voltage regulator steps down the isolated +12V rail to +5V, which powers the DACs and the isolated side of the digital isolator. This ensures stable operation of all analog components.

### Connectors and Adapters
A standard DB-25 female connector provides the ILDA interface. If you're using SMD versions of the ADuM1401 or TL074, you'll need SOIC to DIP adapters for breadboard prototyping.

### Passive Components
Use precision 1% resistors in 10kΩ and 20kΩ values for accurate X/Y signal scaling. Stock up on 0.1µF ceramic capacitors for decoupling and noise filtering across all power pins.## 2. Construction Guide

### Understanding the Two Islands Architecture

The fundamental design principle of this DAC is complete galvanic isolation between your computer and the laser system. To achieve this, divide your breadboard into two distinct "islands" with no direct electrical connection between their ground rails.

**Island 1 (Digital Side):** This is the left side of your breadboard, powered directly from the Pico's USB connection. It contains the Pico microcontroller and the input side of the isolation components. This island shares ground with your computer.

**Island 2 (Analog Side):** This is the right side, powered by the isolated DC-DC converter. It contains all the DACs, op-amps, and connects directly to the ILDA output. This island's ground is completely separate from your computer.

The physical gap between these islands on the breadboard represents the isolation barrier that protects your computer from laser system faults.

### Power Bridge Setup

Begin by establishing the isolated power supply. Connect the Pico's VBUS pin (5V from USB) to pin 1 of the RECOM RS-0512D module. Connect the Pico's ground to RECOM pin 2. This provides the input power to the isolation barrier.

On the output side, the RECOM generates ±12V and a common reference. Connect the +12V output to pin 4 of the TL074 op-amp for positive rail power. Connect the -12V output to pin 11 of the TL074 for negative rail power. The common pin (0V reference) connects to Island 2's ground rail, which will eventually connect to ILDA pin 25.

### Creating the Isolated 5V Rail

The DACs and isolator require +5V on the isolated side. Feed the RECOM's +12V output into the L7805 voltage regulator's input pin. Connect the L7805's ground pin to Island 2's ground rail. The L7805's output provides a stable +5V that powers pin 16 of the ADuM1401, pin 1 of all three MCP4922 DACs, and pin 11 of all three DACs (the Vref input).

Place decoupling capacitors (0.1µF) close to each IC's power pins to filter noise and prevent oscillation.

### Data Bridge Configuration

The ADuM1401 digital isolator physically spans the gap between the two islands. Its placement is critical—it literally bridges the isolation barrier.

On the Island 1 side, connect pin 1 (VDD1) to the Pico's 3.3V output. Pin 8 (GND1) connects to the Pico's ground. Pins 2, 3, and 4 are the data inputs and should connect to Pico GPIO pins GP10, GP11, and GP12, which will carry the CS (chip select), SCK (clock), and SDI (data) SPI signals.

On the Island 2 side, pin 16 (VDD2) connects to your isolated 5V rail. Pin 9 (GND2) connects to Island 2's ground. Pins 15, 14, and 13 are the isolated outputs that connect to the corresponding SPI pins on all three DAC chips simultaneously.

### X/Y Analog Stage Construction

The first MCP4922 DAC generates the X and Y position signals. However, ILDA requires ±5V differential signals, while the DAC outputs 0-4.096V. This is where the TL074 op-amp performs critical level shifting and buffering.

For the X channel, connect DAC1's output A to the non-inverting input of TL074 channel A. Create a 2.5V reference using a voltage divider (10kΩ and 10kΩ from +5V to ground), and connect this to the inverting input through a 20kΩ resistor. This configuration shifts and scales the DAC output to the required ±5V range. Connect the op-amp's output directly to DB-25 pin 1 (X+).

Repeat this exact configuration for the Y channel using DAC1's output B and TL074 channel B, with the output going to DB-25 pin 2 (Y+).

### Color Channel Wiring

The second DAC (DAC2) generates red and green outputs. Connect output A directly to DB-25 pin 5 (Red) and output B to DB-25 pin 6 (Green). The third DAC (DAC3) generates blue and intensity, with output A going to DB-25 pin 7 (Blue).

For additional protection and impedance matching, you can route these signals through the remaining two channels of the TL074 configured as voltage followers (unity-gain buffers). This provides better drive capability and protects the DACs from downstream faults.

### Final DB-25 Connections

Connect DB-25 pin 25 to Island 2's ground rail. This is the signal ground reference for the ILDA interface. Finally, create the interlock loop by connecting a jumper wire between DB-25 pins 4 and 17. This shorts the interlock circuit that many professional lasers require for safety compliance.

## 3. Software Strategy: Achieving 16kpps with PIO

The challenge with standard SPI libraries is speed. To reach 16,000 points per second (16kpps), we need to transmit six 16-bit words (X, Y, Red, Green, Blue, Intensity) in just 62.5 microseconds. This translates to a sustained data rate of approximately 1.5 Mbps with minimal inter-word gaps.

Traditional bit-banging approaches that toggle GPIO pins in software loops can't maintain this pace reliably. Context switches, interrupt latency, and instruction timing variations introduce jitter that degrades scan quality. Even dedicated SPI peripheral hardware may not provide the precise timing control needed for clean laser projection.

The Raspberry Pi Pico's Programmable I/O (PIO) subsystem solves this elegantly. PIO is a set of independent state machines that run alongside the main ARM core, executing simple assembly-like programs at fixed clock rates. Once configured, a PIO state machine can generate perfectly timed SPI transactions without any CPU intervention.

The implementation strategy involves writing a PIO program that bit-bangs the SPI protocol with exact timing. The program takes 16-bit words from a FIFO buffer and serially clocks them out while managing the chip select line. The main firmware running on the ARM core simply keeps the FIFO filled with scan data—the PIO handles all the real-time signaling.

This architecture provides several critical advantages. First, it guarantees deterministic timing regardless of what the main CPU is doing. Second, it frees the CPU to perform higher-level tasks like calculating scan patterns, handling USB commands, or implementing safety features. Third, it achieves the sustained throughput needed for smooth 16kpps operation without dropping frames or introducing visible artifacts.

The PIO clock can be configured to run at precise multiples of the desired bit rate, ensuring clean edges on the SPI signals even at high speeds. With careful tuning, this approach can push well beyond 16kpps if needed, potentially reaching 30-40kpps for high-speed scanning applications.

---

**Safety Note:** Always verify your laser system's interlock requirements before connecting this DAC. Never operate laser equipment without proper safety glasses, interlocks, and key switches. This DAC provides no inherent safety features beyond isolation—system safety depends entirely on your laser's built-in protection mechanisms.