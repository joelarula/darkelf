//! Rust model for ILDA Image Data Transfer Format
//! Based on ILDA specification (see ilda.md)

use std::collections::HashMap;

// If the X or Y coordinate value is 0x8000 (-32768 in two’s complement), the point is a blanked point and no line is drawn to it
pub const ILDA_BLANK: u16 = 32768;


/// ILDA Format Codes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IldaFormatCode {
	Format0_3DIndexed = 0,
	Format1_2DIndexed = 1,
	Format2_Palette = 2,
	Format4_3DTrueColor = 4,
	Format5_2DTrueColor = 5,
}

impl IldaFormatCode {
	pub fn from_u8(val: u8) -> Option<IldaFormatCode> {
		match val {
			0 => Some(IldaFormatCode::Format0_3DIndexed),
			1 => Some(IldaFormatCode::Format1_2DIndexed),
			2 => Some(IldaFormatCode::Format2_Palette),
			4 => Some(IldaFormatCode::Format4_3DTrueColor),
			5 => Some(IldaFormatCode::Format5_2DTrueColor),
			_ => None,
		}
	}
}

/// ILDA Header (32 bytes)
#[derive(Debug, Clone)]
pub struct IldaHeader {
	pub format_code: IldaFormatCode,
	pub frame_or_palette_name: String, // 8 bytes, null-terminated
	pub company_name: String,          // 8 bytes, null-terminated
	pub num_records: u16,
	pub frame_or_palette_number: u16,
	pub total_frames_or_0: u16,
	pub projector_number: u8,
}

/// ILDA Point Data (all formats)
#[derive(Debug, Clone, PartialEq)]
pub enum IldaPoint {
	/// Format 0: 3D Indexed Color
	Format0 {
		x: i16,
		y: i16,
		z: i16,
		status: u8,
		color_index: u8,
	},
	/// Format 1: 2D Indexed Color
	Format1 {
		x: i16,
		y: i16,
		status: u8,
		color_index: u8,
	},
	/// Format 4: 3D True Color
	Format4 {
		x: i16,
		y: i16,
		z: i16,
		status: u8,
		blue: u8,
		green: u8,
		red: u8,
	},
	/// Format 5: 2D True Color
	Format5 {
		x: i16,
		y: i16,
		status: u8,
		blue: u8,
		green: u8,
		red: u8,
	},
}

/// ILDA Color Palette Entry (Format 2)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IldaPaletteColor {
	pub red: u8,
	pub green: u8,
	pub blue: u8,
}

/// ILDA Section: Frame or Palette
#[derive(Debug, Clone)]
pub enum IldaSection {
	Frame {
		header: IldaHeader,
		points: Vec<IldaPoint>,
	},
	Palette {
		header: IldaHeader,
		colors: Vec<IldaPaletteColor>,
	},
}

/// ILDA File: Sequence of sections (frames and palettes)
#[derive(Debug, Clone)]
pub struct IldaFile {
	pub sections: Vec<IldaSection>,
}

/// Helper: Status code bitfields
pub mod status {
	pub const LAST_POINT: u8 = 0b1000_0000;
	pub const BLANKING: u8 = 0b0100_0000;
	// Bits 0-5 are reserved (should be 0)
}

