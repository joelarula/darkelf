// Rust implementation of Helios DAC library
// Based on the C++ SDK and C# implementations
// Uses dynamic loading to avoid linking issues

use crate::ilda::model::{IldaPoint, default_palette, status};
use libloading;
use std::os::raw::{c_int, c_uchar, c_uint};
use std::sync::Arc;

// Point structures matching the C++ definitions
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HeliosPoint {
    pub x: u16, // 0 to 0xFFF for original model
    pub y: u16, // 0 to 0xFFF for original model
    pub r: u8,  // 0 to 0xFF
    pub g: u8,  // 0 to 0xFF
    pub b: u8,  // 0 to 0xFF
    pub i: u8,  // Intensity, 0 to 0xFF
}

impl HeliosPoint {
    pub fn new(x: u16, y: u16, r: u8, g: u8, b: u8, i: u8) -> Self {
        Self { x, y, r, g, b, i }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HeliosPointHighRes {
    pub x: u16, // 0 to 0xFFFF
    pub y: u16, // 0 to 0xFFFF
    pub r: u16, // 0 to 0xFFFF
    pub g: u16, // 0 to 0xFFFF
    pub b: u16, // 0 to 0xFFFF
}

impl HeliosPointHighRes {
    pub fn new(x: u16, y: u16, r: u16, g: u16, b: u16) -> Self {
        Self { x, y, r, g, b }
    }
}

// Frame limits
pub const HELIOS_MAX_POINTS: usize = 0xFFF;
pub const HELIOS_MAX_PPS: u32 = 0xFFFF;
pub const HELIOS_MIN_PPS: u32 = 7;

// Flags
pub const HELIOS_FLAGS_START_IMMEDIATELY: u8 = 1 << 0;
pub const HELIOS_FLAGS_SINGLE_MODE: u8 = 1 << 1;
pub const HELIOS_FLAGS_DONT_BLOCK: u8 = 1 << 2;
pub const HELIOS_FLAGS_DEFAULT: u8 = HELIOS_FLAGS_SINGLE_MODE;

// Error codes
pub const HELIOS_SUCCESS: i32 = 1;
pub const HELIOS_ERROR_NOT_INITIALIZED: i32 = -1;
pub const HELIOS_ERROR_INVALID_DEVNUM: i32 = -2;
pub const HELIOS_ERROR_NULL_POINTS: i32 = -3;
pub const HELIOS_ERROR_TOO_MANY_POINTS: i32 = -4;
pub const HELIOS_ERROR_PPS_TOO_HIGH: i32 = -5;
pub const HELIOS_ERROR_PPS_TOO_LOW: i32 = -6;

// Library name for different platforms
#[cfg(windows)]
const LIB_NAME: &str = "HeliosLaserDAC.dll";
#[cfg(target_os = "linux")]
const LIB_NAME: &str = "libHeliosLaserDAC.so";
#[cfg(target_os = "macos")]
const LIB_NAME: &str = "libHeliosLaserDAC.dylib";

// Function type definitions for dynamic loading
type OpenDevicesFn = unsafe extern "C" fn() -> c_int;
type CloseDevicesFn = unsafe extern "C" fn() -> c_int;
type GetStatusFn = unsafe extern "C" fn(c_uint) -> c_int;
type WriteFrameFn =
    unsafe extern "C" fn(c_uint, c_uint, c_uchar, *const HeliosPoint, c_uint) -> c_int;
type WriteFrameHighResFn =
    unsafe extern "C" fn(c_uint, c_uint, c_uchar, *const HeliosPointHighRes, c_uint) -> c_int;
type StopFn = unsafe extern "C" fn(c_uint) -> c_int;
type SetShutterFn = unsafe extern "C" fn(c_uint, c_uchar) -> c_int;
type GetNameFn = unsafe extern "C" fn(c_uint) -> *const i8;

// Internal library handle
struct HeliosLib {
    #[allow(dead_code)]
    lib: libloading::Library,
    open_devices: OpenDevicesFn,
    close_devices: CloseDevicesFn,
    get_status: GetStatusFn,
    write_frame: WriteFrameFn,
    write_frame_high_res: Option<WriteFrameHighResFn>,
    stop: StopFn,
    set_shutter: SetShutterFn,
    get_name: GetNameFn,
}

impl HeliosLib {
    fn load() -> Result<Self, String> {
        unsafe {
            let lib = libloading::Library::new(LIB_NAME)
                .map_err(|e| format!("Failed to load library {}: {}", LIB_NAME, e))?;

            let open_devices = *lib
                .get::<OpenDevicesFn>(b"OpenDevices")
                .map_err(|e| format!("Failed to load OpenDevices: {}", e))?;
            let close_devices = *lib
                .get::<CloseDevicesFn>(b"CloseDevices")
                .map_err(|e| format!("Failed to load CloseDevices: {}", e))?;
            let get_status = *lib
                .get::<GetStatusFn>(b"GetStatus")
                .map_err(|e| format!("Failed to load GetStatus: {}", e))?;
            let write_frame = *lib
                .get::<WriteFrameFn>(b"WriteFrame")
                .map_err(|e| format!("Failed to load WriteFrame: {}", e))?;
            let write_frame_high_res = lib
                .get::<WriteFrameHighResFn>(b"WriteFrameHighResolution")
                .ok()
                .map(|f| *f);
            let stop = *lib
                .get::<StopFn>(b"Stop")
                .map_err(|e| format!("Failed to load Stop: {}", e))?;
            let set_shutter = *lib
                .get::<SetShutterFn>(b"SetShutter")
                .map_err(|e| format!("Failed to load SetShutter: {}", e))?;
            let get_name = *lib
                .get::<GetNameFn>(b"GetName")
                .map_err(|e| format!("Failed to load GetName: {}", e))?;

            Ok(Self {
                lib,
                open_devices,
                close_devices,
                get_status,
                write_frame,
                write_frame_high_res,
                stop,
                set_shutter,
                get_name,
            })
        }
    }
}

/// Helios DAC Controller for Rust
pub struct HeliosDacController {
    num_devices: i32,
    lib: Arc<HeliosLib>,
}

impl HeliosDacController {
    /// Create a new controller instance and load the library
    pub fn new() -> Result<Self, String> {
        let lib = HeliosLib::load()?;
        Ok(Self {
            num_devices: 0,
            lib: Arc::new(lib),
        })
    }

    /// Open and initialize all connected Helios DAC devices
    /// Returns the number of devices found
    pub fn open_devices(&mut self) -> Result<i32, String> {
        unsafe {
            self.num_devices = (self.lib.open_devices)();
            if self.num_devices < 0 {
                Err(format!(
                    "Failed to open devices: error {}",
                    self.num_devices
                ))
            } else {
                Ok(self.num_devices)
            }
        }
    }

    /// Close all Helios DAC devices
    pub fn close_devices(&mut self) -> Result<(), String> {
        unsafe {
            let result = (self.lib.close_devices)();
            if result < 0 {
                Err(format!("Failed to close devices: error {}", result))
            } else {
                self.num_devices = 0;
                Ok(())
            }
        }
    }

    /// Get the number of opened devices
    pub fn num_devices(&self) -> i32 {
        self.num_devices
    }

    /// Get the status of the specified DAC
    /// Returns true if ready to receive new frame, false otherwise
    pub fn get_status(&self, dac_num: u32) -> Result<bool, String> {
        unsafe {
            let status = (self.lib.get_status)(dac_num);
            match status {
                1 => Ok(true),
                0 => Ok(false),
                err => Err(format!("Failed to get status: error {}", err)),
            }
        }
    }

    /// Write a frame to the specified DAC (native HeliosPoint format)
    /// This will block until the transfer is complete (unless HELIOS_FLAGS_DONT_BLOCK is set)
    pub fn write_frame_native(
        &self,
        dac_num: u32,
        pps: u32,
        flags: u8,
        points: &[HeliosPoint],
    ) -> Result<(), String> {
        if points.is_empty() {
            return Err("Points array is empty".to_string());
        }
        if points.len() > HELIOS_MAX_POINTS {
            return Err(format!(
                "Too many points: {} (max is {})",
                points.len(),
                HELIOS_MAX_POINTS
            ));
        }
        if pps > HELIOS_MAX_PPS {
            return Err(format!("PPS too high: {} (max is {})", pps, HELIOS_MAX_PPS));
        }
        if pps < HELIOS_MIN_PPS {
            return Err(format!("PPS too low: {} (min is {})", pps, HELIOS_MIN_PPS));
        }

        unsafe {
            let result = (self.lib.write_frame)(
                dac_num,
                pps,
                flags,
                points.as_ptr(),
                points.len() as c_uint,
            );
            if result < 0 {
                Err(format!("Failed to write frame: error {}", result))
            } else {
                Ok(())
            }
        }
    }

    /// Write a high resolution frame to the specified DAC
    pub fn write_frame_high_res(
        &self,
        dac_num: u32,
        pps: u32,
        flags: u8,
        points: &[HeliosPointHighRes],
    ) -> Result<(), String> {
        // Check if high res function is available
        let write_fn = self
            .lib
            .write_frame_high_res
            .ok_or("WriteFrameHighResolution is not available in this library version")?;

        if points.is_empty() {
            return Err("Points array is empty".to_string());
        }
        if points.len() > HELIOS_MAX_POINTS {
            return Err(format!(
                "Too many points: {} (max is {})",
                points.len(),
                HELIOS_MAX_POINTS
            ));
        }
        if pps > HELIOS_MAX_PPS {
            return Err(format!("PPS too high: {} (max is {})", pps, HELIOS_MAX_PPS));
        }
        if pps < HELIOS_MIN_PPS {
            return Err(format!("PPS too low: {} (min is {})", pps, HELIOS_MIN_PPS));
        }

        unsafe {
            let result = write_fn(dac_num, pps, flags, points.as_ptr(), points.len() as c_uint);
            if result < 0 {
                Err(format!("Failed to write frame: error {}", result))
            } else {
                Ok(())
            }
        }
    }

    /// Write an ILDA frame (Vec<IldaPoint>) to the specified DAC
    /// Automatically handles coordinate conversion and blanking
    /// `shift` controls the scaling:
    /// - 4: Standard 1:1 scaling (applies /16 divisor to fit 16-bit ILDA into 12-bit DAC)
    /// - 3: 2x Zoom
    /// - 2: 4x Zoom
    /// - 1: 8x Zoom
    /// - 0: 16x Zoom (Raw / Pixel-Perfect 1:1 mapping of ILDA units to DAC output)
    pub fn write_frame(
        &self,
        dac_num: u32,
        pps: u32,
        flags: u8,
        points: &[IldaPoint],
        shift: u8,
    ) -> Result<(), String> {
        if points.is_empty() {
            return Err("Points array is empty".to_string());
        }

        // Optimized Branching: Check capability ONCE and convert directly to the required format.
        if let Some(write_fn) = self.lib.write_frame_high_res {
            // High Resolution Path
            // Upscale 8-bit ILDA colors to 16-bit for Helios HighRes
            let high_res_points: Vec<HeliosPointHighRes> = points
                .iter()
                .map(|p| ilda_point_to_helios_high_res(p, shift))
                .collect();

            unsafe {
                let result = write_fn(
                    dac_num,
                    pps,
                    flags,
                    high_res_points.as_ptr(),
                    high_res_points.len() as std::ffi::c_uint,
                );
                if result < 0 {
                    Err(format!("Failed to write high-res frame: error {}", result))
                } else {
                    Ok(())
                }
            }
        } else {
            // Low Resolution (Native) Path
            // Standard 8-bit conversion
            let helios_points: Vec<HeliosPoint> = points
                .iter()
                .map(|p| ilda_point_to_helios(p, shift))
                .collect();

            self.write_frame_native(dac_num, pps, flags, &helios_points)
        }
    }

    /// Stop output on the specified DAC
    pub fn stop(&self, dac_num: u32) -> Result<(), String> {
        unsafe {
            let result = (self.lib.stop)(dac_num);
            if result < 0 {
                Err(format!("Failed to stop DAC: error {}", result))
            } else {
                Ok(())
            }
        }
    }

    /// Set shutter level for the specified DAC
    /// level: 0 = closed, 1 = open
    pub fn set_shutter(&self, dac_num: u32, level: bool) -> Result<(), String> {
        unsafe {
            let result = (self.lib.set_shutter)(dac_num, if level { 1 } else { 0 });
            if result < 0 {
                Err(format!("Failed to set shutter: error {}", result))
            } else {
                Ok(())
            }
        }
    }

    /// Get the name of the specified DAC
    pub fn get_name(&self, dac_num: u32) -> Result<String, String> {
        unsafe {
            let name_ptr = (self.lib.get_name)(dac_num);
            if name_ptr.is_null() {
                Err("Failed to get device name".to_string())
            } else {
                let c_str = std::ffi::CStr::from_ptr(name_ptr);
                Ok(c_str.to_string_lossy().into_owned())
            }
        }
    }

    /// Wait for the DAC to be ready to receive a new frame
    /// max_attempts: maximum number of status checks before giving up (0 = infinite)
    /// Returns true if ready, false if timed out
    pub fn wait_for_ready(&self, dac_num: u32, max_attempts: u32) -> Result<bool, String> {
        let mut attempts = 0;
        loop {
            match self.get_status(dac_num) {
                Ok(true) => return Ok(true),
                Ok(false) => {
                    attempts += 1;
                    if max_attempts > 0 && attempts >= max_attempts {
                        return Ok(false);
                    }
                    std::thread::yield_now();
                }
                Err(e) => return Err(e),
            }
        }
    }
}

impl Drop for HeliosDacController {
    fn drop(&mut self) {
        if self.num_devices > 0 {
            let _ = self.close_devices();
        }
    }
}

/// Helper function to convert an IldaPoint to a HeliosPointHighRes
fn ilda_point_to_helios_high_res(p: &IldaPoint, shift: u8) -> HeliosPointHighRes {
    let (x_ilda, y_ilda, _z_ilda, status_byte, r, g, b) = match p {
        IldaPoint::Format0 {
            x,
            y,
            z,
            status,
            color_index,
        } => {
            let palette = default_palette();
            let color = palette
                .get(*color_index as usize)
                .copied()
                .unwrap_or(palette[0]);
            (*x, *y, *z, *status, color.red, color.green, color.blue)
        }
        IldaPoint::Format1 {
            x,
            y,
            status,
            color_index,
        } => {
            let palette = default_palette();
            let color = palette
                .get(*color_index as usize)
                .copied()
                .unwrap_or(palette[0]);
            (*x, *y, 0, *status, color.red, color.green, color.blue)
        }
        IldaPoint::Format4 {
            x,
            y,
            z,
            status,
            blue,
            green,
            red,
        } => (*x, *y, *z, *status, *red, *green, *blue),
        IldaPoint::Format5 {
            x,
            y,
            status,
            blue,
            green,
            red,
        } => (*x, *y, 0, *status, *red, *green, *blue),
    };

    // Check for blanking
    let is_blanked = (status_byte & status::BLANKED) != 0;

    // Convert coordinates
    let offset: i32 = 2048 << shift;
    let x_helios = (((x_ilda as i32 + offset) >> shift).max(0).min(4095)) as u16;
    let y_helios = (((y_ilda as i32 + offset) >> shift).max(0).min(4095)) as u16;

    if is_blanked {
        HeliosPointHighRes::new(x_helios, y_helios, 0, 0, 0)
    } else {
        // Upscale 8-bit color to 16-bit: (val * 257) maps 0->0, 255->65535, 128->32896
        let r16 = (r as u16) * 257;
        let g16 = (g as u16) * 257;
        let b16 = (b as u16) * 257;
        HeliosPointHighRes::new(x_helios, y_helios, r16, g16, b16)
    }
}

/// Helper function to convert an IldaPoint to a HeliosPoint
fn ilda_point_to_helios(p: &IldaPoint, shift: u8) -> HeliosPoint {
    let (x_ilda, y_ilda, z_ilda, status_byte, r, g, b) = match p {
        IldaPoint::Format0 {
            x,
            y,
            z,
            status,
            color_index,
        } => {
            let palette = default_palette();
            let color = palette
                .get(*color_index as usize)
                .copied()
                .unwrap_or(palette[0]);
            (*x, *y, *z, *status, color.red, color.green, color.blue)
        }
        IldaPoint::Format1 {
            x,
            y,
            status,
            color_index,
        } => {
            let palette = default_palette();
            let color = palette
                .get(*color_index as usize)
                .copied()
                .unwrap_or(palette[0]);
            (*x, *y, 0, *status, color.red, color.green, color.blue)
        }
        IldaPoint::Format4 {
            x,
            y,
            z,
            status,
            blue,
            green,
            red,
        } => (*x, *y, *z, *status, *red, *green, *blue),
        IldaPoint::Format5 {
            x,
            y,
            status,
            blue,
            green,
            red,
        } => (*x, *y, 0, *status, *red, *green, *blue),
    };

    // Check for blanking
    let is_blanked = (status_byte & status::BLANKED) != 0;

    // Convert coordinates
    // ILDA: signed 16-bit (-32768 to 32767)
    // Helios: unsigned 12-bit (0 to 4095)
    // Old Formula: (val + 32768) >> 4 (Fixed Shift 4)
    // New Dynamic Formula: (val + offset) >> shift
    // where offset = 2048 << shift

    // Calculate scaling parameters
    // Center point for 12-bit is 2048. We want 0 ILDA to map to 2048 Helios.
    // The divisor is 2^shift.
    // To map 0 (ILDA) -> 2048 (Helios), we need to add an offset before shifting.
    // (0 + offset) >> shift = 2048  => offset = 2048 << shift
    let offset: i32 = 2048 << shift;

    let x_helios = (((x_ilda as i32 + offset) >> shift).max(0).min(4095)) as u16;
    let y_helios = (((y_ilda as i32 + offset) >> shift).max(0).min(4095)) as u16;

    if is_blanked {
        HeliosPoint::new(x_helios, y_helios, 0, 0, 0, 0)
    } else {
        // Intensity is max (255) if not blanked for now.
        // Some ILDA formats don't have explicit intensity, derived from color.
        HeliosPoint::new(x_helios, y_helios, r, g, b, 255)
    }
}
