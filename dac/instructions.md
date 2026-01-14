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

Begin by establishing the isolated power supply. Connect the Pico's ground (e.g., Pin 38) to pin 1 (-Vin) of the RECOM RS-0512D module. Connect the Pico's VBUS pin (Pin 40, 5V from USB) to pin 2 (+Vin). Pin 3 (CTRL) should be left open (high impedance) to keep the converter ON. Pin 5 (NC) is not connected internally.

On the output side, the RECOM generates ±12V and a common reference. Connect the +12V output (pin 6) to pin 4 of the TL074 op-amp for positive rail power. Connect the -12V output (pin 8) to pin 11 of the TL074 for negative rail power. The common pin (pin 7, 0V reference) connects to Island 2's ground rail, which will eventually connect to ILDA pin 25.

### Creating the Isolated 5V Rail (L7805 Guide)

The RECOM module produces raw ±12V. While the ±12V powers your op-amps, the digital chips (DACs and ADuM) require a clean 5V supply. The L7805 regulator acts as a "power plant" on the isolated side to safely step down the +12V.

#### 1. The Power Chain Visualization (Isolated Side)
- **RECOM (+12V Pin)** $\rightarrow$ **L7805 (Input Pin)**
- **L7805 (Output Pin)** $\rightarrow$ **Clean 5V Rail** (Powers DACs and ADuM Side B)
- **L7805 (Ground Pin)** $\rightarrow$ **Isolated Ground** (Common/0V)

#### 2. Step-by-Step Connection Guide
**Step A: The Regulator (L7805)**
Look at the L7805 from the front (the side with the writing):
- **Left Pin (Input):** Connect to RECOM Pin 6 (+12V).
- **Middle Pin (Ground):** Connect to RECOM Pin 7 (Common/0V).
- **Right Pin (Output):** This is now your Clean **Isolated 5V Rail**.

**Step B: Distributing the Clean 5V**
Connect the Output Pin of the L7805 to:
- **ADuM1401:** Pin 16 ($V_{DD2}$) on both chips.
- **MCP4922 (All 3):** Pin 1 ($V_{DD}$).
- **MCP4922 (All 3):** Pin 11 ($V_{REFA}$) and Pin 13 ($V_{REFB}$) — This sets the 5V reference for full-scale output.
- **MCP4922 (All 3):** Pin 9 ($SHDN$) — Connect to 5V to keep DAC active.

#### 3. The Isolated Ground Return Path
For the circuit to work, all isolated components must share the same return path to the RECOM Common pin:
- **L7805:** Middle Pin.
- **ADuM1401:** Pin 9 and Pin 15 ($GND_2$).
- **MCP4922 (All 3):** Pin 12 ($V_{SS}$).
- **MCP4922 (All 3):** Pin 8 ($LDAC$) — *Note: Connecting LDAC to Ground enables immediate output updates.*

> [!CAUTION]
> **Isolation Barrier:** Never connect any part of this "Isolated 5V Rail" or "Isolated Ground" to the Pico's VBUS or Ground. Bridging these will destroy the isolation and put your computer at risk.

#### Power Distribution Checklist

| Component | Pin to +5V (Isolated) | Pin to Ground (Isolated) |
| :--- | :--- | :--- |
| **ADuM1401** | Pin 16 | Pins 9 & 15 |
| **MCP4922** | Pins 1, 11, 13, and 9 ($SHDN$) | Pin 12 and 8 ($LDAC$) |

Place decoupling capacitors (0.1µF) close to each IC's power pins to filter noise and prevent oscillation.

## 3. The Full Bridge Configuration

This setup is the "Full Bridge" for your laser projector. It uses two isolators to manage three DACs: one for movement (X/Y), one for primary colors (Red/Green), and one for Blue and Intensity.

The key to this setup is the **SPI Bus**: The Clock (SCK) and Data (SDI) are shared by everyone, but each DAC has its own "Private Line" (Chip Select) so the Pico can talk to them individually.

### Part 1: The Pico "Control Center"
The Pico sends all signals through the first 8 pins (Side A) of your two ADuM chips.