/// Default color palette (64 colors, as per ILDA Appendix A)
pub fn default_palette() -> Vec<IldaPaletteColor> {
	vec![
		IldaPaletteColor { red: 255, green: 0, blue: 0 },
		IldaPaletteColor { red: 255, green: 16, blue: 0 },
		IldaPaletteColor { red: 255, green: 32, blue: 0 },
		IldaPaletteColor { red: 255, green: 48, blue: 0 },
		IldaPaletteColor { red: 255, green: 64, blue: 0 },
		IldaPaletteColor { red: 255, green: 80, blue: 0 },
		IldaPaletteColor { red: 255, green: 96, blue: 0 },
		IldaPaletteColor { red: 255, green: 112, blue: 0 },
		IldaPaletteColor { red: 255, green: 128, blue: 0 },
		IldaPaletteColor { red: 255, green: 144, blue: 0 },
		IldaPaletteColor { red: 255, green: 160, blue: 0 },
		IldaPaletteColor { red: 255, green: 176, blue: 0 },
		IldaPaletteColor { red: 255, green: 192, blue: 0 },
		IldaPaletteColor { red: 255, green: 208, blue: 0 },
		IldaPaletteColor { red: 255, green: 224, blue: 0 },
		IldaPaletteColor { red: 255, green: 240, blue: 0 },
		IldaPaletteColor { red: 255, green: 255, blue: 0 },
		IldaPaletteColor { red: 224, green: 255, blue: 0 },
		IldaPaletteColor { red: 192, green: 255, blue: 0 },
		IldaPaletteColor { red: 160, green: 255, blue: 0 },
		IldaPaletteColor { red: 128, green: 255, blue: 0 },
		IldaPaletteColor { red: 96, green: 255, blue: 0 },
		IldaPaletteColor { red: 64, green: 255, blue: 0 },
		IldaPaletteColor { red: 32, green: 255, blue: 0 },
		IldaPaletteColor { red: 0, green: 255, blue: 0 },
		IldaPaletteColor { red: 0, green: 255, blue: 36 },
		IldaPaletteColor { red: 0, green: 255, blue: 73 },
		IldaPaletteColor { red: 0, green: 255, blue: 109 },
		IldaPaletteColor { red: 0, green: 255, blue: 146 },
		IldaPaletteColor { red: 0, green: 255, blue: 182 },
		IldaPaletteColor { red: 0, green: 255, blue: 219 },
		IldaPaletteColor { red: 0, green: 255, blue: 255 },
		IldaPaletteColor { red: 0, green: 227, blue: 255 },
		IldaPaletteColor { red: 0, green: 198, blue: 255 },
		IldaPaletteColor { red: 0, green: 170, blue: 255 },
		IldaPaletteColor { red: 0, green: 142, blue: 255 },
		IldaPaletteColor { red: 0, green: 113, blue: 255 },
		IldaPaletteColor { red: 0, green: 85, blue: 255 },
		IldaPaletteColor { red: 0, green: 56, blue: 255 },
		IldaPaletteColor { red: 0, green: 28, blue: 255 },
		IldaPaletteColor { red: 0, green: 0, blue: 255 },
		IldaPaletteColor { red: 32, green: 0, blue: 255 },
		IldaPaletteColor { red: 64, green: 0, blue: 255 },
		IldaPaletteColor { red: 96, green: 0, blue: 255 },
		IldaPaletteColor { red: 128, green: 0, blue: 255 },
		IldaPaletteColor { red: 160, green: 0, blue: 255 },
		IldaPaletteColor { red: 192, green: 0, blue: 255 },
		IldaPaletteColor { red: 224, green: 0, blue: 255 },
		IldaPaletteColor { red: 255, green: 0, blue: 255 },
		IldaPaletteColor { red: 255, green: 32, blue: 255 },
		IldaPaletteColor { red: 255, green: 64, blue: 255 },
		IldaPaletteColor { red: 255, green: 96, blue: 255 },
		IldaPaletteColor { red: 255, green: 128, blue: 255 },
		IldaPaletteColor { red: 255, green: 160, blue: 255 },
		IldaPaletteColor { red: 255, green: 192, blue: 255 },
		IldaPaletteColor { red: 255, green: 224, blue: 255 },
		IldaPaletteColor { red: 255, green: 255, blue: 255 },
		IldaPaletteColor { red: 255, green: 224, blue: 224 },
		IldaPaletteColor { red: 255, green: 192, blue: 192 },
		IldaPaletteColor { red: 255, green: 160, blue: 160 },
		IldaPaletteColor { red: 255, green: 128, blue: 128 },
		IldaPaletteColor { red: 255, green: 96, blue: 96 },
		IldaPaletteColor { red: 255, green: 64, blue: 64 },
		IldaPaletteColor { red: 255, green: 32, blue: 32 },
	]
}
