//! ILDA protocol model structs for frame sending (formats 0, 1, 4, 5)
//! See: ILDA Image Data Transfer Format Specification (Revision 011)

use std::fmt;

/// ILDA file header (32 bytes)
#[repr(C, packed)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IldaHeader {
    pub identifier: [u8; 4],      // "ILDA"
    pub reserved1: [u8; 3],       // Reserved, set to 0
    pub format_code: u8,          // Format code (0, 1, 2, 4, 5)
    pub frame_name: [u8; 8],      // Frame or color palette name
    pub company_name: [u8; 8],    // Company name
    pub num_records: u16,         // Number of records (big endian)
    pub frame_number: u16,        // Frame or color palette number (big endian)
    pub total_frames: u16,        // Total frames in sequence or 0 (big endian)
    pub projector_number: u8,     // Projector number
    pub reserved2: u8,            // Reserved, set to 0
}

impl Default for IldaHeader {
    fn default() -> Self {
        Self {
            identifier: *b"ILDA",
            reserved1: [0; 3],
            format_code: 0,
            frame_name: [0; 8],
            company_name: [0; 8],
            num_records: 0,
            frame_number: 0,
            total_frames: 0,
            projector_number: 0,
            reserved2: 0,
        }
    }
}

/// Format 0: 3D Coordinates with Indexed Color (8 bytes)
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IldaPoint3DIndexed {
    pub x: i16,           // X coordinate
    pub y: i16,           // Y coordinate
    pub z: i16,           // Z coordinate
    pub status_code: u8,  // Status code (see spec)
    pub color_index: u8,  // Color index
}

/// Format 1: 2D Coordinates with Indexed Color (6 bytes)
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IldaPoint2DIndexed {
    pub x: i16,           // X coordinate
    pub y: i16,           // Y coordinate
    pub status_code: u8,  // Status code (see spec)
    pub color_index: u8,  // Color index
}

/// Format 2: Color Palette Entry (3 bytes)
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IldaColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

/// Format 4: 3D Coordinates with True Color (10 bytes)
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IldaPoint3DTrueColor {
    pub x: i16,
    pub y: i16,
    pub z: i16,
    pub status_code: u8,
    pub b: u8,
    pub g: u8,
    pub r: u8,
}

/// Format 5: 2D Coordinates with True Color (8 bytes)
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IldaPoint2DTrueColor {
    pub x: i16,
    pub y: i16,
    pub status_code: u8,
    pub b: u8,
    pub g: u8,
    pub r: u8,
}

/// Helper for status code bits
pub mod status {
    pub const LAST_POINT: u8 = 0b1000_0000;
    pub const BLANKING: u8 = 0b0100_0000;
}

// Optional: Display for debugging
impl fmt::Display for IldaHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ILDAHeader(format={}, frame='{}', company='{}', records={}, frame_no={}, total={}, proj={})",
            self.format_code,
            String::from_utf8_lossy(&self.frame_name),
            String::from_utf8_lossy(&self.company_name),
            self.num_records,
            self.frame_number,
            self.total_frames,
            self.projector_number
        )
    }
}