| Signal Name | Pico GPIO | Pico Physical Pin | Destination (Side A) |
| :--- | :--- | :--- | :--- |
| **SCK (Clock)** | GP18 | Pin 24 | ADuM #1, Pin 4 (VIB) |
| **SDI (Data)** | GP19 | Pin 25 | ADuM #1, Pin 5 (VIC) |
| **CS1 (X/Y)** | GP16 | Pin 21 | ADuM #1, Pin 3 (VIA) |
| **CS2 (Red/Grn)**| GP17 | Pin 22 | ADuM #2, Pin 3 (VIA) |
| **CS3 (Blu/Int)**| GP20 | Pin 26 | ADuM #2, Pin 4 (VIB) |
| **SHUTTER** | GP21 | Pin 27 | ADuM #2, Pin 5 (VIC) |

---

### Part 2: The Isolation Bridges (Side B)
This is where the signals emerge on the "Analog Island." You must jump the Clock and Data from ADuM1 so that ADuM2 doesn't have to carry them.

#### ADuM1 (The Main Bus & X/Y)
- **Pin 14 (VOA):** Connects **ONLY** to DAC 1, Pin 3 (X/Y Select).
- **Pin 13 (VOB):** The Master Clock. Connect to Pin 4 of **ALL THREE DACs**.
- **Pin 12 (VOC):** The Master Data. Connect to Pin 5 of **ALL THREE DACs**.

#### ADuM2 (Color & Safety Selects)
- **Pin 14 (VOA):** Connects **ONLY** to DAC 2, Pin 3 (Red/Green Select).
- **Pin 13 (VOB):** Connects **ONLY** to DAC 3, Pin 3 (Blue/Inten Select).
- **Pin 12 (VOC):** Connects to your Safety Relay or Shutter circuit.

---

### Part 3: The Three DACs (MCP4922)
All three DACs share the same power and data bus, but listen for different Chip Select (CS) signals.

| DAC Pin | Name | Connection Logic |
| :--- | :--- | :--- |
| **Pin 1** | $V_{DD}$ | All to Isolated 5V Rail |
| **Pin 3** | **CS** | DAC1: ADuM1-P14 \| DAC2: ADuM2-P14 \| DAC3: ADuM2-P13 |
| **Pin 4** | **SCK** | All to **ADuM1 Pin 13** (Shared Clock) |
| **Pin 5** | **SDI** | All to **ADuM1 Pin 12** (Shared Data) |
| **Pin 8** | $LDAC$ | All to Isolated Ground (Immediate Update) |
| **Pin 9** | $SHDN$ | All to Isolated 5V (Keeps them awake) |
| **Pin 11 & 13**| $V_{REF}$ | All to Isolated 5V (Sets 0–5V output range) |
| **Pin 12** | $V_{SS}$ | All to Isolated Ground |

---

### Part 4: The Final Output (To Laser)
To move a professional laser galvo, we need to convert the DAC's single-ended 0V to 5V signal into a differential -5V to +5V signal. This is called **"level shifting and scaling."**

#### 1. The ILDA Standard: Differential Signaling
The ILDA protocol uses balanced pairs for the X and Y axes (X+/X− and Y+/Y−). This is done to cancel out noise over long cable runs (e.g., a 50ft cable from your desk to the stage).

*   **Single-Ended (Simple Setup):** You send X+ (signal) and connect X− to Ground. Range: ±5V (10V total swing).
*   **Differential (Full ILDA):** You send X+ (e.g., +5V) and X− (e.g., −5V). The projector sees the difference between them. Range: ±10V (20V total swing).

#### 2. How many Op-Amps do you need?
Since you have a TL074, you have four independent amplifiers in one chip. Here is how you should use them for a "Full ILDA" setup:

**Option A: The "Minimum" Setup (2 Op-Amps used)**
You use one Op-Amp for X and one for Y.
- **X+ (Pin 1):** From Op-Amp 1.
- **X- (DB25 Pin 14):** Tied to Analog Ground (COM).
- *Result:* Your laser will work, but the image will be half-size, and you might get "noise" lines in the projection.

**Option B: The "Pro" Setup (All 4 Op-Amps used)**
You use two Op-Amps per axis to create a true balanced signal.
- **Op-Amp 1:** Level shifts DAC-X to X+ (±5V).
- **Op-Amp 2:** Inverts X+ to create X− (when X+ is +5V, X− is −5V).
- **Op-Amp 3:** Level shifts DAC-Y to Y+ (±5V).
- **Op-Amp 4:** Inverts Y+ to create Y− (when Y+ is +5V, Y− is −5V).

#### 3. What about the Colors (RGB)?
ILDA colors (Red, Green, Blue) are not differential. They are "Single-Ended" 0V to +5V.
- **Do they need an Op-Amp?** Not necessarily. If your DAC outputs 0V to 5V, you can connect them directly to the DB25 connector.
- **Safety Tip:** Using an Op-Amp as a "Buffer" is safer. It protects your expensive DAC from a short circuit in the laser cable.

