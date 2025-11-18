//! Rust model for ILDA Image Data Transfer Format
//! Based on ILDA specification (see ilda.md)

use serde::{Serialize, Deserialize};
use uuid::Uuid;

// If the X or Y coordinate value is 0x8000 (-32768 in two’s complement), the point is a blanked point and no line is drawn to it
pub const ILDA_BLANK: u16 = 32768;

// Total possible 16-bit coordinate values
pub const ILDA_COORD_RANGE: u32 = 65536;

// Drawable coordinates: –32767 to +32767 (65535 values)
pub const ILDA_MIN_COORD: i16 = -32767;
pub const ILDA_MAX_COORD: i16 = 32767;

/// ILDA Format Codes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IldaHeader {
	/// Format code (0, 1, 2, 4, 5)
	pub format_code: IldaFormatCode,
	/// Frame or palette name (8 bytes, null-terminated)
	pub frame_or_palette_name: String,
	/// Company name (8 bytes, null-terminated)
	pub company_name: String,
	/// Number of records (points or colors) in this section
	pub num_records: u16,
	/// Frame or palette number (sequence index)
	pub frame_or_palette_number: u16,
	/// Total number of frames (or 0 if not specified)
	pub total_frames_or_0: u16,
	/// Projector number (0-255)
	pub projector_number: u8,
}

/// ILDA Point Data (all formats)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum IldaPoint {
	/// Format 0: 3D Indexed Color
	Format0 {
		/// X coordinate (signed 16-bit)
		x: i16,
		/// Y coordinate (signed 16-bit)
		y: i16,
		/// Z coordinate (signed 16-bit)
		z: i16,
		/// Status bitfield (see status module)
		status: u8,
		/// Color index (palette)
		color_index: u8,
	},
	/// Format 1: 2D Indexed Color
	Format1 {
		/// X coordinate (signed 16-bit)
		x: i16,
		/// Y coordinate (signed 16-bit)
		y: i16,
		/// Status bitfield (see status module)
		status: u8,
		/// Color index (palette)
		color_index: u8,
	},
	/// Format 4: 3D True Color
	Format4 {
		/// X coordinate (signed 16-bit)
		x: i16,
		/// Y coordinate (signed 16-bit)
		y: i16,
		/// Z coordinate (signed 16-bit)
		z: i16,
		/// Status bitfield (see status module)
		status: u8,
		/// Blue channel (0-255)
		blue: u8,
		/// Green channel (0-255)
		green: u8,
		/// Red channel (0-255)
		red: u8,
	},
	/// Format 5: 2D True Color
	Format5 {
		/// X coordinate (signed 16-bit)
		x: i16,
		/// Y coordinate (signed 16-bit)
		y: i16,
		/// Status bitfield (see status module)
		status: u8,
		/// Blue channel (0-255)
		blue: u8,
		/// Green channel (0-255)
		green: u8,
		/// Red channel (0-255)
		red: u8,
	},
}

/// ILDA Color Palette Entry (Format 2)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct IldaPaletteColor {
	/// Red channel (0-255)
	pub red: u8,
	/// Green channel (0-255)
	pub green: u8,
	/// Blue channel (0-255)
	pub blue: u8,
}

/// Common named palette colors (1-based index for ILDA default palette)
pub mod palette {
	use super::IldaPaletteColor;
	pub const RED: IldaPaletteColor = IldaPaletteColor { red: 255, green: 0, blue: 0 };
	pub const GREEN: IldaPaletteColor = IldaPaletteColor { red: 0, green: 255, blue: 0 };
	pub const BLUE: IldaPaletteColor = IldaPaletteColor { red: 0, green: 0, blue: 255 };
	pub const YELLOW: IldaPaletteColor = IldaPaletteColor { red: 255, green: 255, blue: 0 };
	pub const CYAN: IldaPaletteColor = IldaPaletteColor { red: 0, green: 255, blue: 255 };
	pub const PURPLE: IldaPaletteColor = IldaPaletteColor { red: 128, green: 0, blue: 128 };
	pub const MAGENTA: IldaPaletteColor = IldaPaletteColor { red: 255, green: 0, blue: 255 };
	pub const WHITE: IldaPaletteColor = IldaPaletteColor { red: 255, green: 255, blue: 255 };
	pub const ORANGE: IldaPaletteColor = IldaPaletteColor { red: 255, green: 128, blue: 0 };
	pub const PINK: IldaPaletteColor = IldaPaletteColor { red: 255, green: 105, blue: 180 };
	pub const GRAY: IldaPaletteColor = IldaPaletteColor { red: 128, green: 128, blue: 128 };
	pub const BLACK: IldaPaletteColor = IldaPaletteColor { red: 0, green: 0, blue: 0 };
	// 1-based index mapping for ILDA default palette (approximate):
	// 1: RED, 2: GREEN, 3: BLUE, 4: YELLOW, 5: CYAN, 6: PURPLE, 7: WHITE
}

/// ILDA Section: Frame or Palette
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IldaSection {
	/// Frame section: contains header and point data
	Frame {
		/// Section header
		header: IldaHeader,
		/// List of points (Format 0, 1, 4, or 5)
		points: Vec<IldaPoint>,
	},
	/// Palette section: contains header and color data
	Palette {
		/// Section header
		header: IldaHeader,
		/// List of palette colors (Format 2)
		colors: Vec<IldaPaletteColor>,
	},
}

/// ILDA File: Sequence of sections (frames and palettes)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IldaFile {
	/// Sequence of ILDA sections (frames and palettes)
	pub sections: Vec<IldaSection>,
}

/// Helper: Status code bitfields
pub mod status {
	pub const NORMAL: u8 = 0;
	pub const BLANKED: u8 = 64;
	pub const LAST_POINT: u8 = 128;
	pub const LAST_POINT_BLANKED: u8 = 192;

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