#### 4. Summary Table for DB25 (J1)
Use your TL074 to drive the X and Y differential pairs like this for professional performance:

| Signal | Component | Output Pin | DB25 Pin |
| :--- | :--- | :--- | :--- |
| **X+** | U4 (Amp 1) | Pin 1 | 1 |
| **X-** | U4 (Amp 2) | Pin 7 | 14 |
| **Y+** | U4 (Amp 3) | Pin 8 | 2 |
| **Y-** | U4 (Amp 4) | Pin 14 | 15 |
| **Red** | DAC 2 (U2) | VoutA | 3 |
| **Green** | DAC 2 (U2) | VoutB | 4 |
| **Blue** | DAC 3 (U7) | VoutA | 5 |

#### 5. Powering the Op-Amp (U4)
Since the Op-Amp needs to "swing" below zero to hit $-5V$, it must have a negative power supply:
- **Pin 4 ($V_{CC}+$):** Connect to RECOM Pin 6 (+12V).
- **Pin 11 ($V_{CC}-$):** Connect to RECOM Pin 8 (-12V).
- **Bypass Caps:** Place a $100\text{nF}$ capacitor between Pin 4 and Ground, and another between Pin 11 and Ground to prevent high-frequency noise.

---

### Logical Checklist for Success
Before powering on, verify the following:

- [ ] **Shared Ground:** Are Pins 15 and 9 of both ADuMs connected to the same ground as Pin 12 of all three DACs? (Yes, the Isolated Ground).
- [ ] **Shared Power:** Are Pin 16 of both ADuMs and Pin 1 of all DACs connected to the same 5V regulator? (Yes, the L7805 Output).
- [ ] **The Bus:** Does a single wire from ADuM1 Pin 13 reach Pin 4 of all three DACs? (Yes, this is the SPI Clock).
- [ ] **The Interlock:** Are DB-25 pins 4 and 17 shorted together with a jumper wire?
- [ ] **X-Channel:** Connect DAC U1 Pin 14 to Op-Amp U4 Pin 3 (via resistor).
- [ ] **X-Channel:** Wire the feedback loop (Output Pin 1 back to Negative Input Pin 2).
- [ ] **X-Channel:** Ensure the DB25 connector Pin 1 is receiving the signal from the Op-Amp output.

## 3. Software Strategy: Achieving 16kpps with PIO

The challenge with standard SPI libraries is speed. To reach 16,000 points per second (16kpps), we need to transmit six 16-bit words (X, Y, Red, Green, Blue, Intensity) in just 62.5 microseconds. This translates to a sustained data rate of approximately 1.5 Mbps with minimal inter-word gaps.

Traditional bit-banging approaches that toggle GPIO pins in software loops can't maintain this pace reliably. Context switches, interrupt latency, and instruction timing variations introduce jitter that degrades scan quality. Even dedicated SPI peripheral hardware may not provide the precise timing control needed for clean laser projection.

The Raspberry Pi Pico's Programmable I/O (PIO) subsystem solves this elegantly. PIO is a set of independent state machines that run alongside the main ARM core, executing simple assembly-like programs at fixed clock rates. Once configured, a PIO state machine can generate perfectly timed SPI transactions without any CPU intervention.

The implementation strategy involves writing a PIO program that bit-bangs the SPI protocol with exact timing. The program takes 16-bit words from a FIFO buffer and serially clocks them out while managing the chip select line. The main firmware running on the ARM core simply keeps the FIFO filled with scan data—the PIO handles all the real-time signaling.

This architecture provides several critical advantages. First, it guarantees deterministic timing regardless of what the main CPU is doing. Second, it frees the CPU to perform higher-level tasks like calculating scan patterns, handling USB commands, or implementing safety features. Third, it achieves the sustained throughput needed for smooth 16kpps operation without dropping frames or introducing visible artifacts.

The PIO clock can be configured to run at precise multiples of the desired bit rate, ensuring clean edges on the SPI signals even at high speeds. With careful tuning, this approach can push well beyond 16kpps if needed, potentially reaching 30-40kpps for high-speed scanning applications.

---

**Safety Note:** Always verify your laser system's interlock requirements before connecting this DAC. Never operate laser equipment without proper safety glasses, interlocks, and key switches. This DAC provides no inherent safety features beyond isolation—system safety depends entirely on your laser's built-in protection mechanisms.